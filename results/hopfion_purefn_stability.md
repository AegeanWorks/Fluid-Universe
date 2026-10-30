# Pure-FN candidate Hessian (claims 14b/14d)

N=64, length=10, mu=0, 1200 steps at dt=5.0e-04, fixed vacuum faces, 2x spectral upsampling.

## Candidate

| E | E2 | E4 | virial | H | H upsampled | Z3 | force RMS | max bond |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 356.733 | 217.456 | 139.276 | +78.180165 | 0.999999932 | 1.000000000 | 0.000003 | 3.154e-03 | 0.586 |

## Operator validation (N=8)

- Passed: **True**; max absolute error: 1.332e-15; symmetry error: 6.661e-16.

## Spectrum (N=64)

- Tangent dimension: 476656; requested modes: 8; negative modes: 8.
- Smallest eigenvalues: -2.600799, -2.600799, -2.456051, -2.456051, -2.167258, -2.159256, -1.951652, -1.906687

**Minimum eigenvalue:** -2.600799.

## Verdict

- Claim 14b: **UNVERIFIED**.
- Claim 14e: **FALSIFIED (negative Hessian modes at N=64)**.
- Local minimum at N=64: **False**; stability proved: **False**.

The Hessian acts on the two-dimensional tangent space at every site, so the radial normalization modes are removed. A negative eigenvalue falsifies this candidate as a local minimum; a positive spectrum is evidence at one grid, not a stability proof. Fixed vacuum boundaries lift exact translation zero modes.

- Elapsed: 100.4 s (G-003 cap 900 s).
- Config SHA-256: `65d81240159f72d1b29f22e43bab2b50e33b694e7303aedebf3bccfe06908536`
- Git: `7a4ddef40000b7f00d662be627a7f6670aad74a8` (dirty=True)
