//! Safe first-run reset for Hermes SSD LLM managed runtime state.

use crate::config::HermesSsdLlmConfig;
use crate::device::verify_volume;
use crate::errors::{HermesSsdLlmError, Result};
use crate::paths::{ensure_ssd_layout, ssd_root, SsdPaths};
use crate::APP_NAME;
use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone, Default)]
pub struct ResetOptions {
    pub dry_run: bool,
    pub include_models: bool,
    pub all_managed_data: bool,
}

#[derive(Debug, Clone)]
pub struct ResetReport {
    pub dry_run: bool,
    pub cleaned: Vec<String>,
    pub preserved: Vec<String>,
    pub bytes_recovered: u64,
    pub models_removed: bool,
    pub config_preserved: bool,
    pub manifest_path: Option<String>,
    pub success: bool,
}

pub fn run_reset(opts: &ResetOptions) -> Result<ResetReport> {
    let cfg = HermesSsdLlmConfig::load()?;
    let vol = verify_volume(&cfg)?;
    let paths = SsdPaths::from_mount(&vol.mount_point);
    let root = ssd_root(&vol.mount_point);

    let targets = collect_targets(&paths, &root, opts)?;
    let mut cleaned = Vec::new();
    let preserved = default_preserved(opts);
    let mut bytes_recovered = 0u64;

    let manifest_path = if opts.dry_run {
        None
    } else {
        Some(write_manifest(&paths, &targets)?)
    };

    for target in &targets {
        validate_managed_path(&root, target)?;
        let size = dir_size(target).unwrap_or(0);
        if opts.dry_run {
            cleaned.push(target.display().to_string());
            bytes_recovered += size;
        } else {
            remove_path(target)?;
            cleaned.push(target.display().to_string());
            bytes_recovered += size;
        }
    }

    if !opts.dry_run {
        ensure_ssd_layout(&vol.mount_point)?;
    }

    Ok(ResetReport {
        dry_run: opts.dry_run,
        cleaned,
        preserved,
        bytes_recovered,
        models_removed: opts.include_models || opts.all_managed_data,
        config_preserved: true,
        manifest_path,
        success: true,
    })
}

fn default_preserved(opts: &ResetOptions) -> Vec<String> {
    let mut preserved = vec![
        "~/.config/hermes-ssd-llm/config.toml (SSD registration)".into(),
        "repository source code".into(),
        "Git history".into(),
        "credentials / keychain".into(),
    ];
    if !opts.include_models && !opts.all_managed_data {
        preserved.push("downloaded models".into());
    }
    if !opts.all_managed_data {
        preserved.push("user repositories".into());
        preserved.push("user workspaces".into());
        preserved.push("backups".into());
    }
    preserved
}

fn collect_targets(paths: &SsdPaths, root: &Path, opts: &ResetOptions) -> Result<Vec<PathBuf>> {
    let mut targets = BTreeSet::new();

    for sub in [
        "runtime/locks",
        "runtime/sessions",
        "runtime/sockets",
        "runtime/state",
        "tmp",
        "logs",
        "benchmarks",
        "cache/hermes",
        "cache/huggingface",
        "cache/transformers",
        "cache/rust",
        "cache/build",
        "cache/inference",
    ] {
        let p = root.join(sub);
        if p.exists() {
            targets.insert(p);
        }
    }

    let downloads = root.join("models/downloads");
    if downloads.exists() {
        add_partial_downloads(&downloads, &mut targets);
    }

    if opts.include_models || opts.all_managed_data {
        for sub in [
            "models/gguf",
            "models/draft",
            "models/vision",
            "models/adapters",
            "models/downloads",
        ] {
            let p = root.join(sub);
            if p.exists() {
                targets.insert(p);
            }
        }
    }

    if opts.all_managed_data {
        for sub in ["repositories", "workspaces", "backups", "data", "bin"] {
            let p = root.join(sub);
            if p.exists() {
                targets.insert(p);
            }
        }
        let runtime = root.join("runtime");
        if runtime.exists() {
            targets.insert(runtime);
        }
        let cache = root.join("cache");
        if cache.exists() {
            targets.insert(cache);
        }
    }

    // Never touch SSD config directory in default/include-models modes.
    let _ = paths;

    Ok(targets.into_iter().collect())
}

fn add_partial_downloads(downloads: &Path, targets: &mut BTreeSet<PathBuf>) {
    if let Ok(entries) = fs::read_dir(downloads) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name.ends_with(".part")
                || name.ends_with(".tmp")
                || name.ends_with(".incomplete")
                || name.starts_with('.')
            {
                targets.insert(path);
            }
        }
    }
}

/// Reject paths outside the managed SSD root or known unsafe locations.
pub fn validate_managed_path(root: &Path, path: &Path) -> Result<()> {
    if path.as_os_str().is_empty() {
        return Err(HermesSsdLlmError::InvalidConfig(
            "reset refused empty path".into(),
        ));
    }

    let home = dirs::home_dir();
    let home_path = home.as_deref().unwrap_or(Path::new("/"));
    let forbidden: [&Path; 4] = [
        Path::new("/"),
        Path::new("/Users"),
        Path::new("/Volumes"),
        home_path,
    ];
    for f in &forbidden {
        if path == *f {
            return Err(HermesSsdLlmError::InvalidConfig(format!(
                "reset refused unsafe path: {}",
                path.display()
            )));
        }
    }

    let canon_root = canonicalize_lossy(root)?;
    let canon_path = canonicalize_lossy(path)?;

    if canon_path == canon_root {
        return Err(HermesSsdLlmError::InvalidConfig(format!(
            "reset refused SSD root itself: {}",
            path.display()
        )));
    }

    if !canon_path.starts_with(&canon_root) {
        return Err(HermesSsdLlmError::InvalidConfig(format!(
            "reset path escapes managed root: {}",
            path.display()
        )));
    }

    Ok(())
}

