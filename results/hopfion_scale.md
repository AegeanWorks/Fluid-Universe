# Damped scale iteration (claim 14b/14c)

N=64, 600 steps per iteration, fixed vacuum boundaries. A candidate search, not a stability proof.

| iter | E | E2-E4 | H | Z3 | force RMS | lambda |
|---:|---:|---:|---:|---:|---:|---:|
| 0 | 359.121803 | +80.209646 | 1.000000055 | 0.000001 | 3.248e-03 | 0.796778 |
| 1 | 338.949279 | +36.956721 | 0.999990491 | 0.000007 | 3.078e-03 | 0.896311 |
| 2 | 327.849953 | +15.458609 | 0.999927141 | 0.000016 | 2.893e-03 | 0.953910 |
| 3 | 319.425952 | +3.248378 | 0.999525755 | 0.000023 | 2.719e-03 | 0.989882 |
| 4 | 315.865705 | +0.009610 | 0.997297414 | 0.000026 | 3.002e-03 | 0.999970 |
| 5 | 312.996658 | +0.575276 | 0.986238277 | 0.000027 | 4.094e-03 | 0.998164 |

Best |virial| at iteration 4: virial = +0.009610, H = 0.997297414, force RMS = 3.002e-03.

Scale-stationary candidate: **True**; H within 0.01 of 1: **True**; full stability: **False**.

Claim 14b: **UNVERIFIED**; claim 14c: **NUMERIC (scale-stationary H=1 candidate; no stability proof)**.

The damped scale iteration reaches a near-virial-free H=1 candidate, but the sequence is not monotone and the Hessian is not computed. This is a candidate, not a stable Hopfion proof.

- Config SHA-256: `34196c8644e556d5f15c35f68cb95c29a2f4126d709c51f137b03caa2975c9d9`
- Git: `fdbf1d8738304cc3ef583d768a54f822c2d6b166` (dirty=True)
