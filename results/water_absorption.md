# Bound-free absorption in the finite well (claim 34)

Model: the same finite spherical well as claim 33, with the 1s bound state and the p-wave continuum. The continuum is matched at \(R\) and normalized asymptotically; the oscillator strength is \(f(\omega)=(2/3)\omega|\int u_f r u_i\,dr|^2\) in atomic units.
Observed hydrated-electron absorption: 1.7 eV.

## Born-depth continuum

| quantity | value |
|---|---:|
| radius | 2.858071056e-10 m (2.858071 Angstrom) |
| Born depth | 2.486988 eV |
| 1s binding | 0.527753 eV |
| continuum peak | 1.022658 eV |
| peak oscillator strength | 27.391101 |
| TRK sum rule (to 5 Hartree) | 0.986301 |

The Born-depth peak is 1.023 eV, below the observed 1.7 eV. The continuum is broad; there is no bound 1p state at this depth.

## Depth scan

| \(V_0\) (eV) | 1s binding (eV) | continuum peak (eV) | peak strength |
|---:|---:|---:|---:|
| 2.486988 | 0.527753 | 1.022658 | 27.391101 |
| 3.000000 | 0.867137 | 1.369501 | 33.821740 |
| 3.300000 | 1.082250 | 1.524151 | 41.248934 |
| 3.600000 | 1.306698 | 1.660885 | 54.124408 |
| 3.689281 | 1.375040 | 1.700070 | 59.892357 |
| 3.870092 | 1.515374 | 1.778131 | 76.793443 |

## Required extra depth

| quantity | value |
|---|---:|
| required depth | 3.689281 eV |
| extra depth above Born | 1.202293 eV |
| inverted peak | 1.700070 eV |
| 1s binding at that depth | 1.375040 eV |
| optical reorganization energy | 1.383104 eV |
| extra / reorganization | 0.869272 |

The extra depth is 1.202 eV, about 0.869 of the optical reorganization energy 1.383 eV. It is a quantitative target for a band-offset or polaron correction, not a fitted parameter.

## Verdict

- Born-depth peak is below 1.7 eV: **True**.
- TRK sum rule is consistent with 1: **True**.
- Required extra depth is positive: **True**.
- Inverted peak matches 1.7 eV: **True**.
- Claim 34: **NUMERIC (bound-free peak 1.023 eV; extra depth 1.202 eV)**.

The Born-depth finite well's bound-free 1s->p absorption peaks at 1.023 eV, not 1.7 eV. The continuum oscillator-strength sum rule is 0.986, validating the normalization. A 1.7 eV peak requires an extra 1.202 eV of well depth (V0=3.689 eV), about 0.869 of the optical reorganization energy 1.383 eV. The bound-free channel is broad and qualitatively matches the observed band; the extra depth is a target for a band-offset or polaron correction, not a fitted parameter.

- Config SHA-256: `d4e4a04a2c228edced0c5dadade8954f1a1e700593ceb8f683fe25d62f7d1695`
- Git: `c9881180dc5981415cbd46d88be2c98ac6f83834` (dirty=True)
