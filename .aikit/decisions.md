# Decisions

Why this project is built the way it is, oldest first. One entry per decision that changes a
contract, a dependency, a data format or the architecture; IDs are consecutive and never reused.

<!-- Entry format:
### ADR-NNN: <decision in one line>
- **Date**: YYYY-MM-DD
- **Status**: Accepted
- **Context**: <what forced the decision>
- **Decision**: <what was decided>
- **Consequences**: <what follows, including what it costs>
-->

### ADR-001: Pre-registered Claim-15 production scan with spectral upsampling and 15-minute wall-clock cap (G-003)
- **Date**: 2026-10-26
- **Status**: Accepted
- **Context**: An unbounded `N=96, steps=9000` CPU gradient flow took ~75 minutes and was terminated by user direction, while the native `N=64` Chern-Simons Hopf estimator drops below 1 when grid-scale gradients develop under the Z3 potential.
- **Decision**: Enforce a hard 15-minute (`900.0 s`) wall-clock budget (`G-003`, `MAX_WALL_SECONDS = 900.0`) on `sims/production.py`, scope `phenomenology/production_report.py` to `N=64, steps=3600` (~3.1 min total) with $2\times$ spectral upsampling (`N=128`), `N=48` `dt` convergence and `N=32` perturbed-seed checks, and evaluate Claim 15 against pre-registered tolerances (`H_TOL`, `Z3_MIN`, `FORCE_TOL`, `DRIFT_TOL`, `BOND_TOL`, `PRE_NORM_TOL`).
- **Consequences**: `make production` completes in ~3.1 minutes; spectral upsampling shows the smooth topological charge stays near 1 (`0.989238`) while native `H` drops (`0.788000`) due to under-resolved bond differences (`1.480 > 0.50`), keeping Claim 15 rigorously `UNVERIFIED`.

## Licensing migration (2026-10-26)

- New versions use the Aegean Science License 1.1 (ASL-1.1).
- Rights holder: AegeanWorks; commercial use under ACL-1.0.
- Counsel review of the 1.1 texts remains pending.

### ADR-002: Pre-registered N=96 resolution check for claim 15
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: The N=64 production scan (ADR-001) gives native \(H=0.787999784\), 2x upsampled \(H=0.989238052\), \(Z_3=0.607430\), force RMS \(3.716\times10^{-2}\) and max bond \(1.480\). Claim 15 is UNVERIFIED because the native and upsampled Hopf estimates disagree and the field is under-resolved, so one resolution cannot separate a grid-scale estimator artifact from a real failure of the configuration.
- **Decision**: Before the run, fix a second resolution at N=96 with the same deterministic scan (length 10, \(\mu=20\), 3000 steps, \(dt=5\times10^{-4}\), seed 20261026, chunk 600, fixed vacuum faces, 2x spectral upsampling), plus a fresh N=64 rerun in the same process for a same-environment comparison. Both run within one 900 s G-003 budget. The resolution diagnostic is `resolution_converged` only when all five hold: \(|H_{96}-H_{64}|\le10^{-2}\), \(|H^{\rm up}_{96}-H^{\rm up}_{64}|\le10^{-2}\), \(|Z_3^{96}-Z_3^{64}|\le2\times10^{-2}\), \(\mathrm{force}^{96}\le\mathrm{force}^{64}\), and \(\mathrm{maxbond}^{96}\le\mathrm{maxbond}^{64}\). The claim-15 acceptance rule is unchanged: the convergence report cannot promote the claim, and claim 15 is VERIFIED only if the N=96 run itself satisfies the existing `H_TOL`, `Z3_MIN`, `FORCE_TOL`, `DRIFT_TOL`, `BOND_TOL` and `PRE_NORM_TOL` criteria.
- **Revision (2026-10-30)**: The first attempt at 3600 steps for both resolutions exceeded the 900 s cap during the N=96 scan (the fresh N=64 scan alone takes about 226 s and the N=96 scan about 680 s). Both scans in the convergence check therefore use 3000 steps, the same physical time for the comparison; the 3600-step production result remains the primary claim-15 evidence. No acceptance or resolution tolerance changed. A fresh 3600-step N=64 run reproduced the committed production values to the reported precision.
- **Consequences**: New `sims/convergence.py`, `phenomenology/convergence_report.py`, `tests/test_convergence.py`, `results/hopfion3d_convergence.{json,md}` and `make convergence`. If native \(H\) moves toward 1 while max bond falls, the under-resolution explanation is strengthened; if not, the deficit is not a simple resolution artifact. Two 3000-step scans cost about 12 minutes, inside the per-process 900 s cap.

