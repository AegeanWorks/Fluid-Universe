# Independent verification

Every claim currently marked PROVEN, VERIFIED, or NUMERIC is re-derived here without importing `theory` or `phenomenology`.

**147/147 checks passed.**

| claim | result | detail |
|---|---|---|
| 2a: M = M^dagger | PASS | symbolic difference is the zero matrix |
| 2a: M != M^T for generic delta | PASS | M - M^T has off-diagonal 2 i c sin(delta) |
| 2b/F1: eigenpair k=0 | PASS | M v_k - lambda_k v_k simplifies to zero |
| 2b/F1: eigenpair k=1 | PASS | M v_k - lambda_k v_k simplifies to zero |
| 2b/F1: eigenpair k=2 | PASS | M v_k - lambda_k v_k simplifies to zero |
| 1a/F2: Q identity | PASS | symbolic difference simplifies to zero |
| 1c/F3: Q=2/3 iff c^2/a^2=1/2 | PASS | sympy solves c^2 = [a**2/2] |
| F1 numeric: eigenvalues vs numpy | PASS | max |eigvalsh - formula| = 1.243e-14 over 100000 draws |
| F2 numeric: Q identity | PASS | max relative |Q - (1/3+2c^2/3a^2)| = 2.963e-14 |
| 1d: signed vs physical Q | PASS | roots [-0.8, 1.8999999999999988, 1.9000000000000008], signed Q=0.8733333333, physical Q=0.3714555766 |
| F8: c/a <= 1 for positive roots | PASS | max c/a = 0.977566 over 20000 random positive-root triples |
| F8: 1/3 <= Q <= 1 for positive masses | PASS | Q range [0.333340, 0.970424] |
| 1b: 3 parameters fit 3 masses exactly | PASS | max residual 8.527e-14 over 1000 random positive-root triples |
| 1b: Q is not fixed by the Z3/circulant structure | PASS | random triples span Q in [0.3334, 0.9431] |
| 5d: c/a=1/2 bound is false | PASS | (10,1,1): a=4.000, c/a=0.750000, Q=0.7083333333 |
| 3a: independent (a,c,delta) fit | PASS | a=17.715838478, c=12.526948122, delta=2.316619864297 (stored 17.715838478, 12.526948122, 2.316619864297) |
| 3a: Q from PDG pole masses | PASS | Q=0.666664463415; Q-2/3=-2.203e-06 |
| 3a: exact fit residual | PASS | stored max residual 9.992e-15 |
| 3a: tau prediction | PASS | independent m_tau=1776.969027 MeV, stored 1776.969027 |
| 3a: quoted uncertainties | PASS | sigma_Q=5.063e-06, sigma_r=5.370e-06 |
| 3a: (Q-2/3)/sigma | PASS | (Q-2/3)/sigma = -0.435 |
| 3a: Monte Carlo mean/std reproduce | PASS | Q mean/std 0.666664474471/5.063e-06 vs stored 0.666664474471/5.063e-06 |
| 4b: independent RGE reproduces Q(mu) and r(mu) | PASS | max |Delta Q|=4.441e-16, max |Delta r|=4.441e-16 vs stored table |
| F10: RK45 and DOP853 independent integrators agree | PASS | max |Delta Q|=2.220e-16, max |Delta r|=1.110e-16 |
| 3b: MSbar Q(M_Z) != 2/3 | PASS | independent Q(M_Z)=0.667823994186, Q-2/3=+1.157e-03 |
| 4b: one-loop drift is nonzero but small | PASS | Delta Q=+2.888e-06 over ln(1e16/MZ)=32.328 |
| 4a: exact one-loop invariance would require a generation-blind RHS | PASS | the (3/2)y_k^2 term is generation-dependent, so Q and r run; the nonzero drift confirms it |
| F9/3b: pole-to-MSbar conversion reproduces Xing-Zhang | PASS | independent Q=0.667915958, stored 0.667915958, literature 0.66792 |
| F9: converted e vs Xing-Zhang | PASS | relative difference +1.891e-04 |
| F9: converted mu vs Xing-Zhang | PASS | relative difference +1.067e-04 |
| F9: converted tau vs Xing-Zhang | PASS | relative difference +3.404e-05 |
| F10: one-loop absolute running vs three-loop literature | PASS | max relative mass difference 5.024%; loop truncation, not a precision check |
| F10: Q from literature masses vs one-loop Q | PASS | max relative Q difference 3.937e-05; the shape is stable at this level |
| F9: conversion reproduced to 2e-4 | PASS | max relative difference 1.891e-04 |
| 5a: independent rotor spectrum | PASS | independent E0=0.891186159736, E1=1.317034514042, E2=1.317034514042 |
| 5c: k=1 and k=2 exactly degenerate | PASS | |E1-E2|=0.000e+00 |
| 5a: truncation convergence | PASS | E0 at n=20,40,80,160: [np.float64(0.8911861597406692), np.float64(0.8911861597407595), np.float64(0.8911861597404851), np.float64(0.8911861597373525)] |
| 5b: free-rotor limit | PASS | E(sector 0)=0, 4.5 (twofold); E(sector 1)=0.5, 2, 8, 12.5 at mu=0, I=1 |
| 5c: naive map gives two distinct roots | PASS | mapped roots [1.4589839654780183, 1.0331356111713113, 1.033135611171311] -> 2 distinct (3 needed) |
| 6a: 1-cos(3phi) = 2 sin^2(3phi/2) | PASS | symbolic identity used to remove the absolute value on 0 <= phi <= 2pi/3 |
| 6a: domain-wall tension symbolic | PASS | sympy integral = 8*sqrt(mu)/3 (sin(3phi/2) >= 0 on the interval) |
| 6a: tension quadrature mu=0.5 | PASS | 1.885618083164 vs 1.885618083164 |
| 6a: tension quadrature mu=1.0 | PASS | 2.666666666667 vs 2.666666666667 |
| 6a: tension quadrature mu=7.3 | PASS | 7.204936579257 vs 7.204936579257 |
| 7a: Q_nu at m_light=0 (NO and IO) | PASS | NO=0.585612009369, IO=0.500006739587 |
| 7a: 3-sigma corner maximum | PASS | corner max=0.590606037134; stored=0.590606037134 |
| 7a: no scanned point reaches 2/3 | PASS | scan max=0.585612009369; margin=0.081055 |
| 8: Q_up and Q_down at M_Z | PASS | Q_up=0.887569231319, Q_down=0.748065054803 |
| 8: six-scale ranges and no 2/3 | PASS | Q_up range [0.887569231,0.892553734], Q_down range [0.738535231,0.748065055] |
| 7b: minimal PMNS branch angles | PASS | DFT={'theta12_deg': 45.000000000000014, 'theta13_deg': 35.264389682754654, 'theta23_deg': 45.000000000000014}; identity={'theta12_deg': 0.0, 'theta13_deg': 0.0, 'theta23_deg': 0.0} |
| 7b: both branches excluded; theta13-only claim ill-posed | PASS | measured theta13=8.58, DFT=35.264, identity=0 |
| 9: delta closed-form search | PASS | best=2/9, diff=2.540e-06, sigma=0.407, p_LEE=7.927e-04 |
| 10: Frank-Oseen elastic matrix | PASS | equal=[28.0, 28.0], x=[4.0, 8.0], z=[12.0, 12.0] |
| 10: two modes and no photon | PASS | two transverse director components; no gauge field or vector potential in the F-O energy |
| 14a: Derrick derivative equals virial | PASS | numeric dE/dlambda=2.499999998, virial=2.500000000 |
| 14a: stationary scale and curvature | PASS | lambda^2=0.387425887, second derivative=5.000000 |
| 14b: kappa>0 necessary, existence unproved | PASS | kappa=0 has no stationary point; kappa<0 is unbounded; Derrick scaling is not an existence proof |
| 13a: RP2 homotopy has no Z3 factor | PASS | homotopy={'pi1': 'Z2', 'pi2': 'Z', 'pi3': 'Z'} |
| 13b: standard extensions do not derive Z3 | PASS | extensions=['rank_three_order_parameter', 'three_coupled_nematics', 'three_fold_lattice'] |
| 6b: trivial 2D lower bound | PASS | T=2.666666666667, T*L_y(2)=5.333333333333 |
| 6c: moment convergence and log divergence | PASS | I_eff finite iff alpha>4; alpha=4 log-divergent; alpha=2 power-divergent |
| 16: no new prediction found | PASS | 7 candidate classes, none new |
| 11a: axis speeds for K=(1,2,3) | PASS | x=[1.0, 1.4142135623730951], z=[1.7320508075688772, 1.7320508075688772] |
| 11a/F23: residual anisotropy xi | PASS | xi(K=1,2,3)=1.003479956; xi(equal)=0 |
| 12a: photon bounds are not directly applicable | PASS | all four bounds flagged non-applicable; director modes are neutral |
| 12b: conditional bound | PASS | equal K is exactly isotropic; O(1) xi fails every Delta c/c bound |
| 15/4: preimage linking converges to 1 | PASS | H(100/200/400)=0.999331467/0.999832946/0.999958242 |
| 15/4: reflection, rotation, shear | PASS | reflection=-0.999832946, rotation=0.999832946, shear=0.999834138 |
| 15/4: grid estimator matches known answer | PASS | grid(32/48/64)=1.000141490/0.999999788/1.000000039; baseline_4=True |
| 15: single-seed 3D pilot values | PASS | H(mu=0)=0.997143405, H(mu=20)=0.852975980, Z3(mu=20)=0.443233719 |
| 15: no H/Z3 coexistence, pilot only | PASS | no row satisfies |H|>=1 and Z3>=0.5; single seed |
| docs: claims Q | PASS | searched for '0.666664463' |
| docs: claims r | PASS | searched for '0.707104444' |
| docs: claims MSbar Q | PASS | searched for '0.667823994' |
| docs: claims Delta Q | PASS | searched for '2.89\times10^{-6}' |
| docs: claims Delta r | PASS | searched for '3.06\times10^{-6}' |
| docs: claims E0 | PASS | searched for '0.8911861597' |
| docs: claims E1 | PASS | searched for '1.3170345140' |
| docs: scorecard Q | PASS | searched for '0.666664463' |
| docs: scorecard MSbar Q | PASS | searched for '0.6678240' |
| docs: scorecard Delta Q | PASS | searched for '2.89\times10^{-6}' |
| docs: registry F9 downgraded to reproduced | PASS | searched for 'REPRODUCED at one loop' |
| docs: registry F10 downgraded to partial | PASS | searched for 'CITED + partial cross-check' |
| docs: shape-only diagnostic labelled | PASS | searched for 'NOT a cross-check' |
| docs: paper Q | PASS | searched for '0.66666446' |
| docs: paper r | PASS | searched for '0.70710444' |
| docs: paper MSbar Q | PASS | searched for '0.667823994' |
| docs: paper E0 | PASS | searched for '0.8911861597' |
| docs: paper E1 | PASS | searched for '1.3170345140' |
| docs: claims Q_nu max | PASS | searched for '0.585612' |
| docs: claims Q_up | PASS | searched for '0.887569' |
| docs: claims Q_down | PASS | searched for '0.748065' |
| docs: claims trimaximal theta13 | PASS | searched for '35.26' |
| docs: scorecard Q_nu | PASS | searched for '0.585612' |
| docs: scorecard Q_up range | PASS | searched for '0.8876' |
| docs: scorecard Q_down range | PASS | searched for '0.7385' |
| docs: README Q_nu | PASS | searched for '0.585612' |
| docs: README Q_up | PASS | searched for '0.888' |
| docs: README Q_down | PASS | searched for '0.748' |
| docs: paper Q_nu | PASS | searched for '0.585612' |
| docs: paper Q_up | PASS | searched for '0.887569' |
| docs: paper Q_down | PASS | searched for '0.748065' |
| docs: summary Q_nu | PASS | searched for '0.585612' |
| docs: claims delta sigma | PASS | searched for '0.407' |
| docs: claims Q_nu | PASS | searched for '0.585612' |
| docs: scorecard delta sigma | PASS | searched for '0.407' |
| docs: README delta sigma | PASS | searched for '0.407' |
| docs: paper delta sigma | PASS | searched for '0.41\sigma' |
| docs: paper B Derrick virial | PASS | searched for 'E_2-E_4+3V=0' |
| docs: paper B Frank-Oseen dispersion | PASS | searched for '\omega^2=(K/\rho)k^2' |
| docs: summary Derrick virial | PASS | searched for 'E2-E4+3V' |
| docs: claims exact reduction row | PASS | searched for '6c' |
| docs: claims direct topology row | PASS | searched for '13a' |
| docs: claims extension row | PASS | searched for '13b' |
| docs: claims no new prediction | PASS | searched for 'none found' |
| docs: scorecard I_eff | PASS | searched for 'log-divergent' |
| docs: README I_eff | PASS | searched for 'log-divergent' |
| docs: summary I_eff | PASS | searched for 'log-divergent' |
| docs: paper B I_eff | PASS | searched for 'logarithmically divergent' |
| docs: paper A no prediction | PASS | searched for 'No genuinely new prediction' |
| docs: claims xi | PASS | searched for '1.00348' |
| docs: scorecard xi | PASS | searched for '1.00348' |
| docs: README xi | PASS | searched for '1.00348' |
| docs: summary xi | PASS | searched for '1.00348' |
| docs: paper B xi | PASS | searched for '1.00348' |
| docs: paper B GRB order | PASS | searched for '10^{-18}' |
| docs: claims Hopf preimage | PASS | searched for '0.999958' |
| docs: README Hopf preimage | PASS | searched for '0.999958' |
| docs: summary Hopf preimage | PASS | searched for '0.999958' |
| docs: paper B Hopf preimage | PASS | searched for '0.999958' |
| docs: claims grid Hopf | PASS | searched for '1.000000' |
| docs: README grid Hopf | PASS | searched for '1.000000' |
| docs: summary grid Hopf | PASS | searched for '1.000000' |
| docs: paper B grid Hopf | PASS | searched for '1.00000004' |
| docs: claims 3D pilot Z3 | PASS | searched for '0.4432' |
| docs: README 3D pilot Z3 | PASS | searched for '0.443' |
| docs: summary 3D pilot Z3 | PASS | searched for '0.443234' |
| docs: paper B 3D pilot Z3 | PASS | searched for '0.4432' |
| docs: claims baseline 4 | PASS | searched for 'baseline 4 passes' |
| docs: summary 6b row | PASS | searched for '| 6b | 2D reduction lower bound' |
| docs: summary 6c row | PASS | searched for '| 6c | Exact dimensional reduction' |
| docs: registry preimage rows | PASS | searched for 'F16a' |
| docs: registry grid rows | PASS | searched for 'F16b' |
| docs: summary lists the rotor result | PASS | summary.md must describe the rotor output it now includes |
| docs: abstract distinguishes scheme conversion from full-SM matching | PASS | abstract must quote both the one-pass QED conversion (0.6679160) and the full-SM matching value (0.6678240), without conflating them |
| docs: F10 not overstated as an exact verification | PASS | F10 is cross-checked only at the few-percent level against three-loop literature |
| docs: fit numbers match JSON | PASS | stored fit and RG values match the regenerated outputs |
