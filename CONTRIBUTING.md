# Contributing — Hermes SSD LLM

Read `CONSTITUTION.md` before any change. All contributions must follow its engineering lifecycle.

## Workflow

1. Understand the problem and read `ARCHITECTURE.md`
2. Design before implementing
3. Write or update tests for behavior changes
4. Run quality gates (below)
5. Update `CHANGELOG.md` and user docs if behavior changes

## Quality gates

```bash
cargo fmt --check
cargo test --lib --tests
cargo build --release
```

ShellCheck (if installed):

```bash
shellcheck install.sh uninstall.sh scripts/*.sh benchmarks/scripts/*.sh
```

## Code standards

- One module, one purpose
- No silent fallbacks in SSD mode
- No secrets or personal paths in committed files
- Measured benchmarks only — no estimated numbers in `BENCHMARKS.md`
- Preserve `hermes` passthrough behavior

## Commit style

Use logical commits:

```text
feat: add ...
fix: correct ...
docs: update ...
bench: measure ...
chore: ...
```

## Pull requests

- Describe what changed and why
- List tests run
- Confirm workflow unchanged: `hermes` and `hermes ssd`
- No force-push to `main`

## Benchmarks

Run on the registered SSD only:

```bash
./scripts/capture-test-system.sh
./benchmarks/scripts/generate-report.sh
```

Commit sanitized results under `benchmarks/results/`. Raw private captures stay gitignored.

## License

Contributions are licensed under MIT. See `LICENSE` and `NOTICE`.
