//! Hermes SSD LLM — SSD-backed Hermes Agent launcher and local inference runtime.

#![allow(dead_code)] // inference engine retains APIs for future wiring

// Hermes SSD LLM infrastructure
pub mod bootstrap;
pub mod cli;
pub mod config;
pub mod device;
pub mod diagnostics;
pub mod environment;
pub mod errors;
pub mod launcher;
pub mod locks;
pub mod paths;
pub mod reset;
pub mod runtime;

// SSD-streaming inference engine
pub mod api;
pub mod benchmark;
pub mod config_inference;
pub mod inference;
pub mod metal;
pub mod model;
pub mod pull;
pub mod ssd;

pub use errors::{ExitCode, HermesSsdLlmError, Result};

pub const APP_NAME: &str = "Hermes SSD LLM";
pub const CONFIG_NAMESPACE: &str = "hermes-ssd-llm";
pub const SSD_ROOT_DIR: &str = "Hermes-SSD-LLM";
