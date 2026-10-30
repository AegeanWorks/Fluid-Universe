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
| 6b | PROVEN (trivial) | \(E_{2D}\ge T L_y\); equality iff \(\partial_y\phi=0\). |
| 6c | FALSIFIED | Exact reduction false; \(I_{\rm eff}\) log-divergent for a \(1/r^4\) tail. |
| 7a | NUMERIC (negative) | \(Q_\nu^{\rm max}=0.585612\) (NO), \(0.500007\) (IO); never reaches \(2/3\). |
| 7b | FALSIFIED (ill-posed) | Both minimal circulant branches excluded; \(\theta_{13}\)-only wording is not general. |
| 8 | FALSIFIED | \(Q_U\in[0.8876,0.8926]\), \(Q_D\in[0.7385,0.7481]\); no scale reaches \(2/3\). |
| 9 | UNVERIFIED (null) | 2/9 closest at 0.407\(\sigma\); look-elsewhere \(p=7.93\times10^{-4}\); not promoted. |
| 10 | PROVEN (equal K) | \(\omega^2=(K/\rho)k^2\); 2 transverse modes; no photon; preferred frame. |
| 11a | FALSIFIED | No metric; preferred frame; \(\xi=1.00348\) for \(K=(1,2,3)\). |
| 11b | UNVERIFIED | No one-loop anisotropy/RG analysis. |
| 12a | UNVERIFIED (not applicable) | Photon bounds do not directly constrain neutral director modes. |
| 12b | PROVEN (conditional) | Equal \(K\): \(\xi=0\); O(1) \(\xi\) excluded if a photon coupling existed. |
| 13a | FALSIFIED | \(RP^2\) has no Z3 factor; \(\cos3\phi\) inserted by hand. |
| 13b | UNVERIFIED (null) | Rank-3, three-nematic, and 3-fold-lattice extensions all require an inserted Z3. |
| 14a | PROVEN | \(d=3\) exponents \((1,-1,3)\); virial \(E_2-E_4+3V=0\). |
| 14b | UNVERIFIED | Pure-FN N=64: near-stationary H=1 candidate (RMS 0.00315); not a stability proof; the scale candidate (14d) and the pure-FN candidate (14e) are both saddles; a saddle-escape search collapses to H=0.24-0.32; a length scan 8-14 is a saddle at every length. |
| 14c | NUMERIC (candidate) | Scale iteration: virial 0.00961, H=0.99730, RMS 0.0030. |
| 14d | FALSIFIED | Validated tangent Hessian: min -5.799863, 8 negative modes at N=64; the scale candidate is a saddle. |
| 14e | FALSIFIED | Pure-FN N=64 candidate: min -2.600799, 8 negative modes; a saddle. |
| 15 | UNVERIFIED (N=96 extension) | N=64 production scan (`make production`, <15 min): native H=0.78800, 2x upsampled H=0.98924, Z3=0.60743, max bond 1.480; at N=96 (3000 steps, same t) native H=0.99994, upsampled H=1.00000, but Z3=0.17207 and force RMS 0.01495; a same-N dt check gives \|dZ3\|=4.6e-6; a checkpointed extension to t=6.0 raises Z3 through 0.172→0.599 but native H falls to 0.97262, max bond rises to 0.857 and force RMS is 0.01075; the final state fails H, force and resolution, so no verified coexistence. |
| 16 | UNVERIFIED (none found) | Seven candidate classes audited; no new falsifiable prediction. |
| 17 | FALSIFIED | Vortex knots T(2,3),T(2,5),T(2,7): energy ratios 1:1.44:1.94, Q=0.339, not 2/3; needs m∝E^12.3 or an exponentially small core. |

Claim audit: two separate runs agree per claim, 184/184 checks each (`python -m verification.verify`, `results/verification.md`). Guideline G-001 requires two runs for claims; the pytest suite is unchanged.
