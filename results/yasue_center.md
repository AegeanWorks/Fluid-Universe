# Yasue center-width discriminator (claim 55)

Yasue 2-path trial: classical center (2pi) with collapsing width;
the G-M twin co-oscillates (2pi and pi).

| check | value |
|---|---:|
| static residual min | 1.414214 |
| newton attempts converged | 5 |
| Yasue release mu0=0.1, s0=1.0 | track=2.301e-09, hit=0.7830000000000006, mono=True, up=0 |
| Yasue release mu0=0.0, s0=1.01 | track=0.000e+00, hit=0.7930000000000006, mono=True, up=0 |
| Yasue release mu0=0.3, s0=1.2 | track=9.865e-09, hit=0.9620000000000007, mono=True, up=0 |
| arc min/max sigma | 0.708120437225 / 0.807225363490 |
| G-M twin mu0=0.1, s0=1.0 | T_mu=6.283185045, T_sg=nan |
| G-M twin mu0=0.0, s0=1.01 | T_mu=nan, T_sg=3.141592130 |
| G-M twin mu0=0.3, s0=1.2 | T_mu=6.283185045, T_sg=3.141592104 |
| term match max rel err | 0.000e+00 |
| scaling yasue hit / twin mu / twin sg | 0.3960000000000003 / 3.141592130 / 1.570795279 |

## Verdict

- Claim 55: **NUMERIC (Yasue center-width discriminator demonstrated)**.

Yasue 2-path trial: center tracks Newton over the collapse window with collapsing width (floor-hit < pi, zero returns); G-M twin co-oscillates (2pi and pi); single BVP arc recorded; no fitted parameters.

- Config SHA-256: `aa42690452424bd13048266841145e49da32c9232f97877b866c2f8a81a124fc`
- Git: `96eb57c0dc34b84b6a1d948bd8b69f8cd870e1dc` (dirty=True)
