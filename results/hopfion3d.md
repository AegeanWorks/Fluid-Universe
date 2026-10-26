# 3D E2+E4+Z3 pilot (claim 15)

Single-seed pilot, N=32, L=10, dt=5e-4, 300 gradient steps. The initial condition is the analytic H=1 map with a smooth vacuum compactification. This is not a production run.

Initial: H = 1.000141490, Z3 = 2.319e-18.

| mu | E2 | E4 | V | H | Z3 | monotone | max dE |
|---:|---:|---:|---:|---:|---:|---|---:|
| 0 | 192.491089 | 95.182287 | 0.000000 | 0.997143405 | 0.000000 | True | -2.059e-02 |
| 1 | 191.558057 | 95.209028 | 221.869418 | 0.997268391 | 0.023426 | True | -4.953e-02 |
| 5 | 188.573121 | 95.154307 | 980.968722 | 0.989493727 | 0.116980 | True | -5.646e-01 |
| 20 | 186.754087 | 93.189795 | 2308.410928 | 0.852975980 | 0.443234 | True | -6.019e+00 |

**Verdict:** coexistence |H|>=1 and Z3>=0.5: **False**. Maximum Z3 = 0.443234 at mu=20, where H = 0.852976.

Claim 15: **UNVERIFIED (single-seed pilot; no coexistence)**.

The Z3 potential increases the azimuthal order parameter but lowers the Hopf charge. No tested mu reaches both |H|>=1 and Z3>=0.5. This is a single-seed pilot; it is not a production or spontaneous-order claim.

- Config SHA-256: `2a34d2aa623aa666298c6513b2ddbed970bd72a39caff1a0c28a6763d9093f99`
- Git: `8c073c28a57bd0bac3b358948691482087fe7190` (dirty=True)
