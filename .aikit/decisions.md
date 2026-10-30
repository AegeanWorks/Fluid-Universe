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

### ADR-006: Pre-registered saddle-escape search from the pure-FN candidate (claim 14b)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: The scale-iteration candidate (14d, minimum \(-5.799863\)) and the pure-FN candidate (14e, minimum \(-2.600799\)) are both saddles. A negative Hessian eigenvector is a principled descent direction; following it may reach a nearby local minimum rather than only proving the starting point unstable.
- **Decision**: Build the pure-FN N=64 candidate, compute the lowest tangent eigenvector, normalize it to unit interior RMS, and perturb the candidate with \(\epsilon=0.05\) and \(0.15\) (two pre-registered amplitudes). Relax each perturbed field for 600 steps at \(\mu=0\) and accept it for a Hessian test only if \(H>0.9\) and force RMS \(<0.05\). A candidate with minimum eigenvalue \(\ge-10^{-6}\) raises claim 14b to **NUMERIC (local minimum at N=64; not a proof)**; otherwise 14b stays UNVERIFIED and the search is a negative result. The whole search (candidate build, one eigenvector, two relaxations and at most two Hessians) must fit one 900 s G-003 process.
- **Consequences**: New `sims.stability.hessian_modes`/`perturb_along_mode`, `phenomenology/saddle_escape_report.py`, `make escape`, `tests/test_saddle_escape.py`, and `results/hopfion_saddle_escape.{json,md}`. A collapsed or unresolved perturbed field is recorded, not hidden; no result is promoted from a single grid.

### ADR-007: Pre-registered box-length scan of the pure-FN candidate (claim 14b)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: The pure-FN candidate at length 10 is a saddle (minimum \(-2.600799\)) and its lowest-mode escape collapses to \(H\approx0.32\). The box length sets the soliton-to-boundary ratio and the virial balance, so a different length could admit a local minimum. No length scan has been run.
- **Decision**: At N=64, \(\mu=0\), 1200 steps and \(dt=5\times10^{-4}\), scan lengths \(L=8,10,12,14\). For each, accept the relaxed field for a Hessian test only if \(H>0.9\) and force RMS \(<0.05\), then compute the eight lowest tangent eigenvalues. A candidate with minimum eigenvalue \(\ge-10^{-6}\) raises claim 14b to **NUMERIC (local minimum at N=64; not a proof)**; otherwise 14b stays UNVERIFIED. Stop at the first local minimum. The whole scan must fit one 900 s G-003 process.
- **Consequences**: New `phenomenology/length_scan_report.py`, `make lengthscan`, `results/hopfion_length_scan.{json,md}`, and audit checks. A saddle at every length is a valid negative result; a local minimum at one length is single-grid evidence, not a proof.

### ADR-008: Pre-registered vortex-knot mass hypothesis (claim 17)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: The user direction is that particles are vortices in a fluid. The simplest falsifiable realization assigns the three charged leptons to the lowest torus knots \(T(2,3), T(2,5), T(2,7)\) and sets the rest mass proportional to the thin-filament Biot-Savart energy at a fixed core radius.
- **Decision**: Use \(R=2\), \(r=1\), 400 samples, core radius \(a=0.1\), and the regularized proxy \(E=\sum_{i\ne j}\Delta r_i\cdot\Delta r_j/r_{ij}+L/a\). Pre-register the test: the hypothesis is falsified if the charged-lepton mass ratios are not reproduced within \(10\%\) or if the Koide \(Q\) of the knot energies differs from \(2/3\) by more than \(0.01\). Report the power \(p\) in \(m\propto E^p\) that would be required to match the hierarchy. Falsify only this simplest assignment, not the general fluid-vortex idea.
- **Consequences**: New `theory/vortex_particles.py`, `phenomenology/vortex_spectrum_report.py`, `tests/test_vortex_particles.py`, `results/vortex_knot_spectrum.{json,md}`, and claim 17. A negative is expected if the thin-filament spectrum is too compressed; it identifies the missing hierarchy mechanism (for example an exponentially small core or a topological energy gap).

### ADR-009: Pre-registered one-parameter vortex hierarchy test (claim 18)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: Claim 17 falsifies the direct thin-filament knot spectrum. A viable vortex model needs a hierarchy mechanism. The simplest one-parameter hierarchies are a geometric cascade \(m_n\propto\lambda^n\) and a power law \(m_n\propto(n+1)^p\); each has one shape parameter and therefore predicts the remaining mass ratio and the Koide \(Q\).
- **Decision**: For each model, fit the single parameter to \(m_\mu/m_e\), then predict \(m_\tau/m_e\) and \(Q\). Also solve \(Q=2/3\) for the parameter and predict \(m_\tau/m_e\). The hypothesis is falsified if any prediction misses the observed value by more than \(10\%\) in ratio or \(0.01\) in \(Q\). A two-parameter offset model may be reported as a fit, but it is not a prediction and cannot rescue the one-parameter test.
- **Consequences**: New `theory/vortex_cascade.py`, `phenomenology/vortex_cascade_report.py`, `tests/test_vortex_cascade.py`, `results/vortex_cascade.{json,md}`, and claim 18. A negative means a single fluid hierarchy cannot explain both the ratios and Koide; the next model needs an explicit Z3/circulant coupling or another fixed structure.

