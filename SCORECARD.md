# Scorecard

One line per claim. See `claims.md` for the full statement, evidence, and kill test.

| ID | Status | One-line result |
|---|---|---|
| 1a | PROVEN | Circulant eigenvalues and \(Q=1/3+2c^2/(3a^2)\) verified symbolically and numerically. Likely known (Brannen 2006). |
| 1b | FALSIFIED | Z3/circulant alone does not predict \(Q=2/3\); 3 parameters fit 3 masses. |
| 1c | PROVEN | \(Q=2/3\iff \lvert c\rvert/a=1/\sqrt2\). |
| 1d | FALSIFIED | The \(Q(a,c)\) identity fails for physical masses when a signed eigenvalue is negative. |
| 2a | PROVEN | The matrix is Hermitian, not complex symmetric. |
| 2b | PROVEN | Eigenvectors are DFT columns, independent of \(\delta\). |
| 3a | NUMERIC | \(Q=0.666664463\), \(r=0.707104444\), \(\sigma_Q=5.1\times10^{-6}\); reproduced from `make results`. |
| 3b | FALSIFIED | MS-bar \(Q(M_Z)=0.6678240\ne2/3\). |
| 4a | FALSIFIED | Exact one-loop RG invariance of \(\lvert c\rvert/a\) is false. |
| 4b | NUMERIC | \(\Delta Q=+2.89\times10^{-6}\), \(\Delta r=+3.06\times10^{-6}\) over 16 decades. |
| 4c | RETRACTED | Earlier 2-loop drift estimate had no evidence. |
| 5a | PROVEN (numeric) | Fourier spectrum converges; \(E_0=0.89118616\), \(E_1=1.31703451\). |
| 5b | PROVEN | Free-rotor levels \(n^2/(2I)\) reproduced. |
| 5c | FALSIFIED | \(E_1=E_2\) exactly; naive map yields two distinct roots. |
| 5d | FALSIFIED | \(c/a\le1/2\) is false; \((10,1,1)\) gives \(c/a=0.75\). |
| 6a | PROVEN | \(T=(8/3)\sqrt\mu\) verified analytically and by quadrature. |
| 6b | UNVERIFIED | Dimensional-reduction/BPS proof not written; effective moment of inertia may diverge. |
| 7 | UNVERIFIED | Neutrino scan not done. |
| 8 | UNVERIFIED | Quark extension not re-derived (literature \(\sim0.89,\sim0.74\)). |
| 9 | UNVERIFIED | No closed-form \(\delta\) search with a stated prior. |
| 10 | UNVERIFIED | Frank-Oseen mode analysis not done. |
| 11 | UNVERIFIED | Emergent-metric claim unsupported. |
| 12 | UNVERIFIED | Lorentz-violation bounds not computed. |
| 13 | UNVERIFIED | Z3 not derived from nematic topology; \(\cos3\phi\) inserted by hand. |
| 14 | UNVERIFIED | Derrick/virial analysis not done. |
| 15 | UNVERIFIED / NOT ACHIEVED | No validated 3D run; lost numbers not cited. |
| 16 | UNVERIFIED | No new falsifiable prediction found so far. |

Claim audit: two separate runs agree per claim, 66/66 checks each (`python -m verification.verify`, `results/verification.md`). Guideline G-001 requires two runs for claims; the pytest suite is unchanged.
