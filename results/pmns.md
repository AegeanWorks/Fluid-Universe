# Circulant PMNS branches

Measured angles (NuFit 5.2 with SK-atm, NO): theta12=33.41 deg, theta13=8.58 deg, theta23=42.20 deg.

| branch | U | theta12 | theta13 | theta23 |
|---|---|---:|---:|---:|
| same_basis_both_circulant | identity (up to diagonal phases and permutations) | 0.000000 | 0.000000 | 0.000000 |
| neutrino_only_circulant | DFT (trimaximal, |U_alpha i| = 1/sqrt(3)) | 45.000000 | 35.264390 | 45.000000 |

**Verdict:** same-basis branch excluded: **True**. Trimaximal branch excluded by theta13: **True**. Original theta13-only claim well-posed: **False**.

A circulant PMNS is not a single model. If both mass matrices are circulant in the same basis the prediction is no mixing; if only the neutrino matrix is circulant the prediction is trimaximal mixing. Both are excluded, but only the second is specifically a theta13 statement.

- Config SHA-256: `5da0c58e80e9eb663df6af3af1d878d9c1816d635124861a5fbcc519bd929c1a`
- Git: `21b77fe5ff59be1785193139084ab924f920f9de` (dirty=True)
