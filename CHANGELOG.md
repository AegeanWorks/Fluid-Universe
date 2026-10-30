# Changelog

Append-only. Corrections and retractions are recorded with the reason and the evidence path. Do not silently edit an earlier entry.

## 2026-10-25 — Project scaffold and first falsification pass

**Added**

- Claim ledger (`claims.md`), scorecard, formula registry, literature notes.
- Symbolic/numeric circulant core and its tests (claims 1-2).
- PDG pole-mass fit with Monte Carlo uncertainty propagation.
- One-loop Standard Model lepton RGE from \(M_Z\) to \(10^{16}\,\mathrm{GeV}\), with a published-running cross-check.

**Falsified / corrected**

- **1b:** The Z3/circulant structure alone does not force \(Q=2/3\). The map has three parameters for three masses; Koide is the independent condition \(|c|/a=1/\sqrt2\). The earlier framing ("forcing a Hermitian circulant whose eigenvalues give \(\sqrt{m_k}\)") is a reparametrisation, not a prediction.
- **1d:** The identity \(Q=1/3+2c^2/(3a^2)\) uses signed eigenvalues. Physical masses use \(|\lambda_k|\); the identity fails when an eigenvalue is negative. Counterexample recorded in the tests.
- **3b:** \(Q=2/3\) is a **pole-mass** statement. MS-bar charged-lepton masses at \(M_Z\) give \(Q\approx0.6678\), about \(0.18\%\) high. The difference is dominantly a scheme/threshold conversion, not RG drift.
- **4a:** "\(|c|/a\) is RG-invariant at one loop" is false as an exact statement. The \(y_k^2\) term is generation-dependent. The numerical drift is about \(3\times10^{-6}\) over 16 decades, so 4b is recorded as a numerical stability result, not an invariance theorem.

**Retracted**

- **4c:** The earlier claim of a "two-loop drift \(\sim4\times10^{-9}\)/decade" is retracted. No script, derivation, or output existed in this repository. No two-loop number is used in any result.

**Not used**

- The lost 3D-run values (\(|H|\sim0.16\), \(Z_3\sim0.72\) at \(N=36\)) are not cited. There is no simulation result in this release.

**Open**

- Claims 5-16: rotor, dimensional reduction, neutrinos, quarks, $\delta$, Frank-Oseen modes, Lorentz bounds, nematic topology, Derrick, 3D simulation, and the search for a new prediction.
- No genuinely new prediction beyond the built-in Koide relation has been found (claim 16).

## 2026-10-25 — Second pass: rotor, sector degeneracy, domain wall

**Added**

- Exact diagonalisation of the quantum rotor in the Fourier sectors \(n=k+3j\), with truncation convergence and a finite-difference cross-check.
- Verification of the domain-wall tension \(T=(8/3)\sqrt{\mu}\) analytically and by quadrature.

**Falsified / corrected**

- **5c:** The naive rotor-to-circulant map cannot give three lepton masses. The real \(\cos3\phi\) potential makes the \(k=1\) and \(k=2\) sectors exactly degenerate (\(E_1-E_2=0\) at \(I=\mu=1\)); the map yields roots \((1.45898,1.03314,1.03314)\), only two distinct.
- **5d:** The bound \(c/a\le1/2\) is false; positive roots \((10,1,1)\) give \(c/a=0.75\), \(Q=0.7083\). The general bound is \(c/a\le1\).

**Status changes**

- 5a and 5b become PROVEN (numeric and exact free-rotor limit, respectively).
- 6a becomes PROVEN. 6b (exact dimensional reduction / finite effective moment of inertia) remains UNVERIFIED.

**Open**

- Claims 6b, 7-16 unchanged. No simulation result exists.

## 2026-10-25 — Third pass: independent verification

**Method**

- Added `verification/audit.py`, which re-derives every PROVEN, VERIFIED, or NUMERIC claim without importing `theory` or `phenomenology`: sympy for the algebra and the domain wall, `numpy.linalg.eigvalsh` for the circulant, a linear solve for the fit, a separate `solve_ivp` RGE implementation, and an independently assembled rotor matrix.
- Added a regression test for the gauge-coupling RHS, the bug found in this pass.

**Corrected (numbers changed)**

- **Gauge-coupling RGE bug.** The integrated state is logarithmic, but the gauge derivatives were returned as `dg/dt` instead of `d ln g/dt`. Corrected to `d ln g/dt = c g^2`. Consequence: the one-loop drift changes from `Delta Q = +2.903e-6`, `Delta r = +3.073e-6` to `Delta Q = +2.888e-6`, `Delta r = +3.058e-6`; `Q(1e16)` becomes `0.667826882` and `r(1e16)` becomes `0.708336306`. All prose and tables updated.
- **The old "full RHS vs ratio-only" check was mislabelled.** Dropping the common gauge/T factor is not an exact equivalence: the generation-dependent `(3/2)y_k^2` term scales with the absolute Yukawa normalisation, so the approximation misses about 4-5% of the drift. It is now `shape_only_approximation` and is explicitly not a cross-check.
- **Paper abstract.** The abstract attributed `Q(M_Z) = 0.6678240` to the pole-to-MSbar conversion. That value is the full-SM matching result; the one-pass QED conversion gives `0.6679160`. Both are now stated separately.
- **Registry F9/F10 downgraded.** F9 is REPRODUCED at the `2e-4` level, not VERIFIED. F10 is CITED + partial cross-check: the absolute high-scale masses agree with the three-loop literature only at 3-5%, while Q agrees to about `4e-5`.
- **Stale summary text.** The summary claimed no rotor spectrum existed; it now reports the rotor result.
- **Dead test references** in `theory/formulas.md` (F8, F11) corrected.

