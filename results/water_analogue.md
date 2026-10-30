# Shallow-water acoustic horizon (claim 27)

Depth h=0.1 m, flow rate Q=0.001 m^3/s, g=9.81 m/s^2.

## Scales

- Sound speed: 0.990454441 m/s.
- Horizon radius: 1.606888076e-03 m.
- Surface gravity: 610.496783 m/s^2.
- Analogue temperature: 7.421580e-10 K.

## Metric across the horizon

| r/r_H | r (m) | v/c | g_tt |
|---:|---:|---:|---:|
| 0.5 | 8.034440378e-04 | 2.000000000 | 2.943000e+00 |
| 0.9 | 1.446199268e-03 | 1.111111111 | 2.301111e-01 |
| 1 | 1.606888076e-03 | 1.000000000 | -2.220446e-16 |
| 1.1 | 1.767576883e-03 | 0.909090909 | -1.702562e-01 |
| 2 | 3.213776151e-03 | 0.500000000 | -7.357500e-01 |

## Verdict

- Horizon exists: **True**.
- Claim 27: **PROVEN (acoustic horizon; analogue)**.

The long-wavelength shallow-water metric has a horizon at r_H=Q/(2 pi h c), surface gravity c^2/r_H and analogue temperature hbar kappa/(2 pi k_B). This is an acoustic analogue, not a fundamental metric; capillary dispersion breaks it at short wavelengths.

- Config SHA-256: `b5a7ff9f16e6374b5477ae87d5ec627377272e677015c8c84a6558072add9840`
- Git: `2e4f283a9477348337a8101234773b90f41fd012` (dirty=True)
