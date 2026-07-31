//! Seed SSD `HERMES_HOME` from the user's normal `~/.hermes` on first launch.

use crate::errors::{HermesSsdLlmError, Result};
use crate::APP_NAME;
use std::fs;
use std::path::{Path, PathBuf};

/// Files copied from the default Hermes home when missing on the SSD home.
const BOOTSTRAP_FILES: &[&str] = &["config.yaml", ".env", "ENGINEERING-CONSTITUTION.md"];

pub fn default_hermes_home() -> PathBuf {
    dirs::home_dir()
        .map(|home| home.join(".hermes"))
        .unwrap_or_else(|| PathBuf::from(".hermes"))
}

/// Copy essential Hermes config from `~/.hermes` into the SSD home when absent.
pub fn bootstrap_hermes_home(ssd_home: &Path) -> Result<()> {
    fs::create_dir_all(ssd_home).map_err(|e| HermesSsdLlmError::DirectoryInitFailed {
        path: ssd_home.display().to_string(),
        reason: e.to_string(),
    })?;

    let source = default_hermes_home();
    if source == ssd_home {
        return Ok(());
    }

    for name in BOOTSTRAP_FILES {
        let dest = ssd_home.join(name);
        if dest.exists() {
            continue;
        }
        let src = source.join(name);
        if !src.is_file() {
            continue;
        }
        fs::copy(&src, &dest).map_err(|e| HermesSsdLlmError::DirectoryInitFailed {
            path: dest.display().to_string(),
            reason: format!("failed to seed {name} from {}: {e}", source.display()),
        })?;
        eprintln!(
            "{APP_NAME}: seeded {name} from {}",
            source.display()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn skips_when_dest_already_exists() {
        let ssd = TempDir::new().unwrap();
        let dest = ssd.path().join("config.yaml");
        fs::write(&dest, "existing").unwrap();
        bootstrap_hermes_home(ssd.path()).unwrap();
        assert_eq!(fs::read_to_string(dest).unwrap(), "existing");
    }
}
