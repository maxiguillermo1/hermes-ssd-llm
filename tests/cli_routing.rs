use hermes_ssd_llm::cli::handle_ssd_subcommand;

#[test]
fn doctor_help_paths() {
    // doctor subcommand returns success code without launching Hermes
    let code = handle_ssd_subcommand(&["help".to_string()]).unwrap_or(1);
    assert_eq!(code, 0);
}
