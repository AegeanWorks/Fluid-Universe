# Targeted N=64 3D diagnostic (claim 15)

Fixed vacuum boundaries, dt=5e-4. This is a diagnostic, not part of `make results`.

| mu | steps | E | H | Z3 | force RMS | monotone |
|---:|---:|---:|---:|---:|---:|---|
| 0 | 1200 | 356.732700 | 0.999999932 | 0.000003 | 3.154e-03 | True |
| 20 | 1200 | 3691.572575 | 0.998909622 | 0.230829 | 4.907e-02 | True |
| 20 | 2400 | 2687.653328 | 0.982743188 | 0.438402 | 4.299e-02 | True |
| 20 | 3600 | 1932.842061 | 0.787999784 | 0.607430 | 3.716e-02 | True |

**Verdict:** coexistence: **False**; converged: **False**; claim 15: **UNVERIFIED (N=64 diagnostic; not converged)**.

At N=64 the H/Z3 trend is resolved better than N=32, but the force RMS is still above 1e-2 at 3600 steps. No coexistence is seen; no production or spontaneous-order claim.

- Config SHA-256: `e95515bc14d0b28eaeca66a63caa5722e62310c553f22fffb76b7fda0caa2378`
- Git: `ac88b6e3637b79e2757b4d2472785205061f3491` (dirty=True)
