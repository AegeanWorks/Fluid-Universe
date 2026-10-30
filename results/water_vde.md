# VDE and 1.7 eV absorption constrain the finite well (claim 35)

Measured vertical detachment energy: \(3.27\pm0.10\) eV (Tang et al., PCCP 12, 3653, 2010). Observed absorption: 1.7 eV. Because \(1.7<3.27\), the absorption is below the detachment threshold and must be bound-bound. The model identifies the VDE with the 1s binding \(-E_{1s}\) of the cavity.

## Two-observable solution

| quantity | value |
|---|---:|
| radius | 3.732122844e-10 m (3.732123 Angstrom) |
| well depth | 4.997523 eV |
| \(u_{1s}\) | 2.513080 |
| \(u_{1p}\) | 3.539848 |
| \(E_{1s}\) | -3.270000 eV |
| \(E_{1p}\) | -1.570000 eV |
| 1s->1p gap | 1.700000 eV |
| Born depth at this radius | 1.904543 eV |
| excess depth | 3.092980 eV |

## VDE uncertainty band

| binding (eV) | radius (Angstrom) | depth (eV) | \(E_{1p}\) (eV) | gap (eV) |
|---:|---:|---:|---:|---:|
| 3.17 | 3.714321 | 4.902845 | -1.470000 | 1.700000 |
| 3.27 | 3.732123 | 4.997523 | -1.570000 | 1.700000 |
| 3.37 | 3.749021 | 5.092648 | -1.670000 | 1.700000 |

## Capillary/Born comparison

| quantity | value |
|---|---:|
| capillary radius | 2.858071056e-10 m (2.858071 Angstrom) |
| Born depth | 2.486988 eV |
| capillary 1s binding | 0.527753 eV |
| capillary 1p | none |
| required/capillary radius | 1.305819 |
| required/Born depth | 2.009468 |
| required depth minus capillary Born | 2.510535 eV |
| optical reorganization at required radius | 1.059185 eV |
| excess/reorganization | 2.920150 |

The capillary/Born cavity is outside the VDE band: it is 30.6% smaller and 2.01x shallower than the two-observable solution. The required excess is 2.92x the optical reorganization energy, so the missing depth is not a simple polaron shift.

## Verdict

- The 1.7 eV absorption is bound-bound: **True**.
- The finite well satisfies both observables: **True**.
- The capillary cavity is outside the VDE band: **True**.
- Claim 35: **NUMERIC (two-observable inversion; R=3.732 Angstrom, V0=4.998 eV)**.

The measured VDE 3.27 +/- 0.10 eV and the 1.7 eV absorption force a bound-bound 1s->1p assignment. The finite well satisfies both only at R=3.732 Angstrom and V0=4.998 eV, where the 1p is bound by 1.57 eV. The capillary/Born cavity (R=2.858 Angstrom, V0=2.487 eV) is 30.6% smaller and 2.01x shallower, outside the VDE band. The required excess depth 3.093 eV is 2.92x the optical reorganization energy at that radius, so a simple polaron correction is insufficient; a soft-wall or different potential is the next test.

- Config SHA-256: `716a2ab806ca42a2c0216b3cd88dc9465209dd82792d8fbb8affd5061e8cc706`
- Git: `7b7ca67c06a6a478c3b3dd7d552f20ab53288574` (dirty=True)