### ADR-003: Pre-registered same-time dt check at N=64 for claim 15
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: The N=96 resolution check (ADR-002) gives \(Z_3=0.172071\) at \(t=1.5\), against \(0.528226\) at N=64. Both runs used \(dt=5\times10^{-4}\), so the gap could be spatial resolution or temporal discretization. The production dt check at N=48 only covers \(t=0.15\).
- **Decision**: Before interpreting the N=96 gap, run N=64 at \(dt=5\times10^{-4}\) for 3000 steps and at \(dt=2.5\times10^{-4}\) for 6000 steps, the same \(t=1.5\), seed 20261026, length 10, \(\mu=20\), chunk 600 and 2x upsampling, in one process under the 900 s G-003 budget. The pre-registered temporal criteria are \(|H_{\rm fine}-H_{\rm coarse}|\le5\times10^{-3}\), \(|H^{\rm up}_{\rm fine}-H^{\rm up}_{\rm coarse}|\le5\times10^{-3}\) and \(|Z_{3,\rm fine}-Z_{3,\rm coarse}|\le2\times10^{-2}\). The claim-15 acceptance rule is unchanged and the check cannot promote it.
- **Consequences**: New `results/hopfion3d_temporal.{json,md}` and `make temporal`. If the criteria pass, the N=64 \(Z_3\) at \(t=1.5\) is temporally resolved and the N=96 gap is spatial (or a longer-time effect); if they fail, the N=96 comparison is confounded by the timestep and the published wording must be softened.

### ADR-004: Pre-registered, checkpointed N=96 extension for claim 15
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: At N=96 and \(t=1.5\), \(Z_3=0.172071\) and still rising by about 0.034 per 600 steps, while the force RMS is \(1.495\times10^{-2}\) and falling slowly. A single time point cannot tell whether \(Z_3\) reaches the 0.5 threshold, saturates below it, or whether the force ever reaches \(10^{-2}\). A 3000-step N=96 chunk takes about 600 s, but 9000 steps would exceed the 900 s G-003 cap in one process.
- **Decision**: Add an `initial_field`/`return_field` path to `run_production`, run consecutive 3000-step N=96 chunks (length 10, \(\mu=20\), \(dt=5\times10^{-4}\), seed 20261026, chunk 600, 2x upsampling), and save the final field atomically between chunks. Each invocation runs exactly one chunk under the 900 s cap. The extension reports \(H\), \(H^{\rm up}\), \(Z_3\), force RMS and max bond at \(t=1.5, 3.0, 4.5\) (up to four chunks). The claim-15 acceptance rule is unchanged: the extension cannot promote the claim unless the final state itself satisfies `H_TOL`, `Z3_MIN`, `FORCE_TOL`, `DRIFT_TOL`, `BOND_TOL` and `PRE_NORM_TOL`. Linear projections to \(Z_3=0.5\) and force RMS \(=10^{-2}\) are reported as diagnostics, not claims.
- **Consequences**: New `phenomenology/extension_report.py`, `make extend`, `results/hopfion3d_extension.{json,md}`, and a transient `results/hopfion3d_extension_state.npy` that is git-ignored. A chunking-exactness test requires two 3-step chunks to reproduce one 6-step run. The extension stops after four chunks or when the unchanged criteria pass; a saturated trend is a valid negative result.

### ADR-005: Pre-registered Hessian of the pure-FN N=64 candidate (claim 14b)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: The only stability test so far (14d) used the scale-iteration candidate, which is a saddle with 8 negative modes. The pure-Faddeev-Niemi candidate at N=64, \(\mu=0\), 1200 steps is a different field with \(H=0.99999993\) and force RMS \(3.15\times10^{-3}\), but its tangent Hessian has never been computed.
- **Decision**: Build the pure-FN candidate at N=64 (length 10, \(\mu=0\), 1200 steps, \(dt=5\times10^{-4}\), fixed vacuum faces) with the production runner's `return_field` path, validate the tangent operator at N=8, and compute the eight lowest tangent-space eigenvalues. The pre-registered negative tolerance is \(10^{-6}\). A positive spectrum raises claim 14b to **NUMERIC (local minimum at N=64; not a proof)** and records the candidate's stability as **NUMERIC (local minimum at N=64)**; a negative spectrum leaves 14b UNVERIFIED and falsifies this candidate (recorded as claim 14e). A single-grid local minimum is never a proof of stability.
- **Consequences**: New `phenomenology/purefn_stability_report.py`, `make purefn`, `tests/test_purefn_stability.py`, and `results/hopfion_purefn_stability.{json,md}`. The relax plus the matrix-free spectrum must fit the 900 s G-003 cap; the report aborts before writing if it does not.
