# fluid-universe

<!-- aegean:media:start -->
<p align="center"><img src="assets/social-preview.png" alt="Fluid-Universe social preview" width="100%"></p>
<p align="center"><a href="LICENSE"><img src="https://img.shields.io/badge/license-ASL--1.1-1c7ed6" alt="License"></a> <a href="https://github.com/AegeanWorks/AIKit"><img src="https://img.shields.io/badge/AIKit-integrated-6f42c1" alt="AIKit Integrated"></a></p>
<!-- aegean:media:end -->


Falsification-first research codebase testing a Z3 circulant origin of the Koide lepton relation, with a separately labelled nematic-fluid UV speculation.

**Epistemic status (2026-10-25):** the algebra is verified and is likely known (Brannen 2006). The Z3/circulant structure by itself does **not** predict the Koide value; it has three parameters for three masses, and Koide is the extra condition \(|c|/a=1/\sqrt2\). The exact one-loop statement "\(|c|/a\) is RG-invariant" is false, although the numerical drift for charged leptons is only about \(3\times10^{-6}\) over 16 decades. The nematic-fluid UV track has **no** validated calculation yet and is kept in a separate, clearly speculative section.

## 30-Second Cold-Start Quick Reference

- **Runtime Prerequisites**: Python 3.10 or newer; `numpy`, `scipy`, `sympy`, `jax`; `pytest` and `hypothesis` for the test suite.
- **Install**: `python -m pip install -e ".[dev]"`
- **Canonical Regression Shield Gate**:
  ```bash
  python -m pytest -q
  ```
- **Regenerate every number**:
  ```bash
  python -m phenomenology.run_all
  ```
  This writes `results/pdg_fit.json`, `results/pdg_fit.md`, `results/rg_run.json`, `results/rg_run.md`, `results/rotor.json`, `results/rotor.md`, `results/neutrinos.json`, `results/neutrinos.md`, `results/quarks.json`, `results/quarks.md`, `results/pmns.json`, `results/pmns.md`, `results/delta_search.json`, `results/delta_search.md`, `results/frank_oseen.json`, `results/frank_oseen.md`, `results/derrick.json`, `results/derrick.md`, `results/topology.json`, `results/topology.md`, `results/reduction.json`, `results/reduction.md`, `results/predictions.json`, `results/predictions.md`, `results/lorentz.json`, `results/lorentz.md`, `results/sim_core.json`, `results/sim_core.md`, `results/hopf.json`, `results/hopf.md`, `results/hopfion.json`, `results/hopfion.md`, `results/hopfion3d.json`, `results/hopfion3d.md`, `results/hopf_resolution.json`, `results/hopf_resolution.md`, and `results/summary.md`, each with a config hash, package versions, and git commit (or `dirty`).
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
- **Claim audit status**: two separate runs agree per claim, 173/173 checks each (`python -m verification.verify`).
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
| Closed form for \(\delta\) | UNVERIFIED (null): 2/9 at 0.407\(\sigma\), look-elsewhere \(p=7.9\times10^{-4}\), not promoted | `results/delta_search.json` |
| Frank-Oseen equal-\(K\) dispersion | PROVEN for that statement only: 2 transverse modes, no photon | `results/frank_oseen.json` |
| Derrick scaling | PROVEN: \(d=3\) exponents \((1,-1,3)\), virial \(E_2-E_4+3V=0\) | `results/derrick.json` |
| Stable Hopfion existence | UNVERIFIED; \(\kappa>0\) is necessary, not sufficient | `results/derrick.json` |
| Finite-energy H=1 configuration | NUMERIC: \(E_2=233.6\), \(E_4=159.3\), \(H=1.000000\), \(\lambda^*=0.82563\); no stability proof | `results/hopfion.json` |
| Pure-FN H=1 candidate | N=64, force RMS 0.00315, \(H=0.99999993\); near-stationary, not a stability proof | `results/hopfion3d_n64.json` |
| Scale-stationary H=1 candidate | NUMERIC candidate: virial 0.00961, \(H=0.99730\), RMS 0.0030 | `results/hopfion_scale.json` |
| Stability of that candidate | FALSIFIED: validated tangent Hessian, min \(-5.799863\) with 8 negative modes at N=64; the candidate is a saddle | `results/hopfion_stability.json` |
| 2D reduction lower bound | PROVEN (trivial): \(E_{2D}\ge T L_y\); equality only for \(y\)-independent walls | `results/reduction.json` |
| Exact reduction / finite \(I_{\rm eff}\) | FALSIFIED: \(1/r^4\) tail gives log-divergent \(I_{\rm eff}\) | `results/reduction.json` |
| Z3 from nematic topology | FALSIFIED: \(RP^2\) has no Z3 factor; \(\cos3\phi\) inserted by hand | `results/topology.json` |
| New prediction beyond Koide | UNVERIFIED (none found): seven candidate classes audited | `results/predictions.json` |
| Emergent metric at tree level | FALSIFIED: preferred frame; no metric; \(\xi=1.00348\) for \(K=(1,2,3)\) | `results/lorentz.json` |
| One-loop RG to a Lorentz-invariant IR | UNVERIFIED: no calculation | `results/lorentz.json` |
| Photon LV bounds on director modes | UNVERIFIED (not applicable): modes are neutral | `results/lorentz.json` |
| 1D simulation baseline | PASS: kink, gradient, monotone flow, rotor; Hessian partial | `results/sim_core.json` |
| Hopf preimage linking | PASS: H=0.999958 at 400 samples; reflection flips sign; rotation/shear invariant | `results/hopf.json` |
| Hopf grid Chern-Simons estimator | PASS (normalized): 1.000000 at N=64; agrees with preimage | `results/hopf.json` |
| Hopf grid resolution | N>=32 within 1e-3; N=16/24 under-resolved | `results/hopf_resolution.json` |
| 3D E2+E4+Z3 pilot | UNVERIFIED: N=32 not converged/resolution-limited; H/Z3 vary with steps; no conclusion | `results/hopfion3d.json` |
| N=64 3D diagnostic | Targeted, not part of `make results`: `make n64`; H falls below 0.8 as Z3 exceeds 0.6; no coexistence; not converged | `results/hopfion3d_n64.json` |
| N=64 candidate Hessian | Targeted, not part of `make results`: `make stability`; dense-validated tangent operator; min \(-5.799863\), 8 negative modes; saddle | `results/hopfion_stability.json` |
| 3D Hopfion + Z3 order | UNVERIFIED: N=64 production scan (`make production`, <15 min G-003) gives native H=0.7880, 2x upsampled H=0.9892, Z3=0.6074, max bond 1.480, min pre-norm 1.000000; native H drop is an estimator artifact; not converged/under-resolved | `results/hopfion3d_production.json` |
| N=96 resolution check | UNVERIFIED: at the same physical time (3000 steps), native H=0.99994 and upsampled H=1.00000, but Z3=0.17207 and force RMS 0.01495; the N=64 Z3 order is not spatially converged, so no coexistence is demonstrated | `results/hopfion3d_convergence.json` |
| N=64 dt check | PASS: 3000 steps at dt=5e-4 versus 6000 at dt=2.5e-4, both at t=1.5, give ΔH=-1.07e-4 and ΔZ3=-4.6e-6; the N=96 Z3 gap is not a timestep artifact | `results/hopfion3d_temporal.json` |
| Neutrino \(Q_\nu\) for both orderings | NUMERIC negative: max \(0.585612\), never \(2/3\) | `results/neutrinos.json` |
| Minimal circulant PMNS branches | FALSIFIED: both no-mixing and trimaximal branches excluded | `results/pmns.json` |
| Direct quark Koide extension | FALSIFIED: \(Q_U\approx0.888\), \(Q_D\approx0.748\) at \(M_Z\) | `results/quarks.json` |
| Nematic-fluid UV picture | SPECULATION; structural F-O/Derrick checks only | `sims/README.md` |

