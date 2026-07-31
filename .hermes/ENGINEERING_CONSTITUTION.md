# Engineering Constitution

Version: 1.0  
Priority: ABSOLUTE

---

## Canonical source

For this repository, the full engineering constitution is:

**[CONSTITUTION.md](../CONSTITUTION.md)** (repository root)

Read it before any implementation, review, or design work. It is the highest-priority standard for all engineering in this project.

---

## Workstation-level constitution

Hermes sessions on this workstation also follow a personal engineering constitution bootstrapped from `~/.hermes/ENGINEERING-CONSTITUTION.md` into the SSD Hermes home on first launch (see `src/bootstrap.rs`).

Both constitutions apply. Repository-specific rules in CONSTITUTION.md take precedence for code in this repo.

---

## Non-negotiable summary

- Never vibe code. Design before implementing.
- Never introduce complexity without measurable benefit.
- Never optimize prematurely.
- Never hide uncertainty.
- Never ship code you cannot fully explain.
- Correctness before speed. Maintainability before cleverness.
- Documentation is part of the product (see [DOCUMENTATION_STANDARD.md](DOCUMENTATION_STANDARD.md)).
- Test before merge. Measure before claiming performance.

---

## Engineering lifecycle

Every significant task follows:

1. Understand the real problem
2. Discover hidden requirements and constraints
3. Design multiple architectures
4. Challenge each design from multiple engineering perspectives
5. Select the strongest tradeoffs
6. Implement incrementally (correct first, fast later)
7. Validate correctness continuously
8. Self-review: races, leaks, API consistency, maintenance cost
9. Update documentation to match implementation
10. Refactor until production quality

Implementation is the final step — not the first.

---

## Multi-perspective review

Before accepting any significant design, review from:

- Software Architect
- Security Engineer
- Reliability / SRE
- Performance Engineer
- Future Maintainer
- Developer Experience

Each perspective should search for weaknesses, not consensus.
