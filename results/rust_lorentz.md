# lorentz

| id | value | detail |
| --- | --- | --- |
| `lorentz.c2_along_x_low` | `1.000000000000000e0` | lower elastic eigenvalue along x for K=(1,2,3) |
| `lorentz.c2_along_x_high` | `2.000000000000000e0` | upper elastic eigenvalue along x for K=(1,2,3) |
| `lorentz.c2_along_z` | `3.000000000000000e0` | degenerate elastic eigenvalue along the background z axis |
| `lorentz.residual_anisotropy_equal_k` | `0.000000000000000e0` | xi vanishes at the isotropic Frank point K1=K2=K3 |
| `lorentz.residual_anisotropy_k123` | `1.000969374684728e0` | xi for K=(1,2,3) over 4000 deterministic unit directions |

## Provenance

- Generator: `fluid-universe-rust` (Rust edition 2024, no external crates).
- In-process determinism fingerprint (not G-001 evidence): run1=5cbe02b3bd3d042f, run2=5cbe02b3bd3d042f, agreement=PASS.
- Check-results FNV-1a 64-bit fingerprint: `13f283de41fd351d` (not a configuration hash or cryptographic digest).
