use crate::config::HermesSsdLlmConfig;
use crate::device::VolumeInfo;
use crate::paths::SsdPaths;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RoutedEnvironment {
    pub vars: BTreeMap<String, String>,
    pub paths: SsdPaths,
    pub mount: PathBuf,
}

impl RoutedEnvironment {
    pub fn build(cfg: &HermesSsdLlmConfig, vol: &VolumeInfo) -> Self {
        let paths = SsdPaths::from_mount(&vol.mount_point);
        let mut vars = BTreeMap::new();

        vars.insert("HERMES_SSD_LLM_MODE".into(), "1".into());
        vars.insert(
            "HERMES_SSD_LLM_MOUNT".into(),
            vol.mount_point.display().to_string(),
        );
        vars.insert(
            "HERMES_SSD_LLM_ROOT".into(),
            paths.root.display().to_string(),
        );

        // Hermes respects HERMES_HOME for all profile data, caches, sessions, skills, etc.
        vars.insert(
            "HERMES_HOME".into(),
            paths.hermes_home.display().to_string(),
        );

        // Heavy caches and temp work on SSD — scoped to this process tree only.
        vars.insert("TMPDIR".into(), paths.tmp.display().to_string());
        vars.insert(
            "XDG_CACHE_HOME".into(),
            paths.cache_hermes.join("xdg-cache").display().to_string(),
        );
        vars.insert(
            "XDG_DATA_HOME".into(),
            paths.hermes_home.join("xdg-data").display().to_string(),
        );
        vars.insert(
            "XDG_STATE_HOME".into(),
            paths.runtime_state.join("xdg-state").display().to_string(),
        );
        vars.insert("HF_HOME".into(), paths.cache_hf.display().to_string());
        vars.insert(
            "HUGGINGFACE_HUB_CACHE".into(),
            paths.cache_hf.join("hub").display().to_string(),
        );
        vars.insert(
            "TRANSFORMERS_CACHE".into(),
            paths.cache_transformers.display().to_string(),
        );
        vars.insert(
            "CARGO_TARGET_DIR".into(),
            paths.cache_rust.join("target").display().to_string(),
        );
        vars.insert(
            "HERMES_SSD_LLM_MODELS".into(),
            paths.models_gguf.display().to_string(),
        );
        vars.insert(
            "HERMES_SSD_LLM_LOG_DIR".into(),
            paths.logs.display().to_string(),
        );

        if cfg.debug_startup {
            vars.insert("HERMES_SSD_LLM_DEBUG".into(), "1".into());
        }

        Self {
            vars,
            paths,
            mount: vol.mount_point.clone(),
        }
    }

    pub fn apply_to_process(&self) {
        for (k, v) in &self.vars {
            std::env::set_var(k, v);
        }
    }

    pub fn redacted_report(&self) -> Vec<(String, String)> {
        self.vars
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::HermesSsdLlmConfig;
    use crate::device::VolumeInfo;
    use std::path::PathBuf;

    fn sample_vol() -> VolumeInfo {
        VolumeInfo {
            mount_point: PathBuf::from("/Volumes/Test SSD"),
            volume_uuid: "UUID".into(),
            volume_name: "Test SSD".into(),
            filesystem: "APFS".into(),
            protocol: "USB".into(),
            total_bytes: 2_000_000_000_000,
            free_bytes: 1_500_000_000_000,
            writable: true,
            internal: false,
            device_node: "/dev/disk4s1".into(),
        }
    }

    #[test]
    fn routes_hermes_home_to_ssd() {
        let env = RoutedEnvironment::build(&HermesSsdLlmConfig::default(), &sample_vol());
        assert!(env.vars["HERMES_HOME"].contains("Hermes-SSD-LLM/data/hermes"));
        assert!(env.vars["TMPDIR"].contains("Hermes-SSD-LLM/tmp"));
        assert_eq!(env.vars["HERMES_SSD_LLM_MODE"], "1");
    }
}
