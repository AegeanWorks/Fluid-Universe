# simulation

| id | value | detail |
| --- | --- | --- |
| `simulation.hopf_map_unit` | `9.999999999999998e-1` | \|hopf_map(x,y,z)\| = 1 on S^2 |
| `simulation.analytic_field_unit_error` | `2.220446049250313e-16` | max \| \|n\|-1 \| over the analytic n=8 field |
| `simulation.energy_e2` | `9.800824226342861e1` | gradient energy E2 (k=1, n=8) |
| `simulation.energy_e4` | `1.076531173919216e1` | topological energy E4 (kappa=1, n=8) |
| `simulation.energy_potential` | `6.862981786251160e1` | triatic potential V (mu=0.5, n=8) |
| `simulation.energy_total` | `1.774033718651324e2` | E2+E4+V for the analytic field |
| `simulation.z3_order` | `3.782558649610059e-18` | triatic order of the analytic field |
| `simulation.hopf_charge` | `9.998329459454982e-1` | Gauss linking of the two analytic Hopf fibers |
| `simulation.force_gradient_error` | `4.925482244289014e-6` | \|analytic grad - finite-difference grad\| at the centre site |
| `simulation.metrics_energy` | `1.774033718651324e2` | metrics energy with k=kappa=1 |
| `simulation.relax_energy_drop` | `4.380238659961833e0` | energy decrease over 8 normalized GD steps |

## Provenance

- Generator: `fluid-universe-rust` (Rust edition 2024, no external crates).
- In-process determinism fingerprint (not G-001 evidence): run1=de3a8bd4b7496240, run2=de3a8bd4b7496240, agreement=PASS.
- Check-results FNV-1a 64-bit fingerprint: `6b968cebdce030bf` (not a configuration hash or cryptographic digest).