**Verification result**

- `python -m verification.audit`: 66/66 checks pass. Project rule G-001: every claim in `claims.md` is verified twice in separate processes, and the per-claim results must match; the rule does not apply to the pytest suite. `make verify` wrote `results/verification_run1.json`, `verification_run2.json`, and the combined `verification.json`; the two runs agree per claim.

**Open**

- Claims 6b, 7-16 remain unverified. No simulation result exists. The one-loop absolute high-scale masses are truncation-limited and are not used for any claim.

## 2026-10-25 — Fourth pass: change audit and harness hardening

**Guideline conflict fixed**

- G-001 and G-002 overlapped: G-001 said "every independent verification", G-002 narrowed it to claims. G-002 is merged into a single scoped G-001 that explicitly applies to `claims.md`, not to pytest. The ID G-002 is retired, not reused.

**Harness gaps closed**

- **Stale-run rejection.** Both runs now record a source fingerprint (code, tests, docs, paper) and a physics-results fingerprint. `verification.compare` rejects the pair if either fingerprint differs between runs or from the current files. Editing any of those after a run forces both runs to be repeated.
- **Single-run overwrite.** A bare `python -m verification.audit` no longer writes the canonical `results/verification.json`; it writes `verification_single.{json,md}` and prints NOT ACCEPTED. Only `verification.compare` writes the canonical file.
- **Fresh physics.** `make verify` now depends on `make results`, so the audits always run against regenerated numbers.
- **Integrator cross-check.** The audit now also integrates the RGE with DOP853 and compares it to RK45, rather than relying on one algorithm.
- **Rotor cross-check tightened.** The Fourier-vs-grid test tolerance is now `1e-5` relative (observed ~`1e-6`), instead of the vacuous `5e-3`.
- **Naming and prose.** `pauli_obstruction` renamed to `parity_obstruction` (the argument is parity); the naive map docstring now states the sign convention and that the opposite sign, not the prompt's sign, reproduces `(l0,l1,l1)`; F11's drift text corrected from `<1e-6` to `~3.06e-6`.
- **Dead code removed.** The unused `MSBAR_MZ_SIGMA` and the hard-coded `verdict.pole_Q` were removed.
- **Cross-platform workflow.** `python -m verification.verify` now orchestrates results regeneration, two separate audit processes, and the comparison; `make verify` delegates to it. This matters because `make` is not installed on the test machine.
- **Documentation integrity.** Unescaped `|` characters inside Markdown table cells were replaced with `\lvert ... \rvert` in `claims.md`, `SCORECARD.md`, `README.md`, and the F12 row of `theory/formulas.md`; table column counts are now uniform. A dead `theory/circulant.py:free_parameters` reference and a stale `make`-only workflow were removed.

**Verification result**

- Two separate runs, 66/66 checks each, per-claim agreement and matching fingerprints; `results/verification.md` records both runs.

## 2026-10-25 — Fifth pass: neutrino and quark checks

**Added**

- NuFit 5.2 neutrino scan: both orderings, lightest mass 0 to 0.2 eV, plus the 3-sigma splitting corners.
- Quark Koide ratios at six full-SM MS-bar scales from Huang & Zhou (2021, Table II).
- Minimal circulant PMNS branches: same-basis (no mixing) and neutrino-only (trimaximal).

**Results**

- **7a NUMERIC (negative):** no scanned neutrino point reaches `Q=2/3`. Maximum `Q_nu=0.585612` (NO), `0.500007` (IO); 3-sigma corner maximum `0.590606`.
- **7b FALSIFIED (ill-posed):** the original "circulant PMNS ruled out by theta13" does not specify which sector is circulant. The same-basis branch predicts `(0,0,0)`; the neutrino-only branch predicts `(45,35.26,45)`; measured `(33.41,8.58,42.20)`. Both are excluded, but only the second is a theta13 statement.
- **8 FALSIFIED:** at `M_Z`, `Q_U=0.887569`, `Q_D=0.748065`; over six scales `Q_U in [0.887569,0.892554]`, `Q_D in [0.738535,0.748065]`. The original `0.85/0.73` values were scale-dependent estimates; the direct extension is falsified.

**Open**

- No new prediction beyond Koide. Claims 6b, 9-16 remain unverified. No simulation result.

- Verification: two separate runs, 87/87 checks each, per-claim agreement; `results/verification.md` records both.

## 2026-10-25 — Sixth pass: delta, Frank-Oseen, Derrick

**Added**

- Post-hoc delta closed-form search with an explicit candidate prior and look-elsewhere penalty.
- Frank-Oseen linearised mode analysis, symbolic Hessian check, and polarisation/photon counting.
- Derrick scaling for the exact FN+Skyrme+Z3 energy, with symbolic virial check and parameter region.

**Results**

