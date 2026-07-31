# PROJECT_MANIFEST.md — Agent Entry Point

**Purpose:** Machine-friendly project summary for AI coding agents. Humans should start with README.md.

**Last verified:** YYYY-MM-DD  
**Repository:** {{REPO_NAME}}

---

## Identity

| Field | Value |
|-------|-------|
| **Name** | {{DISPLAY_NAME}} |
| **Domain** | {{DOMAIN}} |
| **Status** | {{STATUS}} |
| **License** | {{LICENSE}} |

---

## What this project does

{{ONE_PARAGRAPH_PURPOSE}}

---

## Tech stack

| Layer | Technology |
|-------|------------|
| **Language** | {{LANGUAGE}} |
| **Framework / runtime** | {{FRAMEWORK}} |
| **Backend / data** | {{BACKEND}} |
| **Build / deploy** | {{BUILD}} |
| **Test** | {{TEST}} |

---

## Repository layout

```text
{{LAYOUT_TREE}}
```

---

## Key commands

| Command | Purpose |
|---------|---------|
| {{CMD_INSTALL}} | Install dependencies |
| {{CMD_DEV}} | Start development |
| {{CMD_TEST}} | Run tests |
| {{CMD_BUILD}} | Build / package |
| {{CMD_VERIFY}} | Full quality gate (if any) |

---

## Architecture (summary)

{{ARCHITECTURE_SUMMARY}}

**Canonical detail:** [ARCHITECTURE.md](ARCHITECTURE.md) · [TECHNICAL.md](TECHNICAL.md)

---

## Data & configuration

| Item | Location |
|------|----------|
| **Config** | {{CONFIG_PATH}} |
| **User / runtime data** | {{DATA_PATH}} |
| **Secrets** | {{SECRETS_POLICY}} |

---

## Engineering standards

Read before editing:

1. [.hermes/README.md](.hermes/README.md) — policy index
2. [.hermes/PROJECT_STANDARDS.md](.hermes/PROJECT_STANDARDS.md) — quality gates
3. [.hermes/ARCHITECTURE_PRINCIPLES.md](.hermes/ARCHITECTURE_PRINCIPLES.md) — design invariants
4. [CONTRIBUTING.md](CONTRIBUTING.md) — workflow

---

## Testing workflow

{{TESTING_WORKFLOW}}

---

## Deployment

{{DEPLOYMENT_SUMMARY}}

---

## Constraints (do not violate)

{{CONSTRAINTS_LIST}}

---

## Important files

| File | Why it matters |
|------|----------------|
| {{IMPORTANT_FILES}} |

---

## Documentation map

| Audience | Start here |
|----------|------------|
| **Beginner** | [README.md](README.md) |
| **Contributor** | [CONTRIBUTING.md](CONTRIBUTING.md) |
| **Engineer** | [TECHNICAL.md](TECHNICAL.md) |
| **Agent** | This file |
