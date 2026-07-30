//! Hermes command dispatcher — `hermes` and `hermes ssd`.

use hermes_ssd_llm::cli;
use hermes_ssd_llm::config::HermesSsdLlmConfig;
use hermes_ssd_llm::errors::{ExitCode, HermesSsdLlmError};
use hermes_ssd_llm::launcher::{exec_hermes_passthrough, resolve_real_hermes};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.first().map(|s| s.as_str()) == Some("ssd") {
        let rest = args[1..].to_vec();
        let code = cli::handle_ssd_subcommand(&rest).unwrap_or(ExitCode::RuntimeFailure.code());
        process::exit(code);
    }

    let cfg = HermesSsdLlmConfig::load_or_default();
    let real = match resolve_real_hermes(&cfg) {
        Ok(p) => p,
        Err(HermesSsdLlmError::HermesMissing) => {
            eprintln!("{}", HermesSsdLlmError::HermesMissing);
            process::exit(ExitCode::HermesMissing.code());
        }
        Err(e) => {
            eprintln!("{e}");
            process::exit(e.exit_code().code());
        }
    };

    if let Err(e) = exec_hermes_passthrough(&real, &args) {
        eprintln!("{e}");
        process::exit(ExitCode::RuntimeFailure.code());
    }
}
