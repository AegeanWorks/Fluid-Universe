# Soft-wall cavity and the hard VDE depth bound (claim 36)

The VDE is identified with the 1s binding. A bound state cannot be deeper than its well, so \(E_b\le V_0\). The Born depth is 2.486988 eV and the VDE is 3.27 eV: the Born well is 0.783012 eV too shallow (ratio 1.314844) before any shape is chosen.

## Woods-Saxon scan

Model: \(V(r)=-V_0(1+e^{-R/a})/(1+e^{(r-R)/a})\), central depth \(V_0\), half-depth radius \(R\), diffuseness \(a\). At each \(a\), solve \(V_0(R)\) for \(E_b=3.27\) eV, then invert for the radius where the 1s->1p gap is 1.7 eV.

| \(a\) (Angstrom) | \(R\) (Angstrom) | \(V_0\) (eV) | \(E_{1s}\) (eV) | \(E_{1p}\) (eV) | gap (eV) | Born at \(R\) (eV) | excess (eV) |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 0.001 | 3.729629 | 4.997536 | -3.270000 | -1.570000 | 1.700000 | 1.905816 | 3.091720 |
| 0.200 | 3.777781 | 5.039126 | -3.270000 | -1.570000 | 1.700000 | 1.881525 | 3.157601 |
| 0.500 | 3.852079 | 5.265075 | -3.270000 | -1.570000 | 1.700000 | 1.845234 | 3.419841 |
| 0.800 | 3.803087 | 5.636333 | -3.270000 | -1.570000 | 1.700000 | 1.869005 | 3.767329 |
| 1.000 | 3.686558 | 5.919724 | -3.270000 | -1.570000 | 1.700000 | 1.928082 | 3.991642 |
| 1.200 | 3.499496 | 6.211048 | -3.270000 | -1.570000 | 1.700000 | 2.031146 | 4.179901 |
| 1.500 | 3.078071 | 6.643300 | -3.270000 | -1.570000 | 1.700000 | 2.309235 | 4.334065 |

The square limit \(a=0.001\) Angstrom gives \(R=3.729629\) Angstrom and \(V_0=4.997536\) eV, close to the claim-35 square-well solution. Between \(a=1.0\) and \(1.5\) Angstrom, the softer well has a smaller radius but a larger depth; the scan does not lower the required well depth.

## Capillary comparison

| quantity | value |
|---|---:|
| capillary radius | 2.858071056e-10 m (2.858071 Angstrom) |
| Born depth | 2.486988 eV |
| maximum possible binding at Born depth | 2.486988 eV |
| VDE | 3.27 eV |
| closest soft scan (a=1.5 Angstrom) | R=3.078071 Angstrom, V0=6.643300 eV |
| closest radius / capillary | 1.076975 |
| closest depth / Born | 2.671223 |

The closest soft scan still requires a depth 2.67x the Born depth. The hard bound alone rules out the capillary/Born cavity; the scan shows that a diffuse interface makes the depth problem worse.

## Verdict

- Born depth is below the VDE: **True**.
- Softness trades radius for depth: **True**.
- No scan point is near the capillary depth: **True**.
- Claim 36: **NUMERIC (hard depth bound 3.27 eV; Woods-Saxon scan)**.

The VDE 3.27 eV is a hard lower bound on the well depth: a bound state cannot be deeper than its well. The Born depth 2.486988 eV is 0.783012 eV too shallow, independent of the cavity shape. A Woods-Saxon scan trades radius for depth: at a=1.5 Angstrom the two-observable solution is R=3.078 Angstrom but V0=6.643 eV (2.67x the Born depth). Softness does not rescue the minimal cavity. Either the VDE-to-1s-binding identification, the cavity model, or the 1.7 eV assignment must change.

- Config SHA-256: `c5ce0d7cbfcb5b5958273af00aa161580ff4a473d512e2e1b8231fcdd72b8d6d`
- Git: `0bb0d2162efd17c16425d86dc9518099a6240752` (dirty=True)
