# Tangent-space Hessian of the scale-stationary candidate (claim 14d)

The Hessian is computed on the two-dimensional tangent space of the unit sphere at every interior site, so the radial normalization modes are removed. This is evidence at one grid, not a stability proof.

Candidate: H=0.997297414, Z3=2.584e-05, E2-E4=+0.009610, force RMS=3.002e-03.

## Dense validation

- N=8, tangent dimension 432.
- Maximum |dense - matrix-free|: 1.332e-15.
- Symmetry error: 6.661e-16.
- Passed (both below 1e-8): **True**.

## N=64 spectrum

- Tangent dimension: 476656.
- Requested eigenvalues: 8.
- Minimum eigenvalue: -5.799863.
- Negative eigenvalues in the requested set: 8.
- Near-zero eigenvalues: 0.

Smallest eigenvalues: -5.799863, -5.799863, -5.015290, -5.015290, -4.857861, -4.078258, -4.038450, -3.853634.

**Verdict:** operator validated: **True**; local minimum at N=64: **False**; claim 14d: **FALSIFIED (negative Hessian modes at N=64)**; claim 14b: **UNVERIFIED**.

The Hessian acts on the two-dimensional tangent space at every site, so the radial normalization modes are removed. A negative eigenvalue falsifies this candidate as a local minimum; a positive spectrum is evidence at one grid, not a stability proof. Fixed vacuum boundaries lift exact translation zero modes.

- Config SHA-256: `a90c4fe60beee7c5a865cec227e71c70bed2477316c30c8251856227ccf0b92a`
- Git: `930559aa4fe4d018299605fbcc46b171d2a7813c` (dirty=True)
