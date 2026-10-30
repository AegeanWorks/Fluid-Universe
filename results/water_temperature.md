# Temperature-dependent Born cavity (claim 41)

Model: water's static dielectric constant and surface tension

\(\epsilon_s(T)=87.740-0.40008T+9.398\times10^{-4}T^2-1.410\times10^{-6}T^3\),

\(\gamma(T)=0.07564-1.4\times10^{-4}T-3.0\times10^{-7}T^2\),

with \(T\) in Celsius. The self-consistent Born cavity is solved at each temperature; \(\lambda(T)\) and the Marcus width follow.

## Temperature scan

| \(T\) (C) | \(R\) (Angstrom) | \(\epsilon_s\) | \(\gamma\) (N/m) | \(\lambda\) (eV) | VDE (eV) | FWHM (eV) |
|---:|---:|---:|---:|---:|---:|---:|
| 0 | 2.419922 | 87.740000 | 0.075640 | 1.637567 | 3.337567 | 0.653822 |
| 10 | 2.432193 | 83.831770 | 0.074210 | 1.627733 | 3.327733 | 0.663680 |
| 20 | 2.445206 | 80.103040 | 0.072720 | 1.617435 | 3.317435 | 0.673159 |
| 25 | 2.452029 | 78.303344 | 0.071953 | 1.612092 | 3.312092 | 0.677753 |
| 30 | 2.459027 | 76.545350 | 0.071170 | 1.606646 | 3.306646 | 0.682257 |
| 37 | 2.469214 | 74.152205 | 0.070049 | 1.598788 | 3.298788 | 0.688399 |
| 45 | 2.481377 | 71.511009 | 0.068733 | 1.589506 | 3.289506 | 0.695194 |

## Shift and width

| quantity | value |
|---|---:|
| lambda slope | -1.068036 meV/K |
| observed peak shift | 2.4-2.8 meV/K |
| fraction of lower | 0.445015 |
| fraction of upper | 0.381441 |
| FWHM at 0 C | 0.653822 eV |
| FWHM at 45 C | 0.695194 eV |
| width change | 0.041373 eV |

The cavity is not temperature-independent, but its lambda/VDE slope covers only 38%-45% of the observed peak shift. The residual points to a temperature-dependent bare gap or a non-cavity structure.

## Verdict

- Nonzero temperature slope: **True**.
- Covers less than half of the observed shift: **True**.
- Width broadens with temperature: **True**.
- Claim 41: **NUMERIC (temperature slope -1.068 meV/K vs observed 2.4-2.8 meV/K)**.

The water-property Born cavity predicts a lambda/VDE red shift of 1.068036 meV/K from 0 to 45 C, 38.1%-44.5% of the observed 2.4-2.8 meV/K peak shift. The model is not temperature-independent, but it is insufficient. The Marcus width broadens from 0.653822 eV at 0 C to 0.695194 eV at 45 C. The residual shift points toward a temperature-dependent bare gap or non-cavity structure, consistent with the Farr/Schwartz critique.

- Config SHA-256: `387f20e1cbf60c52e079f2d93afac1b9d1e3c749b9848400cbcefa391d45487f`
- Git: `28b84b563cd38e8bdafda102ab6aae9c22457ad0` (dirty=True)
