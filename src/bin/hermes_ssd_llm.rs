//! Hermes SSD LLM management CLI — doctor, register, and inference utilities.

use clap::{Parser, Subcommand};
use hermes_ssd_llm::cli::{launch_ssd_mode_quiet, register_mount};
use hermes_ssd_llm::config::HermesSsdLlmConfig;
use hermes_ssd_llm::diagnostics::{print_doctor, run_doctor};
use hermes_ssd_llm::errors::Result;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[command(
    name = "hermes-ssd-llm",
    version,
    about = "Hermes SSD LLM — external SSD storage routing and local LLM inference"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Diagnostic report for SSD registration and routing
    Doctor {
        #[arg(long, default_value_t = false)]
        throughput: bool,
    },
    /// Register SSD volume (install helper)
    Register { mount: PathBuf },
    /// Prepare SSD environment and exec Hermes (used by install tests)
    Launch {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        hermes_args: Vec<String>,
    },
    /// Show GGUF model metadata
    Info { model: PathBuf },
    /// List GGUF models in a directory
    Models {
        #[arg(long)]
        dir: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();
    let code = match run(cli) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{e}");
            e.exit_code().code()
        }
    };
    if code != 0 {
        process::exit(code);
    }
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Doctor { throughput } => {
            let report = run_doctor(throughput)?;
            print_doctor(&report);
            Ok(())
        }
        Commands::Register { mount } => {
            let mut cfg = HermesSsdLlmConfig::load().unwrap_or_default();
            register_mount(&mut cfg, &mount)?;
            println!(
                "Hermes SSD LLM: registered {} ({})",
                cfg.expected_volume_name, cfg.volume_uuid
            );
            Ok(())
        }
        Commands::Launch { hermes_args } => {
            launch_ssd_mode_quiet(&hermes_args)?;
            Ok(())
        }
        Commands::Info { model } => {
            let gguf = hermes_ssd_llm::model::gguf::GgufFile::open(&model)
                .map_err(|e| hermes_ssd_llm::HermesSsdLlmError::Other(e.to_string()))?;
            println!("Architecture: {}", gguf.architecture());
            println!("Layers: {}", gguf.n_layers());
            println!("Context: {}", gguf.n_ctx());
            Ok(())
        }
        Commands::Models { dir } => {
            let scan = dir.unwrap_or_else(|| PathBuf::from("models"));
            if scan.is_dir() {
                for entry in std::fs::read_dir(&scan)
                    .map_err(|e| hermes_ssd_llm::HermesSsdLlmError::Other(e.to_string()))?
                {
                    let path = entry
                        .map_err(|e| hermes_ssd_llm::HermesSsdLlmError::Other(e.to_string()))?
                        .path();
                    if path.extension().map(|e| e == "gguf").unwrap_or(false) {
                        println!("{}", path.display());
                    }
                }
            }
            Ok(())
        }
    }
}
