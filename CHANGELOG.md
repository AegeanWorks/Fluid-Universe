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
