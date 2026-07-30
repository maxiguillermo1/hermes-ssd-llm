use crate::errors::{HermesSsdLlmError, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub struct VolumeInfo {
    pub mount_point: PathBuf,
    pub volume_uuid: String,
    pub volume_name: String,
    pub filesystem: String,
    pub protocol: String,
    pub total_bytes: u64,
    pub free_bytes: u64,
    pub writable: bool,
    pub internal: bool,
    pub device_node: String,
}

pub fn list_external_volumes() -> Result<Vec<VolumeInfo>> {
    let output = Command::new("ls")
        .arg("/Volumes")
        .output()
        .map_err(|e| HermesSsdLlmError::Other(format!("cannot list /Volumes: {e}")))?;
    if !output.status.success() {
        return Ok(vec![]);
    }
    let names = String::from_utf8_lossy(&output.stdout);
    let mut volumes = Vec::new();
    for name in names.lines() {
        let trimmed = name.trim();
        if trimmed.is_empty() || trimmed.starts_with('.') {
            continue;
        }
        let mount = PathBuf::from("/Volumes").join(trimmed);
        if let Ok(info) = volume_info_at(&mount) {
            if !info.internal {
                volumes.push(info);
            }
        }
    }
    Ok(volumes)
}

pub fn discover_volume_by_uuid(uuid: &str) -> Result<Option<VolumeInfo>> {
    let target = uuid.trim().to_uppercase();
    for vol in list_external_volumes()? {
        if vol.volume_uuid.to_uppercase() == target {
            return Ok(Some(vol));
        }
    }
    Ok(None)
}

pub fn volume_info_at(mount: &Path) -> Result<VolumeInfo> {
    let output = Command::new("diskutil")
        .args(["info", "-plist", mount.to_string_lossy().as_ref()])
        .output()
        .map_err(|e| HermesSsdLlmError::Other(format!("diskutil failed: {e}")))?;
    if !output.status.success() {
        return Err(HermesSsdLlmError::SsdMissing);
    }
    let value: plist::Value = plist::from_bytes(&output.stdout)
        .map_err(|e| HermesSsdLlmError::Other(format!("cannot parse diskutil plist: {e:?}")))?;
    let dict = match value {
        plist::Value::Dictionary(d) => d,
        _ => {
            return Err(HermesSsdLlmError::Other(
                "diskutil plist root is not a dictionary".into(),
            ));
        }
    };

    let volume_uuid = string_field(&dict, "VolumeUUID")
        .or_else(|| string_field(&dict, "DiskUUID"))
        .unwrap_or_default();
    let writable = bool_field(&dict, "WritableVolume")
        .or_else(|| bool_field(&dict, "Writable"))
        .unwrap_or(false);
    let internal = bool_field(&dict, "Internal").unwrap_or(true);
    let total_bytes = u64_field(&dict, "VolumeSize")
        .or_else(|| u64_field(&dict, "Size"))
        .or_else(|| u64_field(&dict, "TotalSize"))
        .unwrap_or(0);
    let free_bytes = u64_field(&dict, "FreeSpace")
        .or_else(|| u64_field(&dict, "VolumeFreeSpace"))
        .unwrap_or(0);

    Ok(VolumeInfo {
        mount_point: PathBuf::from(
            string_field(&dict, "MountPoint").unwrap_or_else(|| mount.display().to_string()),
        ),
        volume_uuid,
        volume_name: string_field(&dict, "VolumeName").unwrap_or_default(),
        filesystem: string_field(&dict, "FilesystemUserVisibleName")
            .or_else(|| string_field(&dict, "FileSystemPersonality"))
            .unwrap_or_else(|| "unknown".into()),
        protocol: string_field(&dict, "BusProtocol")
            .or_else(|| string_field(&dict, "Protocol"))
            .unwrap_or_else(|| "unknown".into()),
        total_bytes,
        free_bytes,
        writable,
        internal,
        device_node: string_field(&dict, "DeviceNode").unwrap_or_default(),
    })
}

fn string_field(dict: &plist::Dictionary, key: &str) -> Option<String> {
    match dict.get(key)? {
        plist::Value::String(s) => Some(s.clone()),
        plist::Value::Boolean(b) => Some(b.to_string()),
        _ => None,
    }
}

fn bool_field(dict: &plist::Dictionary, key: &str) -> Option<bool> {
    match dict.get(key)? {
        plist::Value::Boolean(b) => Some(*b),
        plist::Value::String(s) => Some(s == "Yes" || s == "true"),
        _ => None,
    }
}

fn u64_field(dict: &plist::Dictionary, key: &str) -> Option<u64> {
    match dict.get(key)? {
        plist::Value::Integer(i) => i.as_unsigned().or_else(|| i.as_signed().map(|v| v as u64)),
        plist::Value::Real(f) => Some(*f as u64),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_extreme_ssd_when_mounted() {
        let mount = Path::new("/Volumes/Extreme SSD");
        if mount.exists() {
            let info = volume_info_at(mount).expect("parse volume");
            assert!(!info.volume_uuid.is_empty());
            assert!(!info.internal);
        }
    }
}
