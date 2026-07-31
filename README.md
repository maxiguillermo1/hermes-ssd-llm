# Hermes SSD LLM

Hermes SSD LLM turns a portable external drive into a dedicated home for AI work. Instead of filling your MacBook with large models, caches, logs, and project files, everything heavy lives on the SSD. Plug it in, run one command, and your full AI environment is ready. Unplug it, and your MacBook stays clean.

This project is for anyone who wants to use [Hermes Agent](https://hermes-agent.nousresearch.com/) — an AI assistant you run from the terminal — without letting AI data consume their laptop's limited internal storage.

---

## The Problem

My MacBook Air has only 8 GB of memory and 256 GB of internal storage.

AI work creates a lot of data. Large model files, download caches, conversation logs, databases, search indexes, temporary files, and Git repositories all add up quickly. On a small laptop, that data competes with everything else — photos, apps, system files, and macOS itself.

I did not want my MacBook to become a permanent warehouse for AI files. I wanted a separate, portable workspace that behaves like a dedicated AI workstation — one I can plug in when I need it and leave behind when I do not.

---

## The Solution

Hermes SSD LLM makes a portable SSD (Solid State Drive — a small, fast external hard drive with no moving parts) into that dedicated workspace.

When you run SSD mode:

- Your MacBook stays lightweight. Internal storage is not used for AI data.
- The SSD stores everything related to AI development — models, caches, logs, projects, and more.
- Your environment is portable. Move the SSD to another Mac and your setup comes with you.
- Plugging in the SSD and running one command restores the complete environment instantly.

Hermes itself — the AI assistant you talk to — works exactly the same. SSD mode only changes where files are stored, not how Hermes behaves.

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

The real Hermes Agent starts with your usual interface, skills, and tools. You use it normally.

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
| **Models** | AI model files (GGUF format — compressed files that contain an AI brain) |
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

Your passwords and API keys stay in macOS Keychain on the MacBook — they are **not** copied to the SSD.

---

## Why Rust?

**Rust** is a programming language. It was chosen because it is extremely fast, reliable, and uses very little memory compared to many other languages.

For Hermes SSD LLM specifically:

- **Memory efficiency** matters on an 8 GB Mac. Rust gives precise control over how much RAM the launcher and inference engine use.
- **Reliability** matters when handling large files and GPU work. Rust catches many bugs at compile time instead of at runtime.
- **Performance** matters when streaming multi-gigabyte model layers from an SSD. Rust compiles to native machine code with no garbage-collection pauses.
- **Safety** matters for a tool that validates drives, manages locks, and launches other programs. Rust's ownership system prevents common crashes and data races.

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

It frees internal storage and reduces memory pressure from large local models. Your MacBook will feel less cramped. Actual AI response speed depends on whether you use remote providers (cloud) or local models (on your machine).

**Can I unplug the SSD?**

Not while Hermes is running. Unplugging during active work will cause errors. Quit Hermes first, then eject the drive safely.

**Can I move to another Mac?**

Yes. Install Hermes SSD LLM on the new Mac, plug in the same SSD, and run `hermes ssd`. Your data and configuration travel with the drive.

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
