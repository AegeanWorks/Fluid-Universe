# Dispersive shallow-water horizon (claim 29)

Depth 0.1 m, flow rate 0.05 m^3/s; capillary frequency 84.824 rad/s (13.500 Hz).

| omega (rad/s) | k (1/m) | c_g/c | r_H/r_H0 |
|---:|---:|---:|---:|
| 0.1 | 0.050482 | 0.999987 | 1.000013 |
| 1 | 0.505247 | 0.998728 | 1.001273 |
| 10 | 5.541848 | 0.868417 | 1.151520 |
| 100 | 230.150696 | 0.192589 | 5.192407 |
| 1e+03 | 1291.979000 | 0.459270 | 2.177367 |

## Verdict

- Low-frequency ratio: 1.000013.
- High-frequency ratio: 5.192407.
- Claim 29: **PROVEN (dispersive horizon correction)**.

The long-wavelength acoustic horizon is accurate for omega << omega_c: at 0.1 and 1 rad/s the shift is below 0.2%. At 10 rad/s it is about 15%; at 100 rad/s the horizon moves outward by a factor 5.2. The high-frequency horizon is not the acoustic metric horizon.

- Config SHA-256: `75d6e072d8709111b73531c78efea0ef7ac0058c0056e55ed01a371d715522ac`
- Git: `9731a4fac4ecaa08b2153c191a3fd234320ad9ea` (dirty=True)
