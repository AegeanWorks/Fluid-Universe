# Box-length scan of the pure-FN candidate (claim 14b)

N=64, mu=0, 1200 steps at dt=5.0e-04, fixed vacuum faces, 2x spectral upsampling.

## Attempts

| length | H | H upsampled | Z3 | force RMS | max bond | accepted | min eigenvalue | negative | local minimum |
|---:|---:|---:|---:|---:|---:|---|---:|---:|---|
| 8 | 1.000000047 | 1.000000000 | 0.000002 | 3.208e-03 | 0.475 | True | -1.469962 | 8 | False |
| 10 | 0.999999932 | 1.000000000 | 0.000003 | 3.154e-03 | 0.586 | True | -2.600799 | 8 | False |
| 12 | 0.999966431 | 0.999999999 | 0.000003 | 3.149e-03 | 0.703 | True | -4.223930 | 8 | False |
| 14 | 0.998468715 | 0.999998860 | 0.000003 | 3.643e-03 | 0.844 | True | -6.669667 | 8 | False |

## Verdict

- Local minimum found: **False**.
- Claim 14b: **UNVERIFIED**.

A positive spectrum at one grid and one box length is numerical evidence, not a stability proof; a saddle at every length does not rule out a different ansatz.

- Elapsed: 383.5 s (G-003 cap 900 s).
- Config SHA-256: `38bf81e7789b33794bc75f88950ebc5c9eddf59e6303fa2ed6d2fb10bef89532`
- Git: `aaaa67ae66644bc61b17d37fd178f3b8d739917a` (dirty=True)
