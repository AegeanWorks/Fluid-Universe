# Capillary validity of the shallow-water horizon (claim 28)

Water at 20 C: rho=998 kg/m3, gamma=0.0728 N/m, g=9.81 m/s2; capillary length 2.726879301e-03 m.

| case | Q (m3/s) | r_H (m) | r_H/ell_c | capillary ratio | long wavelength |
|---|---:|---:|---:|---:|---|
| lab | 0.001 | 1.606888e-03 | 0.589 | 2.879788 | False |
| valid example | 0.05 | 8.034440e-02 | 29.464 | 0.001152 | True |

- Minimum flow for r_H >= 10 ell_c: 1.696994e-02 m3/s.
- Valid example surface gravity: 12.209936 m/s2; temperature 1.484316e-11 K.

## Verdict

- Lab example is long-wavelength: **False**.
- Valid example is long-wavelength: **True**.
- Claim 28: **PROVEN (capillary validity condition)**.

The lab example Q=1e-3 m3/s has r_H=1.607 mm, below the 2.727 mm capillary length, so capillary dispersion dominates at the horizon and the long-wavelength acoustic metric does not apply. A valid analogue needs r_H >= 10 ell_c, i.e. Q >= 0.01697 m3/s for h=0.1 m. Claim 27's formula remains exact in the long-wavelength limit; its lab example is flagged as capillary-dominated.

- Config SHA-256: `1dcd2d8cadeb893e223e17565d428eb28d7b5199fdaf2bec6fb760856120404b`
- Git: `fe63e51888c958533cc7791577aaceb86d7cf3c5` (dirty=True)
