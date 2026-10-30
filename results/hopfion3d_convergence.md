# Resolution check for the 3D scan (claim 15)

N=64 and N=96 at the same physical time: mu=20, steps=3000, dt=5.0e-04, fixed vacuum faces, 2x spectral upsampling.

The 3600-step production scan remains the primary claim-15 evidence. This check uses 3000 steps so both resolutions fit one G-003 process; no acceptance criterion changes.

## Pre-registered resolution criteria

- |dH| <= 1.0e-02 and |dH_upsampled| <= 1.0e-02.
- |dZ3| <= 2.0e-02.
- force RMS and max bond do not increase with resolution.

## Results

| N | H | H upsampled | Z3 | force RMS | max bond | energy | min pre-norm | seconds |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 64 | 0.936964507 | 0.999520564 | 0.528226 | 3.988e-02 | 1.176 | 2282.822 | 1.000000 | 163.8 |
| 96 | 0.999942144 | 1.000000000 | 0.172071 | 1.495e-02 | 0.418 | 4011.516 | 1.000000 | 604.0 |

Deltas (N=96 minus N=64): H +0.062978, H upsampled +0.000479, Z3 -0.356154, force -2.494e-02, max bond -0.758.

**Resolution verdict:** converged: **False**; native H toward 1: **True**; upsampled H toward 1: **True**.

| criterion | value |
|---|---|
| H_agrees | False |
| H_upsampled_agrees | True |
| Z3_agrees | False |
| force_not_worse | True |
| max_bond_not_worse | True |
| H_toward_one | True |
| H_upsampled_toward_one | True |

**Claim 15 stays UNVERIFIED (Z3 below threshold; not converged).** Production ready: **False**.

Resolution convergence is a diagnostic, not claim verification: the fine run must still satisfy the unchanged production criteria.

- Config SHA-256: `efbd38806110f5a2f8406ee69236fa1de685daebe6dd9688eef69763bf257e07`
- Git: `03eb177ea09dcfe3e8f631e05eba157b4e04fb4d` (dirty=True)
- Total wall time: 768.9 s (G-003 cap 900 s).
