# audit

| id | value | detail |
| --- | --- | --- |
| `audit.q_identity` | `6.217248937900877e-15` | PROVEN \| max \|Q_signed - (1/3+2c^2/3a^2)\| = 6.217e-15; algebraic sum/sum-sq residual 7.105e-15 |
| `audit.koide_ratio_condition` | `7.071067811865476e-1` | PROVEN \| q_from_ac(1, 1/sqrt2) = 0.666666666667; c^2 = a^2/2 solves 1/3+2c^2/3a^2 = 0.666666666667 |
| `audit.z3_not_fixed` | `4.864864864864865e-1` | FALSIFIED \| five positive-root triples span Q in [0.333333, 0.819820] (spread 0.486486) |
| `audit.sign_caveat` | `8.733333333333333e-1` | FALSIFIED \| a=1, c=0.9, delta=pi: signed Q=0.8733333333, physical Q=0.3714555766 |
| `audit.tau_prediction` | `1.776969027083014e3` | NUMERIC \| m_tau = 1776.969027 MeV; Q(m_e,m_mu,m_tau) = 0.666666666667 |
| `audit.r_invariance` | `3.057662803063188e-6` | FALSIFIED \| \|r(10^16 GeV) - r(M_Z)\| = 3.058e-6 (nonzero; generation-dependent RHS) |
| `audit.neutrino_q_max` | `5.856120093687199e-1` | FALSIFIED \| Q_nu(NO,m0=0)=0.585612009369, Q_nu(IO,m0=0)=0.500006628767; max=0.585612009369 < 2/3 |
| `audit.quark_q` | `8.875692313187327e-1` | FALSIFIED \| Huang-Zhou M_Z masses: Q_up=0.887569231319, Q_down=0.748065054803, both > 2/3 |
| `audit.rotor_degeneracy` | `2.220446049250313e-16` | PROVEN \| E1=1.317034514042, E2=1.317034514042, \|E1-E2\|=2.220e-16 (exact by H(phi)=H(-phi)) |
| `audit.wall_tension` | `2.666666666667977e0` | PROVEN \| Simpson T(1)=2.666666666668 vs (8/3)=2.666666666667 |
| `audit.derrick_virial` | `1.871001131803496e-9` | PROVEN \| d=3 exponents [1,-1,3]; numeric dE/dlambda=2.499999998, virial=2.500000000, residual 1.87e-9 |
| `audit.equal_k_isotropy` | `5.551115123125783e-16` | PROVEN \| xi(K=K=K)=5.6e-16; the elastic matrix is 2K*I, two degenerate transverse modes |
| `audit.k123_anisotropy` | `1.001633494422914e0` | FALSIFIED \| xi(K=1,2,3)=1.001633; axis speeds along x=[1,sqrt2], z=[sqrt3,sqrt3]: speeds split |
| `audit.delta_closed_form` | `2.222247619040263e-1` | UNVERIFIED \| delta_mod(2pi/3)=0.222224762; nearest 2/9 at 2.540e-6; post-hoc, not promoted |
| `audit.hopf_preimage_charge` | `9.999999999999938e-1` | NUMERIC \| Gauss linking of two linked circles = -1.000000; reflection gives 0.9999999999999938 |
| `audit.z3_limits` | `6.123233995736765e-17` | NUMERIC \| four evenly spaced phases -> Z3=6.123e-17 (min); single phase -> Z3=1.000e0 (max 1) |
| `audit.vortex_blob_fraction` | `8.000000000000000e-1` | NUMERIC \| r^2/(r^2+delta^2) at r=2*delta = 0.800000000000 (delta point 0.5, 20-delta point 400/401) |
| `audit.marcus_width` | `6.809145309595547e-1` | NUMERIC \| FWHM(lambda=1.617132544 eV, T=300K)=0.680914531 eV; scales as sqrt(T) |

## Provenance

- Generator: `fluid-universe-rust` (Rust edition 2024, no external crates).
- G-001 two-process fingerprint: run1=b276f1b9c6d822a8, run2=b276f1b9c6d822a8, agreement=PASS.
- config_sha256 (FNV-1a): `255b8af2fd3dd5bc`.
