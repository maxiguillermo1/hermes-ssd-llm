# Advanced — Local inference engine

**Audience:** Developers exploring optional local GGUF inference. Not required for normal `hermes ssd` use with a cloud provider.

---

## What this is

Hermes SSD LLM includes Rust modules for streaming GGUF inference from the SSD with Apple Metal acceleration. This code is inherited from the upstream [ssd-llm](https://github.com/redbasecap-buiss/ssd-llm) project and kept in-tree for future integration.

```text
src/
  model/       GGUF parser, layer metadata, LRU cache
  ssd/         mmap pool, prefetch, block swap, memory pressure
  metal/       GPU compute kernels
  inference/   Transformer forward pass, KV cache, sampling
  api/         OpenAI/Ollama-compatible HTTP server (library)
  benchmark.rs Criterion helpers
```

The crate root allows `dead_code` on these APIs because they are **not yet wired** to the user-facing launch path (`hermes ssd` → `hermes.real`).

---

## What is NOT shipped in the CLI yet

The `hermes-ssd-llm` binary today only exposes:

```bash
hermes-ssd-llm doctor [--throughput]
hermes-ssd-llm register <mount>
hermes-ssd-llm launch [hermes args...]
hermes-ssd-llm info <model.gguf>
hermes-ssd-llm models [--dir <path>]
```

These commands are **planned** but not implemented in the binary:

```bash
hermes-ssd-llm bench <model.gguf>    # roadmap
hermes-ssd-llm serve                  # roadmap
```

Do not document them as available user commands until they appear in `src/bin/hermes_ssd_llm.rs`.

---

## Exploring as a developer

### GGUF metadata

```bash
hermes-ssd-llm info /path/to/model.gguf
hermes-ssd-llm models --dir "/Volumes/Extreme SSD/Hermes-SSD-LLM/models/gguf"
```

### Criterion micro-benchmarks

```bash
cargo bench --bench inference_bench
```

### Using llama.cpp today (recommended for real local inference)

Until the integrated server CLI ships, use `llama.cpp` or Ollama and point Hermes at a local OpenAI-compatible URL:

```bash
llama-server \
  -m "/Volumes/Extreme SSD/Hermes-SSD-LLM/models/gguf/your-model.gguf" \
  -ngl 99 \
  -c 65536
```

Configure Hermes (on the SSD `HERMES_HOME`) to use `http://127.0.0.1:8080/v1` as a custom OpenAI-compatible provider. See [Hermes AI Providers](https://hermes-agent.nousresearch.com/docs/integrations/providers).

---

## Memory reality check

Storing a GGUF on the SSD:

- **Does** free internal-drive space
- **Does not** reduce RAM needed at inference time

Weights are loaded into Apple Silicon unified memory while the model runs. On an 8 GB Mac, large models and long context windows remain constrained regardless of where the file sits on disk.

---

## Related docs

- [ROADMAP.md](ROADMAP.md) — shipped vs planned features
- [TECHNICAL.md](TECHNICAL.md) — full engineering reference (inference sections marked advanced)
- [BENCHMARKS.md](BENCHMARKS.md) — measured launcher/storage numbers only unless you run inference benches yourself
