# Osmotic-closure no-go (claim 48)

Stationary FP with b=D*grad(ln rho) is the identity; sigma map flat.

| probe | slope_interior | finest interior residual |
|---|---|---:|
| gauss_0.125 | -1.976072 | 1.720e-03 |
| gauss_0.25 | -1.988026 | 3.042e-04 |
| gauss_0.5 | -1.994011 | 5.379e-05 |
| gauss_1.0 | -1.997005 | 9.511e-06 |
| gauss_2.0 | -1.998502 | 1.681e-06 |
| bimodal | -1.983850 | 1.521e-04 |

| sigma0^2 | sigma* @N=2048 | |err| |
|---|---|---:|
| 0.125 | 0.353553390593 | 0.000e+00 |
| 0.25 | 0.500000000000 | 5.551e-17 |
| 0.5 | 0.707106781187 | 0.000e+00 |
| 1.0 | 1.000000000000 | 0.000e+00 |
| 2.0 | 1.414213562316 | 5.677e-11 |

## Verdict

- Claim 48: **NUMERIC (osmotic-closure no-go demonstrated)**.

Stationary FP with self-consistent osmotic drift is the identity on smooth positive densities; the sigma map is its moment projection. Selection lives in the HJ/energy leg (claim 22 V(sigma) minimum), never in osmotic data.

- Config SHA-256: `ec89583f3c80651fe038d40db1fb4d1748ade9a4ed50fe7603ac049158a0d12e`
- Git: `a077b0b504e997404bc1910d2de3b633de10d737` (dirty=True)
