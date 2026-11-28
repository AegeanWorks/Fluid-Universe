# multi_radial_shell

| id | value | detail |
| --- | --- | --- |
| `multi_radial_shell.node_inner` | `1.901923788646684e0` | inner 3s node (9-3*sqrt(3))*a0/2 |
| `multi_radial_shell.node_outer` | `7.098076211353316e0` | outer 3s node (9+3*sqrt(3))*a0/2 |
| `multi_radial_shell.singular_slope_inner` | `-1.000000000000000e0` | inner-node singular phase-gradient slope |
| `multi_radial_shell.singular_slope_outer` | `-1.000000000000000e0` | outer-node singular phase-gradient slope |
| `multi_radial_shell.phase_slip` | `6.203291923081967e0` | full-domain product-phase decrease |
| `multi_radial_shell.phase_deficit_bound` | `1.466095645967166e-1` | sum of the two pre-registered single-node bounds |
| `multi_radial_shell.velocity_cap` | `1.000370233246946e1` | actual finite radial velocity magnitude at both nodes |
| `multi_radial_shell.q_inner` | `-4.984327794347278e1` | inner-node Q0 from u=r*R_eps |
| `multi_radial_shell.q_outer` | `-4.997157800329269e1` | outer-node Q0 from u=r*R_eps |
| `multi_radial_shell.q_fine_error_inner` | `2.523470376919985e-5` | inner-node fine FD vs analytic Q error |
| `multi_radial_shell.q_fine_error_outer` | `2.504139868409196e-5` | outer-node fine FD vs analytic Q error |
| `multi_radial_shell.q_step_error_inner` | `7.569667885858327e-5` | inner-node fine vs previous FD Q error |
| `multi_radial_shell.q_step_error_outer` | `7.511673762215791e-5` | outer-node fine vs previous FD Q error |
| `multi_radial_shell.loop_circulation` | `0.000000000000000e0` | m=0 radial shell has zero azimuthal circulation |
| `multi_radial_shell.norm_relative_error` | `2.070575567994570e-10` | raw radial norm vs closed form |
| `multi_radial_shell.gate_status` | `1.000000000000000e0` | NUMERIC (finite-core regularized HO-3 double radial shell demonstrated at eps0; both phases regular, velocity/Q finite and N-converged, circulation zero) |

## Provenance

- Generator: `fluid-universe-rust` (Rust edition 2024, no external crates).
- In-process determinism fingerprint (not G-001 evidence): run1=d902d29ae7cdd8ec, run2=d902d29ae7cdd8ec, agreement=PASS.
- Check-results FNV-1a 64-bit fingerprint: `23a5062a76ac81cf` (not a configuration hash or cryptographic digest).
