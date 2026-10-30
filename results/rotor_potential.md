# Real Z3 rotor potential scan (claim 19)

Hamiltonian: H = (L-A)^2/(2I) + mu(1-cos 3phi) + nu(1-cos 6phi), with mu=1, I=1 and n_cut=120.

## Maximum Koide Q by second harmonic

| nu | max Q over flux |
|---:|---:|
| 0.0 | 0.337639 |
| 1.0 | 0.334542 |
| 3.0 | 0.333589 |
| 10.0 | 0.333338 |

## Best scan point

- nu=0, flux=0.5.
- Energies: 1.003563, 1.003563, 1.598732.
- Koide Q: 0.337639 (target 0.666667).
- Q at n_cut=240: 0.337639; delta: 9.437e-15.
- Zero-flux real potential is degenerate: **True**.

**Status: FALSIFIED (real Z3 rotor never reaches Q=2/3 in the scan).**

A real potential cannot split the k=1,2 sectors at zero flux; a fractional flux splits them but the lowest levels remain close. Reaching Q=2/3 needs a complex phase or a different mass mapping.

- Config SHA-256: `967dda8fdd3718efb7bf68f0534f2ccec788d57f1252e98afcbaae47eeed455a`
- Git: `8f242fccea9e0f803dce6ac7889ad27c300c3f18` (dirty=True)
