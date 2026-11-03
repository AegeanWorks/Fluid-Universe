# Variational dynamics reproduces Ermakov (claim 51)

G-M action over sigma(t) gives the Ermakov equation; period pi (pi/2 at omega=2).

| check | value |
|---|---:|
| extremal max deviation | 0.000e+00 |
| extremal max residual | 2.220e-16 |
| release f=1.01 period | 3.141592129852 |
| release f=0.99 period | 3.141592129851 |
| release f=1.1 period | 3.141592122822 |
| term-match max rel err | 0.000e+00 |
| scaling omega=2 period | 1.570795279141 |

## Verdict

- Claim 51: **NUMERIC (variational dynamics reproduces Ermakov)**.

Time-dependent Gaussian Ritz variation of the G-M action yields the Ermakov equation term-by-term; the discrete extremum is the constant path and small oscillations run at pi (pi/2 at omega=2).

- Config SHA-256: `068003eac6ed4799352cdfadcec6e8095e80609ca079d4b1358ec738b6f4e5be`
- Git: `2d3a59baf4db3f6018ec7925ebf1e6ea7a16452d` (dirty=True)
