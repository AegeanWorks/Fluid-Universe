# Two-component band model vs observed side widths (claim 45)

Join at Emax with zero free parameters; thresholds frozen.

| quantity | value |
|---|---:|
| T ref | 299.15 K |
| Marcus FWHM | 0.679949 eV |
| red HWHM (pred) | 0.339975 eV |
| blue HWHM (pred) | 0.293466 eV |
| R_pred | 0.863201 |
| R_obs (L/G) | 1.585732 |
| ratio diff | 0.722531 |
| FWHM model | 0.633441 eV |
| FWHM obs (G+L) | 0.786751 eV |
| Emax obs | 1.732640 eV |

## Verdict

- Ratio kill: **True**.
- Width kill: **True**.
- Claim 45: **FALSIFIED (two-component join; ratio diff 0.7225 > 0.15; classical rel 0.1028 (tol 0.05)/quantized rel 0.1805 (tol 0.15))**.

Zero-free-parameter join at Emax=1.72 eV: Gaussian red HWHM from the fixed Marcus scale plus n=4.4 power-law blue tail. Kill thresholds frozen from ADR-034. Observed side widths are the Jou-Freeman fits (G red, L blue) via the Coe review; Wb/Wr definitions per Bartels.

- Config SHA-256: `072926df5cc7767316dad90ba33c9a648b2634f65c9781709c448aef36c4f42e`
- Git: `5e6bb615dc7bd30cd6b819efbc7d9d714b87770d` (dirty=True)
