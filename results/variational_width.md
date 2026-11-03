# Gaussian Ritz width-selection (claim 49)

G-M extremum selects sigma^2 = 0.5 (maximum); Yasui selects nothing.

| quantity | value |
|---|---:|
| analytic sigma*^2 | 0.500000000000 |
| grid argmax | 0.500000000000 |
| Newton refined | 0.500000000000 |
| curvature d2J/d(s2)^2 | -2.000000000000 |
| curvature d2J/d(sigma)^2 | -4.000000000000 |
| Yasui max derivative | -0.531250 |

## Scalings (predicted s2* = nu/omega)

| omega | mass | predicted | refined | curvature_s2 |
|---|---|---|---|---:|
| 2.0 | 1.0 | 0.250000000000 | 0.250000000000 | -16.000000000000 |
| 1.0 | 2.0 | 0.250000000000 | 0.250000000000 | -8.000000000000 |

## Verdict

- Claim 49: **NUMERIC (Ritz width-selection demonstrated)**.

G-M Ritz extremum selects sigma^2 = nu/omega = 0.5 with maximum curvature; Yasui has no finite extremum. Location coincides with the Ermakov fixed point (claim 22) from an independent route.

- Config SHA-256: `df0557050c86adbd5dcbf6a2a944c2e8d1d574165c3748065757e728798748a0`
- Git: `c5f428203702f80257980071d94eb3684d3e08d2` (dirty=True)
