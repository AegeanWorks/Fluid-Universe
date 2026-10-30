# 1D water-wave scattering at the horizon (claim 31)

Depth 0.1 m, length 20 m, N=1200, t_max=15 s, dt=2.0e-04 s; c=0.990454441 m/s.

| case | transmitted fraction | final center (m) | total energy |
|---|---:|---:|---:|
| horizon | 7.503e-06 | -1.433333 | 7.940523 |
| uniform control | 1.000000 | 4.433333 | 7.109958 |

## Verdict

- Horizon blocks: **True**.
- Uniform flow transmits: **True**.
- Claim 31: **NUMERIC (horizon blocking)**.

A right-moving shallow-water packet leaves essentially no energy in the supersonic region at the horizon, while the uniform-flow control transmits. The 1D background flow is prescribed (kinematic analogue), not a consistent shallow-water equilibrium; a full radial simulation remains future work.

- Config SHA-256: `ca99e1212471d053643b25caeb0d862f955cdf3e19c3322de6abe81e0a47db33`
- Git: `fe1c68f588c4d5de605e4ec0503b6cfa574e3a35` (dirty=True)
