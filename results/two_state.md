# Shared-lambda two-state Marcus assignment (claim 44)

Conditional test: IF the observed components were two Marcus states
sharing the frozen cavity lambda, their widths would be identical.

\(\lambda=1.617132544\) eV; \(T=300.0\) K.

## Predicted widths (shared lambda)

| quantity | value |
|---|---:|
| G1 FWHM | 0.680915 eV |
| G2 FWHM | 0.680915 eV |
| predicted ratio | 1.000000 |

## Observed comparator (Jou & Freeman 1979, no fit)

| quantity | value |
|---|---:|
| G1 width | 0.71 eV |
| G2 width | 0.37 eV |
| observed ratio | 1.9189 +/- 0.46 |
| ratio diff vs 1 | 0.9189 |
| kill threshold | 0.15 |

## Verdict

- Widths identical: **True**.
- Shape kill (shared lambda): **True**.
- Claim 44: **FALSIFIED (shared-lambda two-state Marcus assignment; component ratio 1.9189 vs 1, diff 0.9189 > 0.15)**.

IF the Jou-Freeman 0.71 vs 0.37 eV components were two classical Marcus states sharing the frozen Born-cavity lambda=1.617132544 eV at one temperature, both would have FWHM 0.680915 eV (ratio exactly 1, T-independent). The observed ratio is 1.9189 (+/- 0.46), diff 0.9189 > 0.15, so the shared-lambda two-state assignment is falsified. This shares its shape fact with claim 43 and is not independent evidence. A state-dependent solvation (lambda1 != lambda2) would fit but predict nothing; a non-Marcus line shape is required.

- Config SHA-256: `2ce6a5433f3fb62ccedaeeca1456ec306b49b468a2481ad07d7d113b002c85b9`
- Git: `5e6bb615dc7bd30cd6b819efbc7d9d714b87770d` (dirty=True)
