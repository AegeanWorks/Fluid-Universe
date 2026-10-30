# Self-consistent finite-well pressure balance (claim 37)

Model: at fixed well depth \(V_0\), minimize \(E_{1s}(R,V_0)+4\pi\gamma R^2\). This is the Laplace pressure balance between the electron pressure and surface tension, using the finite-well 1s eigenvalue rather than the crude \(\hbar^2/(2mR^2)\).

## Pressure-balanced scan

| \(V_0\) (eV) | \(R\) (Angstrom) | binding (eV) | \(E_{1p}\) (eV) | gap (eV) | surface (eV) | total (eV) |
|---:|---:|---:|---:|---:|---:|---:|
| 2.486988 | 3.973423 | 1.177351 | -0.044358 | 1.132993 | 0.901489 | -0.275862 |
| 3.270000 | 4.163213 | 1.947875 | -0.681130 | 1.266744 | 0.989665 | -0.958210 |
| 4.000000 | 4.270885 | 2.667017 | -1.349121 | 1.317896 | 1.041518 | -1.625499 |
| 4.500000 | 4.325301 | 3.160615 | -1.820563 | 1.340051 | 1.068227 | -2.092387 |
| 5.000000 | 4.369710 | 3.654971 | -2.298274 | 1.356697 | 1.090275 | -2.564696 |
| 5.500000 | 4.406914 | 4.149994 | -2.780242 | 1.369752 | 1.108919 | -3.041075 |
| 6.000000 | 4.438638 | 4.645548 | -3.265194 | 1.380355 | 1.124942 | -3.520606 |

## VDE-matched point

| quantity | value |
|---|---:|
| depth | 4.610697 eV |
| radius | 4.335877860e-10 m (4.335878 Angstrom) |
| binding | 3.270000 eV |
| \(E_{1p}\) | -1.925875 eV |
| gap | 1.344125 eV |
| surface energy | 1.073458 eV |
| total energy | -2.196542 eV |

## Crude fixed-radius comparison

| quantity | value |
|---|---:|
| crude capillary radius | 2.858071056e-10 m (2.858071 Angstrom) |
| crude total energy | -0.061333 eV |
| pressure-balanced total at Born depth | -0.275862 eV |
| pressure radius / crude radius | 1.390246 |

The crude fixed-radius cavity is not the pressure minimum. The pressure balance moves the radius to 3.973 Angstrom at the Born depth and to 4.336 Angstrom at the VDE-matched depth. The gap at the VDE-matched point is 1.344 eV, not 1.7 eV.

## Verdict

- Pressure radius is larger than the crude radius: **True**.
- The VDE-matched cavity is found: **True**.
- Its gap is below 1.7 eV: **True**.
- The crude radius is not the pressure minimum: **True**.
- Claim 37: **NUMERIC (pressure-balance radius 3.973 Angstrom; VDE-matched gap 1.344 eV)**.

The fluid pressure balance makes the cavity larger and the 1s->1p gap smaller. At the Born depth the minimum is R=3.973 Angstrom, binding 1.177 eV, gap 1.133 eV. At the VDE-matched depth V0=4.611 eV the minimum is R=4.336 Angstrom, binding 3.27 eV, gap 1.344 eV, not 1.7 eV. The crude fixed-radius cavity is not the pressure minimum. Adding the R-dependent Born energy separately double-counts the solvation and collapses the cavity; a self-consistent dielectric interface is the next test.

- Config SHA-256: `4c1f7dc55448723bf3527381775c9ff6c341570c75b0ae55e5e46c96a57421b2`
- Git: `5ed248e6c0f4b1329b217ae0d2b7da8facf7bbb9` (dirty=True)
