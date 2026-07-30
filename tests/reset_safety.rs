use hermes_ssd_llm::reset::{validate_managed_path, ResetOptions};
use std::fs;
use std::path::Path;

#[test]
fn reset_rejects_root_and_home() {
    let root = std::path::PathBuf::from("/Volumes/Test/Hermes-SSD-LLM");
    assert!(validate_managed_path(&root, Path::new("/")).is_err());
    assert!(validate_managed_path(&root, Path::new("/Volumes")).is_err());
    if let Some(home) = dirs::home_dir() {
        assert!(validate_managed_path(&root, &home).is_err());
    }
}

#[test]
fn reset_options_default_preserves_models() {
    let opts = ResetOptions::default();
    assert!(!opts.include_models);
    assert!(!opts.all_managed_data);
    assert!(!opts.dry_run);
}

#[test]
fn reset_rejects_empty_path() {
    let root = std::path::PathBuf::from("/Volumes/Test/Hermes-SSD-LLM");
    assert!(validate_managed_path(&root, Path::new("")).is_err());
}

#[test]
fn reset_accepts_path_with_spaces() {
    let tmp = tempfile::TempDir::new().unwrap();
    let root = tmp.path().join("Hermes-SSD-LLM");
    let target = root.join("tmp/with spaces");
    fs::create_dir_all(&target).unwrap();
    assert!(validate_managed_path(&root, &target).is_ok());
}
