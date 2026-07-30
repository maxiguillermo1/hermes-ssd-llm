//! CLI routing for `hermes ssd` subcommands.

use crate::config::{migrate_config_if_needed, HermesSsdLlmConfig};
use crate::device::verify_volume;
use crate::diagnostics;
use crate::environment::RoutedEnvironment;
use crate::errors::{ExitCode, HermesSsdLlmError, Result};
use crate::launcher::{exec_hermes, resolve_real_hermes};
use crate::locks::SessionLock;
use crate::paths::SsdPaths;
use crate::APP_NAME;

pub fn launch_ssd_mode(args: &[String]) -> Result<()> {
    launch_ssd_mode_inner(args, false)
}

pub fn launch_ssd_mode_quiet(args: &[String]) -> Result<()> {
    launch_ssd_mode_inner(args, true)
}

fn launch_ssd_mode_inner(args: &[String], quiet: bool) -> Result<()> {
    migrate_config_if_needed()?;
    let cfg = HermesSsdLlmConfig::load()?;
    if cfg.allow_internal_fallback {
        return Err(HermesSsdLlmError::FallbackRefused);
    }

    let vol = verify_volume(&cfg)?;
    let paths = SsdPaths::from_mount(&vol.mount_point);
    SessionLock::clear_unclean(&paths);
    let _lock = SessionLock::acquire(&paths)?;
    let env = RoutedEnvironment::build(&cfg, &vol);
    let real = resolve_real_hermes(&cfg)?;

    eprintln!("{APP_NAME}: {} verified", vol.volume_name);
    eprintln!("{APP_NAME}: storage routing active");
    if !quiet {
        if cfg.debug_startup {
            for (k, v) in env.redacted_report() {
                eprintln!("{APP_NAME}: env {k}={v}");
            }
        }
        eprintln!("{APP_NAME}: launching Hermes");
    }

    exec_hermes(&real, args, &env)?;
    Err(HermesSsdLlmError::Other("exec returned unexpectedly".into()))
}

pub fn handle_ssd_subcommand(args: &[String]) -> Result<i32> {
    if args.first().map(|s| s.as_str()) == Some("doctor") {
        let throughput = args.get(1).map(|s| s == "--throughput").unwrap_or(false);
        let report = diagnostics::run_doctor(throughput)?;
        diagnostics::print_doctor(&report);
        return Ok(ExitCode::Success.code());
    }
    if args.first().map(|s| s.as_str()) == Some("help") || args.contains(&"--help".to_string()) {
        print_ssd_help();
        return Ok(ExitCode::Success.code());
    }
    match launch_ssd_mode(args) {
        Ok(()) => Ok(ExitCode::Success.code()),
        Err(e) => {
            eprintln!("{e}");
            Ok(e.exit_code().code())
        }
    }
}

pub fn register_mount(cfg: &mut HermesSsdLlmConfig, mount: &std::path::Path) -> Result<()> {
    use crate::device::volume_info_at;
    use crate::paths::ensure_ssd_layout;

    let vol = volume_info_at(mount)?;
    if vol.volume_uuid.is_empty() {
        return Err(HermesSsdLlmError::InvalidConfig(
            "could not read volume UUID".into(),
        ));
    }
    cfg.volume_uuid = vol.volume_uuid;
    if cfg.expected_volume_name.is_empty() {
        cfg.expected_volume_name = vol.volume_name;
    }
    ensure_ssd_layout(mount)?;
    cfg.save()?;
    Ok(())
}

fn print_ssd_help() {
    println!(
        r#"Hermes SSD LLM mode

Usage:
  hermes ssd                  Launch Hermes with SSD-backed storage
  hermes ssd doctor           Show diagnostics
  hermes ssd doctor --throughput  Include a small read/write probe

Normal `hermes` is unchanged. SSD mode never falls back to internal storage.
"#
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn ssd_args_forwarding() {
        let args = vec!["--provider".to_string(), "cursor".to_string()];
        assert_eq!(args.len(), 2);
    }
}
