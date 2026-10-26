# Project Guidelines

Rules for this project only, written by people or AI agents. They add to AIKit's principles and
rank below them: a guideline never overrides a principle (PRINCIPLE-000 R6, PRINCIPLE-001 R4).
Add one with `python aikit.py guideline <name> "<rule>" --source <user|ADR-NNN|person>`, or by
hand as one line below in the form `- **G-NNN** (source): rule`; IDs are never reused.

Persisted state carries a format version. A loader refuses a version it does not support,
without modifying or quarantining that file; older versions migrate when a migration exists.

## Rules
- **G-001** (user): Every claim in `claims.md` marked PROVEN, NUMERIC, or VERIFIED must be independently re-derived in two separate runs (separate processes), never twice inside one run; the per-claim results must agree before the claim is accepted. This applies to claims, not to the pytest suite; tests remain a single run unless explicitly requested.
- **G-003** (user): Every simulation, diagnostic, or verification run must complete within a hard 15-minute (900-second) wall-clock limit.
