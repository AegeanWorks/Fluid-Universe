# Production 3D scan (claim 15)

Fixed vacuum faces. The main run relaxes the analytic H=1 map at N=64, mu=20, dt=5.0e-04. It is a production diagnostic, not part of `make results`.

## Pre-registered acceptance criteria

- H within 5.0e-03 of 1 and the native and upsampled estimates agree within the same tolerance.
- Z3 >= 0.5.
- Force RMS <= 1.0e-02 and relative energy drift over the last chunk <= 1.0e-02.
- Maximum bond difference <= 0.50 (at least two grid points per feature).

## Main run

| step | E | H | Z3 | force RMS | max bond | min pre-norm | monotone |
|---:|---:|---:|---:|---:|---:|---:|---|
| 600 | 4291.493 | 0.999840585 | 0.116991 | 5.172e-02 | 0.602 | 1.000000 | True |
| 1200 | 3691.573 | 0.998909622 | 0.230829 | 4.907e-02 | 0.659 | 1.000000 | True |
| 1800 | 3156.416 | 0.995366393 | 0.338734 | 4.613e-02 | 0.761 | 1.000000 | True |
| 2400 | 2687.653 | 0.982743188 | 0.438402 | 4.299e-02 | 0.919 | 1.000000 | True |
| 3000 | 2282.822 | 0.936964507 | 0.528226 | 3.988e-02 | 1.176 | 1.000000 | True |
| 3600 | 1932.842 | 0.787999784 | 0.607430 | 3.716e-02 | 1.480 | 1.000000 | True |

Final native H: 0.787999784; upsampled H: 0.989238052; Z3: 0.607430; force RMS: 3.716e-02; max bond: 1.480.

**Verdict:** coexistence: **False**; converged: **False**; resolved: **False**; topology preserved: **True**; claim 15: **UNVERIFIED (H estimator not converged; not converged; field under-resolved)**.

A failed scan cannot prove that no 3D simulation can achieve coexistence. The Hopf charge is conserved by the normalized flow when the pre-normalization norm stays positive and the boundary is fixed; a native estimator below 1 at grid scale is an estimator-resolution issue, not a topological transition.

## dt check (N=48, same physical time)

| dt | H | Z3 | force RMS |
|---:|---:|---:|---:|
| 5e-4 | 0.998614422 | 0.138620 | 1.217e-01 |
| 2.5e-4 | 0.998612762 | 0.138616 | 1.217e-01 |

Differences: H 1.660e-06, Z3 4.076e-06, force 1.219e-06.

## Seed check (N=32, 300 steps, perturbation 1e-3)

| seed | H | Z3 | force RMS |
|---:|---:|---:|---:|
| 1 | 0.853402434 | 0.443059 | 3.498e-01 |
| 2 | 0.852345282 | 0.443074 | 3.498e-01 |

The potential breaks the azimuthal symmetry explicitly; the seed check is a robustness diagnostic, not a spontaneous-ordering test.

- Config SHA-256: `2552e121bec8f318843d1cfcd5e124e77eecc5013926b87c41698361fe7ea9a9`
- Git: `6657b24d2339c81d95e94827fb48598adab74d3b` (dirty=True)
