use crate::config::{migrate_config_if_needed, HermesSsdLlmConfig, CONFIG_VERSION};
use crate::device::verify_volume;
use crate::environment::RoutedEnvironment;
use crate::errors::{HermesSsdLlmError, Result};
use crate::launcher::{hermes_version, resolve_real_hermes};
use crate::locks::SessionLock;
use crate::paths::SsdPaths;
use crate::APP_NAME;
use std::path::Path;

pub struct DoctorReport {
    pub config_path: String,
    pub config_version: u32,
    pub volume_uuid: String,
    pub volume_name: String,
    pub mount_path: String,
    pub filesystem: String,
    pub protocol: String,
    pub total_gb: u64,
    pub free_gb: u64,
    pub writable: bool,
    pub external: bool,
    pub hermes_path: String,
    pub hermes_version: String,
    pub hermes_ssd_version: String,
    pub rust_binary_version: String,
    pub models_dir: String,
    pub cache_dir: String,
    pub logs_dir: String,
    pub lock_active: bool,
    pub internal_fallback_disabled: bool,
    pub unclean_shutdown: bool,
    pub routed_env: Vec<(String, String)>,
    pub throughput_mbps: Option<f64>,
}

pub fn run_doctor(throughput_test: bool) -> Result<DoctorReport> {
    migrate_config_if_needed()?;
    let cfg = HermesSsdLlmConfig::load()?;
    let vol = verify_volume(&cfg)?;
    let paths = SsdPaths::from_mount(&vol.mount_point);
    let env = RoutedEnvironment::build(&cfg, &vol);
    let real = resolve_real_hermes(&cfg)?;
    let lock_path = paths.root.join("runtime/locks/hermes-ssd-llm.session.lock");
    let lock_active = lock_path.exists();
    let throughput = if throughput_test {
        Some(measure_throughput(&paths.tmp)?)
    } else {
        None
    };

    Ok(DoctorReport {
        config_path: HermesSsdLlmConfig::config_path().display().to_string(),
        config_version: CONFIG_VERSION,
        volume_uuid: vol.volume_uuid.clone(),
        volume_name: vol.volume_name.clone(),
        mount_path: vol.mount_point.display().to_string(),
        filesystem: vol.filesystem.clone(),
        protocol: vol.protocol.clone(),
        total_gb: vol.total_bytes / (1024 * 1024 * 1024),
        free_gb: vol.free_bytes / (1024 * 1024 * 1024),
        writable: vol.writable,
        external: !vol.internal,
        hermes_path: real.display().to_string(),
        hermes_version: hermes_version(&real).unwrap_or_else(|| "unknown".into()),
        hermes_ssd_version: env!("CARGO_PKG_VERSION").into(),
        rust_binary_version: env!("CARGO_PKG_VERSION").into(),
        models_dir: paths.models_gguf.display().to_string(),
        cache_dir: paths.cache_hermes.display().to_string(),
        logs_dir: paths.logs.display().to_string(),
        lock_active,
        internal_fallback_disabled: !cfg.allow_internal_fallback,
        unclean_shutdown: SessionLock::was_unclean(&paths),
        routed_env: env.redacted_report(),
        throughput_mbps: throughput,
    })
}

pub fn print_doctor(report: &DoctorReport) {
    println!("{APP_NAME} doctor");
    println!("────────────────────────────────────────");
    println!("Config:              {}", report.config_path);
    println!("Config version:      {}", report.config_version);
    println!("Volume UUID:         {}", report.volume_uuid);
    println!("Volume name:         {}", report.volume_name);
    println!("Mount path:          {}", report.mount_path);
    println!("Filesystem:          {}", report.filesystem);
    println!("Protocol:            {}", report.protocol);
    println!("Total capacity:      {} GB", report.total_gb);
    println!("Available:           {} GB", report.free_gb);
    println!("Writable:            {}", report.writable);
    println!("External device:     {}", report.external);
    println!("Hermes executable:   {}", report.hermes_path);
    println!("Hermes version:      {}", report.hermes_version);
    println!("Hermes SSD LLM version:  {}", report.hermes_ssd_version);
    println!("Models directory:    {}", report.models_dir);
    println!("Cache directory:     {}", report.cache_dir);
    println!("Logs directory:      {}", report.logs_dir);
    println!("Session lock active: {}", report.lock_active);
    println!(
        "Internal fallback:   disabled={}",
        report.internal_fallback_disabled
    );
    println!("Unclean shutdown:    {}", report.unclean_shutdown);
    println!();
    println!("Environment routing:");
    for (k, v) in &report.routed_env {
        if is_sensitive_key(k) {
            println!("  {k}=<redacted>");
        } else {
            println!("  {k}={v}");
        }
    }
    if let Some(mbps) = report.throughput_mbps {
        println!();
        println!("Throughput probe:    {mbps:.1} MB/s (write+read, 4 MiB)");
    }
}

fn is_sensitive_key(key: &str) -> bool {
    let upper = key.to_uppercase();
    upper.contains("KEY")
        || upper.contains("TOKEN")
        || upper.contains("SECRET")
        || upper.contains("PASSWORD")
}

fn measure_throughput(tmp: &Path) -> Result<f64> {
    use std::io::Write;
    use std::time::Instant;

    let size = 4 * 1024 * 1024;
    let path = tmp.join(format!(".doctor-throughput-{}", std::process::id()));
    let data = vec![0xABu8; size];
    let start = Instant::now();
    let mut file =
        std::fs::File::create(&path).map_err(|e| HermesSsdLlmError::Other(e.to_string()))?;
    file.write_all(&data)
        .map_err(|e| HermesSsdLlmError::Other(e.to_string()))?;
    file.sync_all().ok();
    let read_back = std::fs::read(&path).map_err(|e| HermesSsdLlmError::Other(e.to_string()))?;
    let elapsed = start.elapsed().as_secs_f64();
    std::fs::remove_file(&path).ok();
    if read_back.len() != size || elapsed <= 0.0 {
        return Ok(0.0);
    }
    Ok((size as f64 * 2.0) / (1024.0 * 1024.0) / elapsed)
}
