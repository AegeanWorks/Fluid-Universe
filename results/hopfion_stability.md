# Coarse Hessian stability at N=64 (claim 14b/14c)

Scale-iteration candidate, fixed vacuum boundaries. This is a coarse Hessian diagnostic, not a full stability proof.

Candidate: H=0.997297414, Z3=2.584e-05, E2-E4=+0.009610, force RMS=3.002e-03.

| quantity | value |
|---|---:|
| interior dimension | 714984 |
| minimum eigenvalue | -5.799863 |
| negative eigenvalues | 8 |
| near-zero eigenvalues | 0 |

Smallest eigenvalues: -5.799863, -5.799863, -5.015290, -5.015290, -4.857861, -4.078258, -4.038450, -4.038450.

**Verdict:** local minimum at N=64: **False**; stable at N=64: **False**; claim 14c: **NUMERIC (scale-stationary H=1 candidate exists)**; claim 14d: **FALSIFIED (negative Hessian modes; not a local minimum)**; claim 14b: **UNVERIFIED**.

The scale-stationary H=1 candidate has negative Hessian eigenvalues at N=64, so it is a saddle, not a stable Hopfion. No stable Hopfion is demonstrated.

- Config SHA-256: `428a7dc874dff56ad4dfaa77934ae61860436e80ce5a257295c18bc9f94ca711`
- Git: `fa15995bef8f2074bc7247c9f0fd76333671598a` (dirty=True)
