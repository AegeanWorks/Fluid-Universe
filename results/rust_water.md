# water

| id | value | detail |
| --- | --- | --- |
| `water.sound_speed` | `9.904544411531507e-1` | c = sqrt(g h) at h=0.1 m, g=9.81 m/s^2 |
| `water.horizon_radius` | `1.606888075604941e-3` | r_H = Q / (2 pi h c) at Q=1e-3, h=0.1 |
| `water.surface_gravity` | `6.104967825034645e2` | kappa = c^2 / r_H at the recorded horizon |
| `water.hawking_temperature` | `7.421580272821032e-10` | T_H = hbar kappa / (2 pi kB) in kelvin |
| `water.capillary_length` | `2.726879300973213e-3` | l_c = sqrt(gamma/(rho g)) for water (gamma=0.0728, rho=998) |
| `water.capillary_ratio` | `2.879788378353929e0` | (l_c / r_H)^2: capillary versus acoustic-horizon scale |

## Provenance

- Generator: `fluid-universe-rust` (Rust edition 2024, no external crates).
- G-001 two-process fingerprint: run1=688c44f88c23ffb7, run2=688c44f88c23ffb7, agreement=PASS.
- config_sha256 (FNV-1a): `2b5dd03ee8a1da0b`.
