# Checkpointed N=96 extension (claim 15)

N=96, length=10, mu=20, dt=5.0e-04, 3000 steps per chunk, fixed vacuum faces, 2x upsampling.

Each chunk is one G-003 run; the field is saved atomically and the next chunk resumes from it.

## Rows

| chunk | steps | t | H | H upsampled | Z3 | force RMS | max bond | seconds |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 3000 | 1.5 | 0.999942144 | 1.000000000 | 0.172071 | 1.495e-02 | 0.418 | 577.3 |
| 2 | 6000 | 3 | 0.999484353 | 0.999999976 | 0.333942 | 1.368e-02 | 0.463 | 533.9 |
| 3 | 9000 | 4.5 | 0.996350415 | 0.999998598 | 0.477792 | 1.224e-02 | 0.564 | 577.5 |
| 4 | 12000 | 6 | 0.972623788 | 0.999903154 | 0.598690 | 1.075e-02 | 0.857 | 566.3 |

## Trend

- Z3 slope per step: +4.740e-05; monotone: **True**.
- Force slope per step: -4.668e-07; monotone: **True**.
- Linear projection to Z3=0.5: 0 steps, t=6.
- Linear projection to force=1e-2: 1599 steps, t=6.8.

Linear projections are diagnostics, not claims; claim 15 is verified only when the final state satisfies the production rule.

**Claim 15: UNVERIFIED (H estimator not converged; not converged; field under-resolved).** Production ready: **False**.

- Config SHA-256: `60e7405d980fbc5387238d50bea18efe0a506b0c52e73d26451957e709377ea1`
- Git: `023a0e59c95c8bbf1a43a370feefdf6a5187c2be` (dirty=True)
