# Security — Hermes SSD LLM

## Threat model

Hermes SSD LLM runs locally on macOS. It:

- Wraps and launches the real Hermes Agent executable
- Routes storage to a registered external SSD
- Optionally runs local GGUF inference

It does not handle remote authentication itself — that remains in upstream Hermes and provider SDKs.

## Secrets and credentials

- API keys, tokens, and passwords stay in Keychain and upstream Hermes config
- SSD mode does **not** redirect credential stores to the external drive
- Doctor output redacts env vars matching `TOKEN`, `SECRET`, `PASSWORD`, `API_KEY`
- Volume UUID is shown in local `doctor` output for registration verification only — public docs and committed reports use `REDACTED`
- Never commit `.env`, `config.toml` with live UUIDs, or raw hardware captures

## Path safety

Reset and managed operations validate:

- Path is under `<SSD>/Hermes-SSD-LLM/`
- No symlink escape outside managed root
- Reject `/`, home directory, `/Volumes`, volume root, empty paths

## Subprocess execution

- Real Hermes path resolved to absolute `hermes.real`
- Arguments forwarded without shell interpolation
- Install scripts quote all paths (spaces in volume names supported)

## File permissions

- Config files created with mode `0600`
- SSD directories created with user-only write (not world-writable)
- Temporary probe files deleted immediately after validation

## Supply chain

- Dependencies pinned in `Cargo.lock`
- Release builds: `cargo build --release` with LTO
- Review new dependencies before adding

## Reporting

If you discover a security issue, do not open a public issue with exploit details. Contact the repository maintainer directly.

## What SSD mode does not protect against

- Physical theft of the external SSD (encrypt the drive if needed)
- Malware on the host Mac
- Compromised upstream Hermes or provider credentials
- Data loss if the SSD is unplugged during active writes
