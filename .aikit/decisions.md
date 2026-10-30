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

### ADR-013: Gaussian Madelung reduction and the quantum width (claim 22)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: The Madelung dictionary and Nelson kinematics are exact, but the harmonic-oscillator ground-state width was inserted from the Schrodinger solution. A Gaussian density ansatz with \(v=A(t)x\) reduces the Madelung equations to a closed equation for the width.
- **Decision**: Derive and test the Gaussian reduction. Continuity gives \(\dot\sigma=A\sigma\); Euler gives \(\dot A+A^2=-\omega^2+\hbar^2/(4m^2\sigma^4)\), hence the Ermakov equation \(\ddot\sigma=-\omega^2\sigma+\hbar^2/(4m^2\sigma^3)\). The effective potential \(V(\sigma)=\frac12\omega^2\sigma^2+\hbar^2/(8m^2\sigma^2)\) has its minimum at \(\sigma^2=\hbar/(2m\omega)\). Pre-register: the fixed point, the small-oscillation frequency \(2\omega\), and energy conservation to \(10^{-6}\) over several periods. A numerical RK4 integration from \(2\sigma_q\) must oscillate around the fixed point.
- **Consequences**: New `theory/gaussian_fluid.py`, `phenomenology/gaussian_fluid_report.py`, `tests/test_gaussian_fluid.py`, `results/gaussian_fluid.{json,md}`, and claim 22. A positive result derives the ground-state width from the fluid equations, not from the wavefunction; the Born rule and measurement remain outside this reduction.

### ADR-014: Moving-center Gaussian reduction and the coherent state (claim 23)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: The ground-state reduction fixed the center at zero. A displaced Gaussian is the natural coherent state and avoids the node singularity of the first excited state. The fluid equations should separate into a classical center trajectory and the quantum width.
- **Decision**: Use the ansatz \(\rho\propto\exp[-(x-x_0)^2/(2\sigma^2)]\) and \(v=\dot x_0+A(x-x_0)\). Derive and test: continuity gives \(\dot\sigma=A\sigma\); Euler gives \(\ddot x_0=-\omega^2x_0\) and \(\dot A+A^2=-\omega^2+\hbar^2/(4m^2\sigma^4)\). Pre-register the center period \(2\pi/\omega\), the width fixed point \(\sigma^2=\hbar/(2m\omega)\), and energy conservation to \(10^{-6}\) over two periods. Integrate the coupled system with RK4.
- **Consequences**: New center functions in `theory/gaussian_fluid.py`, `phenomenology/coherent_fluid_report.py`, `tests/test_coherent_fluid.py`, `results/coherent_fluid.{json,md}`, and claim 23. The first excited state remains a separate node-singularity test.

### ADR-015: First-excited-state node and the limits of the single-valued fluid (claim 24)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: The Gaussian reductions cover node-free states. The first excited harmonic-oscillator state has a density node at \(x=0\); the phase jumps by \(\pi\) and the Madelung velocity may be singular, so a single-valued fluid representation may fail.
- **Decision**: Use the exact \(\psi_1(x)\) and \(\rho_1(x)\). Test the node density, the phase slip, the discrete phase-gradient maximum on \(N=256,512,1024,2048\) grids, and the regular quantum pressure away from the node against the analytic expression \(\frac{\hbar^2}{2m\sigma^2}(3-x^2/\sigma^2)\). The claim "a regular single-valued Madelung representation exists" is falsified if the phase slip is \(\pi\) and the gradient maximum scales as \(dx^{-1}\). The central-difference maximum is \(\pi/(2dx)\), so the reported value and slope, not the prefactor, are the test.
- **Consequences**: New `theory/excited_fluid.py`, `phenomenology/excited_fluid_report.py`, `tests/test_excited_fluid.py`, `results/excited_fluid.{json,md}`, and claim 24. A negative means a two-component or defect model is required for excited states.

### ADR-016: Hydrogen 1s Madelung reduction (claim 25)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: The first excited state exposes the node singularity. The hydrogen 1s state is nodeless and realistic; its Coulomb attraction and quantum pressure should balance at the Bohr radius, providing a non-Gaussian test of the fluid program.
- **Decision**: Use the 1s density \(\rho=(1/\pi a_0^3)e^{-2r/a_0}\) and the quantum pressure \(Q=-\frac{\hbar^2}{2m a_0^2}+\frac{\hbar^2}{m a_0 r}\). The Euler equation for a stationary state requires \(V+Q=\mu\); the \(1/r\) terms cancel at \(a_0=\hbar^2/(m e^2)\), and the constant is \(\mu=-\hbar^2/(2m a_0^2)\). Pre-register: density normalization, zero force-balance residual at \(a_0\), nonzero residual for \(a\ne a_0\), and the SI values \(a_0=5.29177\times10^{-11}\) m and \(E=-13.6057\) eV.
- **Consequences**: New `theory/hydrogen_fluid.py`, `phenomenology/hydrogen_fluid_report.py`, `tests/test_hydrogen_fluid.py`, `results/hydrogen_fluid.{json,md}`, and claim 25. A positive result derives the Bohr radius from the fluid force balance; the node limitation of excited states remains.