fn canonicalize_lossy(path: &Path) -> Result<PathBuf> {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                return Err(HermesSsdLlmError::InvalidConfig(format!(
                    "reset refused parent traversal in {}",
                    path.display()
                )));
            }
            Component::CurDir => {}
            Component::RootDir => out.push("/"),
            Component::Normal(c) => out.push(c),
            Component::Prefix(p) => out.push(p.as_os_str()),
        }
    }
    if let Ok(real) = fs::canonicalize(path) {
        if real.is_symlink() {
            let target = fs::read_link(path).map_err(|e| {
                HermesSsdLlmError::InvalidConfig(format!("cannot read symlink: {e}"))
            })?;
            if target.is_absolute() {
                return Err(HermesSsdLlmError::InvalidConfig(format!(
                    "reset refused absolute symlink: {}",
                    path.display()
                )));
            }
        }
        return Ok(real);
    }
    Ok(out)
}

fn dir_size(path: &Path) -> Result<u64> {
    if path.is_file() {
        return Ok(fs::metadata(path).map(|m| m.len()).unwrap_or(0));
    }
    let mut total = 0u64;
    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                total += dir_size(&p).unwrap_or(0);
            }
        }
    }
    Ok(total)
}

fn remove_path(path: &Path) -> Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path).map_err(|e| HermesSsdLlmError::Other(e.to_string()))?;
    } else if path.exists() {
        fs::remove_file(path).map_err(|e| HermesSsdLlmError::Other(e.to_string()))?;
    }
    Ok(())
}

fn write_manifest(paths: &SsdPaths, targets: &[PathBuf]) -> Result<String> {
    let backups = paths.root.join("backups");
    fs::create_dir_all(&backups).map_err(|e| HermesSsdLlmError::Other(e.to_string()))?;
    let ts = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let manifest = backups.join(format!("reset-manifest-{ts}.txt"));
    let mut file = File::create(&manifest).map_err(|e| HermesSsdLlmError::Other(e.to_string()))?;
    writeln!(file, "Hermes SSD LLM reset manifest")
        .map_err(|e| HermesSsdLlmError::Other(e.to_string()))?;
    writeln!(file, "timestamp={ts}").map_err(|e| HermesSsdLlmError::Other(e.to_string()))?;
    for t in targets {
        writeln!(file, "removed={}", t.display())
            .map_err(|e| HermesSsdLlmError::Other(e.to_string()))?;
    }
    Ok(manifest.display().to_string())
}

pub fn print_reset_report(report: &ResetReport) {
    let mode = if report.dry_run { "dry-run" } else { "reset" };
    println!("{APP_NAME} {mode}");
    println!("────────────────────────────────────────");
    if report.dry_run {
        println!("Dry run — no files were deleted.");
    }
    println!("Paths to clean ({}):", report.cleaned.len());
    for p in &report.cleaned {
        println!("  - {p}");
    }
    println!();
    println!("Preserved:");
    for p in &report.preserved {
        println!("  + {p}");
    }
    println!();
    let mib = report.bytes_recovered as f64 / (1024.0 * 1024.0);
    println!("Bytes recovered: {:.2} MiB", mib);
    println!("Models removed: {}", report.models_removed);
    println!("Configuration preserved: {}", report.config_preserved);
    if let Some(m) = &report.manifest_path {
        println!("Backup manifest: {m}");
    }
    println!("Status: {}", if report.success { "ok" } else { "failed" });
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn make_root() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join("Hermes-SSD-LLM");
        fs::create_dir_all(root.join("tmp")).unwrap();
        fs::create_dir_all(root.join("runtime/locks")).unwrap();
        (tmp, root)
    }

    #[test]
    fn rejects_root_path() {
        let (_tmp, root) = make_root();
        assert!(validate_managed_path(&root, &root).is_err());
    }

    #[test]
    fn rejects_home_directory() {
        let (_tmp, root) = make_root();
        if let Some(home) = dirs::home_dir() {
            assert!(validate_managed_path(&root, &home).is_err());
        }
    }

    #[test]
    fn rejects_volumes_root() {
        let (_tmp, root) = make_root();
        assert!(validate_managed_path(&root, Path::new("/Volumes")).is_err());
    }

    #[test]
    fn accepts_managed_subdirectory() {
        let (_tmp, root) = make_root();
        let target = root.join("tmp");
        assert!(validate_managed_path(&root, &target).is_ok());
    }

    #[test]
    fn dry_run_lists_targets_without_deleting() {
        let (_tmp, root) = make_root();
        fs::write(root.join("tmp/probe.txt"), b"data").unwrap();
        let paths = SsdPaths {
            root: root.clone(),
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
        };
        let opts = ResetOptions {
            dry_run: true,
            include_models: false,
            all_managed_data: false,
        };
        let targets = collect_targets(&paths, &root, &opts).unwrap();
        assert!(targets.iter().any(|p| p.ends_with("tmp")));
        assert!(root.join("tmp/probe.txt").exists());
    }

    #[test]
    fn include_models_adds_model_dirs() {
        let (_tmp, root) = make_root();
        fs::create_dir_all(root.join("models/gguf")).unwrap();
        let paths = SsdPaths::from_mount(root.parent().unwrap());
        let opts = ResetOptions {
            dry_run: true,
            include_models: true,
            all_managed_data: false,
        };
        let targets = collect_targets(&paths, &root, &opts).unwrap();
        assert!(targets.iter().any(|p| p.ends_with("models/gguf")));
    }
}
