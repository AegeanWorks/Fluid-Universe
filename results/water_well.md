# Finite spherical well for the hydrated electron (claim 33)

Model: \(V(r)=-V_0\) for \(r<R\) and zero outside, with the Born depth \(V_0(R)=e^2(1-1/\epsilon_r)/(8\pi\epsilon_0R)\). The l=0 and l=1 matching conditions give the 1s and 1p roots.
Observed hydrated-electron absorption: 1.7 eV.

## Capillary cavity

| quantity | value |
|---|---:|
| radius | 2.858071056e-10 m (2.858071 Angstrom) |
| Born depth | 2.486988 eV |
| kappa | 2.309130 |
| 1s root | 2.049533 |
| 1s energy | -0.527753 eV |
| 1s binding | 0.527753 eV |
| 1p root | none |
| s->p gap | none |

The 1s->continuum threshold is 0.528 eV; there is no bound-bound line in this well.

## Depth scan at the capillary radius

| \(V_0\) (eV) | \(\kappa\) | \(u_{1s}\) | \(u_{1p}\) | gap (eV) |
|---:|---:|---:|---:|---:|
| 2.486988 | 2.309130 | 2.049533 | none | none |
| 5.000000 | 3.274135 | 2.343763 | 3.214248 | 2.256614 |
| 10.000000 | 4.630326 | 2.556648 | 3.614068 | 3.043406 |
| 30.000000 | 8.019960 | 2.786718 | 3.977093 | 3.755365 |

At the 1p threshold \(V_0=4.603378\) eV, the gap is 2.106463 eV. No depth gives 1.7 eV at this radius.

## Born-depth scan over radius

| quantity | value |
|---|---:|
| 1p threshold radius | 5.290247544e-10 m (5.290248 Angstrom) |
| maximum Born-depth s->p gap | 0.614820 eV |
| infinite-well s->p spacing at 4.703 Angstrom | 1.777772 eV |
| infinite-well radius for 1.7 eV | 4.809506420e-10 m (4.809506 Angstrom) |

For \(R<5.290\) Angstrom the Born well has no 1p; for \(R>5.290\) Angstrom the Born-depth gap is below 0.615 eV. The earlier 4.703 Angstrom infinite-well estimate is the ground-state energy scale, not the s->p spacing.

## Inverse: what a 1.7 eV gap would take

| \(R\) (Angstrom) | Born (eV) | required (eV) | excess (eV) |
|---:|---:|---:|---:|
| 3.181453 | 2.234196 | 3.715111 | 1.480915 |
| 3.200000 | 2.221246 | 3.715900 | 1.494654 |

Minimum required depth: 3.715111 eV at \(R=3.181452583e-10\) m. The missing depth is 1.480915 eV; the optical-polarization energy at that radius is 1.242517 eV (ratio 1.191867).

## Verdict

- Born well has no 1p at the capillary radius: **True**.
- The Born well never reaches 1.7 eV: **True**.
- The required depth is above Born: **True**.
- Claim 33: **NUMERIC (negative: Born well cannot give 1.7 eV; required depth 3.715 eV)**.

The minimal Born-depth finite well does not explain the 1.7 eV hydrated-electron absorption: at the capillary radius it binds only 1s, and the maximum Born-depth s->p gap at any radius is 0.615 eV. A 1.7 eV gap requires at least 3.715 eV at R=3.181 Angstrom, 1.481 eV above Born. The missing depth is comparable to the optical-polarization reorganization energy (1.243 eV), so a polaron or band-offset correction is the next test, not a fitted parameter.

- Config SHA-256: `fcc3b5b3a21b6a2f4766bae8f6ef96ec5a7f3d2491c6c88f5385b904807f3167`
- Git: `21b77fe5ff59be1785193139084ab924f920f9de` (dirty=True)
