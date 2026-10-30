# Marcus band width from the water reorganization energy (claim 40)

Model: the classical Marcus Gaussian width

\(\text{FWHM}=2\sqrt{2\ln2}\sqrt{2\lambda k_BT}\),

with the same \(\lambda=1.617132544\) eV from claim 39.

## Width versus temperature

| temperature (K) | FWHM (eV) |
|---:|---:|
| 200 | 0.555964 |
| 250 | 0.621587 |
| 298 | 0.678641 |
| 300 | 0.680915 |
| 350 | 0.735472 |

## Reference comparison

| quantity | value |
|---|---:|
| Marcus FWHM at 300 K | 0.680915 eV |
| classical reference | 0.706 eV |
| quantized reference | 0.773 eV |
| Marcus / classical | 0.964468 |
| Marcus / quantized | 0.880873 |
| sqrt(350/300) | 1.080123 |

The reference widths are the classical (0.706 eV) and quantized (0.773 eV) bulk hydrated-electron values of Turi et al., JCTC 20, 4265 (2024). The experimental band is broad and asymmetric, with a FWHM of order 0.7 eV.

## Verdict

- Marcus width is within 5% of the classical reference: **True**.
- Marcus width is within 15% of the quantized reference: **True**.
- The width follows sqrt(T): **True**.
- Claim 40: **NUMERIC (Marcus FWHM 0.681 eV; reference 0.706-0.773 eV)**.

The same water reorganization energy that predicts the VDE also predicts the band width: FWHM=0.680915 eV at 300 K, 3.6% below the classical reference 0.706 eV and 11.9% below the quantized 0.773 eV. The width scales as sqrt(T): 0.555964 eV at 200 K and 0.735472 eV at 350 K. The classical Gaussian model is approximate; the experimental band is asymmetric with a blue tail. The next test is the asymmetric line shape and its temperature dependence.

- Config SHA-256: `fa7c645e050fd6841bf9d7bc89476024d5e666f9cf11008f7dd4ef8bcb1db530`
- Git: `baaed62863c2c781a7349c262dece024c69eeba2` (dirty=True)
