use super::{HermesSsdLlmConfig, CONFIG_VERSION};
use crate::errors::{HermesSsdLlmError, Result};
use std::fs;

pub fn migrate_config_if_needed() -> Result<()> {
    let path = HermesSsdLlmConfig::config_path();
    if !path.exists() {
        return Ok(());
    }
    let raw = fs::read_to_string(&path).map_err(|e| {
        HermesSsdLlmError::InvalidConfig(format!("cannot read {}: {e}", path.display()))
    })?;
    let mut value: toml::Value = toml::from_str(&raw)
        .map_err(|e| HermesSsdLlmError::InvalidConfig(format!("invalid TOML: {e}")))?;
    let version = value
        .get("version")
        .and_then(|v| v.as_integer())
        .unwrap_or(0) as u32;
    if version == CONFIG_VERSION {
        return Ok(());
    }
    if version > CONFIG_VERSION {
        return Err(HermesSsdLlmError::InvalidConfig(format!(
            "config version {version} is newer than supported ({CONFIG_VERSION})"
        )));
    }
    // v0 -> v1: ensure new fields exist
    let table = value
        .as_table_mut()
        .ok_or_else(|| HermesSsdLlmError::InvalidConfig("config root must be a table".into()))?;
    table.entry("version").or_insert(toml::Value::Integer(1));
    table
        .entry("allow_internal_fallback")
        .or_insert(toml::Value::Boolean(false));
    table
        .entry("require_external_device")
        .or_insert(toml::Value::Boolean(true));
    let migrated = toml::to_string_pretty(&value)
        .map_err(|e| HermesSsdLlmError::InvalidConfig(format!("migration serialize failed: {e}")))?;
    let backup = path.with_extension(format!("toml.v{version}.bak"));
    fs::copy(&path, &backup).ok();
    fs::write(&path, migrated)
        .map_err(|e| HermesSsdLlmError::InvalidConfig(format!("migration write failed: {e}")))?;
    Ok(())
}
