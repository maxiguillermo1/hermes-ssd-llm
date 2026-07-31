# .hermes/ — Repository Engineering Standards

This directory contains **persistent engineering and documentation policies** that travel with the repository. Any human contributor or AI agent working in this repo should read these files before making changes.

These standards are **not** stored in agent memory. They are part of the codebase so behavior is reproducible across sessions, machines, and tools.

## Files

| File | Purpose | Audience |
|------|---------|----------|
| [ENGINEERING_CONSTITUTION.md](ENGINEERING_CONSTITUTION.md) | Highest-priority engineering rules | Everyone |
| [DOCUMENTATION_STANDARD.md](DOCUMENTATION_STANDARD.md) | Required documentation structure and quality bar | Authors, reviewers, agents |
| [PROJECT_STANDARDS.md](PROJECT_STANDARDS.md) | Workflow, quality gates, and repo-specific rules | Contributors |
| [ARCHITECTURE_PRINCIPLES.md](ARCHITECTURE_PRINCIPLES.md) | Design principles and architectural invariants | Architects, senior engineers |

## Templates

The `templates/` subdirectory contains starter outlines for new documentation:

- `templates/README_TEMPLATE.md` — beginner-friendly README structure
- `templates/TECHNICAL_TEMPLATE.md` — senior-engineer technical reference structure

## Bootstrapping other repositories

To apply this standard to any Git repository:

```bash
./scripts/bootstrap-hermes-standards.sh /path/to/repo
```

The script copies policy files and creates `.hermes/` if missing. Customize `PROJECT_STANDARDS.md` per repository after bootstrapping.

## Relationship to root-level docs

| `.hermes/` policy | Root-level doc |
|-------------------|----------------|
| ENGINEERING_CONSTITUTION.md | [CONSTITUTION.md](../CONSTITUTION.md) (canonical for this repo) |
| DOCUMENTATION_STANDARD.md | Governs [README.md](../README.md), [TECHNICAL.md](../TECHNICAL.md), etc. |
| ARCHITECTURE_PRINCIPLES.md | Complements [ARCHITECTURE.md](../ARCHITECTURE.md) |
| PROJECT_STANDARDS.md | Complements [CONTRIBUTING.md](../CONTRIBUTING.md) |

Root-level docs are the **product**. `.hermes/` files are the **policy** that governs how those docs are written and maintained.

## When to update

- **ENGINEERING_CONSTITUTION.md** — rarely; only for fundamental rule changes
- **DOCUMENTATION_STANDARD.md** — when the ecosystem-wide documentation bar changes
- **PROJECT_STANDARDS.md** — when quality gates, tooling, or workflow changes
- **ARCHITECTURE_PRINCIPLES.md** — when core design invariants change
- **Root docs** (README, TECHNICAL, ARCHITECTURE) — whenever implementation changes meaningfully