### ADR-017: Hydrogen 2p vortex identification (claim 26)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: The 1D first-excited node is singular. The hydrogen \(2p_{m=\pm1}\) state has an angular node at the origin: the density vanishes there, but the phase winds by \(2\pi m\), so it is a quantized vortex rather than a radial node. This connects the node problem to the earlier vortex/defect work.
- **Decision**: Use the exact 2p radial function \(R_{21}(r)\) and the azimuthal velocity \(v_\phi=\hbar m/(m_e r\sin\theta)\) away from the poles. Pre-register: radial normalization 1, most-probable radius \(4a_0\), loop circulation \(2\pi\hbar m/m_e\) (atomic units \(2\pi m\)), and angular momentum \(\hbar m\). The \(m=0\) state has no vortex and is excluded from the circulation claim.
- **Consequences**: New `theory/hydrogen_vortex.py`, `phenomenology/hydrogen_vortex_report.py`, `tests/test_hydrogen_vortex.py`, `results/hydrogen_vortex.{json,md}`, and claim 26. A positive identification is not a derivation of the 2p energy; radial nodes still require a defect model.

### ADR-018: Shallow-water acoustic horizon (claim 27)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: The user asks which water properties can link to the quantum-fluid program. The nematic director has no emergent metric (claim 11a), but shallow-water waves do have an acoustic metric set by depth, gravity and background flow. A draining vortex has a horizon where the inward flow speed equals the wave speed.
- **Decision**: Use the long-wavelength shallow-water model \(c=\sqrt{gh}\), \(v_r=Q/(2\pi r h)\), \(r_H=Q/(2\pi h c)\), surface gravity \(\kappa=c^2/r_H=2\pi h c^3/Q\), and analogue temperature \(T_H=\hbar\kappa/(2\pi k_B)\). Pre-register: \(v_r(r_H)=c\), \(g_{tt}=-(c^2-v_r^2)=0\) at the horizon, positive \(\kappa\) and \(T_H\), and lab values for \(h=0.1\) m, \(Q=10^{-3}\) m³/s. The metric is acoustic and long-wavelength only; capillary dispersion breaks it at short scales.
- **Consequences**: New `theory/water_analogue.py`, `phenomenology/water_analogue_report.py`, `tests/test_water_analogue.py`, `results/water_analogue.{json,md}`, and claim 27. A positive horizon result is an analogue, not a fundamental metric; a numerical wave-scattering test is the next step.

### ADR-019: Capillary validity of the shallow-water horizon (claim 28)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: Claim 27's lab example has \(r_H=1.6\) mm. Water's capillary length is about \(2.7\) mm, so the horizon may lie in the capillary-dispersion regime, where the long-wavelength acoustic metric does not apply.
- **Decision**: Use water at 20 C (\(\rho=998\) kg/m³, \(\gamma=0.0728\) N/m, \(g=9.81\) m/s²). Compute \(\ell_c=\sqrt{\gamma/(\rho g)}\), the capillary/gravity ratio \((\ell_c/r_H)^2\), the minimum flow rate for \(r_H=10\ell_c\), and a valid example at \(Q=0.05\) m³/s. Pre-register: the lab \(Q=10^{-3}\) m³/s is capillary-dominated; the long-wavelength condition is \(r_H\gg\ell_c\); the valid example has capillary ratio \(<10^{-2}\). Claim 27's formula remains exact in the long-wavelength limit, but its lab example is flagged.
- **Consequences**: New validity functions in `theory/water_analogue.py`, `phenomenology/water_validity_report.py`, `tests/test_water_validity.py`, `results/water_validity.{json,md}`, and claim 28. A negative lab example is a design constraint, not a failure of the horizon formula.