- **9 UNVERIFIED (null):** 2/9 is the closest candidate at 0.407 sigma, but the look-elsewhere p over the stated prior is 7.93e-4 and the search is post-hoc. Not promoted.
- **10 PROVEN (equal K only):** the 2x2 elastic matrix gives two degenerate transverse modes with omega^2=(K/rho)k^2 when K1=K2=K3. No longitudinal mode, no gauge field, no photon. The background selects a preferred frame; emergent Lorentz invariance is not claimed.
- **14a PROVEN:** d=3 exponents (1,-1,3) and virial E2-E4+3V=0; second derivative 2(E4+3V)>0.
- **14b UNVERIFIED:** kappa>0 is necessary for a stable scaling soliton, but Derrick scaling is not an existence proof. No Hopfion solution is claimed.
- **11/12/13/16** remain unverified and are now explicitly scoped: no metric/RG flow, no physical calibration for Lorentz bounds, no topological derivation of Z3, and no new prediction.

**Open**

- Numerical Hopfion existence, Lorentz-violation bounds, and the 3D simulation remain undone.

- Verification after the 9/10/14 pass: two separate runs, 101/101 checks each, per-claim agreement; `results/verification.md` records both.

## 2026-10-25 — Seventh pass: topology, reduction, new-prediction audit

**Added**

- Nematic topology audit: `RP^2` homotopy groups, the direct Z3 claim, and three standard extensions.
- Dimensional-reduction audit: trivial 2D lower bound and the effective moment of inertia for power-law tails.
- Seven-class new-prediction audit.

**Results**

- **13a FALSIFIED:** `RP^2` has `pi1=Z2`, `pi2=Z`, `pi3=Z`; no Z3 factor. `cos(3 phi)` is inserted by hand.
- **13b UNVERIFIED (null):** rank-3 order parameter, three coupled nematics, and a 3-fold lattice all require an explicit three-fold structure.
- **6b PROVEN (trivial):** `E_2D >= T L_y`, with equality only for a y-independent wall.
- **6c FALSIFIED:** exact dimensional reduction fails; `I_eff = 2*pi int rho r^3 dr` is log-divergent for a `1/r^4` tail (finite only for a tail steeper than `r^-4`).
- **16 UNVERIFIED (none found):** no extra mass relation, viable minimal mixing branch, calibrated dispersion, validated defect count, or testable RG signature.

**Open**

- Numerical Hopfion existence, Lorentz-violation bounds, and the 3D simulation remain undone. No new prediction beyond Koide.

- Verification after the topology/reduction/prediction pass: two separate runs, 115/115 checks each, per-claim agreement.

## 2026-10-25 — Eighth pass: residual anisotropy and LV bounds

**Added**

- Residual anisotropy `xi` of the two transverse director modes, sampled over 4000 directions.
- A table of experimental Lorentz-violation limits with explicit applicability flags.
- A conditional photon-coupling check (equal K vs O(1) anisotropy).

**Results**

- **11a FALSIFIED:** no emergent metric at tree level; the background selects a preferred frame. `K=(1,2,3)` gives `xi=1.00348`; equal K gives `xi=0` but still no metric.
- **11b UNVERIFIED:** no one-loop anisotropy/RG calculation exists.
- **12a UNVERIFIED (not applicable):** the director modes are neutral, so photon birefringence, GRB, CMB, and clock/cavity bounds do not directly apply.
- **12b PROVEN (conditional):** if the director modes were photons with O(1) coupling, equal K would pass all `Delta c/c` bounds and an O(1) anisotropy would be excluded by many orders.

**Open**

- No numerical Hopfion, no 3D simulation, no physical calibration of the director sector.

- Verification after the 11/12 pass: two separate runs, 125/125 checks each, per-claim agreement.

## 2026-10-25 — Ninth pass: validated 1D simulation core

**Added**

- `sims/core.py`: 1D sine-Gordon energy, analytic kink, discrete force, gradient flow, Hessian, L-BFGS relaxation, rotor cross-check.
- `phenomenology/sim_report.py`: reproducible 1D baseline payload and markdown.
- `tests/test_sim_core.py`.

**Results**

- Kink energy converges to `T=(8/3)sqrt(mu)` at observed order ~2.
- Analytic force matches finite differences below `1e-6`.
- Energy decreases monotonically under gradient flow.
- Rotor Fourier and finite-difference spectra agree below `1e-3`.
- Hessian is symmetric and stable; the translation mode is lifted by Dirichlet boundaries, so zero-mode counting is partial.
- **No 3D lattice, Hopf charge, or Z3 order run. Claim 15 remains UNVERIFIED / NOT ACHIEVED.**

## 2026-10-25 — Tenth pass: Hopf estimator validation

**Added**

- Preimage linking estimator for the analytic H=1 Hopf map.
- Grid Chern-Simons integral estimator as a second, independent method.
- `tests/test_hopf.py` and `results/hopf.{json,md}`.

**Results**

- **Preimage linking PASSES:** H = 0.999331 / 0.999833 / 0.999958 at 100/200/400 samples; reflection gives -0.999833; rotation and shear preserve the value.
- **Grid Chern-Simons estimator FAILS:** H = -2.462 / -3.184 / -3.499 at N=32/48/64, not converging to ±1. This is a documented estimator bug, not a physical result.
- **Baseline 4 FAILS** because the two independent methods do not agree. No 3D run or Hopfion claim; claim 15 remains NOT ACHIEVED.

- Verification after the Hopf-baseline pass: two separate runs, 133/133 checks each, per-claim agreement.

**Correction (same pass).** The earlier statement that the grid estimator "fails" was wrong: it used finite-difference derivatives and an unconverged normalization. With spectral derivatives, a smooth vacuum compactification, and the documented convention `-1/(16*pi^2)`, it gives `1.000141`, `0.9999998`, and `1.00000004` at N=32/48/64 and agrees with the preimage estimator. Baseline 4 passes. The earlier failure statement is superseded; claim 15 is still NOT ACHIEVED because no 3D run exists.

