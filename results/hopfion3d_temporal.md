# Timestep check for the 3D scan (claim 15)

N=64 held fixed; both runs reach t=1.5 (dt 5.0e-04 for 3000 steps versus dt 2.5e-04 for 6000 steps).

## Pre-registered temporal criteria

- |dH| <= 5.0e-03 and |dH_upsampled| <= 5.0e-03.
- |dZ3| <= 2.0e-02.

## Results

| dt | steps | H | H upsampled | Z3 | force RMS | max bond | energy | seconds |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 5.0e-04 | 3000 | 0.936964507 | 0.999520564 | 0.528226 | 3.988e-02 | 1.176 | 2282.822 | 185.0 |
| 2.5e-04 | 6000 | 0.936857582 | 0.999518680 | 0.528221 | 3.989e-02 | 1.177 | 2282.838 | 349.1 |

Deltas (fine dt minus coarse dt): H -0.000107, H upsampled -0.000002, Z3 -0.000005, force +1.299e-06, max bond +0.000.

**Temporal verdict:** converged: **True**.

| criterion | value |
|---|---|
| H_agrees | True |
| H_upsampled_agrees | True |
| Z3_agrees | True |
| force_not_worse | False |
| max_bond_not_worse | False |

**Claim 15 stays UNVERIFIED (H estimator not converged; not converged; field under-resolved).** Production ready (fine dt): **False**.

Temporal convergence is a diagnostic, not claim verification: if it fails, the spatial resolution comparison is confounded.

- Config SHA-256: `7a37e007f37922cd5e2619102b63f84428af0336f0fc337527695c5d8bde359b`
- Git: `377fcd7076c00f31b35f08fe2b698974b903a362` (dirty=True)
- Total wall time: 535.7 s (G-003 cap 900 s).