### ADR-020: Dispersive shallow-water horizon (claim 29)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: Claim 27 uses the long-wavelength sound speed. The full shallow-water dispersion includes capillary and finite-depth terms, so the horizon depends on the wave frequency. The acoustic metric is valid only below the capillary frequency.
- **Decision**: Use \(\omega_f^2=(gk+\gamma k^3/\rho)\tanh(kh)\) and the outgoing-mode horizon condition \(k c_g(k)+\omega_f(k)=\omega\), with \(V=c_g(k)\) and \(r=Q/(2\pi h V)\). Pre-register the low-frequency limit \(r\to r_H\), the capillary frequency \(\omega_c=\sqrt{2g/\ell_c}\), and the horizon ratios at \(\omega=0.1,1,10,100\) rad/s. A shift above 10% at low frequency would falsify the long-wavelength metric.
- **Consequences**: New `theory/water_dispersive.py`, `phenomenology/water_dispersive_report.py`, `tests/test_water_dispersive.py`, `results/water_dispersive.{json,md}`, and claim 29. The long-wavelength acoustic metric is valid for \(\omega\ll\omega_c\); high-frequency waves see a shifted horizon.

### ADR-021: Radial rays at the acoustic horizon (claim 30)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: The horizon metric is established, but its causal structure has not been checked. Radial rays should change direction at \(r_H\) and peel away with the surface-gravity rate.
- **Decision**: Use the outgoing radial speed \(dr/dt=c-Q/(2\pi r h)=c(1-r_H/r)\) and the ingoing speed \(dr/dt=-c-Q/(2\pi r h)\). Pre-register: zero speed at \(r_H\), positive outside and negative inside, peeling rate \(\kappa=c/r_H\), and the analytic ray time \(t=(r-r_0)/c+(r_H/c)\ln|(r-r_H)/(r_0-r_H)|\) from RK4. Use the valid \(Q=0.05\) m³/s, \(h=0.1\) m regime.
- **Consequences**: New `theory/water_rays.py`, `phenomenology/water_rays_report.py`, `tests/test_water_rays.py`, `results/water_rays.{json,md}`, and claim 30. The geometric-optics rays are a check; a numerical wave-scattering test remains the next step.

### ADR-022: 1D shallow-water wave scattering at the horizon (claim 31)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: The horizon is established analytically and by radial rays. A numerical wave-scattering test checks whether a right-moving packet is blocked at the horizon in the long-wavelength regime.
- **Decision**: Use the 1D linearized shallow-water equations in Riemann invariants \(R_\pm=u\pm(c/h)\eta\), with upwind advection at speeds \(v\pm c\), a sponge at the boundaries, and a Gaussian packet \(u=(c/h)\eta\). The horizon flow is \(v=-c(1+x/(2L))\); a uniform \(v=-0.5c\) is the transmitting control. Pre-register: transmitted energy fraction \(<10^{-3}\) at the horizon and \(>0.2\) in the control, with finite energy and a stable solution.
- **Consequences**: New `sims/water_scattering.py`, `phenomenology/water_scattering_report.py`, `tests/test_water_scattering.py`, `results/water_scattering.{json,md}`, and claim 31. The 1D flow is a kinematic analogue, not a consistent shallow-water equilibrium; a full radial simulation remains future work.

### ADR-023: Hydrated-electron cavity scale from water properties (claim 32)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: Water's dielectric constant and surface tension are distinctive. A screened hydrogen state gives \(a_0^*=\epsilon_r a_0=41.5\) Å and \(E^*=-2.2\) meV, while the hydrated electron absorbs near 1.7 eV on a few-Å scale. The balance of electron quantum pressure and surface tension may set the cavity scale.
- **Decision**: Minimize \(E(R)=\hbar^2/(2mR^2)+4\pi\gamma R^2\) with water at 20 C (\(\gamma=0.0728\) N/m), deriving \(R^4=\hbar^2/(8\pi m\gamma)\), and compare the resulting radius and energy with the screened hydrogen and infinite-well scales. Pre-register: \(R=2.858\) Å and \(E=0.933\) eV; the observed 1.7 eV is a factor 1.8 higher. This is a scale test, not a derivation of the absorption.
- **Consequences**: New `theory/water_electron.py`, `phenomenology/water_electron_report.py`, `tests/test_water_electron.py`, `results/water_electron.{json,md}`, and claim 32. A finite-well/polaron model is needed for the exact 1.7 eV.