- Verification after the Hopf normalization correction: two separate runs, 137/137 checks each, per-claim agreement.

- Verification after the change audit: two separate runs, 141/141 checks each, per-claim agreement.

## 2026-10-25 — Eleventh pass: static finite-energy H=1 configuration

**Added**

- `theory/hopfion.py`: Faddeev-Niemi E2 and Skyrme E4 on the analytic H=1 map, with vacuum compactification.
- `phenomenology/hopfion_report.py` and `tests/test_hopfion.py`.
- `results/hopfion.{json,md}`.

**Results**

- At N=64: E2=233.635, E4=159.262, H=1.000000.
- Scaling virial can be satisfied at lambda*=0.82563; E_min=385.79.
- This is a static finite-energy configuration, not a stationary solution under non-scale deformations. No full stability proof; no Z3 order measured.
- Claim 14b remains UNVERIFIED; claim 15 remains NOT ACHIEVED.

## 2026-10-25 — Twelfth pass: 3D E2+E4+Z3 pilot

**Added**

- `sims/core3d.py`: JAX gradient flow for E2 + Skyrme E4 + a Z3 azimuthal potential.
- `phenomenology/hopfion3d_report.py` and `tests/test_hopfion3d.py`.
- `results/hopfion3d.{json,md}`.

**Results**

- Single-seed N=32 pilot, 300 steps: for mu=0,1,5,20, H=0.99714,0.99727,0.98949,0.85298 and Z3=0,0.0234,0.1170,0.4432.
- No tested mu reaches both |H|>=1 and Z3>=0.5. The potential raises Z3 while lowering H.
- Single seed only; no spontaneous-order or production claim. Claim 15 remains UNVERIFIED.

**Note**

- JAX is now a declared dependency for the 3D core.

- Verification after the 3D pilot: two separate runs, 147/147 checks each, per-claim agreement.

**Correction (3D pilot).** The 300-step N=32 values were transient and under-resolved. At 1200 steps the force RMS is still 0.10, H/Z3 continue to change with step count, and an N=64 spot check differs materially. The pilot is now recorded as resolution-limited with no coexistence conclusion. The earlier "max Z3=0.443 at H=0.853" statement is superseded; claim 15 remains UNVERIFIED.

## 2026-10-26 — Hopf grid resolution

- Added a resolution study of the grid Chern-Simons estimator on the analytic H=1 map.
- N=16: H=0.801800; N=24: 0.994054; N=32: 1.000141; N=48: 0.9999998; N=64: 1.00000004.
- N>=32 is within 1e-3 of the preimage value; N=16/24 are under-resolved. This substantiates the 3D pilot's resolution caveat.

- Verification after the Hopf resolution study: two separate runs, 150/150 checks each.

## 2026-10-26 — Targeted N=64 3D diagnostic

- Fixed the fixed-boundary flow to constrain the gradient at the boundary instead of resetting nodes after each step; monotone energy now holds.
- N=64, dt=5e-4, fixed vacuum boundaries: H=1.00000,0.99891,0.98274,0.78800 and Z3=0,0.23083,0.43840,0.60743 at steps 1200 (mu=0,20), 2400, 3600.
- Force RMS at 3600 is 0.037, so the run is not converged. No coexistence; supplementary diagnostic, not part of make results.

- Verification after the N=64 diagnostic: two separate runs, 153/153 checks each.

- 2026-10-26: N=64 pure-FN mu=0 reaches a near-stationary H=1 candidate (H=0.99999993, force RMS 0.00315 at 1200 steps). Recorded as a candidate for 14b, not a stability proof; Z3 still destroys H.

- 2026-10-26: damped scale iteration reaches a near-virial-free H=1 candidate (virial 0.00961, H=0.99730, RMS 0.0030) at N=64. Iteration 5 drifts; no Hessian stability. Added as 14c NUMERIC candidate; 14b remains UNVERIFIED.

- Verification after the scale-iteration candidate: two separate runs, 156/156 checks each.

- 2026-10-26: coarse N=64 Hessian of the scale-stationary candidate has minimum eigenvalue -5.80 and 8 negative modes. The candidate is a saddle; 14d FALSIFIED, 14b remains UNVERIFIED.

- Verification after the Hessian stability check: two separate runs, 160/160 checks each.

## 2026-10-26 — Retraction: 14d Hessian stability

The 14d FALSIFIED result and the `hopfion_stability` artifacts are retracted. The matrix-free Hessian operator failed a dense validation at N=8: the dense constrained Hessian has the expected null modes (minimum ~0), while the HVP operator returned 0.2197. The N=64 negative spectrum is therefore not trusted. Stability of the scale-stationary candidate is open (14d UNVERIFIED).

## 2026-10-26 - Validated tangent Hessian (14d)

**Added**

- `sims/stability.py`: matrix-free Hessian on the per-site two-dimensional tangent space, so radial normalization modes are removed.
- `phenomenology/stability_report.py`, `tests/test_stability.py`, `results/hopfion_stability.{json,md}`, and `make stability`.

**Results**

- Dense validation at N=8: maximum |dense - matrix-free| = 1.332e-15, symmetry error 6.661e-16.
- N=64 tangent dimension 476656: minimum eigenvalue -5.799863, 8 negative modes in the requested set.
- The scale-stationary candidate is a saddle; 14d FALSIFIED. This rules out that candidate, not every possible Hopfion; 14b remains UNVERIFIED.

