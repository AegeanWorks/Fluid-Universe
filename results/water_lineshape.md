# Fixed-lambda cavity-Marcus line-shape prediction (claim 43)

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

## Kill thresholds (for a future comparator)

| quantity | value |
|---|---:|
| width ratio tolerance | 0.05 |
| shape ratio tolerance | 0.15 |
| skew tolerance | 0.30 |

## Verdict

- Symmetry exact: **True**.
- Widths match the temperature scan: **True**.
- Ratio matches sqrt(lambda T): **True**.
- Claim 43: **UNVERIFIED (pre-registered prediction; no observed width-vs-T or shape comparator)**.

With lambda fixed at 1.617132544 eV from the Born cavity, the classical Marcus Gaussian predicts W(0 C)=0.653822 eV, W(45 C)=0.695194 eV, ratio 1.0633, with exact symmetry (skew 0, half-width ratio 1). No observed width change or band-shape numbers exist in this repository, so this is a prediction with kill thresholds (ratio 0.05, shape 0.15, skew 0.3), never a comparison. A future comparator kills the sqrt(lambda T) scaling if |observed ratio - 1.0633| > 0.05 and kills Gaussian-only shape if |observed half-width ratio - 1| > 0.15.

- Config SHA-256: `23df712d90f9275c016e9e5440c6f4578732e3f607ca65ff39dfebddaf01fb8b`
- Git: `b5f7c485c07876410704af1ea3dfb9b583bbdc1c` (dirty=True)
