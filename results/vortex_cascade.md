# One-parameter vortex hierarchy test (claim 18)

Observed pole masses: m_e=0.51099895, m_mu=105.658376, m_tau=1776.93 MeV.

| model | parameter | predicted tau ratio | observed | relative error | Q predicted | falsified |
|---|---:|---:|---:|---:|---:|---|
| geometric | 206.768283 | 42753.122850 | 3477.365267 | +11.294689 | 0.870542 | True |
| power_law_muon_fit | 7.691871 | 4676.868795 | 3477.365267 | +0.344946 | 0.696122 | True |
| power_law_koide_fit | 7.064301 | 2347.081108 | 3477.365267 | -0.325040 | 0.666667 | True |

**Status: FALSIFIED (no one-parameter hierarchy matches both ratios and Q).**

A one-parameter fluid hierarchy has no freedom left after the muon ratio or Q is fixed. A two-parameter offset can fit the masses, but then Q is reproduced rather than predicted.

- Observed Koide Q: 0.666664 (target 0.666667).
- Relative tolerance: 0.10; Q tolerance: 0.01.
- Config SHA-256: `57002bfa647ee96b9815062c06e74fcf38384341e52d5913b0e146c2d40f953c`
- Git: `d181dc54f1ad61bbd055f9763999de91a71ad048` (dirty=True)
