# Water reorganization and the VDE-optical difference (claim 38)

Model: \(\lambda(R)=e^2(1/\epsilon_\infty-1/\epsilon_s)/(8\pi\epsilon_0R)\), with water's \(\epsilon_s=78.4\) and \(\epsilon_\infty=1.78\). The measured VDE is \(3.27\pm0.10\) eV and the optical absorption is 1.70 eV, so the observed difference is 1.57 eV.

## Capillary radius

| quantity | value |
|---|---:|
| radius | 2.858071056e-10 m (2.858071 Angstrom) |
| reorganization energy | 1.383104 eV |
| predicted VDE (optical + lambda) | 3.083104 eV |
| residual (predicted - measured) | -0.186896 eV |
| relative residual | 0.119042 |
| residual in VDE sigma | 1.868960 |

## Required radius

| quantity | value |
|---|---:|
| radius for the 1.57 eV difference | 2.517840529e-10 m (2.517841 Angstrom) |
| required / capillary | 0.880958 |

## Pressure-balanced radius

| quantity | value |
|---|---:|
| pressure-balanced radius | 3.973423200e-10 m (3.973423 Angstrom) |
| reorganization energy | 0.994862 eV |
| lambda / observed difference | 0.633670 |
| residual | 0.575138 eV |

The capillary radius is close to the radius required by the VDE-optical difference; the pressure-balanced radius is not. This favors the capillary-sized cavity for the optical reorganization, even though the pressure-balanced cavity is larger.

## Verdict

- Lambda is within 15% of the observed difference: **True**.
- Predicted VDE is within 2 sigma: **True**.
- Required radius is within 15% of the capillary radius: **True**.
- Pressure-balanced radius is inconsistent: **True**.
- Claim 38: **NUMERIC (reorganization relation; 11.9% residual, 1.87 sigma)**.

The VDE-optical difference 1.57 eV is close to water's dielectric reorganization energy at the capillary radius, lambda=1.383 eV (residual 0.187 eV, 11.9%, 1.87 sigma). The required radius is 2.518 Angstrom (0.881x capillary). The pressure-balanced radius 3.973 Angstrom gives only 0.995 eV (0.634x) and is inconsistent. The relation is approximate; a two-state Marcus model with the solvent coordinate is the next test.

- Config SHA-256: `51e17c0c4c4f68002b967b0716573b6cec8dfebf8141a5ee5f209452f52c3f28`
- Git: `014c06f6d05c76821da0176dd6cdb4afd3222ed0` (dirty=True)
