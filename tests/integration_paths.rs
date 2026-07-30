use hermes_ssd_llm::config::HermesSsdLlmConfig;
use hermes_ssd_llm::paths::{ensure_ssd_layout, ssd_root};
use tempfile::TempDir;

#[test]
fn layout_creation_on_temp_mount() {
    let tmp = TempDir::new().unwrap();
    ensure_ssd_layout(tmp.path()).unwrap();
    assert!(ssd_root(tmp.path()).join("cache/hermes").is_dir());
    assert!(ssd_root(tmp.path()).join("models/gguf").is_dir());
}

#[test]
fn config_default_rejects_fallback() {
    let cfg = HermesSsdLlmConfig::default();
    assert!(!cfg.allow_internal_fallback);
}