- Verification after the validated Hessian: two separate runs, 163/163 checks each.

## 2026-10-26 - Claim 15 production scan and 15-minute wall-clock cap (G-003)

**Added**

- Project guideline `G-003`: every simulation, diagnostic, or verification run must complete within a hard 15-minute (900-second) wall-clock limit (`MAX_WALL_SECONDS = 900.0`).
- `sims/production.py`, `phenomenology/production_report.py`, `tests/test_production.py`, `results/hopfion3d_production.{json,md}`, and `make production`.

**Results**

- N=64, 3600 steps, fixed vacuum faces, \(\mu=20\), \(dt=5\times10^{-4}\) (completed in ~185 s): native \(H=0.787999784\), 2x spectrally upsampled (\(N=128\)) \(H=0.989238052\), \(Z_3=0.607430\), force RMS \(3.716\times10^{-2}\), max bond difference \(1.480\), minimum pre-normalization norm \(1.000000\).
- `dt` check at N=48 (\(5\times10^{-4}\) vs \(2.5\times10^{-4}\) at physical time 0.15): \(\Delta H = 1.660\times10^{-6}\), \(\Delta Z_3 = 4.076\times10^{-6}\).
- The smooth topology is preserved (minimum pre-norm 1.000000; upsampled \(H\approx0.9892\)), showing that the native \(H\) drop to 0.788 is a grid-scale estimator artifact rather than a topological transition. Because the field develops grid-scale gradients (max bond 1.480 > 0.50) and the force RMS is above \(10^{-2}\), claim 15 remains **UNVERIFIED (H estimator not converged; not converged; field under-resolved)**.

- Verification after the production scan: two separate runs, 168/168 checks each.

## 2026-10-30 - N=96 resolution check for claim 15 (ADR-002)

**Added**

- `sims/convergence.py`, `phenomenology/convergence_report.py`, `tests/test_convergence.py`, `results/hopfion3d_convergence.{json,md}`, and `make convergence`.
- ADR-002 pre-registers the five resolution criteria before the run.

**Results**

- N=64 (3000 steps): H=0.936964507, upsampled H=0.999520564, Z3=0.528226, force RMS \(3.988\times10^{-2}\), max bond 1.176.
- N=96 (3000 steps): H=0.999942144, upsampled H=1.000000000, Z3=0.172071, force RMS \(1.495\times10^{-2}\), max bond 0.418.
- Deltas: H +0.062978, upsampled H +0.000479, Z3 -0.356154, force \(-2.494\times10^{-2}\), max bond -0.758. The native H deficit and the smoothness improve with resolution, but Z3 does not agree (\(|dZ_3|=0.356>0.02\)), so `resolution_converged` is False.
- Claim 15 remains UNVERIFIED: at N=96 Z3 is below 0.5 and the force RMS is above \(10^{-2}\). The N=64 Z3 order at 3600 steps is resolution-dependent; the N=64 Z3-order reading does not survive the finer grid.
- The 3000-step check completed in 768.9 s, inside the 900 s G-003 cap. A separate 3600-step N=64 run reproduced the committed production values to the reported precision.

- Verification after the N=96 resolution check: two separate runs, 171/171 checks each.

## 2026-10-30 - N=64 timestep check for claim 15 (ADR-003)

**Added**

- `sims/convergence.py::compare_temporal`, `phenomenology/temporal_report.py`, `tests/test_convergence.py` cases, `results/hopfion3d_temporal.{json,md}`, and `make temporal`.

**Results**

- N=64, 3000 steps at \(dt=5\times10^{-4}\) versus 6000 at \(dt=2.5\times10^{-4}\), both at \(t=1.5\). Coarse: H=0.936964507, upsampled H=0.999520564, Z3=0.528226; fine: H=0.936857582, upsampled H=0.999518680, Z3=0.528221.
- Deltas: H \(-1.07\times10^{-4}\), upsampled H \(-1.9\times10^{-6}\), Z3 \(-4.6\times10^{-6}\), force \(+1.3\times10^{-6}\), max bond \(+3.9\times10^{-4}\). The pre-registered H (\(5\times10^{-3}\)) and Z3 (\(2\times10^{-2}\)) criteria pass, so the N=64 Z3 at \(t=1.5\) is temporally resolved.
- The N=96 Z3 gap is therefore spatial (or a longer-time effect), not a timestep artifact. Claim 15 remains UNVERIFIED.
- The check completed in 535.7 s, inside the 900 s G-003 cap.

- Verification after the N=64 dt check: two separate runs, 173/173 checks each.

## 2026-10-30 - Checkpointed N=96 extension for claim 15 (ADR-004)

**Added**

- `run_production(initial_field=..., return_field=...)` with a chunking-exactness test; `phenomenology/extension_report.py`; `make extend`; `tests/test_convergence.py::summarize_trend`; `results/hopfion3d_extension.{json,md}`; and a git-ignored 21 MB `.npy` checkpoint.

**Results**

| chunk | t | H | H upsampled | Z3 | force RMS | max bond | seconds |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 1.5 | 0.999942144 | 1.000000000 | 0.172071 | 1.495e-2 | 0.418 | 577.3 |
| 2 | 3.0 | 0.999484353 | 0.999999976 | 0.333942 | 1.368e-2 | 0.463 | 533.9 |
| 3 | 4.5 | 0.996350415 | 0.999998598 | 0.477792 | 1.224e-2 | 0.564 | 577.5 |
| 4 | 6.0 | 0.972623788 | 0.999903154 | 0.598690 | 1.075e-2 | 0.857 | 566.3 |

