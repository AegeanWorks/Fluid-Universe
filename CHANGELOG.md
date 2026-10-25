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
