# core_L4 positivity — vacuum-mask / positivity-preserving flux sweep (claim-60 support)

Scope: literature sweep for claim-60. Target: vacuum-mask / positivity-preserving
flux numerics for density-only problems in the same class as widened_ritz
flux elimination (exact 1D flux elimination, zero left flux, slice-normalized
mass, exp-parameterized positivity). Record only. No verdict on claim-60.

Date: 2026-11-06. Search: web sweep 2026-11-06 (Hu-Adams-Shu, Zhang-Shu,
Huo-Liu quantum diffusion, Yasue/SVM, Madelung/QHD). This note does not
re-run any code.

## Local reference numbers (for claim-60 use, not literature)

- Mask / floor value (code):
  - `theory/field_variation.py:174-180`: `safe = maximum(rm, 1e-150)`,
    `live = (rm > 1e-12)`. Comment: vacuum cells below 1e-12 carry only
    floating-point noise (flux plateau residue ~1e-14 over true-vacuum
    densities ~1e-31); true action contribution ~1e-12.
  - `theory/widened_ritz.py:40,171-181`: `FLOOR = 1e-12`, same
    `safe 1e-150` + `live (rm > FLOOR)` pattern.
  - `theory/field_variation.py:93`: bump-start floor `maximum(..., 1e-12)`.
- Adjoint verification method (code):
  - Analytic adjoint with exact transpose `_dT` of the central/one-sided
    difference operator (`field_variation.py:221-235`,
    `widened_ritz.py:43-56`).
  - Verified by finite differences: `test_gradient_matches_finite_differences`
    (claim-58 record: analytic adjoint verified to 2e-7 by finite differences;
    L-BFGS-B with analytic adjoint). Same pattern in widened_ritz tests.
- Equation numbers (repo):
  - `theory/formulas.md` F42: unstructured G-M field variation (flux-elimination
    BVP, vacuum-exploit diagnosis, nu=0 control).
  - `theory/formulas.md` F43: Ritz-widened squared-Hermite trial (midpoint flux
    + L-BFGS/Newton + attribution gates).
  - Supporting: F32 (osmotic-closure no-go), F38-F41 (bridge/Yasue BVP family).

## Literature hits (none in same class)

1. Hu, Adams, Shu, "Positivity-preserving flux limiters for high-order
   conservative schemes," arXiv:1203.1540 (JCP 2013). Cut-off flux limiter:
   Eq.(5)/(11) convex combination `U = 1/2 U- + 1/2 U+`, sufficient condition
   `rho(U±)>0, p(U±)>0`. Tests: double rarefaction (vacuum), Sedov blast,
   with WENO-5. Class: compressible Euler (rho,m,E) with pressure. Not
   density-only; no fixed-endpoint density BVP; no 1e-12 mask number.
2. Zhang-Shu framework + follow-ups (DG/finite-volume/WENO positivity,
   e.g. Zhang-Shu 2012 JCP; Huang-Ren-Wang 2023 arXiv:2306.13483 implicit FV).
   All enforce `rho>0, p>0` via limiters / low-order baseline / CFL or residual
   correction. Class: Euler/Navier-Stokes with momentum+energy. Not density-only
   quantum-action BVP; no 1e-12 masked BVP numbers.
3. Huo-Liu, "A positivity-preserving and energy stable scheme for a quantum
   diffusion equation," arXiv:1912.00813. Closest topic match (quantum
   diffusion, Bohm potential, positivity + energy stability, JKO-type
   minimization Eq.(1.5), linearized `M^n` scheme Eq.(2.8) unconditionally
   positivity-preserving Lemma 2.2). Class: time-stepping quantum drift-diffusion,
   not fixed-endpoint density-only BVP with exact 1D flux elimination; no 1e-12
   vacuum-mask BVP number reported.
4. Yasue / stochastic variational method (Yasue 1981 JFA 41:327-340; Guerra-Morato
   PRD 27:1774; Pavon JMP 36:6774; Carlen/Leonard-Schrodinger-bridge surveys;
   Kodama-Koide; Morato 2022 arXiv:2206.04338). Variational principles and
   Yasue-action vs relative-entropy equivalence. No vacuum-mask numerics, no
   adjoint-verified flux-elimination code, no BVP numbers.
5. Madelung/QHD foundations (Madelung 1927; Wallstrom; Foskett-Tronci 2023
   arXiv:2003.08664; Fusca 2017; Schuch/Bonilla-Licea; Zylberman et al. PRA 106).
   Node/vortex singularity discussion (multi-valued phase at rho=0). No
   positivity-preserving density-only BVP numerics with mask/floor reported.

## Explicit negative

No 1e-12 masked BVP numbers found: none of the sources above reports a
density-only fixed-endpoint BVP with exact 1D flux elimination, slice-normalized
mass, exp-parameterized positivity, and a 1e-12 vacuum mask / 1e-150 safe floor
with finite-difference-verified analytic adjoint. The 1e-12 mask is a local
implementation choice (`field_variation.py`, `widened_ritz.py`), not a literature
value recovered by this sweep.

No verdict rendered on claim-60.