- Z3 rises monotonically and crosses 0.5, but the native H falls to 0.972624, the max bond rises to 0.857 and the force RMS is still \(1.075\times10^{-2}\). The final state fails the H, force and resolution criteria, so claim 15 remains **UNVERIFIED (H estimator not converged; not converged; field under-resolved)**.
- Linear projections from the four rows put Z3=0.5 at t=6.0 (reached) and the force RMS at \(10^{-2}\) around t=6.8, but the H and resolution criteria are already failing, so the projection cannot promote the claim.
- Every chunk completed between 533.9 and 577.5 s, inside the 900 s G-003 cap. The first chunk reproduces the N=96 \(t=1.5\) row of the resolution check.

- Verification after the N=96 extension: two separate runs, 176/176 checks each.

## 2026-10-30 - Pure-FN candidate Hessian (claims 14b/14e)

**Added**

- `sims.stability.classify_candidate`, `phenomenology/purefn_stability_report.py`, `make purefn`, `tests/test_purefn_stability.py`, and `results/hopfion_purefn_stability.{json,md}`.

**Results**

- Candidate: N=64, \(\mu=0\), 1200 steps; H=0.999999932, Z3=\(3.0\times10^{-6}\), force RMS \(3.154\times10^{-3}\), virial \(+78.18\).
- The tangent operator passes the N=8 dense check (maximum error \(1.3\times10^{-15}\)). At N=64 the spectrum has 8 negative modes; the minimum eigenvalue is \(-2.600799\).
- The pure-FN candidate is a saddle (14e **FALSIFIED**), so 14b remains **UNVERIFIED**; no stable Hopfion is demonstrated. A different candidate is not ruled out.
- The diagnostic completed in 111.7 s, inside the 900 s G-003 cap.

- Verification after the pure-FN Hessian: two separate runs, 178/178 checks each.

## 2026-10-30 - Saddle-escape search (claim 14b, ADR-006)

**Added**

- `sims.stability.hessian_modes`, `normalize_mode`, `perturb_along_mode` and `classify_escape`; `phenomenology/saddle_escape_report.py`; `make escape`; `tests/test_saddle_escape.py`; `results/hopfion_saddle_escape.{json,md}`.

**Results**

- Base pure-FN candidate: H=0.999999932, Z3=\(3.0\times10^{-6}\), force RMS \(3.154\times10^{-3}\); lowest tangent eigenvalue \(-2.600799\).
- Perturbing along that mode by 0.05 and 0.15 (unit interior RMS) and relaxing for 600 steps gives H=0.320365 and 0.237385, max bond 1.967 and 1.994, force RMS \(6.775\times10^{-3}\) and \(7.640\times10^{-3}\).
- Neither perturbed field passes the pre-registered H>0.9 acceptance test, so no Hessian was computed and no local minimum was found. Claim 14b remains **UNVERIFIED**.
- The search completed in 167.9 s, inside the 900 s G-003 cap.

- Verification after the saddle-escape search: two separate runs, 180/180 checks each.

## 2026-10-30 - Box-length scan (claim 14b, ADR-007)

**Added**

- `phenomenology/length_scan_report.py`, `make lengthscan`, `results/hopfion_length_scan.{json,md}`, and two audit checks.

**Results**

| length | H | force RMS | max bond | min eigenvalue | negative |
|---:|---:|---:|---:|---:|---:|
| 8 | 1.000000047 | 3.208e-3 | 0.475 | -1.469962 | 8 |
| 10 | 0.999999932 | 3.154e-3 | 0.586 | -2.600799 | 8 |
| 12 | 0.999966431 | 3.149e-3 | 0.703 | -4.223930 | 8 |
| 14 | 0.998468715 | 3.643e-3 | 0.844 | -6.669667 | 8 |

- Every length keeps H>0.9 and passes the acceptance gate, but every Hessian has 8 negative modes. The saddle is not a length-10 artifact; the larger boxes are more unstable.
- Claim 14b remains **UNVERIFIED**.
- The scan completed in 383.5 s, inside the 900 s G-003 cap.

- Verification after the box-length scan: two separate runs, 182/182 checks each.

## 2026-10-30 - Vortex-knot mass spectrum (claim 17, ADR-008)

**Added**

- `theory/vortex_particles.py`, `phenomenology/vortex_spectrum_report.py`, `make vortex`, `tests/test_vortex_particles.py`, and `results/vortex_knot_spectrum.{json,md}`.

**Results**

| knot | length | energy | energy ratio | observed ratio |
|---:|---:|---:|---:|---:|
| T(2,3) | 22.956185 | 392.166309 | 1.000000 | 1.000000 |
| T(2,5) | 34.078583 | 563.899493 | 1.437909 | 206.768283 |
| T(2,7) | 45.918809 | 761.854481 | 1.942682 | 3477.365267 |

- The Koide \(Q\) of the knot energies is 0.339340, against the observed 0.666664 and the target \(2/3\).
- Matching the charged-lepton hierarchy would need \(m\propto E^p\) with \(p=12.279\), or an exponentially small core.
- Claim 17 is **FALSIFIED**. This falsifies the simplest vortex assignment, not the general fluid-vortex idea; a viable model needs a hierarchy mechanism.

- Verification after the vortex-knot spectrum: two separate runs, 184/184 checks each.

