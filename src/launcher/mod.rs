use crate::config::HermesSsdLlmConfig;
use crate::environment::RoutedEnvironment;
use crate::errors::{HermesSsdLlmError, Result};
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::Command;

const INSTALL_STATE: &str = "install-state.json";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InstallState {
    pub real_hermes_path: String,
    pub wrapper_path: String,
    pub installed_at: String,
}

pub fn resolve_real_hermes(cfg: &HermesSsdLlmConfig) -> Result<PathBuf> {
    if let Some(path) = &cfg.hermes_executable {
        let p = PathBuf::from(path);
        if p.is_file() && !is_hermes_ssd_wrapper(&p) {
            return Ok(p);
        }
    }

    if let Ok(state) = load_install_state() {
        let p = PathBuf::from(&state.real_hermes_path);
        if p.is_file() && !is_hermes_ssd_wrapper(&p) {
            return Ok(p);
        }
    }

    for candidate in candidate_hermes_paths() {
        if candidate.is_file() && !is_hermes_ssd_wrapper(&candidate) {
            return Ok(candidate);
        }
    }

    Err(HermesSsdLlmError::HermesMissing)
}

pub fn candidate_hermes_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(home) = std::env::var("HOME") {
        paths.push(PathBuf::from(&home).join(".local/bin/hermes-real"));
        paths.push(
            PathBuf::from(&home).join("Desktop/Hermes/hermes-agent/.venv/bin/hermes"),
        );
        paths.push(PathBuf::from(&home).join(".local/bin/hermes.real"));
    }
    paths.push(PathBuf::from("/opt/homebrew/bin/hermes"));
    paths.push(PathBuf::from("/usr/local/bin/hermes"));
    paths
}

/// True when `path` is our Rust dispatcher, not upstream Hermes Agent.
pub fn is_hermes_ssd_wrapper(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }
    if let Ok(meta) = std::fs::metadata(path) {
        if meta.len() > 500_000 {
            return true;
        }
    }
    if let Ok(contents) = std::fs::read_to_string(path) {
        return contents.contains("hermes_ssd_llm");
    }
    false
}

pub fn install_state_path() -> PathBuf {
    HermesSsdLlmConfig::config_dir().join(INSTALL_STATE)
}

pub fn load_install_state() -> Result<InstallState> {
    let raw =
        std::fs::read_to_string(install_state_path()).map_err(|_| HermesSsdLlmError::HermesMissing)?;
    serde_json::from_str(&raw).map_err(|e| HermesSsdLlmError::Other(e.to_string()))
}

pub fn save_install_state(state: &InstallState) -> Result<()> {
    let path = install_state_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| HermesSsdLlmError::Other(e.to_string()))?;
    }
    let json =
        serde_json::to_string_pretty(state).map_err(|e| HermesSsdLlmError::Other(e.to_string()))?;
    std::fs::write(path, json).map_err(|e| HermesSsdLlmError::Other(e.to_string()))
}

pub fn exec_hermes(real: &Path, args: &[String], env: &RoutedEnvironment) -> Result<()> {
    env.apply_to_process();
    exec_hermes_raw(real, args)
}

/// Exec Hermes without modifying the environment (normal `hermes` pass-through).
pub fn exec_hermes_passthrough(real: &Path, args: &[String]) -> Result<()> {
    exec_hermes_raw(real, args)
}

fn exec_hermes_raw(real: &Path, args: &[String]) -> Result<()> {
    let c_path = CString::new(real.as_os_str().as_bytes())
        .map_err(|_| HermesSsdLlmError::Other("invalid hermes path".into()))?;
    let c_args: Result<Vec<CString>> = args
        .iter()
        .map(|a| {
            CString::new(a.as_bytes()).map_err(|_| HermesSsdLlmError::Other("invalid argument".into()))
        })
        .collect();
    let c_args = c_args?;
    let mut argv: Vec<*const libc::c_char> = Vec::with_capacity(c_args.len() + 2);
    argv.push(c_path.as_ptr());
    for arg in &c_args {
        argv.push(arg.as_ptr());
    }
    argv.push(std::ptr::null());

    let rc = unsafe { libc::execv(c_path.as_ptr(), argv.as_ptr()) };
    if rc == -1 {
        let err = std::io::Error::last_os_error();
        return Err(HermesSsdLlmError::Other(format!("exec failed: {err}")));
    }
    Ok(())
}

pub fn run_hermes_subprocess(real: &Path, args: &[String], env: &RoutedEnvironment) -> Result<i32> {
    let mut cmd = Command::new(real);
    cmd.args(args);
    for (k, v) in &env.vars {
        cmd.env(k, v);
    }
    let status = cmd
        .status()
        .map_err(|e| HermesSsdLlmError::Other(e.to_string()))?;
    Ok(status.code().unwrap_or(1))
}

pub fn hermes_version(real: &Path) -> Option<String> {
    let mut child = Command::new(real);
    child.arg("--version").stdin(std::process::Stdio::null());
    let output = child.output().ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}