### ADR-024: Finite spherical well for the hydrated electron (claim 33)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: Claim 32 fixes the cavity scale from surface tension but does not explain the observed 1.7 eV absorption. The minimal closure is a spherical finite well of radius \(R\) and depth \(V_0\), with \(V_0\) set by the Born solvation energy at that radius. The question is whether this well supports a 1p state and an s->p transition near 1.7 eV.
- **Decision**: Solve the l=0 and l=1 spherical-well matching conditions for the 1s and 1p roots, with \(V(r)=-V_0\) for \(r<R\) and zero outside. Use \(V_0(R)=e^2/(8\pi\epsilon_0R)(1-1/\epsilon_r)\) with \(\epsilon_r=78.4\). Pre-register: at \(R=2.858071\times10^{-10}\) m, \(V_0=2.486988\) eV, \(\kappa=2.309130\), \(u_{1s}=2.049533\), \(E_{1s}=-0.527753\) eV, and no 1p. The 1p threshold is \(\kappa=\pi\), at \(R=5.290248\times10^{-10}\) m; the maximum Born-depth s->p gap is \(0.614820\) eV. A 1.7 eV gap requires a minimum depth of \(3.715111\) eV at \(R=3.181453\times10^{-10}\) m (the infimum at the 1p threshold), where the Born depth is \(2.234196\) eV (excess \(1.480915\) eV). The infinite-well s->p spacing reaches 1.7 eV at \(R=4.809506\times10^{-10}\) m; the earlier \(4.703129\times10^{-10}\) m estimate is the infinite-well ground-state energy scale, not the s->p spacing.
- **Consequences**: New `theory/water_well.py`, `phenomenology/water_well_report.py`, `tests/test_water_well.py`, `results/water_well.{json,md}`, and claim 33. The result is a negative one for the minimal Born well: it cannot produce the 1.7 eV absorption as a bound s->p transition. The missing depth of about 1.48 eV is a quantitative target for a polaron or band-offset correction, not a fitted parameter.

### ADR-025: Bound-free absorption in the finite spherical well (claim 34)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: Claim 33 rules out a bound 1p state at the Born depth, but the hydrated-electron absorption band is broad. The remaining natural channel is a bound-free 1s->p transition into the continuum. The same well makes a parameter-free prediction for the continuum peak.
- **Decision**: Compute the 1s bound state and the p-wave continuum of the finite well in atomic units, match the logarithmic derivative at \(R\), normalize the continuum to \(\sqrt{2k/\pi}\) asymptotically, and evaluate the radial dipole integral. Use the oscillator strength \(f(\omega)=(2/3)\omega|\int u_f r u_i\,dr|^2\). Validate the normalization with the TRK sum rule. Pre-register: at the capillary radius and Born depth, the continuum peak is \(1.024880\) eV and the sum rule (to 5 Hartree) is \(0.992395\). A 1.7 eV peak requires an extra depth of \(1.19784\) eV (\(V_0=3.684825\) eV), with the inverted peak at \(1.70092\) eV. The optical reorganization energy at this radius is about \(1.383\) eV, so the extra depth is about \(0.866\) of it.
- **Consequences**: New `theory/water_absorption.py`, `phenomenology/water_absorption_report.py`, `tests/test_water_absorption.py`, `results/water_absorption.{json,md}`, and claim 34. The Born-depth well still misses the observed peak, but the bound-free channel is broad and the extra depth is a quantitative target for a band-offset or polaron correction; it is not a fitted parameter.

### ADR-026: VDE and absorption constrain the finite well (claim 35)
- **Date**: 2026-10-30
- **Status**: Accepted
- **Context**: Claim 34's bound-free peak at 1.7 eV requires a 1s binding of 1.375 eV. The measured vertical detachment energy of the hydrated electron is \(3.27\pm0.10\) eV (Tang et al., PCCP 12, 3653, 2010), so a 1.7 eV photon is below the detachment threshold. The absorption must therefore be bound-bound. The finite well must satisfy both the VDE and the 1.7 eV 1s->1p gap.
- **Decision**: For each radius \(R\), solve the well depth \(V_0(R)\) such that \(-E_{1s}=3.27\) eV, then invert for the radius where the 1s->1p gap is 1.7 eV. Pre-register: \(R=3.732122844\) Angstrom, \(V_0=4.997522901\) eV, \(u_{1s}=2.513080317\), \(u_{1p}=3.539848250\), \(E_{1s}=-3.27\) eV, \(E_{1p}=-1.57\) eV, gap \(1.7\) eV. The Born depth at this radius is \(1.904542890\) eV, so the required depth exceeds it by \(3.092980011\) eV. The VDE \(\pm1\sigma\) band gives \(R=3.7143\)-\(3.7490\) Angstrom and \(V_0=4.9028\)-\(5.0926\) eV. The capillary cavity is \(R=2.858071056\) Angstrom with Born depth \(2.486987864\) eV, outside this band.
- **Consequences**: New `theory/water_vde.py`, `phenomenology/water_vde_report.py`, `tests/test_water_vde.py`, `results/water_vde.{json,md}`, and claim 35. The minimal capillary/Born cavity cannot satisfy the VDE and the 1.7 eV absorption simultaneously; a larger, much deeper well is required. A soft-wall or polaron potential is the next test.
