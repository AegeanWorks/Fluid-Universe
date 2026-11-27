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
| `audit.radial_shell_scale` | `1.000000000000000e-1` | NUMERIC \| eps0=0.1*a0 and r0=2*a0 with a0=1; transferred from claim 60, never scanned |
| `audit.radial_shell_singular_slope` | `-1.000000000000000e0` | NUMERIC \| gmax=pi/dx on N=250/500/1000/2000 gives slope -1.000000000000 |
| `audit.radial_shell_phase` | `3.086078759467158e0` | NUMERIC \| slip=3.086078759467, deficit=0.055513894123 <= bound 0.105555555556, monotone=true |
| `audit.radial_shell_quantum_pressure` | `-4.987499999999999e1` | NUMERIC \| Q0=-49.875000, FD fine=-49.749997, fine error=2.506e-3, step error=7.446e-3 |
| `audit.radial_shell_gates` | `1.000000000000000e0` | NUMERIC \| m=0 circulation=0, norm relative error=2.678e-12, hard gates=true, status=NUMERIC |
| `audit.multi_radial_shell_nodes` | `1.901923788646684e0` | NUMERIC \| r1=1.901923788647, r2=7.098076211353 from L_2^1(x)=x^2/2-3x+3 |
| `audit.multi_radial_shell_singular` | `-1.000000000000000e0` | NUMERIC \| inner slope=-1.000000000000, outer slope=-1.000000000000 |
| `audit.multi_radial_shell_phase` | `6.203291923081967e0` | NUMERIC \| slip=6.203291923082, deficit=0.079893384098 <= bound 0.146609564597, monotone=true |
| `audit.multi_radial_shell_quantum_pressure` | `-4.984327794347278e1` | NUMERIC \| Q1=-49.843277943, Q2=-49.971578003, fine errors=2.523e-5/2.504e-5, step errors=7.570e-5/7.512e-5 |
| `audit.multi_radial_shell_gates` | `1.000000000000000e0` | NUMERIC \| vmax=10.003702332469, m=0 circulation=0, norm relative error=2.071e-10, status=NUMERIC |
| `audit.single_model_search_coverage` | `1.337784000000000e6` | NUMERIC \| models=474, integrable primaries=1337784, N=2..7, k=1..79, Dynkin 0..4 |
| `audit.single_model_known_labels` | `1.000000000000000e0` | PROVEN \| SU(3)_9, SU(2)_13 and SU(3)_49 singlets match 2/9, 1/10 and 2/39 by exact cross-multiplication |
| `audit.single_model_exists` | `0.000000000000000e0` | FALSIFIED \| hit=false after 474 models and 1337784 primaries; bounded negative only |
| `audit.single_model_witness_su17` | `1.000000000000000e0` | PROVEN \| three integrable primaries match 2/9, 1/10 and 2/39 by exact cross-multiplication |
| `audit.single_model_witness_su23` | `1.000000000000000e0` | PROVEN \| second distinct host matches all three phases exactly; existence is non-unique |
| `audit.single_model_inverse_search` | `1.953000000000000e4` | FALSIFIED \| labels=19530, models_with_any_phase=694, all_phase_hit=false |
| `audit.single_model_witness_su7` | `1.000000000000000e0` | PROVEN \| rank-6 host matches all three phases exactly with label sums 19/12/7 <=1163 |
| `audit.single_model_relaxed_inverse` | `1.258880000000000e5` | NUMERIC \| labels=125888, models_with_any_phase=1259, SU(7)_1163_hit=true |
| `audit.lower_rank_inverse_search` | `4.288305000000000e6` | FALSIFIED \| labels=4288305, models_with_any_phase=5499, all_phase_hit=false |
| `audit.minimal_level_inverse_search` | `8.576612100000000e7` | FALSIFIED \| labels=85766121, models_with_any_phase=244, all_phase_hit=false |
| `audit.minimal_witness_search` | `8.576612100000000e7` | FALSIFIED \| labels=85766121, best_sums=[16, 10, 6], expected_minimal=true |
| `audit.minimal_witness_canonical` | `1.000000000000000e0` | PROVEN \| counts=[4, 2, 2], canonical=[[0, 0, 2, 11, 2, 1], [0, 0, 3, 6, 0, 1], [0, 0, 0, 6, 0, 0]], expected=[[0, 0, 2, 11, 2, 1], [0, 0, 3, 6, 0, 1], [0, 0, 0, 6, 0, 0]] |
| `audit.su7_conformal_weights` | `2.280830280830281e-1` | PROVEN \| weights=[0.22808302808302808, 0.10586080586080586, 0.05714285714285714], exact=true |
| `audit.su7_quantum_dimensions` | `1.136464998628996e11` | NUMERIC \| qdim=[113646499862.8996, 407238213.43906987, 456865.8409657278], ordered=true |
| `audit.sm_conformal_embedding` | `4.771282051282051e1` | FALSIFIED \| c_host=55824/1170=9304/195 > 12 >= c_SM_gauge; direct embedding excluded=true |
| `audit.sm_coset_central_charge` | `3.571282051282051e1` | NUMERIC \| c_coset >= c_host-12 = 41784/1170=6964/195 |
| `audit.all_hosts_sm_embedding` | `4.771282051282051e1` | FALSIFIED \| c7=55824/1170, c17=332064/1170, c23=605616/1170; all >12=true |
| `audit.su7_block_sm_coset` | `3.473855391780730e1` | NUMERIC \| two traceless orthogonal block generators=true; c_sub=17624114/1358390=8812057/679195; c_coset=9201758330/264886050=920175833/26488605; exact=true |

## Provenance

- Generator: `fluid-universe-rust` (Rust edition 2024, no external crates).
- In-process determinism fingerprint (not G-001 evidence): run1=8b67325e9175b32c, run2=8b67325e9175b32c, agreement=PASS.
- config_sha256 (FNV-1a): `a590360a74fd87c6`.
