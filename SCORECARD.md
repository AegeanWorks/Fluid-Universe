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
| 18 | FALSIFIED | One-parameter hierarchies fail: geometric tau 11.3x high; power law fitted to muon gives +34.5% tau and Q=0.696; fitted to Q gives -32.5% tau. |
| 19 | FALSIFIED | Real Z3 rotor with second harmonic and flux: max Q=0.3376, zero-flux degeneracy exact, deep wells Q→1/3; no 2/3. |
| 20 | NUMERIC (verified) | Madelung dictionary: free Gaussian L2 8.8e-13, coherent state 1.2e-4, continuity 3.3e-7, Euler 6.1e-3; exact representation, not a microphysical derivation. |
| 21 | NUMERIC (verified) | Nelson stochastic kinematics: OU variance 0.500625 vs 0.5; ground-state L1 0.0177; free-packet L1 0.0195; exact-density drift, so a consistency check. |
| 22 | PROVEN (Ermakov) | Gaussian Madelung reduction: σ²=0.5 is the effective-potential minimum; energy drift 2.8e-12; small-oscillation period 3.141600 vs π (2.3e-6). |
| 23 | PROVEN (moving center) | Displaced Gaussian decouples into x0''=-ω²x0 and the Ermakov width; center period 6.284000 vs 2π, width error 0, energy drift 7.1e-15. |
| 24 | FALSIFIED (node phase slip) | First excited state: density node at x=0, phase jump π, phase-gradient max scales as dx^-1 (slope -1.000000000); regular Q matches away from the node but the node is singular. |
| 25 | PROVEN (Coulomb balance) | Hydrogen 1s: a0=1, E=-0.5, normalization 1.000000000000, force residual 0; wrong radius gives -0.5. SI: a0=5.291773369e-11 m, E=-13.605687 eV. |
| 26 | NUMERIC (2p vortex) | 2p_(m=±1): circulation 2π, axial angular momentum 1, most-probable radius 4a0, radial normalization 1; m=0 has no vortex. |
| 27 | PROVEN (acoustic horizon) | Shallow water h=0.1 m, Q=1e-3 m³/s: c=0.990454 m/s, rH=1.606888e-3 m, κ=610.497 m/s², TH=7.422e-10 K; g_tt=0 at the horizon; long-wavelength only. |
| 28 | PROVEN (capillary validity) | Water ell_c=2.726879e-3 m; lab r_H 1.607 mm is capillary-dominated (ratio 2.8798); r_H>=10 ell_c requires Q>=1.697e-2 m³/s; Q=0.05 gives ratio 1.152e-3. |
| 29 | PROVEN (dispersive horizon) | ω_c=84.824 rad/s (13.5 Hz); horizon ratios 1.000013 at 0.1 rad/s, 1.151520 at 10, 5.192407 at 100; long-wavelength metric valid below ~1 rad/s. |
| 30 | PROVEN (radial separatrix) | Outgoing speed changes sign at r_H (-0.110 at 0.9r_H, +0.090 at 1.1r_H); ray times match analytic to 2e-6; peeling rate 12.209936 = surface gravity. |
| 31 | NUMERIC (horizon blocking) | 1D packet: horizon transmitted fraction 7.503e-6, final center -1.433 m; uniform control transmits 1.000000, center 4.433 m. |
| 32 | NUMERIC (cavity scale) | Hydrated electron: capillary cavity R=2.858 Å, E=0.933 eV vs observed 1.7 eV (factor 1.8); screened hydrogen a*=41.5 Å, E=-2.2 meV, ruled out. |
| 33 | NUMERIC (negative; required depth 3.715 eV) | Finite well: at R=2.858 Å/Born (2.487 eV) only 1s (E=-0.528 eV), no 1p; max Born-depth s->p gap 0.615 eV; a 1.7 eV gap needs V0=3.715 eV at R=3.181 Å (1.481 eV above Born). |
| 34 | NUMERIC (bound-free peak 1.023 eV; extra depth 1.202 eV) | Born-depth bound-free peak 1.023 eV (sum rule 0.986), below 1.7 eV; a 1.7 eV peak needs V0=3.689 eV (extra 1.202 eV, 0.869 of the optical reorganization energy 1.383 eV). |
| 35 | NUMERIC (two-observable inversion; R=3.732 Å, V0=4.998 eV) | VDE 3.27 eV + 1.7 eV absorption force bound-bound; finite well needs R=3.732 Å, V0=4.998 eV (E1s=-3.27, E1p=-1.57), vs capillary R=2.858 Å/Born 2.487 eV; 30.6% larger, 2.01x deeper. |
| 36 | NUMERIC (hard depth bound; Woods-Saxon scan) | VDE 3.27 eV > Born 2.487 eV, so the depth is 0.783 eV too small independent of shape; soft-wall scan: a=1.0 Å R=3.687 Å/V0=5.920 eV, a=1.5 Å R=3.078 Å/V0=6.643 eV (2.67x Born); softness trades radius for depth. |

Claim audit: two separate runs agree per claim, 222/222 checks each (`python -m verification.verify`, `results/verification.md`). Guideline G-001 requires two runs for claims; the pytest suite is unchanged.
