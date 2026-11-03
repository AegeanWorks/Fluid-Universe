# Fixed-lambda cavity-Marcus line-shape vs observed comparator (claim 43)

Model: classical Marcus Gaussian with lambda fixed from claim 39.

\(\lambda=1.617132544\) eV; \(\text{FWHM}=2\sqrt{2\ln2}\sqrt{2\lambda k_BT}\).

## Predicted widths

| quantity | value |
|---|---:|
| W(0 C) | 0.653822 eV |
| W(45 C) | 0.695194 eV |
| change | 0.041373 eV |
| ratio W45/W0 | 1.063278 |

## Symmetry (exact for a Gaussian)

| quantity | value |
|---|---:|
| skewness | 0.000000 |
| half-width ratio | 1.000000 |
| third central moment | 0.000000 |

## Observed comparator (no fit)

| quantity | value | source |
|---|---|---|
| width slope | 2.0e-04 eV/K | Christensen & Sehested 1986 |
| observed change (45 K) | 0.009000 eV | slope x span |
| predicted change | 0.041373 eV | fixed lambda |
| observed ratio on predicted scale | 1.013765 | W0 predicted |
| ratio diff | 0.049513 | vs 0.05 |
| G1 width | 0.71 eV | Jou & Freeman 1979 |
| G2 width | 0.37 eV | Jou & Freeman 1979 |
| component ratio | 1.9189 | 0.71/0.37 |
| tail power n | 4.4 | Jou & Freeman 1979 (water) |

## Kill thresholds (frozen, ADR-034)

| quantity | value |
|---|---:|
| width ratio tolerance | 0.05 |
| shape ratio tolerance | 0.15 |
| skew tolerance | 0.30 |

## Verdict

- Symmetry exact: **True**.
- Widths match the temperature scan: **True**.
- Ratio matches sqrt(lambda T): **True**.
- Shape kill (Gaussian-only): **True**.
- Ratio kill: **False**.
- Claim 43: **FALSIFIED (Gaussian-only shape; sqrt(lambda T) scaling survives: ratio diff 0.0495 <= 0.05)**.

With lambda fixed at 1.617132544 eV from the Born cavity, the classical Marcus Gaussian predicts W(0 C)=0.653822 eV, W(45 C)=0.695194 eV, ratio 1.0633, with exact symmetry (skew 0, half-width ratio 1). Observed comparator: Christensen & Sehested 1986 width slope 2e-4 eV/K (predicted slope 9.194e-04 eV/K) and Jou & Freeman 1979 two-Gaussian-plus-tail decomposition (0.71 vs 0.37 eV, tail n=4.4, asymmetric; Bartels et al. 2005 Gaussian/Lorentzian split). Ratio diff 0.0495 (HWHM reading 0.0357) vs tolerance 0.05; component-width ratio 1.9189 vs 1 (diff 0.9189 > 0.15). The single symmetric Gaussian is falsified on shape; the width-ratio scaling survives this comparator.

- Config SHA-256: `a6f7303b8be40c6bef5c5d9814f1ff3168fa3507bbe1f486f849a68640abe838`
- Git: `faa31f97d31eb39e7a9fb62c30b3f886514009df` (dirty=True)
