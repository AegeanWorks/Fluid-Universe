# Independent claim verification (two runs)

Project rule G-001: every claim in `claims.md` marked PROVEN, NUMERIC, or VERIFIED is verified twice, in two separate processes, and the per-claim results must agree. This applies to claims, not to the pytest suite.

| run | generated (UTC) | commit | passed/total | source SHA-256 | results SHA-256 |
|---|---|---|---|---|---|
| run1 | 2026-10-25T21:41:42+00:00 | `dea2f9e20f6af587d427c2cb8a958c87cfe9378d` | 66/66 | `d4cc8037573cc0f2...` | `9a9193615176a5d3...` |
| run2 | 2026-10-25T21:41:53+00:00 | `dea2f9e20f6af587d427c2cb8a958c87cfe9378d` | 66/66 | `d4cc8037573cc0f2...` | `9a9193615176a5d3...` |

- per-claim agreement: **True**
- per-claim detail equality: **True**
- fingerprints match current files: **True**
- all checks passed: **True**
- accepted: **True**

| claim | run 1 | run 2 | agree | detail match | detail |
|---|---|---|---|---|---|
| 1a/F2: Q identity | PASS | PASS | YES | YES | symbolic difference simplifies to zero |
| 1b: 3 parameters fit 3 masses exactly | PASS | PASS | YES | YES | max residual 8.527e-14 over 1000 random positive-root triples |
| 1b: Q is not fixed by the Z3/circulant structure | PASS | PASS | YES | YES | random triples span Q in [0.3334, 0.9431] |
| 1c/F3: Q=2/3 iff c^2/a^2=1/2 | PASS | PASS | YES | YES | sympy solves c^2 = [a**2/2] |
| 1d: signed vs physical Q | PASS | PASS | YES | YES | roots [-0.8, 1.8999999999999988, 1.9000000000000008], signed Q=0.8733333333, physical Q=0.3714555766 |
| 2a: M != M^T for generic delta | PASS | PASS | YES | YES | M - M^T has off-diagonal 2 i c sin(delta) |
| 2a: M = M^dagger | PASS | PASS | YES | YES | symbolic difference is the zero matrix |
| 2b/F1: eigenpair k=0 | PASS | PASS | YES | YES | M v_k - lambda_k v_k simplifies to zero |
| 2b/F1: eigenpair k=1 | PASS | PASS | YES | YES | M v_k - lambda_k v_k simplifies to zero |
| 2b/F1: eigenpair k=2 | PASS | PASS | YES | YES | M v_k - lambda_k v_k simplifies to zero |
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
| docs: abstract distinguishes scheme conversion from full-SM matching | PASS | PASS | YES | YES | abstract must quote both the one-pass QED conversion (0.6679160) and the full-SM matching value (0.6678240), without conflating them |
| docs: claims Delta Q | PASS | PASS | YES | YES | searched for '2.89\times10^{-6}' |
| docs: claims Delta r | PASS | PASS | YES | YES | searched for '3.06\times10^{-6}' |
| docs: claims E0 | PASS | PASS | YES | YES | searched for '0.8911861597' |
| docs: claims E1 | PASS | PASS | YES | YES | searched for '1.3170345140' |
| docs: claims MSbar Q | PASS | PASS | YES | YES | searched for '0.667823994' |
| docs: claims Q | PASS | PASS | YES | YES | searched for '0.666664463' |
| docs: claims r | PASS | PASS | YES | YES | searched for '0.707104444' |
| docs: fit numbers match JSON | PASS | PASS | YES | YES | stored fit and RG values match the regenerated outputs |
| docs: paper E0 | PASS | PASS | YES | YES | searched for '0.8911861597' |
| docs: paper E1 | PASS | PASS | YES | YES | searched for '1.3170345140' |
| docs: paper MSbar Q | PASS | PASS | YES | YES | searched for '0.667823994' |
| docs: paper Q | PASS | PASS | YES | YES | searched for '0.66666446' |
| docs: paper r | PASS | PASS | YES | YES | searched for '0.70710444' |
| docs: registry F10 downgraded to partial | PASS | PASS | YES | YES | searched for 'CITED + partial cross-check' |
| docs: registry F9 downgraded to reproduced | PASS | PASS | YES | YES | searched for 'REPRODUCED at one loop' |
| docs: scorecard Delta Q | PASS | PASS | YES | YES | searched for '2.89\times10^{-6}' |
| docs: scorecard MSbar Q | PASS | PASS | YES | YES | searched for '0.6678240' |
| docs: scorecard Q | PASS | PASS | YES | YES | searched for '0.666664463' |
| docs: shape-only diagnostic labelled | PASS | PASS | YES | YES | searched for 'NOT a cross-check' |
| docs: summary lists the rotor result | PASS | PASS | YES | YES | summary.md must describe the rotor output it now includes |
