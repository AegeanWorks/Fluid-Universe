# Self-consistent Born cavity synthesis (claim 39)

Model: set the well depth to the R-dependent Born solvation, \(V_0=\text{Born}(R)\), and minimize \(E_{1s}(R,\text{Born}(R))+4\pi\gamma R^2\). The cavity radius is a fluid/dielectric equilibrium, not a fixed-depth well.

## Equilibrium

| quantity | value |
|---|---:|
| radius | 2.444456173e-10 m (2.444456 Angstrom) |
| Born depth | 2.907799 eV |
| \(u_{1s}\) | 1.968689 |
| \(E_{1s}\) | -0.436572 eV |
| 1s binding | 0.436572 eV |
| 1p | none |
| 1s->1p gap | none |
| surface energy | 0.341189 eV |
| total energy | -0.095382 eV |

The well has no bound 1p: the electronic 1s->1p gap is absent. The electronic binding is only 0.437 eV, so the 1.7 eV feature is not a static electronic transition.

## Water reorganization synthesis

| quantity | value |
|---|---:|
| water reorganization energy at equilibrium | 1.617133 eV |
| observed VDE - optical | 1.570000 eV |
| predicted VDE (optical + lambda) | 3.317133 eV |
| residual | 0.047133 eV |
| residual in VDE sigma | 0.471325 |
| radius required by the difference | 2.517840529e-10 m (2.517841 Angstrom) |
| required / equilibrium | 1.030021 |
| crude capillary / equilibrium | 1.169205 |
| fixed-depth pressure / equilibrium | 1.625484 |

The required reorganization radius is only 3.0% above the self-consistent Born equilibrium, while the crude capillary and fixed-depth pressure radii are 16.9% and 62.5% above it.

## Verdict

- Equilibrium is in the physical range: **True**.
- The well has no bound 1p: **True**.
- Predicted VDE is within 1 sigma: **True**.
- Required radius is within 5% of equilibrium: **True**.
- Claim 39: **NUMERIC (Born cavity R=2.444 Angstrom; lambda=1.617 eV; VDE 3.317 eV, 0.47 sigma)**.

The R-dependent Born cavity radius 2.444456 Angstrom is within 3.0% of the radius required by the VDE-optical difference (2.517841 Angstrom). The water reorganization energy there is 1.617133 eV; the Marcus relation predicts VDE=3.317133 eV, within 0.47 sigma of the measured 3.27+/-0.10 eV. The well has no bound 1p, so the 1.7 eV feature is a water reorganization transition, not a static 1s->1p gap. This is a positive synthesis of the fluid mechanics and water's dielectric contrast; the next test is the Marcus band shape and width.

- Config SHA-256: `1ea395c1643dfb39df54157b733a2e7468269c6d44e01e44339ac1b7e21cd25a`
- Git: `48ef324c0e8336963af447d28cd88a0d759d516c` (dirty=True)
