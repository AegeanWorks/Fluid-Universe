# First excited state and the node singularity (claim 24)

psi_1 has a node at x=0; the density vanishes and the phase jumps by pi.

## Phase slip

- Phase slip: 3.141592654 (pi = 3.141592654).
- Node density: 0.000e+00.

| N | dx | max |phase gradient| |
|---:|---:|---:|
| 256 | 0.078125000 | 20.106192983 |
| 512 | 0.039062500 | 40.212385966 |
| 1024 | 0.019531250 | 80.424771932 |
| 2048 | 0.009765625 | 160.849543864 |

- Scaling slope: -1.000000000 (expected -1).

## Regular quantum pressure away from the node

- x=0.996093750: numerical 1.003898620605, analytic 1.003898620605, relative error 4.793e-13.

## Verdict

- Regular representation: **False**.
- Claim 24: **FALSIFIED (node phase slip)**.

The density vanishes at x=0, the phase jumps by pi, and the discrete phase gradient diverges as 1/dx. The regular quantum pressure matches away from the node, but the node is a phase slip, not a regular single-valued fluid point. A two-component or defect model is required.

- Config SHA-256: `d43b5e160eec3b25a77e41e3951e5ae8f556c4fb11e11be8dba448543e06263b`
- Git: `81f091f859e308f1d68823c76750d158fe9bd5de` (dirty=True)
