# Saddle-escape search (claim 14b)

N=64, length=10, mu=0, build 1200 steps, relax 600 steps at dt=5.0e-04.

## Base candidate

H=0.999999932, Z3=0.000003, force RMS=3.154e-03, lowest tangent eigenvalue=-2.600799.

## Attempts

| amplitude | H | Z3 | force RMS | max bond | accepted | min eigenvalue | negative | local minimum |
|---:|---:|---:|---:|---:|---|---:|---:|---|
| 0.05 | 0.320364619 | 0.000046 | 6.775e-03 | 1.967 | False | not tested | None | False |
| 0.15 | 0.237384852 | 0.000052 | 7.640e-03 | 1.994 | False | not tested | None | False |

## Verdict

- Local minimum found: **False**.
- Claim 14b: **UNVERIFIED**.

A positive spectrum at one grid and one perturbed candidate is numerical evidence, not a stability proof; a negative search does not rule out a different ansatz.

- Elapsed: 167.9 s (G-003 cap 900 s).
- Config SHA-256: `f81939dff10f8cabff3888e5219c9dcb4195f1c4cf07882abf9c54eec76186e7`
- Git: `c29368aef33e5b08f33cf76abd6ea1ae36e2ed8b` (dirty=True)
