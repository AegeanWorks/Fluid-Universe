# One-loop SM running of the charged-lepton Koide parameters

Initial conditions: full-SM MS-bar masses at M_Z from Huang & Zhou (2021), Table 3. One-loop RGE, `scipy.solve_ivp` DOP853, rtol=1e-11.

| mu (GeV) | m_e (MeV) | m_mu (MeV) | m_tau (MeV) | Q | r | delta mod 2pi/3 |
|---:|---:|---:|---:|---:|---:|---:|
| 91.1876 | 0.48307 | 101.766 | 1728.56 | 0.667823994186 | 0.708333248746 | 0.221129391953 |
| 1000 | 0.488585507 | 102.927926 | 1748.30001 | 0.667824221532 | 0.708333489465 | 0.221129113257 |
| 1e+06 | 0.489014627 | 103.018329 | 1749.8472 | 0.667824888259 | 0.708334195411 | 0.221128295938 |
| 1e+09 | 0.477526416 | 100.598166 | 1708.7499 | 0.667825538086 | 0.708334883462 | 0.221127499338 |
| 1e+12 | 0.459687451 | 96.8401198 | 1644.92614 | 0.667826148476 | 0.708335529756 | 0.221126751085 |
| 1e+15 | 0.437762703 | 92.2213415 | 1566.48032 | 0.667826707955 | 0.708336122142 | 0.221126065246 |
| 1e+16 | 0.429750465 | 90.5334427 | 1537.81219 | 0.667826881985 | 0.708336306409 | 0.221125851911 |

## Drift

- Q(M_Z) = 0.667823994186
- Q(1e16 GeV) = 0.667826881985
- Delta Q = +2.888e-06 (+4.324e-06 relative)
- r(M_Z) = 0.708333248746
- r(1e16 GeV) = 0.708336306409
- Delta r = +3.058e-06 (+4.317e-06 relative)
- over 32.33 e-folds

**Interpretation.** The exact one-loop invariance claim is false: the (3/2)y_k^2 term differentiates the generations. For charged leptons the effect is a few parts in 10^6 over 16 decades, so the numerical stability is real but is not an invariance theorem.

## Cross-checks

- Shape-only diagnostic (NOT a cross-check): max |Delta Q| = 1.422e-07, max |Delta r| = 1.506e-07. The difference is a few percent of the drift because the (3/2)y_k^2 term scales with the absolute Yukawa normalisation.
- Pole-to-MSbar conversion vs Xing & Zhang (2006): e: +1.89e-04, mu: +1.07e-04, tau: +3.40e-05.

## Literature high-scale check (Huang & Zhou 2021, Table 3)

| mu (GeV) | ours m_e | lit m_e | ours m_mu | lit m_mu | ours m_tau | lit m_tau |
|---:|---:|---:|---:|---:|---:|---:|
| 1e+08 | 0.482232 | 0.49691 | 101.589 | 104.681 | 1725.58 | 1778.52 |
| 1e+12 | 0.459687 | 0.48388 | 96.8401 | 101.936 | 1644.93 | 1731.94 |

The absolute mass scale at high energies is sensitive to the near-cancellation between the top-Yukawa term and the gauge terms, so the one-loop truncation differs from the three-loop literature by up to a few percent at 1e8 GeV. Only the dimensionless ratios Q and r are used for any claim; they are stable at the 10^-6 level.

- Config SHA-256: `009ae1e18929e2c16885342d57f9d5874dd7d9d398589daa4be0a6efed6cde05`
- Git: `cba3210a7ffb3a1ec4b15c18dcd43d55d590929b` (dirty=True)