## 2026-10-30 - One-parameter vortex hierarchy (claim 18, ADR-009)

**Added**

- `theory/vortex_cascade.py`, `phenomenology/vortex_cascade_report.py`, `make cascade`, `tests/test_vortex_cascade.py`, and `results/vortex_cascade.{json,md}`.

**Results**

| model | parameter | predicted tau ratio | observed | relative error | Q predicted |
|---|---:|---:|---:|---:|---:|
| geometric | 206.768283 | 42753.122850 | 3477.365267 | +11.294689 | 0.870542 |
| power law, muon fit | 7.691871 | 4676.868795 | 3477.365267 | +0.344946 | 0.696122 |
| power law, Q fit | 7.064301 | 2347.081108 | 3477.365267 | -0.325040 | 0.666667 |

- No one-parameter hierarchy matches both the tau ratio and \(Q\) within the pre-registered 10% / 0.01 tolerances.
- A two-parameter offset can fit the masses, but then \(Q\) is reproduced rather than predicted; the test therefore falsifies the one-parameter fluid hierarchy, not the general vortex idea.
- Claim 18 is **FALSIFIED**.

- Verification after the one-parameter hierarchy test: two separate runs, 186/186 checks each.

## 2026-10-30 - Real Z3 rotor potential scan (claim 19, ADR-010)

**Added**

- `theory/rotor_potential.py`, `phenomenology/rotor_potential_report.py`, `make rotorpotential`, `tests/test_rotor_potential.py`, and `results/rotor_potential.{json,md}`.

**Results**

- A real potential \(V=\mu(1-\cos3\phi)+\nu(1-\cos6\phi)\) keeps \(E_1=E_2\) at zero flux, as the antiunitary symmetry requires.
- Over \(\mu=1\), \(\nu=0,1,3,10\) and flux \(A=0\ldots1\), the maximum Koide \(Q\) of the three lowest sector energies is 0.337639, against \(2/3\); deep wells drive \(Q\to1/3\).
- The best point is converged in the Fourier cutoff (\(n_{\rm cut}\) 120 vs 240, delta \(9.4\times10^{-15}\)).
- Claim 19 is **FALSIFIED**. A real potential plus a fractional flux is not enough; a complex phase or a different mass mapping is required.

- Verification after the real Z3 rotor potential scan: two separate runs, 188/188 checks each.

## 2026-10-30 - Madelung fluid dictionary (claim 20, ADR-011)

**Added**

- `theory/madelung.py`, `theory/madelung.md`, `phenomenology/madelung_report.py`, `make madelung`, `tests/test_madelung.py`, and `results/madelung.{json,md}`.

**Results**

- The Madelung dictionary maps the Schrodinger equation exactly to continuity plus Euler with the quantum pressure \(Q=-\hbar^2\partial_x^2\sqrt{\rho}/(2m\sqrt{\rho})\).
- Free Gaussian density \(L^2\) error: \(8.8\times10^{-13}\); coherent state: \(1.2\times10^{-4}\); continuity residual: \(3.3\times10^{-7}\); exact-Gaussian Euler core residual: \(6.1\times10^{-3}\).
- Claim 20 is **NUMERIC (verified)**. This is an exact representation, not a microphysical derivation: \(Q\) is nonlocal and singular at nodes, and the Born rule and measurement are not explained.
- The next step is a microphysical model of \(Q\) (stochastic mechanics, pilot wave, walking droplets, or a superfluid vacuum) with a falsifiable correction.

- Verification after the Madelung dictionary: two separate runs, 190/190 checks each.

## 2026-10-30 - Nelson stochastic kinematics (claim 21, ADR-012)

**Added**

- `theory/nelson.py`, `phenomenology/nelson_report.py`, `make nelson`, `tests/test_nelson.py`, and `results/nelson.{json,md}`.

**Results**

- Harmonic-oscillator ground state: variance 0.500625 against 0.5; histogram \(L^1=0.0177\) with 50k particles and 80 bins.
- Free Gaussian packet at \(t=1\): histogram \(L^1=0.0195\).
- Claim 21 is **NUMERIC (verified)**. The drift uses the exact quantum density, so this validates the Nelson stochastic kinematics, not a self-consistent derivation of the density.
- A modified diffusion law or a mean-field closure is the next falsifiable step.

- Verification after the Nelson stochastic kinematics: two separate runs, 192/192 checks each.

## 2026-10-30 - Gaussian Madelung reduction (claim 22, ADR-013)

**Added**

- `theory/gaussian_fluid.py`, `phenomenology/gaussian_fluid_report.py`, `make gaussianfluid`, `tests/test_gaussian_fluid.py`, and `results/gaussian_fluid.{json,md}`.

**Results**

- With a Gaussian density and \(v=A(t)x\), the Madelung equations reduce to the Ermakov equation \(\ddot\sigma=-\omega^2\sigma+\hbar^2/(4m^2\sigma^3)\).
- The effective potential \(V(\sigma)=\frac12\omega^2\sigma^2+\hbar^2/(8m^2\sigma^2)\) has its minimum at \(\sigma^2=\hbar/(2m\omega)=0.5\).
- Integration from \(2\sigma_q\) gives \(\sigma\in[0.353553,1.414214]\) around \(0.707107\) with energy drift \(2.8\times10^{-12}\); small oscillations have period \(3.141600\) against \(\pi\) (relative error \(2.3\times10^{-6}\)).
- Claim 22 is **PROVEN (Ermakov reduction; numerics confirm)**. The ground-state width is derived from the fluid equations, not inserted from the wavefunction; the Born rule and measurement remain outside the reduction.

