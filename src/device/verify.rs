use super::discovery::{discover_volume_by_uuid, VolumeInfo};
use crate::config::HermesSsdLlmConfig;
use crate::errors::{HermesSsdLlmError, Result};
use crate::paths::ensure_ssd_layout;
use std::path::Path;

pub fn verify_volume(cfg: &HermesSsdLlmConfig) -> Result<VolumeInfo> {
    let vol = discover_volume_by_uuid(&cfg.volume_uuid)?.ok_or(HermesSsdLlmError::SsdMissing)?;

    if cfg.require_external_device && vol.internal {
        return Err(HermesSsdLlmError::IdentityMismatch {
            expected: cfg.volume_uuid.clone(),
            found: format!("internal volume at {}", vol.mount_point.display()),
        });
    }

    if vol.volume_uuid.to_uppercase() != cfg.volume_uuid.trim().to_uppercase() {
        return Err(HermesSsdLlmError::IdentityMismatch {
            expected: cfg.volume_uuid.clone(),
            found: vol.volume_uuid.clone(),
        });
    }

    if !cfg.expected_volume_name.is_empty() && vol.volume_name != cfg.expected_volume_name {
        return Err(HermesSsdLlmError::IdentityMismatch {
            expected: cfg.expected_volume_name.clone(),
            found: vol.volume_name.clone(),
        });
    }

    let total_gb = vol.total_bytes / (1024 * 1024 * 1024);
    if total_gb < cfg.minimum_capacity_gb {
        return Err(HermesSsdLlmError::InvalidConfig(format!(
            "volume capacity {total_gb} GB is below minimum {} GB",
            cfg.minimum_capacity_gb
        )));
    }

    let free_gb = vol.free_bytes / (1024 * 1024 * 1024);
    if free_gb < cfg.minimum_free_space_gb {
        return Err(HermesSsdLlmError::InsufficientSpace {
            required_gb: cfg.minimum_free_space_gb,
            available_gb: free_gb,
        });
    }

    if !vol.writable {
        return Err(HermesSsdLlmError::ReadOnlyVolume);
    }

    let supported = matches!(
        vol.filesystem.as_str(),
        "APFS" | "ExFAT" | "MS-DOS FAT32" | "Mac OS Extended (Journaled)"
    );
    if !supported {
        return Err(HermesSsdLlmError::InvalidConfig(format!(
            "unsupported filesystem: {}",
            vol.filesystem
        )));
    }

    ensure_ssd_layout(&vol.mount_point)?;
    probe_read_write(&ssd_root(&vol.mount_point), cfg.minimum_write_space_gb)?;

    Ok(vol)
}

pub fn ssd_root(mount: &Path) -> std::path::PathBuf {
    mount.join(crate::SSD_ROOT_DIR)
}

pub fn probe_read_write(root: &Path, min_write_gb: u64) -> Result<()> {
    use std::fs;
    use std::io::Write;

    let probe_dir = root.join("runtime/locks");
    fs::create_dir_all(&probe_dir).map_err(|e| HermesSsdLlmError::DirectoryInitFailed {
        path: probe_dir.display().to_string(),
        reason: e.to_string(),
    })?;

    let probe = probe_dir.join(format!(".probe-{}", std::process::id()));
    let payload = b"hermes-ssd-llm-probe";
    let mut file = fs::File::create(&probe).map_err(|_| HermesSsdLlmError::ReadOnlyVolume)?;
    file.write_all(payload)
        .map_err(|_| HermesSsdLlmError::ReadOnlyVolume)?;
    drop(file);
    let read_back = fs::read(&probe).map_err(|_| HermesSsdLlmError::ReadOnlyVolume)?;
    if read_back != payload {
        return Err(HermesSsdLlmError::ReadOnlyVolume);
    }
    fs::remove_file(&probe).ok();

    let free = fs::metadata(root)
        .ok()
        .and_then(|_| {
            let stat = std::process::Command::new("df")
                .arg("-k")
                .arg(root)
                .output()
                .ok()?;
            let text = String::from_utf8_lossy(&stat.stdout);
            text.lines()
                .nth(1)
                .and_then(|line| line.split_whitespace().nth(3))
                .and_then(|avail| avail.parse::<u64>().ok())
        })
        .unwrap_or(u64::MAX);
    let free_gb = free / (1024 * 1024);
    if free_gb < min_write_gb {
        return Err(HermesSsdLlmError::InsufficientSpace {
            required_gb: min_write_gb,
            available_gb: free_gb,
        });
    }
    Ok(())
}
