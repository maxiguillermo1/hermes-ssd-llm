use crate::errors::{HermesSsdLlmError, Result};
use crate::paths::SsdPaths;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub struct SessionLock {
    path: PathBuf,
    held: bool,
}

impl SessionLock {
    pub fn acquire(paths: &SsdPaths) -> Result<Self> {
        let path = paths.root.join("runtime/locks/hermes-ssd-llm.session.lock");
        fs::create_dir_all(path.parent().unwrap()).map_err(|e| {
            HermesSsdLlmError::DirectoryInitFailed {
                path: path.display().to_string(),
                reason: e.to_string(),
            }
        })?;

        if path.exists() {
            if let Ok(contents) = fs::read_to_string(&path) {
                if let Some(pid) = parse_pid(&contents) {
                    if process_alive(pid) {
                        return Err(HermesSsdLlmError::LockConflict { pid });
                    }
                }
            }
            fs::remove_file(&path).ok();
        }

        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&path)
            .map_err(|_| HermesSsdLlmError::LockConflict { pid: 0 })?;
        writeln!(file, "pid={}\nstarted={}", std::process::id(), now_iso()).ok();
        Ok(Self { path, held: true })
    }

    pub fn mark_unclean(paths: &SsdPaths) -> Result<()> {
        let flag = paths.runtime_state.join("unclean_shutdown");
        fs::write(flag, now_iso()).map_err(|e| HermesSsdLlmError::Other(e.to_string()))
    }

    pub fn clear_unclean(paths: &SsdPaths) {
        let flag = paths.runtime_state.join("unclean_shutdown");
        let _ = fs::remove_file(flag);
    }

    pub fn was_unclean(paths: &SsdPaths) -> bool {
        paths.runtime_state.join("unclean_shutdown").exists()
    }
}

impl Drop for SessionLock {
    fn drop(&mut self) {
        if self.held {
            let _ = fs::remove_file(&self.path);
        }
    }
}

fn parse_pid(contents: &str) -> Option<u32> {
    for line in contents.lines() {
        if let Some(rest) = line.strip_prefix("pid=") {
            return rest.trim().parse().ok();
        }
    }
    None
}

fn process_alive(pid: u32) -> bool {
    if pid == 0 {
        return false;
    }
    #[cfg(unix)]
    {
        use std::process::Command;
        Command::new("kill")
            .args(["-0", &pid.to_string()])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
    #[cfg(not(unix))]
    {
        false
    }
}

fn now_iso() -> String {
    chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::SsdPaths;
    use tempfile::TempDir;

    #[test]
    fn stale_lock_removed() {
        let tmp = TempDir::new().unwrap();
        let paths = SsdPaths::from_mount(tmp.path());
        fs::create_dir_all(paths.root.join("runtime/locks")).unwrap();
        fs::write(
            paths.root.join("runtime/locks/hermes-ssd-llm.session.lock"),
            "pid=999999999\n",
        )
        .unwrap();
        let lock = SessionLock::acquire(&paths).unwrap();
        drop(lock);
    }
}
