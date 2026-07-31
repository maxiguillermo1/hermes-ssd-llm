# Hermes SSD LLM

Hermes SSD LLM is an **external-storage runtime and launcher** for [Hermes Agent](https://hermes-agent.nousresearch.com/). It redirects Hermes-controlled data — sessions, memories, skills, caches, logs, and projects — to a portable SSD so your MacBook's internal storage stays free. Plug in the drive, run one command, and Hermes launches with SSD-backed paths.

**Important:** This is primarily a **storage router**, not an automatic local GGUF LLM system. Hermes still uses whichever model provider you configure. Cloud providers perform inference remotely; local GGUF inference is an optional, advanced mode.

This project is for anyone who wants to use Hermes — an AI assistant you run from the terminal — without letting Hermes data consume their laptop's limited internal storage.

---

## The Problem

My MacBook Air has only 8 GB of memory and 256 GB of internal storage.

AI work creates a lot of data. Large model files, download caches, conversation logs, databases, search indexes, temporary files, and Git repositories all add up quickly. On a small laptop, that data competes with everything else — photos, apps, system files, and macOS itself.

I did not want my MacBook to become a permanent warehouse for AI files. I wanted a separate, portable workspace that behaves like a dedicated AI workstation — one I can plug in when I need it and leave behind when I do not.

---

## The Solution

Hermes SSD LLM makes a portable SSD (Solid State Drive — a small, fast external hard drive with no moving parts) into that dedicated workspace.

When you run SSD mode:

- Your MacBook stays lighter. Hermes-controlled data and scoped caches move to the SSD instead of internal storage.
- The SSD stores Hermes state, projects, logs, caches, and **optional** local model files.
- Your environment is portable. Move the SSD to another Mac and your Hermes data can come with you.
- Plugging in the SSD and running one command restores the SSD-backed environment.

Hermes itself — the AI assistant you talk to — works exactly the same. SSD mode changes where Hermes stores files, not how the TUI, skills, or tools behave.

**What supplies the AI brain?** The model provider configured inside Hermes (`hermes model`). In the most common setup, that is a cloud provider (Cursor, OpenRouter, Anthropic, etc.) — inference runs remotely and no GGUF file is required. Local GGUF + `llama.cpp` is optional; see [Provider vs local inference](#provider-vs-local-inference) below.

---

## Hardware

This project was built and tested on the following setup.

| Component | Details |
|-----------|---------|
| Computer | MacBook Air (model Mac14,2) |
| Processor | Apple M2 chip (Apple Silicon — Apple's own processor design, fast and power-efficient) |
| Memory | 8 GB unified memory (RAM shared between the processor and graphics) |
| Internal storage | 256 GB SSD |
| External drive | SanDisk Extreme Portable SSD, 2 TB capacity |
| Connection | USB |
| Drive format | ExFAT (a file system that works on both Mac and Windows) |

**Why this hardware?**

The MacBook Air M2 with 8 GB is a capable everyday machine, but it is not designed to hold multi-gigabyte AI models and large development caches. A 2 TB portable SSD adds affordable, fast storage without upgrading the laptop. USB keeps setup simple — plug in and go. The SanDisk Extreme line offers high read/write speeds, which matters when loading large model files from the drive.

---

## How It Works

Here is the full workflow, step by step.

### 1. Plug in the SSD

Connect your registered external drive. macOS mounts it (makes it visible) at a path like `/Volumes/Extreme SSD`.

### 2. Run SSD mode

Open Terminal and type:

```bash
hermes ssd
```

That is the only command you need for daily use. (Normal Hermes without SSD routing is just `hermes`.)

### 3. Hermes SSD LLM runs safety checks

Before launching Hermes, the program verifies everything is ready:

| Check | What it means |
|-------|---------------|
| Correct SSD connected? | Matches the drive you registered during installation (by unique ID) |
| Enough free space? | At least 100 GB free by default |
| Drive healthy? | Can read and write a test file |
| Drive mounted? | macOS sees the volume |
| Required folders present? | Creates the folder layout if missing |
| Drive writable? | Not read-only or locked |
| External drive? | Refuses to use internal storage as a substitute |

If any check fails, Hermes does **not** launch. There is no silent fallback to your MacBook's internal drive. This protects you from accidentally filling internal storage or running with a missing drive.

### 4. Configuration is seeded (first time only)

On first launch, essential Hermes settings are copied from your Mac's normal Hermes folder (`~/.hermes`) to the SSD — things like `config.yaml` and your engineering standards file. Existing files on the SSD are never overwritten.

### 5. Storage paths are redirected

Environment variables (settings that tell programs where to save files) point all heavy data to the SSD. Hermes, caches, temporary files, and build artifacts all land on the external drive.

### 6. Hermes launches

The real Hermes Agent starts with your usual interface, skills, and tools. You use it normally. Hermes reads your configured provider and that provider performs inference.

### Provider vs local inference

```text
hermes ssd
    ↓
Rust launcher verifies the registered SSD
    ↓
Redirects Hermes storage (HERMES_HOME, caches, temp, logs)
    ↓
Launches Hermes Agent
    ↓
Selected provider performs inference
```

**Mode 1 — Provider mode (most common):** Hermes talks to a cloud or hosted provider. Your Mac runs Hermes; the provider's servers run the model. No GGUF on the SSD is required; `models/` may be empty.

**Mode 2 — Local LLM mode (optional):** You configure a local OpenAI-compatible endpoint (for example `llama.cpp` serving a GGUF from the SSD). Your Mac performs inference. Storing a GGUF on the SSD saves **internal-drive space**, not **RAM** — weights are still loaded into unified memory while running.

See [Hermes AI Providers](https://hermes-agent.nousresearch.com/docs/integrations/providers) and `BENCHMARKS.md`.

### Other useful commands

```bash
hermes ssd doctor          # Run all checks and show a health report
hermes ssd reset --dry-run # Preview what a cleanup would remove
hermes ssd reset           # Clean temporary/runtime files on the SSD
```

---

## What Lives On The SSD

Everything below is stored under `<SSD>/Hermes-SSD-LLM/`.

| Item | What it is |
|------|------------|
| **Projects** | Your code repositories and workspaces |
| **Models** (optional) | Local GGUF files — only when you use local inference; empty with cloud providers |
| **Logs** | Records of what Hermes and tools did, useful for debugging |
| **Caches** | Downloaded files kept for speed so they are not re-downloaded |
| **Databases** | Structured data Hermes uses to remember sessions and settings |
| **Embeddings** | Numerical representations of text used for search and memory |
| **Indexes** | Fast lookup tables built from your data |
| **Configuration** | Runtime settings for the SSD environment |
| **Temporary files** | Short-lived files created during work, cleaned up automatically |
| **Vector databases** | Specialized storage for AI similarity search |
| **Memory** | Hermes session memory and conversation history |
| **Backups** | Copies of important data for recovery |
| **Git repositories** | Version-controlled project folders |
| **Benchmarks** | Performance test results for this setup |

**Credentials:** Hermes stores API keys and OAuth tokens in `HERMES_HOME/.env` and `HERMES_HOME/auth.json`. Because SSD mode redirects `HERMES_HOME` to the external drive, **secrets may live on the SSD** after bootstrap — they are not automatically kept only in macOS Keychain. Review placement after first launch; see [SECURITY.md](SECURITY.md).

**Not on the SSD:** Hermes executables (`hermes`, `hermes.real`), macOS system files, and tools that ignore redirected environment variables may still use internal storage.

---

## Why Rust?

**Rust** is a programming language. It was chosen because it is extremely fast, reliable, and uses very little memory compared to many other languages.

For Hermes SSD LLM specifically:

- **Lightweight launcher** matters on an 8 GB Mac. Rust keeps the SSD validator, path router, lock manager, and process launcher small and reliable.
- **Reliability** matters when handling large files and optional GPU work. Rust catches many bugs at compile time instead of at runtime.
- **Performance** matters when streaming multi-gigabyte model layers from an SSD in **optional local inference mode**. Rust compiles to native machine code with no garbage-collection pauses.
- **Safety** matters for a tool that validates drives, manages locks, and launches other programs. Rust's ownership system prevents common crashes and data races.

Rust does **not** replace your configured Hermes provider. When using cloud providers, inference happens remotely. When using local GGUF, `llama.cpp` or the bundled streaming engine performs inference.

---

## Why A Portable SSD?

| Benefit | Explanation |
|---------|-------------|
| **Portable** | Carry your entire AI environment in your pocket |
| **Replaceable** | Upgrade to a larger or faster drive without buying a new laptop |
| **Expandable** | 2 TB costs far less than upgrading MacBook storage |
| **Easy backups** | Copy one folder or clone the whole drive |
| **Easy migration** | Plug into a new Mac, run `hermes ssd`, done |
| **Protects internal SSD** | Your MacBook's built-in storage stays free for macOS and personal files |
| **Clean separation** | AI data and personal data never mix on the internal drive |

---

## Installation

```bash
git clone https://github.com/maxiguillermo1/hermes-ssd-llm.git
cd hermes-ssd-llm
./install.sh
hermes ssd doctor
```

Requirements: macOS, an existing Hermes Agent installation, and Rust (installed automatically by the script if missing).

---

## Frequently Asked Questions

**Why not store everything on my MacBook?**

AI models alone can be 4–70+ GB each. Caches, logs, and build artifacts add more. On a 256 GB MacBook, that space runs out fast and slows the whole system down.

**Will this make my MacBook faster?**

It frees **internal storage** by moving Hermes data and scoped caches to the SSD. That can make the laptop feel less cramped. It does **not** reduce RAM needed for local model inference — a GGUF on the SSD is still loaded into unified memory at runtime. With cloud providers, inference runs on the provider's hardware, not your Mac.

**Can I unplug the SSD?**

Not while Hermes is running. Unplugging during active work will cause errors. Quit Hermes first, then eject the drive safely.

**Can I move to another Mac?**

Your Hermes data and optional models can travel with the SSD, but the new Mac still needs Hermes Agent, this launcher (`./install.sh`), provider authentication, and (for local models) a backend such as `llama.cpp`. Portable data — not necessarily plug-and-run on an unprepared machine.

**Can I upgrade the SSD later?**

Yes. Copy the `Hermes-SSD-LLM` folder to a new drive, re-run `./install.sh` to register the new volume, and you are set.

**Does SSD mode change how Hermes looks or works?**

No. Same terminal interface, same skills, same tools. Only file locations change.

**What if the wrong SSD is plugged in?**

Hermes refuses to start and shows a clear error. It will never silently use internal storage.

---

## Future Roadmap

- Deeper integration between SSD mode and local model inference
- APFS support optimizations for macOS-native drives
- Automated backup workflows
- GUI status indicator for SSD health
- Multi-SSD profiles for different project contexts

---

## More Documentation

| Document | Audience |
|----------|----------|
| [TECHNICAL.md](TECHNICAL.md) | Senior engineers — architecture, boot sequence, ADRs |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Living system overview with diagrams |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Contributors — code style, testing, PR guidelines |
| [BENCHMARKS.md](BENCHMARKS.md) | Measured performance on the test system |
| [SECURITY.md](SECURITY.md) | Security model and path safety |

---

## License

MIT License

Copyright (c) 2026 Maxi Guillermo

See [LICENSE](LICENSE) for the full text.
