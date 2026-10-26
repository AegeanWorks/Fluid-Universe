# Decisions

Why this project is built the way it is, oldest first. One entry per decision that changes a
contract, a dependency, a data format or the architecture; IDs are consecutive and never reused.

<!-- Entry format:
### ADR-NNN: <decision in one line>
- **Date**: YYYY-MM-DD
- **Status**: Accepted
- **Context**: <what forced the decision>
- **Decision**: <what was decided>
- **Consequences**: <what follows, including what it costs>
-->

### ADR-001: Pre-registered Claim-15 production scan with spectral upsampling and 15-minute wall-clock cap (G-003)
- **Date**: 2026-10-26
- **Status**: Accepted
- **Context**: An unbounded `N=96, steps=9000` CPU gradient flow took ~75 minutes and was terminated by user direction, while the native `N=64` Chern-Simons Hopf estimator drops below 1 when grid-scale gradients develop under the Z3 potential.
- **Decision**: Enforce a hard 15-minute (`900.0 s`) wall-clock budget (`G-003`, `MAX_WALL_SECONDS = 900.0`) on `sims/production.py`, scope `phenomenology/production_report.py` to `N=64, steps=3600` (~3.1 min total) with $2\times$ spectral upsampling (`N=128`), `N=48` `dt` convergence and `N=32` perturbed-seed checks, and evaluate Claim 15 against pre-registered tolerances (`H_TOL`, `Z3_MIN`, `FORCE_TOL`, `DRIFT_TOL`, `BOND_TOL`, `PRE_NORM_TOL`).
- **Consequences**: `make production` completes in ~3.1 minutes; spectral upsampling shows the smooth topological charge stays near 1 (`0.989238`) while native `H` drops (`0.788000`) due to under-resolved bond differences (`1.480 > 0.50`), keeping Claim 15 rigorously `UNVERIFIED`.
