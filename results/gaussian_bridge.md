# Fixed-endpoint Gaussian bridge (claim 54)

Two Pinney branches; Newton finds both; no third attractor.

| start | attributed | max residual |
|---|---|---:|
| const_s0 | B_1 (5.65e-08) | 5.164e-11 |
| const_sq | B_1 (5.65e-08) | 7.186e-10 |
| linear | B_1 (5.65e-08) | 5.367e-11 |
| bump | B_1 (5.65e-08) | 4.973e-11 |
| vdip | B_2 (8.19e-06) | 5.052e-11 |
| hbump | B_1 (5.65e-08) | 4.676e-11 |
| Jacobi B_1 first zero | None |
| analytic action B_1 | -0.390634083 |
| scaling omega=2 branches | 2 |

## Verdict

- Claim 54: **NUMERIC (fixed-endpoint Gaussian bridge demonstrated)**.

Discrete G-M action with pinned Gaussian endpoints admits exactly the two analytic Pinney branches (primary B_1 and bounce B_2); all Newton starts land on one of them; no third attractor. Jacobi primary branch is zero-free.

- Config SHA-256: `e23cb4a7ab817b4df588bfe6dc9a2d3e7283b8ac065e8074ef574c0f351288a6`
- Git: `20a4cb7143034ab5366b69cdfe6a68b294d0e8a9 (dirty=True)`
