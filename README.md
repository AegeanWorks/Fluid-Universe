# fluid-universe

[![Built with AIKit](https://img.shields.io/badge/built%20with-AIKit-6f42c1)](https://github.com/AegeanWorks/AIKit)

Falsification-first research codebase testing a Z3 circulant origin of the Koide lepton relation, with a separately labelled nematic-fluid UV speculation.

**Epistemic status (2026-10-25):** the algebra is verified and is likely known (Brannen 2006). The Z3/circulant structure by itself does **not** predict the Koide value; it has three parameters for three masses, and Koide is the extra condition \(|c|/a=1/\sqrt2\). The exact one-loop statement "\(|c|/a\) is RG-invariant" is false, although the numerical drift for charged leptons is only about \(3\times10^{-6}\) over 16 decades. The nematic-fluid UV track has **no** validated calculation yet and is kept in a separate, clearly speculative section.

## 30-Second Cold-Start Quick Reference

- **Runtime Prerequisites**: Python 3.10 or newer; `numpy`, `scipy`, `sympy`; `pytest` and `hypothesis` for the test suite.
- **Install**: `python -m pip install -e ".[dev]"`
- **Canonical Regression Shield Gate**:
  ```bash
  python -m pytest -q
  ```
- **Regenerate every number**:
  ```bash
  python -m phenomenology.run_all
  ```
  This writes `results/pdg_fit.json`, `results/pdg_fit.md`, `results/rg_run.json`, `results/rg_run.md`, `results/rotor.json`, `results/rotor.md`, and `results/summary.md`, each with a config hash, package versions, and git commit (or `dirty`).
- **Verify every claim independently (two runs)**:
  ```bash
  python -m verification.verify
  # or, where make is installed:
  make verify
  # explicit steps, always two separate processes:
  python -m verification.audit --tag run1
  python -m verification.audit --tag run2
  python -m verification.compare
  ```
  This regenerates the physics outputs, runs the audit in two separate processes, and compares the per-claim results. The audit re-derives every PROVEN, VERIFIED, or NUMERIC claim without importing the project modules. Project rule G-001: every claim is verified twice, in two separate processes, and the results must agree; a single run is not accepted. This applies to claims, not to the pytest suite. The combined result is `results/verification.{json,md}`; a bare `python -m verification.audit` writes `verification_single.*` and is marked NOT ACCEPTED.
- **Claim audit status**: two separate runs agree per claim, 66/66 checks each (`python -m verification.verify`).
- **Read the ledger first**: [`claims.md`](claims.md), [`SCORECARD.md`](SCORECARD.md), [`CHANGELOG.md`](CHANGELOG.md), [`theory/formulas.md`](theory/formulas.md).

### AIKit Commands (run from the AIKit folder)

| Role | Command |
|---|---|
| Status, task and gates | `python aikit.py status fluid-universe` |
| Phase 4: every gate | `python aikit.py check fluid-universe` |
| Repair memory and map, then check | `python aikit.py fix fluid-universe` |
| Resume an interrupted task | `python aikit.py resume fluid-universe` |

## What Is Actually Established

| Item | Verdict | Where |
|---|---|---|
| Hermitian circulant \(M=\mathrm{circ}(a,ce^{i\delta},ce^{-i\delta})\) has eigenvalues \(a+2c\cos(\delta+2\pi k/3)\) and DFT eigenvectors independent of \(\delta\) | PROVEN; likely known | `theory/circulant.py`, `tests/test_circulant.py` |
| \(Q=\sum m/(\sum\sqrt m)^2=1/3+2c^2/(3a^2)\) for the signed eigenvalues | PROVEN, with a positivity condition; likely known | `theory/formulas.md` F1-F3 |
| \(Q=2/3\) iff \(\lvert c\rvert/a=1/\sqrt2\) | PROVEN algebra; NOT a prediction of Z3 alone | `claims.md` 1a/1b |
| Pole-mass fit and uncertainty propagation | NUMERIC | `phenomenology/fit.py`, `results/pdg_fit.json` |
| \(Q=2/3\) for MS-bar masses at \(M_Z\) | FALSIFIED: \(Q(M_Z)\approx0.6678\) | `results/rg_run.json` |
| \(\lvert c\rvert/a\) exactly RG-invariant at one loop | FALSIFIED; drift is numerically tiny | `phenomenology/rg.py`, `results/rg_run.json` |
| Rotor spectrum and free-rotor limit | PROVEN numerically | `theory/rotor.py`, `results/rotor.json` |
| Rotor \(\to\) three distinct masses | FALSIFIED: sectors 1 and 2 are exactly degenerate | `results/rotor.json` |
| Domain-wall tension \((8/3)\sqrt\mu\) | PROVEN | `results/rotor.json` |
| Nematic-fluid UV picture | SPECULATION; no validated claim | `sims/README.md` |

## What's Inside

```text
fluid-universe/
├── .aikit/                   <- project memory: ledger and decisions
├── .gitignore                <- transient AIKit state excluded
├── COMMERCIAL.md             <- commercial licensing terms
├── LICENSE                   <- GNU AGPL-3.0
├── README.md
├── Makefile                  <- test, results, and two-run verification targets
├── pyproject.toml            <- dependencies and pytest configuration
├── claims.md                 <- claim ledger with statuses and kill tests
├── CHANGELOG.md              <- append-only corrections and retractions
├── SCORECARD.md              <- one-line status per claim
├── theory/                   <- symbolic algebra, formula registry, literature notes
├── phenomenology/            <- PDG data, circulant fit, one-loop SM RGE
├── tests/                    <- pytest identities, limits, regression numbers
├── verification/             <- independent re-derivation of verified claims
├── sims/                     <- simulation protocol; no validated core yet
├── paper/                    <- two LaTeX documents, separate scopes
└── results/                  <- machine-generated outputs only
```

## Conventions and Non-Negotiables

- Every number in prose must regenerate from `python -m phenomenology.run_all`; the JSON records seed, config hash, package versions, and git commit.
- Labels: **derived**, **assumed**, **analogy**, **speculation**. A numerical fit is not a derivation.
- No simulation number is cited until `sims/core.py` passes the baselines listed in `sims/README.md`. Earlier lost-run numbers (\(|H|\sim0.16\), \(Z_3\sim0.72\), \(N=36\)) are **not** used.
- Negative results are kept. Corrections go to `CHANGELOG.md`; they are not silently edited.
- The Z3/Koide/RG paper must stand without any fluid-universe material.

## License

GNU Affero General Public License v3.0; see [LICENSE](LICENSE). For commercial licensing and proprietary rights, see [COMMERCIAL.md](COMMERCIAL.md).