### ADR-010: Pre-registered real Z3 rotor potential scan (claim 19)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: Claim 5c shows that the real \(\cos3\phi\) rotor has exactly degenerate \(k=1,2\) sectors, so the naive map gives only two distinct masses. The user direction is a changed potential. A real potential alone cannot break the antiunitary degeneracy; a fractional flux can, but the resulting spectrum may still be too compressed to reach Koide \(Q=2/3\).
- **Decision**: Scan \(V(\phi)=\mu(1-\cos3\phi)+\nu(1-\cos6\phi)\) with a flux \(A\) in the kinetic term \((L-A)^2/(2I)\). Fix \(\mu=1\), \(I=1\), \(n_{\rm cut}=120\), \(\nu\in\{0,1,3,10\}\) and \(A\in\{0,0.1,\ldots,1\}\). For each point compute the three lowest sector energies \(E_0,E_1,E_2\) and \(Q=\sum E/(\sum\sqrt E)^2\). The hypothesis is falsified if the maximum \(Q\) is below \(2/3-0.01\). A zero-flux real potential must preserve \(E_1=E_2\); record that as the structural check. A single high point is not enough: check the \(n_{\rm cut}\) convergence at the best point.
- **Consequences**: New `theory/rotor_potential.py`, `phenomenology/rotor_potential_report.py`, `tests/test_rotor_potential.py`, `results/rotor_potential.{json,md}`, and claim 19. A negative means a real potential plus a flux is insufficient; a complex phase or a different mass mapping is required.

### ADR-011: Madelung fluid dictionary and numerical validation (claim 20)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: The user goal is to explain quantum mechanics with fluid mechanics. The exact bridge is the Madelung representation \(\psi=\sqrt{\rho}e^{iS/\hbar}\), which maps the Schrodinger equation to a continuity equation and an Euler equation with the quantum pressure \(Q=-\frac{\hbar^2}{2m}\nabla^2\sqrt{\rho}/\sqrt{\rho}\).
- **Decision**: Implement the dictionary and integrate the equivalent Schrodinger form with a split-step Fourier method because the quantum pressure is stiff. Validate on a free Gaussian packet (exact spreading) and a harmonic-oscillator coherent state (exact displaced Gaussian), and check the continuity and Euler residuals. Pre-register: free-Gaussian density \(L^2\) error \(<10^{-10}\), coherent-state error \(<10^{-3}\) at \(dt=10^{-4}\), and relative fluid residuals \(<10^{-2}\). This establishes the exact dictionary; it does not derive the quantum pressure from a microphysical fluid.
- **Consequences**: New `theory/madelung.py`, `phenomenology/madelung_report.py`, `tests/test_madelung.py`, `results/madelung.{json,md}`, and claim 20. A pass is the foundation for a microphysical model (stochastic mechanics, pilot wave, or walking droplets); the report must state that the quantum pressure is nonlocal and singular at nodes.

### ADR-012: Nelson stochastic kinematics and validation (claim 21)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: The Madelung dictionary is exact but does not derive the quantum pressure. Nelson stochastic mechanics gives it a physical origin: a Brownian particle with diffusion \(D=\hbar/(2m)\) has an osmotic velocity \(u=D\partial_x\ln\rho\), and the stochastic Newton equation yields the Madelung Euler equation with \(Q\).
- **Decision**: Implement the stochastic kinematics and validate two cases numerically: (1) the harmonic-oscillator ground state, whose osmotic drift \(u=-\omega x\) gives an Ornstein-Uhlenbeck process with variance \(\hbar/(2m\omega)\); (2) a free Gaussian packet, using the exact time-dependent \(\rho,v\) and \(b_+=v+D\partial_x\ln\rho\). Pre-register histogram \(L^1\) tolerances of 0.05 (ground state) and 0.10 (free packet) with 50k particles and 80 bins. The drift uses the exact quantum density, so this is a consistency check of the stochastic kinematics, not a self-consistent derivation of the density.
- **Consequences**: New `theory/nelson.py`, `phenomenology/nelson_report.py`, `tests/test_nelson.py`, `results/nelson.{json,md}`, and claim 21. A self-consistent mean-field simulation or a modified diffusion law is the next falsifiable step.