- Verification after the Gaussian Madelung reduction: two separate runs, 194/194 checks each.

## 2026-10-30 - Moving-center Gaussian reduction (claim 23, ADR-014)

**Added**

- Center functions in `theory/gaussian_fluid.py`, `phenomenology/coherent_fluid_report.py`, `make coherentfluid`, `tests/test_coherent_fluid.py`, and `results/coherent_fluid.{json,md}`.

**Results**

- The displaced Gaussian decouples into \(\ddot x_0=-\omega^2x_0\) and the Ermakov width.
- \(x_0(\pi)=-0.999999917\), \(x_0(2\pi)=0.999999983\); center period 6.284000 against \(2\pi\) (0.013%).
- Width error 0 and energy drift \(7.1\times10^{-15}\) for the fixed-width case; displaced width energy drift \(2.7\times10^{-12}\).
- Claim 23 is **PROVEN (moving-center reduction; numerics confirm)**. The first excited state still has a node singularity.

- Verification after the moving-center Gaussian reduction: two separate runs, 196/196 checks each.

## 2026-10-30 - First excited state and the node singularity (claim 24, ADR-015)

**Added**

- `theory/excited_fluid.py`, `phenomenology/excited_fluid_report.py`, `make excitednode`, `tests/test_excited_fluid.py`, and `results/excited_fluid.{json,md}`.

**Results**

- The first excited state has density \(\rho_1\propto x^2e^{-x^2/\sigma^2}\), with a node at \(x=0\): density 0, phase slip \(\pi\).
- The discrete phase-gradient maximum scales as \(dx^{-1}\): values 20.106, 40.212, 80.425 and 160.850 for \(N=256,512,1024,2048\); fitted slope \(-1.000000000\).
- The regular quantum pressure matches the analytic expression away from the node (relative error \(4.8\times10^{-13}\) at \(x=0.996\)).
- Claim 24 is **FALSIFIED**: the node is a phase slip, not a regular single-valued fluid point. A two-component or defect model is required.

- Verification after the first-excited-state node analysis: two separate runs, 198/198 checks each.

## 2026-10-30 - Hydrogen 1s Madelung reduction (claim 25, ADR-016)

**Added**

- `theory/hydrogen_fluid.py`, `phenomenology/hydrogen_fluid_report.py`, `make hydrogen`, `tests/test_hydrogen_fluid.py`, and `results/hydrogen_fluid.{json,md}`.

**Results**

- With \(\rho=(1/\pi a_0^3)e^{-2r/a_0}\), the \(1/r\) terms in \(V+Q=\mu\) cancel at \(a_0=\hbar^2/(me^2)=1\), leaving \(\mu=-0.5\).
- Density normalization 1.000000000000; force-balance residual 0 at \(r=0.5,1,2,5\); a wrong radius \(a=2\) gives a residual of \(-0.5\).
- SI values: \(a_0=5.291773369\times10^{-11}\) m and \(E=-13.605687\) eV.
- Claim 25 is **PROVEN (Coulomb-quantum pressure balance)**. The Bohr radius is derived from the fluid force balance, not inserted from the wavefunction; excited states remain node-singular.

- Verification after the hydrogen 1s reduction: two separate runs, 200/200 checks each.

## 2026-10-30 - Hydrogen 2p vortex identification (claim 26, ADR-017)

**Added**

- `theory/hydrogen_vortex.py`, `phenomenology/hydrogen_vortex_report.py`, `make hydrogenvortex`, `tests/test_hydrogen_vortex.py`, and `results/hydrogen_vortex.{json,md}`.

**Results**

- The \(2p_{m=\pm1}\) angular node at the origin is a quantized vortex: radial normalization 1.000000000000, most-probable radius 4.000000000 \(a_0\).
- Circulation \(2\pi\hbar m/m_e=6.283185307180\) (atomic units); the loop integral at \(r=2,\theta=\pi/2\) matches it.
- Azimuthal velocity 0.5; angular momentum \(\hbar m=1.000000000000\).
- Claim 26 is **NUMERIC (2p vortex identification)**. The \(m=0\) state has no vortex; this identifies the defect, not the 2p energy, and radial nodes still require a separate model.

- Verification after the hydrogen 2p vortex identification: two separate runs, 202/202 checks each.

## 2026-10-30 - Shallow-water acoustic horizon (claim 27, ADR-018)

**Added**

- `theory/water_analogue.py`, `phenomenology/water_analogue_report.py`, `make wateranalogue`, `tests/test_water_analogue.py`, and `results/water_analogue.{json,md}`.

**Results**

- Lab scales: \(h=0.1\) m, \(Q=10^{-3}\) m³/s. Sound speed \(c=\sqrt{gh}=0.990454441\) m/s; horizon \(r_H=Q/(2\pi hc)=1.606888076\times10^{-3}\) m.
- Surface gravity \(\kappa=c^2/r_H=610.496783\) m/s²; analogue temperature \(T_H=\hbar\kappa/(2\pi k_B)=7.421580\times10^{-10}\) K.
- At the horizon \(v/c=1\) and \(g_{tt}=0\); the metric sign changes across it.
- Claim 27 is **PROVEN (acoustic horizon; analogue)**. This is a long-wavelength acoustic analogy; capillary dispersion breaks the metric at short scales.

- Verification after the shallow-water acoustic horizon: two separate runs, 204/204 checks each.
