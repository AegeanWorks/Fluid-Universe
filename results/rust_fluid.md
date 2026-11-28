# fluid

| id | value | detail |
| --- | --- | --- |
| `fluid.ermakov_fixed_point` | `5.000000000000000e-1` | variance sigma^2 = nu/omega at nu=0.5, omega=1 |
| `fluid.gm_action_at_fixed_point` | `-5.000000000000000e-1` | J_G(sigma^2*) = -m*nu*omega at the Ermakov fixed point |
| `fluid.pinney_branch_b1` | `3.660254037844386e-1` | admissible Yasue branch B1 = (-1+sqrt(3))/2 |
| `fluid.pinney_branch_b2` | `-1.366025403784439e0` | killed sibling B2 = (-1-sqrt(3))/2 |
| `fluid.pinney_det_yasue` | `-2.500000000000000e-1` | A*C - B^2 = -nu^2 for the c<0 Pinney bridge |
| `fluid.vortex_core_velocity_max` | `-9.999999999999998e0` | max finite core speed -(hbar/m)/eps at x=0, eps=0.1 |
| `fluid.vortex_core_q_zero` | `-4.949999999999999e1` | analytic Q(0) = -0.5*(1/eps^2 - 1/s^2) at eps=0.1, s=1 |
| `fluid.blob_loop_fraction_delta` | `5.000000000000000e-1` | blob loop fraction at r=delta |
| `fluid.blob_loop_fraction_2delta` | `8.000000000000000e-1` | blob loop fraction at r=2*delta |
| `fluid.blob_loop_fraction_20delta` | `9.975062344139651e-1` | blob loop fraction at r=20*delta |
| `fluid.marcus_fwhm_300k` | `6.809145309595547e-1` | fixed-lambda Marcus FWHM (eV) at 300 K |
| `fluid.koide_physical` | `6.666644634145000e-1` | Koide ratio for (m_e, m_mu, m_tau) in MeV |

## Provenance

- Generator: `fluid-universe-rust` (Rust edition 2024, no external crates).
- In-process determinism fingerprint (not G-001 evidence): run1=de3a8bd4b7496240, run2=de3a8bd4b7496240, agreement=PASS.
- Check-results FNV-1a 64-bit fingerprint: `f914e97c9e716c02` (not a configuration hash or cryptographic digest).
