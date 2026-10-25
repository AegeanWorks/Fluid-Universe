# Claim Ledger

Status vocabulary: **PROVEN** (symbolic/analytic proof), **NUMERIC** (reproducible numerical result with stated inputs), **UNVERIFIED**, **RETRACTED**, **FALSIFIED**.

Every entry names the test most likely to kill it. A claim without an evidence path is not allowed to appear in the paper.

> **Standing warning.** The map \((a,c,\delta)\mapsto(\sqrt{m_0},\sqrt{m_1},\sqrt{m_2})\) is a three-parameter reparametrisation of three masses. A successful fit to the three charged leptons is therefore **not** evidence for Z3; the Koide value is the independent extra condition \(|c|/a=1/\sqrt2\). See 1b.

| ID | Claim | Label | Status | Evidence | Kill test / next test |
|---|---|---|---|---|---|
| 1a | For \(M=\mathrm{circ}(a,ce^{i\delta},ce^{-i\delta})\), the eigenvalues are \(a+2c\cos(\delta+2\pi k/3)\) and \(Q=1/3+2c^2/(3a^2)\). | derived | **PROVEN** | `theory/circulant.py`; `tests/test_circulant.py`; `theory/formulas.md` F1-F2 | Symbolic simplification of LHS\(-\)RHS; 1e4 random numeric draws; limits. |
| 1b | The Z3/circulant structure alone forces \(Q=2/3\). | assumed (in the original hypothesis) | **FALSIFIED** | `claims.md` warning; `verification/audit.py` (1b checks); any three positive \(\sqrt m\) can be fitted | Count parameters: 3 masses, 3 parameters \((a,c,\delta)\). The Koide condition is the extra constraint \(c/a=1/\sqrt2\). |
| 1c | \(Q=2/3\iff \lvert c\rvert/a=1/\sqrt2\) (signed eigenvalues). | derived | **PROVEN** | `tests/test_circulant.py::test_koide_condition` | Symbolic solve; numeric spot checks. |
| 1d | The identity \(Q=1/3+2c^2/(3a^2)\) holds for the **physical** masses when some signed eigenvalues are negative. | derived | **FALSIFIED** | `tests/test_circulant.py::test_sign_caveat` | Counterexample \(a=1,c=0.9,\delta=\pi\): signed \(Q=0.8733\), physical \(Q=0.3715\). Physical \(Q\) uses \(\lvert\lambda_k\rvert\). |
| 2a | \(M=\mathrm{circ}(a,ce^{i\delta},ce^{-i\delta})\) is Hermitian, not complex symmetric (for generic \(\delta\)). | derived | **PROVEN** | `tests/test_circulant.py::test_hermitian_not_symmetric`; `theory/formulas.md` F4 | \(M-M^\dagger=0\); \(M-M^T\ne0\). |
| 2b | The eigenvectors are the DFT columns and are independent of \(\delta\). | derived | **PROVEN** | `tests/test_circulant.py::test_dft_eigenvectors`; F5 | \(Mv_k=\lambda_kv_k\) symbolically; eigenvector entries contain no \(\delta\). |
| 3a | Fit of \((a,\lvert c\rvert,\delta)\) to PDG **pole** masses, with propagated uncertainties. | numeric | **NUMERIC** | `phenomenology/fit.py`; `results/pdg_fit.json` | \(a=17.715838478\), \(c=12.526948122\), \(r=0.707104444\), \(Q=0.666664463\), \(\sigma_Q=5.1\times10^{-6}\). Regenerate from `python -m phenomenology.run_all`. |
| 3b | \(Q=2/3\) for **MS-bar** charged-lepton masses at \(M_Z\). | derived (claimed) | **FALSIFIED** | `results/rg_run.json`; `theory/literature.md` | \(Q(M_Z)=0.667823994\), about \(0.18\%\) above \(2/3\). Pole vs running is a scheme/threshold effect. |
| 4a | \(\lvert c\rvert/a\) is exactly RG-invariant at one loop. | derived (claimed) | **FALSIFIED** | `phenomenology/rg.py`; `results/rg_run.json`; `tests/test_rg.py` | The \(y_k^2\) term is not scalar; \(r(\mu)\) changes by \(3.06\times10^{-6}\) over 16 decades (about \(4.3\times10^{-6}\) relative). |
| 4b | The one-loop drift of \(Q\) and \(r\) is numerically negligible for charged leptons. | numeric | **NUMERIC** | `results/rg_run.json` | \(\Delta Q=+2.89\times10^{-6}\), \(\Delta r=+3.06\times10^{-6}\) from \(M_Z\) to \(10^{16}\) GeV. |
| 4c | Earlier "two-loop drift \(\sim4\times10^{-9}\)/decade" estimate. | estimate | **RETRACTED** | `CHANGELOG.md` 2026-10-25 | No script or derivation existed. Not used. A genuine two-loop run is future work. |
| 5a | Rotor \(H=L^2/(2I)+\mu(1-\cos3\phi)\) spectrum converges in a truncated Fourier basis. | derived | **PROVEN (numeric)** | `theory/rotor.py`; `results/rotor.json`; `tests/test_rotor.py` | \(E_0=0.8911861597\), \(E_1=1.3170345140\) at \(I=\mu=1\); stable to \(10^{-9}\) from \(n_{\rm cut}=20\) to 160; Fourier vs grid agrees to \(<10^{-5}\) relative at \(N=2048\). |
| 5b | Free-rotor limit reproduces \(E_n=n^2/(2I)\). | derived | **PROVEN** | `tests/test_rotor.py::test_free_rotor_limit` | \(E_0=0\), \(E_1=E_2=1/2\), \(E=9/2\) at \(I=1\), exact to \(10^{-12}\). |
| 5c | The rotor-to-circulant map can give three distinct lepton masses. | assumed | **FALSIFIED** | `results/rotor.json`; `theory/rotor.py:parity_obstruction` | \(E_1-E_2=0\) exactly; the naive map gives roots \((1.45898,1.03314,1.03314)\), only two distinct. |
| 5d | Universal bound \(\lvert c\rvert/a\le1/2\). | sketch | **FALSIFIED** | `theory/formulas.md` F8; `tests/test_circulant.py::test_bound_half_is_false` | Positive roots \((10,1,1)\) give \(c/a=0.75\), \(Q=0.7083\). The general bound is \(c/a\le1\). |
| 6a | Domain-wall tension \(T=\int_0^{2\pi/3}\sqrt{2\mu(1-\cos3\phi)}\,d\phi=(8/3)\sqrt\mu\). | derived | **PROVEN** | `theory/rotor.py`; `tests/test_rotor.py`; `results/rotor.json` | Symbolic substitution plus quadrature agree to \(10^{-12}\); \(T(1)=8/3\), \(T(4)=16/3\). |
| 6b | Dimensional reduction is exact (BPS bound), and the rotor moment of inertia is finite for the profile used. | derived (claimed) | **UNVERIFIED** | not implemented | Write the proof; check \(\int\rho\,r^2\) convergence. The original profile makes it log-divergent. |
| 7 | Neutrino \(Q_\nu<2/3\) throughout the allowed parameter space; circulant PMNS ruled out by \(\theta_{13}\). | derived (claimed) | **UNVERIFIED** | not implemented | Scan \((m_1,\text{ordering})\); re-derive the PMNS statement with explicit basis/permutation/phase assumptions. |
| 8 | Koide extended directly to quarks gives \(Q_U\sim0.85\), \(Q_D\sim0.73\). | numeric (literature) | **UNVERIFIED** (literature: \(Q_U\sim0.89\), \(Q_D\sim0.74\) at \(M_Z\)) | `theory/literature.md`; not re-derived here | Recompute with explicit running masses and definitions before recording. |
| 9 | \(\delta\) has a closed form (e.g. a rational multiple of \(\pi\)). | speculation | **UNVERIFIED** | not implemented | Search with a stated prior; report null results; do not promote near-misses. |
| 10 | Frank-Oseen with \(K_1=K_2=K_3\) gives \(\omega^2=c^2k^2\) for linearised director modes. | derived | **UNVERIFIED** | not implemented | Mode count, polarisations, and whether any mode can be a photon. "Emergent Lorentz invariance" remains unverified. |
| 11 | Emergent metric / RG flow to a Lorentz-invariant IR point. | speculation | **UNVERIFIED** | not implemented | Real one-loop anisotropy analysis or downgrade. |
| 12 | Lorentz-violation bounds are satisfied by the residual anisotropy. | speculation | **UNVERIFIED** | not implemented | Compare with birefringence, GRB dispersion, CMB, clock/cavity limits. |
| 13 | Z3 is derived from nematic topology. | speculation | **UNVERIFIED** (expected **FALSIFIED**) | not implemented | Nematic target is \(RP^2\) with \(\pi_1=Z_2\), \(\pi_2=Z\), \(\pi_3=Z\); \(\cos3\phi\) is inserted by hand. Test rank-3/triple-nematic/lattice extensions. |
| 14 | Derrick/virial analysis admits stable Hopfions in the exact FN+Skyrme+Z3 Lagrangian. | derived | **UNVERIFIED** | not implemented | Symbolic scaling exponents and parameter region; no coexistence claim before this. |
| 15 | 3D simulation achieves \(\lvert H\rvert\ge1\) **and** Z3 order. | numeric | **UNVERIFIED / NOT ACHIEVED** | `sims/README.md` | Rebuild from a validated core. Do not cite lost-run values. |
| 16 | The framework has at least one genuinely new falsifiable prediction beyond \(Q=2/3\). | speculation | **UNVERIFIED** | `SCORECARD.md` | Search mass/mixing/dispersion/defect-count predictions; if none, say so in the abstract. |

## Status history

- 2026-10-25: ledger created. 1a/1c/2a/2b proven symbolically; 1b/3b falsified by parameter counting and by the MS-bar value; 4a falsified as an exact statement, with 4b recorded as a numerical stability result; 4c retracted. No simulation claims.

- 2026-10-25 (second pass): 5a and 5b proven; 5c and 5d falsified; 6a proven; 6b remains unverified. Rotor route to three masses is closed.

- 2026-10-25 (verification pass): an independent audit re-derived every PROVEN/VERIFIED/NUMERIC claim. A gauge-coupling RHS bug was found and fixed (the state is logarithmic). Final state: two separate runs agree, 66/66 checks each. Project guideline G-001 requires two separate runs for every claim verification; this does not apply to the pytest suite.
