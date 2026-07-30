use crate::errors::{HermesSsdLlmError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const CONFIG_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HermesSsdLlmConfig {
    pub version: u32,
    pub volume_uuid: String,
    #[serde(default)]
    pub expected_volume_name: String,
    #[serde(default)]
    pub expected_model: String,
    #[serde(default = "default_min_capacity")]
    pub minimum_capacity_gb: u64,
    #[serde(default = "default_min_free")]
    pub minimum_free_space_gb: u64,
    #[serde(default = "default_min_write")]
    pub minimum_write_space_gb: u64,
    #[serde(default = "default_true")]
    pub require_external_device: bool,
    #[serde(default)]
    pub allow_internal_fallback: bool,
    #[serde(default)]
    pub hermes_executable: Option<String>,
    #[serde(default)]
    pub real_hermes_backup: Option<String>,
    #[serde(default = "default_logging")]
    pub logging_level: String,
    #[serde(default)]
    pub debug_startup: bool,
    #[serde(default = "default_prefetch")]
    pub layer_prefetch_depth: u32,
    #[serde(default = "default_ram_target")]
    pub max_ram_target_gb: u64,
    #[serde(default = "default_true")]
    pub ssd_kv_swap: bool,
}

fn default_min_capacity() -> u64 {
    1800
}
fn default_min_free() -> u64 {
    100
}
fn default_min_write() -> u64 {
    20
}
fn default_true() -> bool {
    true
}
fn default_logging() -> String {
    "info".into()
}
fn default_prefetch() -> u32 {
    2
}
fn default_ram_target() -> u64 {
    8
}

impl Default for HermesSsdLlmConfig {
    fn default() -> Self {
        Self {
            version: CONFIG_VERSION,
            volume_uuid: String::new(),
            expected_volume_name: String::new(),
            expected_model: String::new(),
            minimum_capacity_gb: default_min_capacity(),
            minimum_free_space_gb: default_min_free(),
            minimum_write_space_gb: default_min_write(),
            require_external_device: true,
            allow_internal_fallback: false,
            hermes_executable: None,
            real_hermes_backup: None,
            logging_level: default_logging(),
            debug_startup: false,
            layer_prefetch_depth: default_prefetch(),
            max_ram_target_gb: default_ram_target(),
            ssd_kv_swap: true,
        }
    }
}

impl HermesSsdLlmConfig {
    pub fn config_dir() -> PathBuf {
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            if !xdg.is_empty() {
                return PathBuf::from(xdg).join("hermes-ssd-llm");
            }
        }
        PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into()))
            .join(".config")
            .join("hermes-ssd-llm")
    }

    pub fn config_path() -> PathBuf {
        Self::config_dir().join("config.toml")
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path();
        if !path.exists() {
            return Err(HermesSsdLlmError::InvalidConfig(format!(
                "configuration not found at {} — run ./install.sh to register your SSD",
                path.display()
            )));
        }
        let raw = fs::read_to_string(&path).map_err(|e| {
            HermesSsdLlmError::InvalidConfig(format!("cannot read {}: {e}", path.display()))
        })?;
        let cfg: Self = toml::from_str(&raw).map_err(|e| {
            HermesSsdLlmError::InvalidConfig(format!("invalid TOML in {}: {e}", path.display()))
        })?;
        if cfg.version > CONFIG_VERSION {
            return Err(HermesSsdLlmError::InvalidConfig(format!(
                "config version {} is newer than supported ({CONFIG_VERSION})",
                cfg.version
            )));
        }
        cfg.validate()?;
        Ok(cfg)
    }

    pub fn load_or_default() -> Self {
        Self::load().unwrap_or_default()
    }

    pub fn save(&self) -> Result<()> {
        self.validate()?;
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                HermesSsdLlmError::InvalidConfig(format!("cannot create {}: {e}", parent.display()))
            })?;
        }
        let backup = path.with_extension("toml.bak");
        if path.exists() {
            let _ = fs::copy(&path, &backup);
        }
        let content = toml::to_string_pretty(self).map_err(|e| {
            HermesSsdLlmError::InvalidConfig(format!("cannot serialize config: {e}"))
        })?;
        fs::write(&path, content).map_err(|e| {
            HermesSsdLlmError::InvalidConfig(format!("cannot write {}: {e}", path.display()))
        })?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(meta) = fs::metadata(&path) {
                let mut perms = meta.permissions();
                perms.set_mode(0o600);
                let _ = fs::set_permissions(&path, perms);
            }
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        if self.allow_internal_fallback {
            return Err(HermesSsdLlmError::FallbackRefused);
        }
        if self.volume_uuid.trim().is_empty() {
            return Err(HermesSsdLlmError::InvalidConfig(
                "volume_uuid must be set — run ./install.sh".into(),
            ));
        }
        Ok(())
    }

    pub fn runtime_config_path(mount: &Path) -> PathBuf {
        mount.join(crate::SSD_ROOT_DIR).join("config/runtime.toml")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_disallows_internal_fallback() {
        let cfg = HermesSsdLlmConfig::default();
        assert!(!cfg.allow_internal_fallback);
    }

    #[test]
    fn validate_rejects_empty_uuid() {
        let cfg = HermesSsdLlmConfig::default();
        assert!(cfg.validate().is_err());
    }
}
