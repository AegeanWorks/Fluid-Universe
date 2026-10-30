# Hopf-charge estimator validation (claim 15, baseline 4)

Analytic Hopf map on R^3 with n -> (0,0,1) at infinity.

## Preimage linking (validated)

| samples | H |
|---:|---:|
| 100 | 0.999331467 |
| 200 | 0.999832946 |
| 400 | 0.999958242 |

- Second target pair: H = 0.999752012.
- Reflection: H = -0.999832946 (sign flip).
- Rotation: H = 0.999832946.
- Shear: H = 0.999834138.

## Grid Chern-Simons integral (validated, normalized)

| N | H |
|---:|---:|
| 32 | 1.000141490 |
| 48 | 0.999999788 |
| 64 | 1.000000039 |

- Reflected map: H = -0.999999788 (sign flip).

**Verdict:** preimage converges to 1: **True**; sign flips under reflection: **True**; rotation invariant: **True**; shear invariant: **True**.

Grid integral matches 1: **True**; two-method agreement: **True**; baseline 4 pass: **True**; claim 15: **NOT ACHIEVED**.

Preimage linking and the normalized grid Chern-Simons integral both pass the analytic H=1 test and agree. The grid normalization is a documented convention fixed by that map. Baseline 4 passes; no 3D run or Hopfion claim.

- Config SHA-256: `ea007c16579c8a2c6ddd2e87066169562ff7a91c6b755c8a756b4dd8d6850d36`
- Git: `aaaa67ae66644bc61b17d37fd178f3b8d739917a` (dirty=True)
