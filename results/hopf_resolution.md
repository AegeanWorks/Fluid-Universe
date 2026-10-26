# Hopf grid resolution (analytic H=1 map)

| N | H_grid | abs error vs preimage |
|---:|---:|---:|
| 16 | 0.801799781 | 1.982e-01 |
| 24 | 0.994054262 | 5.904e-03 |
| 32 | 1.000141490 | 1.832e-04 |
| 48 | 0.999999788 | 4.155e-05 |
| 64 | 1.000000039 | 4.180e-05 |

Preimage linking: H = 0.999958242.

Coarse (N<32) under-resolved: **True**; fine (N>=32) within 1e-3: **True**.

The grid estimator needs N>=32 for 1e-3 accuracy on the analytic map. This explains why the N=32 3D pilot H values are resolution-limited.

- Config SHA-256: `5b16c483c1242e4a75f5c58702165a85135422063f67310d7069a25f9385d683`
- Git: `991c124c02aee764b5baeadfc58719f60af14e98` (dirty=True)
