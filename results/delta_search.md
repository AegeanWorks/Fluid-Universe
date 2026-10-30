# Delta closed-form search (claim 9)

Post-hoc search, not a prediction. delta = 0.222224761904 rad mod 2*pi/3, sigma_delta = 6.238e-06 rad.

Prior: 20 rational denominators, 12 pi-multiple denominators, 12 simple forms; 327 candidates.

| rank | candidate | value | |diff| | sigma |
|---:|---|---:|---:|---:|
| 1 | 2/9 | 0.222222222222 | 2.540e-06 | 0.407 |
| 2 | 3/14 | 0.214285714286 | 7.939e-03 | 1272.602 |
| 3 | 3/13 | 0.230769230769 | 8.544e-03 | 1369.649 |
| 4 | 4/19 | 0.210526315789 | 1.170e-02 | 1875.221 |
| 5 | 4/17 | 0.235294117647 | 1.307e-02 | 2094.973 |
| 6 | 1/5 | 0.200000000000 | 2.222e-02 | 3562.553 |
| 7 | atan(1/4) | 0.244978663127 | 2.275e-02 | 3647.372 |
| 8 | atan(1/5) | 0.197395559850 | 2.483e-02 | 3980.035 |
| 9 | 1/4 | 0.250000000000 | 2.778e-02 | 4452.275 |
| 10 | 3/16 | 0.187500000000 | 3.472e-02 | 5566.260 |

Best: **2/9**, 0.407 sigma.
Single-candidate p = 2.425e-06; expected candidates within the best distance = 7.930e-04; look-elsewhere p = 7.927e-04.

**Verdict:** no candidate is promoted. The 2/9 near-miss is recorded with its prior and look-elsewhere penalty; it is not evidence for a closed form.

- Config SHA-256: `5947ca1f7edce9428e29811388c938328a636f09d8d9a05de85fbd945f87ccd3`
- Git: `377fcd7076c00f31b35f08fe2b698974b903a362` (dirty=True)
