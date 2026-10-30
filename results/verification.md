# Independent claim verification (two runs)

Project rule G-001: every claim in `claims.md` marked PROVEN, NUMERIC, or VERIFIED is verified twice, in two separate processes, and the per-claim results must agree. This applies to claims, not to the pytest suite.

| run | generated (UTC) | commit | passed/total | source SHA-256 | results SHA-256 |
|---|---|---|---|---|---|
| run1 | 2026-10-30T19:37:03+00:00 | `7b7ca67c06a6a478c3b3dd7d552f20ab53288574` | 220/220 | `f8aaa8acb46e2f19...` | `83a19fd7a9a13746...` |
| run2 | 2026-10-30T19:37:11+00:00 | `7b7ca67c06a6a478c3b3dd7d552f20ab53288574` | 220/220 | `f8aaa8acb46e2f19...` | `83a19fd7a9a13746...` |

- per-claim agreement: **True**
- per-claim detail equality: **True**
- fingerprints match current files: **True**
- all checks passed: **True**
- accepted: **True**

| claim | run 1 | run 2 | agree | detail match | detail |
|---|---|---|---|---|---|
| 10: Frank-Oseen elastic matrix | PASS | PASS | YES | YES | equal=[28.0, 28.0], x=[4.0, 8.0], z=[12.0, 12.0] |
| 10: two modes and no photon | PASS | PASS | YES | YES | two transverse director components; no gauge field or vector potential in the F-O energy |
| 11a/F23: residual anisotropy xi | PASS | PASS | YES | YES | xi(K=1,2,3)=1.003479956; xi(equal)=0 |
| 11a: axis speeds for K=(1,2,3) | PASS | PASS | YES | YES | x=[1.0, 1.4142135623730951], z=[1.7320508075688772, 1.7320508075688772] |
| 12a: photon bounds are not directly applicable | PASS | PASS | YES | YES | all four bounds flagged non-applicable; director modes are neutral |
| 12b: conditional bound | PASS | PASS | YES | YES | equal K is exactly isotropic; O(1) xi fails every Delta c/c bound |
| 13a: RP2 homotopy has no Z3 factor | PASS | PASS | YES | YES | homotopy={'pi1': 'Z2', 'pi2': 'Z', 'pi3': 'Z'} |
| 13b: standard extensions do not derive Z3 | PASS | PASS | YES | YES | extensions=['rank_three_order_parameter', 'three_coupled_nematics', 'three_fold_lattice'] |
| 14a: Derrick derivative equals virial | PASS | PASS | YES | YES | numeric dE/dlambda=2.499999998, virial=2.500000000 |
| 14a: stationary scale and curvature | PASS | PASS | YES | YES | lambda^2=0.387425887, second derivative=5.000000 |
| 14b: box-length scan covers 8, 10, 12 and 14 at N=64 | PASS | PASS | YES | YES | L=8: min=-1.469962, L=10: min=-2.600799, L=12: min=-4.223930, L=14: min=-6.669667 |
| 14b: every box length is a saddle and the claim is not promoted | PASS | PASS | YES | YES | attempts=4, negative=[8, 8, 8, 8] |
| 14b: kappa>0 necessary, existence unproved | PASS | PASS | YES | YES | kappa=0 has no stationary point; kappa<0 is unbounded; Derrick scaling is not an existence proof |
| 14b: pure-FN candidate is the N=64 mu=0 relax | PASS | PASS | YES | YES | H=0.999999932, rms=3.154e-03 |
| 14b: saddle-escape finds no local minimum and does not promote | PASS | PASS | YES | YES | eps=0.05: H=0.320365, eps=0.15: H=0.237385 |
| 14b: saddle-escape starts from the pure-FN candidate | PASS | PASS | YES | YES | base min=-2.600799, attempts=2 |
| 14c: scale-stationary H=1 candidate | PASS | PASS | YES | YES | best iter=4, virial=+0.009610, H=0.997297, rms=3.002e-03 |
| 14d: N=64 scale candidate is a saddle | PASS | PASS | YES | YES | min=-5.799863, negative=8 |
| 14d: tangent Hessian operator passes dense validation | PASS | PASS | YES | YES | N=8, max error=1.332e-15 |
| 14e: pure-FN Hessian verdict follows the negative tolerance | PASS | PASS | YES | YES | min=-2.600799, negative=8 |
| 15/4: grid Hopf resolution | PASS | PASS | YES | YES | N=16 H=0.801799781, N=32 H=1.000141490, N=64 H=1.000000039 |
| 15/4: grid estimator matches known answer | PASS | PASS | YES | YES | grid(32/48/64)=1.000141490/0.999999788/1.000000039; baseline_4=True |
| 15/4: preimage linking converges to 1 | PASS | PASS | YES | YES | H(100/200/400)=0.999331467/0.999832946/0.999958242 |
| 15/4: reflection, rotation, shear | PASS | PASS | YES | YES | reflection=-0.999832946, rotation=0.999832946, shear=0.999834138 |
| 15: 3D pilot values at 1200 steps | PASS | PASS | YES | YES | H(mu=0)=0.749758423, H(mu=20)=0.052372282, Z3(mu=20)=0.960169273 |
| 15: N=64 diagnostic not converged and no coexistence | PASS | PASS | YES | YES | force_rms(3600)=3.716e-02 |
| 15: N=64 diagnostic values | PASS | PASS | YES | YES | H(20,3600)=0.788000, Z3=0.607430 |
| 15: N=96 verdict follows the unchanged production rule | PASS | PASS | YES | YES | status=UNVERIFIED (Z3 below threshold; not converged), ready=False |
| 15: checkpointed extension has four contiguous N=96 chunks | PASS | PASS | YES | YES | chunks=4, final t=6 |
| 15: dt arithmetic and verdict follow the pre-registered rule | PASS | PASS | YES | YES | converged=True, dH=-0.000107, dZ3=-0.000005 |
| 15: dt check holds N=64 fixed and reaches one physical time | PASS | PASS | YES | YES | dt=5.0e-04/2.5e-04, steps=3000/6000 |
| 15: extension first chunk reproduces the resolution check | PASS | PASS | YES | YES | H=0.999942144, Z3=0.172071 |
| 15: extension trend is monotone and the claim is not promoted | PASS | PASS | YES | YES | Z3=0.598690, status=UNVERIFIED (H estimator not converged; not converged; field under-resolved) |
| 15: perturbed seeds are recorded | PASS | PASS | YES | YES | seed 1: H=0.853402, Z3=0.443059, seed 2: H=0.852345, Z3=0.443074 |
| 15: pilot is not converged or resolved | PASS | PASS | YES | YES | force_rms(1200)=1.012e-01; no conclusion |
| 15: production dt check is recorded | PASS | PASS | YES | YES | dH=1.660e-06, dZ3=4.076e-06, converged=True |
| 15: production scan is the pre-registered N=64 run within G-003 limit | PASS | PASS | YES | YES | H=0.788000, Hup=0.989238, Z3=0.607430, rms=3.716e-02 |
| 15: production verdict follows the pre-registered rule | PASS | PASS | YES | YES | status=UNVERIFIED (H estimator not converged; not converged; field under-resolved), ready=False |
| 15: resolution arithmetic and verdict follow the pre-registered rule | PASS | PASS | YES | YES | converged=False, dH=+0.062978, dZ3=-0.356154 |
| 15: resolution check compares N=64 and N=96 at one physical time | PASS | PASS | YES | YES | N=64/96, steps=3000, H=0.999942, Z3=0.172071 |
| 16: no new prediction found | PASS | PASS | YES | YES | 7 candidate classes, none new |
| 17: simple vortex-knot assignment is falsified | PASS | PASS | YES | YES | Q=0.339340, p=12.279 |
| 17: vortex-knot spectrum uses T(2,3), T(2,5), T(2,7) | PASS | PASS | YES | YES | energies=[392.166, 563.899, 761.854] |
| 18: no one-parameter hierarchy matches both ratios and Q | PASS | PASS | YES | YES | min |tau error|=0.325040 |
| 18: one-parameter hierarchy test covers geometric and power-law spectra | PASS | PASS | YES | YES | geometric: tau=42753.1, Q=0.870542, power_law_koide_fit: tau=2347.1, Q=0.666667, power_law_muon_fit: tau=4676.9, Q=0.696122 |
| 19: real Z3 rotor never reaches Q=2/3 | PASS | PASS | YES | YES | best Q=0.337639, max by nu=0.337639 |
| 19: real Z3 rotor scan preserves zero-flux degeneracy and converges | PASS | PASS | YES | YES | rows=44, delta=9.437e-15 |
| 1a/F2: Q identity | PASS | PASS | YES | YES | symbolic difference simplifies to zero |
| 1b: 3 parameters fit 3 masses exactly | PASS | PASS | YES | YES | max residual 8.527e-14 over 1000 random positive-root triples |
| 1b: Q is not fixed by the Z3/circulant structure | PASS | PASS | YES | YES | random triples span Q in [0.3334, 0.9431] |
| 1c/F3: Q=2/3 iff c^2/a^2=1/2 | PASS | PASS | YES | YES | sympy solves c^2 = [a**2/2] |
| 1d: signed vs physical Q | PASS | PASS | YES | YES | roots [-0.8, 1.8999999999999988, 1.9000000000000008], signed Q=0.8733333333, physical Q=0.3714555766 |
| 20: Madelung dictionary reproduces the free Gaussian and coherent state | PASS | PASS | YES | YES | free L2=8.781e-13, coherent L2=1.217e-04, Euler=6.105e-03 |
| 20: dictionary verdict is recorded and not overclaimed | PASS | PASS | YES | YES | NUMERIC (exact representation verified) |
| 21: Nelson osmotic process reproduces the ground-state density | PASS | PASS | YES | YES | variance=0.500625, ground L1=0.0177, free L1=0.0195 |
| 21: stochastic kinematics verdict is recorded and not overclaimed | PASS | PASS | YES | YES | NUMERIC (stochastic kinematics verified) |
| 22: Gaussian Madelung fixed point is the quantum width | PASS | PASS | YES | YES | sigma^2=0.500000000000, drift=2.754e-12 |
| 22: small oscillations have the Ermakov 2 omega frequency | PASS | PASS | YES | YES | period=3.141600, error=2.338e-06 |
| 23: displaced width oscillates with conserved energy | PASS | PASS | YES | YES | energy drift=2.678e-12 |
| 23: moving-center Gaussian reproduces the coherent-state trajectory | PASS | PASS | YES | YES | x(pi)=-0.999999917, period=6.284000 |
| 24: first excited state has a node and a pi phase slip | PASS | PASS | YES | YES | slip=3.141592654, Q error=4.793e-13 |
| 24: phase gradient diverges as dx^-1 and the claim is falsified | PASS | PASS | YES | YES | slope=-1.000000000 |
| 25: SI values and the wrong-radius control are recorded | PASS | PASS | YES | YES | a0=5.291773369e-11, E=-13.605687 |
| 25: hydrogen 1s force balance gives the Bohr radius | PASS | PASS | YES | YES | a0=1.000000000000, residual=0.000e+00 |
| 26: angular momentum is hbar m and the vortex claim is recorded | PASS | PASS | YES | YES | Lz=1.000000000 |
| 26: hydrogen 2p has a quantized vortex with the 4a0 peak | PASS | PASS | YES | YES | a0 peak=4.000000, circulation=6.283185307 |
| 27: shallow-water horizon has v/c=1 and g_tt=0 | PASS | PASS | YES | YES | v/c=1.000000000, g_tt=-2.220e-16 |
| 27: surface gravity and analogue temperature are positive | PASS | PASS | YES | YES | kappa=610.496783, T=7.422e-10 |
| 28: a larger flow reaches the long-wavelength regime | PASS | PASS | YES | YES | Q_min=1.696994e-02, ratio=1.152e-03 |
| 28: lab horizon is capillary-dominated | PASS | PASS | YES | YES | r_H=1.607e-03, ell_c=2.727e-03, ratio=2.879788 |
| 29: high-frequency horizon is shifted by capillary dispersion | PASS | PASS | YES | YES | ratio(10)=1.151520, ratio(100)=5.192407 |
| 29: low-frequency horizon approaches the long-wavelength value | PASS | PASS | YES | YES | ratio(0.1)=1.000013, ratio(1)=1.001273 |
| 2a: M != M^T for generic delta | PASS | PASS | YES | YES | M - M^T has off-diagonal 2 i c sin(delta) |
| 2a: M = M^dagger | PASS | PASS | YES | YES | symbolic difference is the zero matrix |
| 2b/F1: eigenpair k=0 | PASS | PASS | YES | YES | M v_k - lambda_k v_k simplifies to zero |
| 2b/F1: eigenpair k=1 | PASS | PASS | YES | YES | M v_k - lambda_k v_k simplifies to zero |
| 2b/F1: eigenpair k=2 | PASS | PASS | YES | YES | M v_k - lambda_k v_k simplifies to zero |
| 30: outgoing speed changes sign at the horizon | PASS | PASS | YES | YES | v(0.9)=-0.110050, v(1.1)=+0.090041 |
| 30: ray times match and the peeling rate equals the surface gravity | PASS | PASS | YES | YES | max error=2.016e-06, kappa=12.209936 |
| 31: the horizon blocks the right-moving packet | PASS | PASS | YES | YES | fraction=7.503e-06, center=-1.433333 |
| 31: the uniform-flow control transmits | PASS | PASS | YES | YES | fraction=1.000000, center=4.433333 |
| 32: screened hydrogen is far too large and weakly bound | PASS | PASS | YES | YES | a*=4.149e-09, E*=-2.214e-03 |
| 32: the capillary cavity lands near the hydrated-electron scale | PASS | PASS | YES | YES | R=2.858e-10, E=0.932839, ratio=0.548729 |
| 33: no Born radius reaches 1.7 eV; the required depth exceeds Born | PASS | PASS | YES | YES | maxgap=0.614820, Rmin=3.181e-10, Vreq=3.715111, Born=2.234196, excess=1.480915 |
| 33: the Born-depth well at the capillary radius has no 1p | PASS | PASS | YES | YES | kappa=2.309130, u1s=2.049533, Born=2.486988, threshold=4.603378 |
| 34: a 1.7 eV peak requires an extra depth of about 1.202 eV | PASS | PASS | YES | YES | depth=3.689281, extra=1.202293, peak=1.700070 |
| 34: the Born-depth bound-free peak is below 1.7 eV | PASS | PASS | YES | YES | peak=1.022658, sum=0.986301, binding=0.527753 |
| 35: the 1.7 eV absorption is bound-bound and the well fits both | PASS | PASS | YES | YES | R=3.732e-10, V0=4.997523, E1s=-3.270000, E1p=-1.570000, gap=1.700000 |
| 35: the capillary/Born cavity is outside the VDE band | PASS | PASS | YES | YES | Rcap=2.858e-10, Born=2.486988, Rreq=3.732e-10, Vreq=4.997523 |
| 3a: (Q-2/3)/sigma | PASS | PASS | YES | YES | (Q-2/3)/sigma = -0.435 |
| 3a: Monte Carlo mean/std reproduce | PASS | PASS | YES | YES | Q mean/std 0.666664474471/5.063e-06 vs stored 0.666664474471/5.063e-06 |
| 3a: Q from PDG pole masses | PASS | PASS | YES | YES | Q=0.666664463415; Q-2/3=-2.203e-06 |
| 3a: exact fit residual | PASS | PASS | YES | YES | stored max residual 9.992e-15 |
| 3a: independent (a,c,delta) fit | PASS | PASS | YES | YES | a=17.715838478, c=12.526948122, delta=2.316619864297 (stored 17.715838478, 12.526948122, 2.316619864297) |
| 3a: quoted uncertainties | PASS | PASS | YES | YES | sigma_Q=5.063e-06, sigma_r=5.370e-06 |
| 3a: tau prediction | PASS | PASS | YES | YES | independent m_tau=1776.969027 MeV, stored 1776.969027 |
| 3b: MSbar Q(M_Z) != 2/3 | PASS | PASS | YES | YES | independent Q(M_Z)=0.667823994186, Q-2/3=+1.157e-03 |
| 4a: exact one-loop invariance would require a generation-blind RHS | PASS | PASS | YES | YES | the (3/2)y_k^2 term is generation-dependent, so Q and r run; the nonzero drift confirms it |
| 4b: independent RGE reproduces Q(mu) and r(mu) | PASS | PASS | YES | YES | max |Delta Q|=4.441e-16, max |Delta r|=4.441e-16 vs stored table |
| 4b: one-loop drift is nonzero but small | PASS | PASS | YES | YES | Delta Q=+2.888e-06 over ln(1e16/MZ)=32.328 |
| 5a: independent rotor spectrum | PASS | PASS | YES | YES | independent E0=0.891186159736, E1=1.317034514042, E2=1.317034514042 |
| 5a: truncation convergence | PASS | PASS | YES | YES | E0 at n=20,40,80,160: [np.float64(0.8911861597406692), np.float64(0.8911861597407595), np.float64(0.8911861597404851), np.float64(0.8911861597373525)] |
| 5b: free-rotor limit | PASS | PASS | YES | YES | E(sector 0)=0, 4.5 (twofold); E(sector 1)=0.5, 2, 8, 12.5 at mu=0, I=1 |
| 5c: k=1 and k=2 exactly degenerate | PASS | PASS | YES | YES | |E1-E2|=0.000e+00 |
| 5c: naive map gives two distinct roots | PASS | PASS | YES | YES | mapped roots [1.4589839654780183, 1.0331356111713113, 1.033135611171311] -> 2 distinct (3 needed) |
| 5d: c/a=1/2 bound is false | PASS | PASS | YES | YES | (10,1,1): a=4.000, c/a=0.750000, Q=0.7083333333 |
| 6a: 1-cos(3phi) = 2 sin^2(3phi/2) | PASS | PASS | YES | YES | symbolic identity used to remove the absolute value on 0 <= phi <= 2pi/3 |
| 6a: domain-wall tension symbolic | PASS | PASS | YES | YES | sympy integral = 8*sqrt(mu)/3 (sin(3phi/2) >= 0 on the interval) |
| 6a: tension quadrature mu=0.5 | PASS | PASS | YES | YES | 1.885618083164 vs 1.885618083164 |
| 6a: tension quadrature mu=1.0 | PASS | PASS | YES | YES | 2.666666666667 vs 2.666666666667 |
| 6a: tension quadrature mu=7.3 | PASS | PASS | YES | YES | 7.204936579257 vs 7.204936579257 |
| 6b: trivial 2D lower bound | PASS | PASS | YES | YES | T=2.666666666667, T*L_y(2)=5.333333333333 |
| 6c: moment convergence and log divergence | PASS | PASS | YES | YES | I_eff finite iff alpha>4; alpha=4 log-divergent; alpha=2 power-divergent |
| 7a: 3-sigma corner maximum | PASS | PASS | YES | YES | corner max=0.590606037134; stored=0.590606037134 |
| 7a: Q_nu at m_light=0 (NO and IO) | PASS | PASS | YES | YES | NO=0.585612009369, IO=0.500006739587 |
| 7a: no scanned point reaches 2/3 | PASS | PASS | YES | YES | scan max=0.585612009369; margin=0.081055 |
| 7b: both branches excluded; theta13-only claim ill-posed | PASS | PASS | YES | YES | measured theta13=8.58, DFT=35.264, identity=0 |
| 7b: minimal PMNS branch angles | PASS | PASS | YES | YES | DFT={'theta12_deg': 45.000000000000014, 'theta13_deg': 35.264389682754654, 'theta23_deg': 45.000000000000014}; identity={'theta12_deg': 0.0, 'theta13_deg': 0.0, 'theta23_deg': 0.0} |
| 8: Q_up and Q_down at M_Z | PASS | PASS | YES | YES | Q_up=0.887569231319, Q_down=0.748065054803 |
| 8: six-scale ranges and no 2/3 | PASS | PASS | YES | YES | Q_up range [0.887569231,0.892553734], Q_down range [0.738535231,0.748065055] |
| 9: delta closed-form search | PASS | PASS | YES | YES | best=2/9, diff=2.540e-06, sigma=0.407, p_LEE=7.927e-04 |
| F1 numeric: eigenvalues vs numpy | PASS | PASS | YES | YES | max |eigvalsh - formula| = 1.243e-14 over 100000 draws |
| F10: Q from literature masses vs one-loop Q | PASS | PASS | YES | YES | max relative Q difference 3.937e-05; the shape is stable at this level |
| F10: RK45 and DOP853 independent integrators agree | PASS | PASS | YES | YES | max |Delta Q|=2.220e-16, max |Delta r|=1.110e-16 |
| F10: one-loop absolute running vs three-loop literature | PASS | PASS | YES | YES | max relative mass difference 5.024%; loop truncation, not a precision check |
| F2 numeric: Q identity | PASS | PASS | YES | YES | max relative |Q - (1/3+2c^2/3a^2)| = 2.963e-14 |
| F8: 1/3 <= Q <= 1 for positive masses | PASS | PASS | YES | YES | Q range [0.333340, 0.970424] |
| F8: c/a <= 1 for positive roots | PASS | PASS | YES | YES | max c/a = 0.977566 over 20000 random positive-root triples |
| F9/3b: pole-to-MSbar conversion reproduces Xing-Zhang | PASS | PASS | YES | YES | independent Q=0.667915958, stored 0.667915958, literature 0.66792 |
| F9: conversion reproduced to 2e-4 | PASS | PASS | YES | YES | max relative difference 1.891e-04 |
| F9: converted e vs Xing-Zhang | PASS | PASS | YES | YES | relative difference +1.891e-04 |
| F9: converted mu vs Xing-Zhang | PASS | PASS | YES | YES | relative difference +1.067e-04 |
| F9: converted tau vs Xing-Zhang | PASS | PASS | YES | YES | relative difference +3.404e-05 |
| docs: F10 not overstated as an exact verification | PASS | PASS | YES | YES | F10 is cross-checked only at the few-percent level against three-loop literature |
| docs: README 3D pilot status | PASS | PASS | YES | YES | searched for 'resolution-limited' |
| docs: README Hopf preimage | PASS | PASS | YES | YES | searched for '0.999958' |
| docs: README Hopf resolution | PASS | PASS | YES | YES | searched for 'N>=32' |
| docs: README I_eff | PASS | PASS | YES | YES | searched for 'log-divergent' |
| docs: README N=64 diagnostic | PASS | PASS | YES | YES | searched for '0.8' |
| docs: README Q_down | PASS | PASS | YES | YES | searched for '0.748' |
| docs: README Q_nu | PASS | PASS | YES | YES | searched for '0.585612' |
| docs: README Q_up | PASS | PASS | YES | YES | searched for '0.888' |
| docs: README delta sigma | PASS | PASS | YES | YES | searched for '0.407' |
| docs: README grid Hopf | PASS | PASS | YES | YES | searched for '1.000000' |
| docs: README scale candidate | PASS | PASS | YES | YES | searched for 'Scale-stationary' |
| docs: README validated Hessian | PASS | PASS | YES | YES | searched for 'FALSIFIED: validated tangent Hessian' |
| docs: README xi | PASS | PASS | YES | YES | searched for '1.00348' |
| docs: abstract distinguishes scheme conversion from full-SM matching | PASS | PASS | YES | YES | abstract must quote both the one-pass QED conversion (0.6679160) and the full-SM matching value (0.6678240), without conflating them |
| docs: claims Delta Q | PASS | PASS | YES | YES | searched for '2.89\times10^{-6}' |
| docs: claims Delta r | PASS | PASS | YES | YES | searched for '3.06\times10^{-6}' |
| docs: claims E0 | PASS | PASS | YES | YES | searched for '0.8911861597' |
| docs: claims E1 | PASS | PASS | YES | YES | searched for '1.3170345140' |
| docs: claims Hopf preimage | PASS | PASS | YES | YES | searched for '0.999958' |
| docs: claims MSbar Q | PASS | PASS | YES | YES | searched for '0.667823994' |
| docs: claims N=64 diagnostic | PASS | PASS | YES | YES | searched for '0.60743' |
| docs: claims Q | PASS | PASS | YES | YES | searched for '0.666664463' |
| docs: claims Q_down | PASS | PASS | YES | YES | searched for '0.748065' |
| docs: claims Q_nu | PASS | PASS | YES | YES | searched for '0.585612' |
| docs: claims Q_nu max | PASS | PASS | YES | YES | searched for '0.585612' |
| docs: claims Q_up | PASS | PASS | YES | YES | searched for '0.887569' |
| docs: claims baseline 4 | PASS | PASS | YES | YES | searched for 'baseline 4 passes' |
| docs: claims delta sigma | PASS | PASS | YES | YES | searched for '0.407' |
| docs: claims direct topology row | PASS | PASS | YES | YES | searched for '13a' |
| docs: claims exact reduction row | PASS | PASS | YES | YES | searched for '6c' |
| docs: claims extension row | PASS | PASS | YES | YES | searched for '13b' |
| docs: claims grid Hopf | PASS | PASS | YES | YES | searched for '1.000000' |
| docs: claims no new prediction | PASS | PASS | YES | YES | searched for 'none found' |
| docs: claims r | PASS | PASS | YES | YES | searched for '0.707104444' |
| docs: claims scale candidate | PASS | PASS | YES | YES | searched for '14c' |
| docs: claims trimaximal theta13 | PASS | PASS | YES | YES | searched for '35.26' |
| docs: claims upsampled H | PASS | PASS | YES | YES | searched for '0.9892' |
| docs: claims validated Hessian | PASS | PASS | YES | YES | searched for '-5.799863' |
| docs: claims xi | PASS | PASS | YES | YES | searched for '1.00348' |
| docs: fit numbers match JSON | PASS | PASS | YES | YES | stored fit and RG values match the regenerated outputs |
| docs: paper A no prediction | PASS | PASS | YES | YES | searched for 'No genuinely new prediction' |
| docs: paper B 3D pilot status | PASS | PASS | YES | YES | searched for 'not converged' |
| docs: paper B Derrick virial | PASS | PASS | YES | YES | searched for 'E_2-E_4+3V=0' |
| docs: paper B Frank-Oseen dispersion | PASS | PASS | YES | YES | searched for '\omega^2=(K/\rho)k^2' |
| docs: paper B GRB order | PASS | PASS | YES | YES | searched for '10^{-18}' |
| docs: paper B Hessian minimum | PASS | PASS | YES | YES | searched for '$-5.799863$' |
| docs: paper B Hopf preimage | PASS | PASS | YES | YES | searched for '0.999958' |
| docs: paper B I_eff | PASS | PASS | YES | YES | searched for 'logarithmically divergent' |
| docs: paper B dense validation | PASS | PASS | YES | YES | searched for '1.3\times10^{-15}' |
| docs: paper B grid Hopf | PASS | PASS | YES | YES | searched for '1.00000004' |
| docs: paper B xi | PASS | PASS | YES | YES | searched for '1.00348' |
| docs: paper E0 | PASS | PASS | YES | YES | searched for '0.8911861597' |
| docs: paper E1 | PASS | PASS | YES | YES | searched for '1.3170345140' |
| docs: paper MSbar Q | PASS | PASS | YES | YES | searched for '0.667823994' |
| docs: paper Q | PASS | PASS | YES | YES | searched for '0.66666446' |
| docs: paper Q_down | PASS | PASS | YES | YES | searched for '0.748065' |
| docs: paper Q_nu | PASS | PASS | YES | YES | searched for '0.585612' |
| docs: paper Q_up | PASS | PASS | YES | YES | searched for '0.887569' |
| docs: paper delta sigma | PASS | PASS | YES | YES | searched for '0.41\sigma' |
| docs: paper r | PASS | PASS | YES | YES | searched for '0.70710444' |
| docs: registry F10 downgraded to partial | PASS | PASS | YES | YES | searched for 'CITED + partial cross-check' |
| docs: registry F9 downgraded to reproduced | PASS | PASS | YES | YES | searched for 'REPRODUCED at one loop' |
| docs: registry grid rows | PASS | PASS | YES | YES | searched for 'F16b' |
| docs: registry preimage rows | PASS | PASS | YES | YES | searched for 'F16a' |
| docs: scorecard Delta Q | PASS | PASS | YES | YES | searched for '2.89\times10^{-6}' |
| docs: scorecard I_eff | PASS | PASS | YES | YES | searched for 'log-divergent' |
| docs: scorecard MSbar Q | PASS | PASS | YES | YES | searched for '0.6678240' |
| docs: scorecard Q | PASS | PASS | YES | YES | searched for '0.666664463' |
| docs: scorecard Q_down range | PASS | PASS | YES | YES | searched for '0.7385' |
| docs: scorecard Q_nu | PASS | PASS | YES | YES | searched for '0.585612' |
| docs: scorecard Q_up range | PASS | PASS | YES | YES | searched for '0.8876' |
| docs: scorecard delta sigma | PASS | PASS | YES | YES | searched for '0.407' |
| docs: scorecard validated Hessian | PASS | PASS | YES | YES | searched for 'min -5.799863' |
| docs: scorecard xi | PASS | PASS | YES | YES | searched for '1.00348' |
| docs: shape-only diagnostic labelled | PASS | PASS | YES | YES | searched for 'NOT a cross-check' |
| docs: summary 3D pilot status | PASS | PASS | YES | YES | searched for 'resolution-limited' |
| docs: summary 6b row | PASS | PASS | YES | YES | searched for '| 6b | 2D reduction lower bound' |
| docs: summary 6c row | PASS | PASS | YES | YES | searched for '| 6c | Exact dimensional reduction' |
| docs: summary Derrick virial | PASS | PASS | YES | YES | searched for 'E2-E4+3V' |
| docs: summary Hopf preimage | PASS | PASS | YES | YES | searched for '0.999958' |
| docs: summary Hopf resolution | PASS | PASS | YES | YES | searched for 'N=16' |
| docs: summary I_eff | PASS | PASS | YES | YES | searched for 'log-divergent' |
| docs: summary Q_nu | PASS | PASS | YES | YES | searched for '0.585612' |
| docs: summary grid Hopf | PASS | PASS | YES | YES | searched for '1.000000' |
| docs: summary lists the rotor result | PASS | PASS | YES | YES | summary.md must describe the rotor output it now includes |
| docs: summary xi | PASS | PASS | YES | YES | searched for '1.00348' |
