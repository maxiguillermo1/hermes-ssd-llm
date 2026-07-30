use crate::errors::{HermesSsdLlmError, Result};
use crate::SSD_ROOT_DIR;
use std::fs;
use std::path::{Path, PathBuf};

const SUBDIRS: &[&str] = &[
    "bin",
    "config",
    "data",
    "models/gguf",
    "models/draft",
    "models/vision",
    "models/adapters",
    "models/downloads",
    "cache/hermes",
    "cache/huggingface",
    "cache/transformers",
    "cache/rust",
    "cache/build",
    "cache/inference",
    "runtime/locks",
    "runtime/sessions",
    "runtime/sockets",
    "runtime/state",
    "tmp",
    "logs",
    "benchmarks",
    "repositories",
    "workspaces",
    "backups",
];

pub fn ssd_root(mount: &Path) -> PathBuf {
    let canonical = mount.join(SSD_ROOT_DIR);
    if canonical.exists() {
        return canonical;
    }
    // Backward compatibility with earlier Hermes-SSD layout on the same volume.
    let legacy = mount.join("Hermes-SSD");
    if legacy.exists() {
        return legacy;
    }
    canonical
}

pub fn ensure_ssd_layout(mount: &Path) -> Result<()> {
    let root = ssd_root(mount);
    for sub in SUBDIRS {
        let dir = root.join(sub);
        fs::create_dir_all(&dir).map_err(|e| HermesSsdLlmError::DirectoryInitFailed {
            path: dir.display().to_string(),
            reason: e.to_string(),
        })?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(meta) = fs::metadata(&dir) {
                let mut perms = meta.permissions();
                perms.set_mode(0o750);
                let _ = fs::set_permissions(&dir, perms);
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct SsdPaths {
    pub root: PathBuf,
    pub hermes_home: PathBuf,
    pub models_gguf: PathBuf,
    pub cache_hermes: PathBuf,
    pub cache_hf: PathBuf,
    pub cache_transformers: PathBuf,
    pub cache_rust: PathBuf,
    pub tmp: PathBuf,
    pub logs: PathBuf,
    pub sessions: PathBuf,
    pub repositories: PathBuf,
    pub workspaces: PathBuf,
    pub runtime_state: PathBuf,
}

impl SsdPaths {
    pub fn from_mount(mount: &Path) -> Self {
        let root = ssd_root(mount);
        Self {
            hermes_home: root.join("data/hermes"),
            models_gguf: root.join("models/gguf"),
            cache_hermes: root.join("cache/hermes"),
            cache_hf: root.join("cache/huggingface"),
            cache_transformers: root.join("cache/transformers"),
            cache_rust: root.join("cache/rust"),
            tmp: root.join("tmp"),
            logs: root.join("logs"),
            sessions: root.join("runtime/sessions"),
            repositories: root.join("repositories"),
            workspaces: root.join("workspaces"),
            runtime_state: root.join("runtime/state"),
            root,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn creates_layout() {
        let tmp = TempDir::new().unwrap();
        ensure_ssd_layout(tmp.path()).unwrap();
        assert!(ssd_root(tmp.path()).join("models/gguf").is_dir());
        assert!(ssd_root(tmp.path()).join("cache/hermes").is_dir());
    }
}
