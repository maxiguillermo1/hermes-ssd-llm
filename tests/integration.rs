use hermes_ssd_llm::config::HermesSsdLlmConfig;
use hermes_ssd_llm::environment::RoutedEnvironment;
use hermes_ssd_llm::errors::HermesSsdLlmError;
use hermes_ssd_llm::paths::{ensure_ssd_layout, ssd_root};
use std::path::PathBuf;

#[test]
fn config_rejects_internal_fallback() {
    let mut cfg = HermesSsdLlmConfig::default();
    cfg.allow_internal_fallback = true;
    assert!(matches!(
        cfg.validate(),
        Err(HermesSsdLlmError::FallbackRefused)
    ));
}

#[test]
fn ssd_layout_creates_expected_dirs() {
    let tmp = tempfile::tempdir().unwrap();
    ensure_ssd_layout(tmp.path()).unwrap();
    assert!(ssd_root(tmp.path()).join("models/gguf").is_dir());
    assert!(ssd_root(tmp.path()).join("cache/hermes").is_dir());
}

#[test]
fn environment_routes_hermes_home() {
    let vol = hermes_ssd_llm::device::VolumeInfo {
        mount_point: PathBuf::from("/Volumes/Test"),
        volume_uuid: "UUID".into(),
        volume_name: "Test".into(),
        filesystem: "APFS".into(),
        protocol: "USB".into(),
        total_bytes: 2_000_000_000_000,
        free_bytes: 1_500_000_000_000,
        writable: true,
        internal: false,
        device_node: "/dev/disk4s1".into(),
    };
    let env = RoutedEnvironment::build(&HermesSsdLlmConfig::default(), &vol);
    assert!(env.vars["HERMES_HOME"].contains("Hermes-SSD-LLM/data/hermes"));
    assert!(!env.vars["HERMES_HOME"].contains("/Users/"));
}

#[test]
fn exit_codes_are_nonzero_for_failures() {
    assert_ne!(HermesSsdLlmError::SsdMissing.exit_code().code(), 0);
    assert_ne!(HermesSsdLlmError::FallbackRefused.exit_code().code(), 0);
}
