# Neutrino Koide scan

Inputs: NuFit 5.2 (2022) with SK-atm. Scan: lightest mass 0 to 0.2 eV, 2001 points per ordering.

| ordering | Q at m_light=0 | Q min | Q max | m_light at Q max (eV) | monotonic decreasing |
|---|---:|---:|---:|---:|---|
| NO | 0.585612009369 | 0.333350018656 | 0.585612009369 | 0 | True |
| IO | 0.500006739587 | 0.333350576408 | 0.500006739587 | 0 | True |

| ordering | Q max at 3-sigma corner | dm21^2 corner | dm31^2 corner |
|---|---:|---:|---:|
| NO | 0.590606037134 | 6.820000e-05 | 2.590000e-03 |
| IO | 0.500008420310 | 8.030000e-05 | -2.406000e-03 |

**Verdict:** Q_nu below 2/3 everywhere scanned: **True**. Maximum Q_nu = 0.585612009369; 3-sigma corner maximum = 0.590606037134; margin to 2/3 = 0.076061.

- Config SHA-256: `679277a40cc0befde1c51ec80c8a86a774efa3adb9e6bc3216d232df6b892675`
- Git: `f3fe7a4bfb6607e99b7b4839bb75f00bb1e5fd49` (dirty=True)
