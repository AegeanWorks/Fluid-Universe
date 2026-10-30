# 1D simulation baseline

This is a validation baseline, not a 3D Hopfion result. Energy: E = int dx [1/2 (dphi/dx)^2 + mu(1-cos 3phi)], Dirichlet boundaries.

## Kink convergence

| N | dx | energy | energy - T | relative error |
|---:|---:|---:|---:|---:|
| 201 | 0.150000 | 2.659236767620 | -7.429899e-03 | -2.786e-03 |
| 401 | 0.075000 | 2.664796083728 | -1.870583e-03 | -7.015e-04 |
| 801 | 0.037500 | 2.666198193324 | -4.684733e-04 | -1.757e-04 |
| 1601 | 0.018750 | 2.666549496467 | -1.171702e-04 | -4.394e-05 |

Analytic tension T = (8/3) sqrt(mu) = 2.666666666667. Observed orders: 1.99, 2.00, 2.00.

## Baselines

- Gradient check: max relative error = 4.168e-09 (pass = True).
- Gradient flow: E 2.834069209595 -> 2.726098348898; monotone = True.
- Hessian: symmetric = True; kink min eigenvalue = -5.208e-08; vacuum min eigenvalue = 9.010966; near-zero count = 1.
- Rotor: |E0 Fourier - grid| = 7.438e-07; |E1 Fourier - grid| = 1.302e-06.

## Verdicts

- Baseline 1 (kink): True
- Baseline 2 (gradient): True
- Baseline 3 (monotone flow): True
- Baseline 6 (rotor): True
- Baseline 7 (Hessian, partial): True
- Production ready: False
- Claim 15: UNVERIFIED / NOT ACHIEVED

1D baselines only. No 3D lattice, no Hopf charge, no Z3 order parameter, and no production run. The earlier lost-run numbers are not used.

- Config SHA-256: `d7d364e4c8477fa288d3c3bd105f2fe7e3d2e03ccd1c2ebbf302a6967b461d23`
- Git: `57a59effaa2ccb976393a2537a90bd1b44337666` (dirty=True)