## What's Inside

```text
fluid-universe/
├── .aikit/                   <- project memory: ledger and decisions
├── .gitignore                <- transient AIKit state excluded
├── COMMERCIAL.md             <- commercial licensing terms
├── LICENSE                   <- Aegean Science License 1.1 (ASL-1.1): free non-commercial research
├── LICENSES/                 <- REUSE license texts
├── REUSE.toml                <- Machine-readable SPDX metadata
├── ASSETS.md                 <- Asset/data manifest: what the license covers
├── assets/                   <- brand artwork and social preview
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
├── sims/                     <- validated 1D core and bounded 3D diagnostics
├── paper/                    <- two LaTeX documents, separate scopes
└── results/                  <- machine-generated outputs only
```

## Conventions and Non-Negotiables

- Every number in prose must regenerate from `python -m phenomenology.run_all`; the JSON records seed, config hash, package versions, and git commit.
- Labels: **derived**, **assumed**, **analogy**, **speculation**. A numerical fit is not a derivation.
- No production 3D simulation number is cited until `sims/core.py` passes the full baseline set in `sims/README.md`. Earlier lost-run numbers (\(|H|\sim0.16\), \(Z_3\sim0.72\), \(N=36\)) are **not** used.
- Negative results are kept. Corrections go to `CHANGELOG.md`; they are not silently edited.
- The Z3/Koide/RG paper must stand without any fluid-universe material.

<!-- aegean:brand-assets:start -->
## 🎨 Brand Assets

> **AI-Generated Brand Artwork.** The banner and logo are **AI-generated brand assets**. They are illustrative and may not literally depict every feature or the full project scope.
<!-- aegean:brand-assets:end -->

## 📜 License

Free for non-commercial research and teaching under the Aegean Science License 1.1 (ASL-1.1); see [LICENSE](LICENSE). It allows publication, reproducibility sharing, text/data mining and non-commercial ML training. Commercial or industry-sponsored use needs the Aegean Commercial License 1.0 (ACL-1.0); see [COMMERCIAL.md](COMMERCIAL.md).
