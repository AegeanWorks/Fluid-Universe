# radial_shell

| id | value | detail |
| --- | --- | --- |
| `radial_shell.eps0` | `1.000000000000000e-1` | pre-registered eps0=0.1*a0 in atomic units |
| `radial_shell.node_radius` | `2.000000000000000e0` | hydrogen-2s nodal shell r0=2*a0 |
| `radial_shell.singular_slope` | `-1.000000000000000e0` | singular phase-gradient log-log slope (gate -1+-0.01) |
| `radial_shell.phase_slip` | `3.086078759467158e0` | finite-domain regular phase decrease on [0,20] |
| `radial_shell.phase_deficit_bound` | `1.055555555555556e-1` | pre-registered bound 2*eps/r0+eps/(L-r0) |
| `radial_shell.velocity_max` | `9.999999999999998e0` | finite radial velocity cap (hbar/m)/eps=10 |
| `radial_shell.q_zero` | `-4.987499999999999e1` | Q0=-0.5*(1/eps^2-1/r0^2)=-49.875 |
| `radial_shell.q_fd_fine` | `-4.974999689048047e1` | finite-difference Q0 at N=2000 |
| `radial_shell.q_fine_relative_error` | `2.506328010416408e-3` | fine FD vs analytic Q relative error (gate <=0.05) |
| `radial_shell.q_step_relative_error` | `7.446371103549456e-3` | fine vs previous FD Q relative error (gate <=0.05) |
| `radial_shell.loop_circulation` | `0.000000000000000e0` | m=0 radial shell has zero azimuthal circulation |
| `radial_shell.norm_relative_error` | `2.678046202641949e-12` | raw radial norm vs closed form (gate <=1e-8) |
| `radial_shell.gate_status` | `1.000000000000000e0` | NUMERIC (finite-core regularized HO-2 radial shell demonstrated at eps0; phase regular, velocity/Q finite and N-converged, circulation zero) |

## Provenance

- Generator: `fluid-universe-rust` (Rust edition 2024, no external crates).
- In-process determinism fingerprint (not G-001 evidence): run1=1efdc0df34635e53, run2=1efdc0df34635e53, agreement=PASS.
- Check-results FNV-1a 64-bit fingerprint: `ba09fa07904e4808` (not a configuration hash or cryptographic digest).
