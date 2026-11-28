# Codebase Map & Symbol Catalog 🗺️

> **AUTO-GENERATED & VERIFIED BY `aikit update-kb`**
> This document catalogues source modules, types, and exported functions. See Map Coverage for any stack it does not index.

---

## 📊 Codebase Metrics
- **Total Rust Modules**: `97`
- **Total Public Types**: `20`
- **Total Public Functions**: `603`
- **Total Source Lines**: `16325`

## 🧭 Map Coverage
- **Indexed**: rust (`97` module(s))
- **Not indexed by this map**: none

## 🦀 Rust Modules

### `src/amplitude_seesaw.rs` (85 lines)
> Two structural tests: rational amplitudes and a seesaw link to the GUT scale.
- **Functions**: `a2_lepton`, `a2_down`, `a2_up`, `a2_nu`, `rational_a2`, `inverse_a2`, `seesaw_mnu_ev`, `mu_gut`, `observed_mnu_ev`, `checks`

### `src/anomaly.rs` (86 lines)
> Gauge-anomaly sector: can an anomaly-free generation charge supply the mass pattern?
- **Functions**: `anomaly1`, `anomaly3`, `identity_residual`, `cubic_from_pair`, `checks`

### `src/audit.rs` (2379 lines)
> Independent, falsification-first audit of the frozen claim ledger.
- **Types**: `AuditRecord`
- **Functions**: `run_audit`, `audit_fingerprint`, `checks`

### `src/circulant.rs` (294 lines)
> Exact algebra of the Hermitian 3x3 circulant mass matrix.
- **Types**: `Fit`
- **Functions**: `eigenvalues`, `q_signed`, `q_masses`, `q_physical_from_roots`, `q_from_ac`, `extract_ac_delta`, `fit_masses`, `koide_tau_prediction`, `checks`

### `src/circulant_inverse.rs` (83 lines)
> Exact inversion of the circulant mass triplet from its two hierarchy ratios.
- **Functions**: `roots_from_ratios`, `implied_q`, `up_ratios`, `down_ratios`, `checks`

### `src/classical_scan.rs` (94 lines)
> B_n = SO(2n+1) and C_n = Sp(2n) WZW scan -- the last classical families.
- **Functions**: `b2_qdim`, `c2_qdim`, `b3_qdim`, `checks`

### `src/compact_identity.rs` (86 lines)
> Compact algebraic identity and a new SU(Fib) structural test.
- **Functions**: `fib`, `q_compact`, `c2_box`, `required_level`, `d_box`, `checks`

### `src/confinement.rs` (84 lines)
> Cornell-potential sector: three lowest s-wave levels of V(r) = -A/r + sigma*r.
- **Functions**: `hamiltonian`, `levels`, `q_of_levels`, `checks`

### `src/conformal_block.rs` (92 lines)
> Conditional SU(N)_N conformal weights and phase comparisons.
- **Functions**: `casimir_box`, `casimir_lambda2`, `h_box`, `braiding_ratio`, `solve_n_for_h`, `checks`

### `src/cosmology_validation.rs` (69 lines)
> Test of the d=3 framework's cosmological predictions against observations.
- **Functions**: `ratio_pred`, `ratio_obs`, `geom_obs`, `checks`

### `src/d3_chain.rs` (137 lines)
> Non-post-hoc derivation chain from d=3 + electric charge.
- **Functions**: `dimension`, `fib`, `index_from_three_q`, `fib_label`, `amplitude_squared`, `delta`, `q`, `ratios`, `checks`

### `src/d3_neutrino.rs` (74 lines)
> Experimental validation of the d=3 framework's neutrino predictions.
- **Functions**: `d`, `fib9`, `beta`, `m3_mev`, `observed_m3_mev`, `nu_ratio_pred`, `nu_ratio_obs`, `sum_nu_pred_mev`, `sum_nu_obs_mev`, `sin2_theta_h`, `checks`

### `src/d3_only_synthesis.rs` (107 lines)
> Complete d=3-ONLY parameter set for all three sectors (no measured inputs).
- **Functions**: `d`, `beta`, `sin2tw`, `a2`, `fib`, `delta`, `q`, `ratios`, `checks`

### `src/d3_running.rs` (72 lines)
> Test of the proposed derivation: run the d=3 fixed point 3/13 to M_Z.
- **Functions**: `sin2_at`, `fixed_point_scale`, `checks`

### `src/d3_spectrum.rs` (92 lines)
> Independent verification of the d=3 framework's spectral (SUSY-QM) axioms.
- **Functions**: `beta`, `s_from_beta`, `w`, `potential`, `alpha_star`, `u_max`, `log_derivative_psi_s`, `checks`

### `src/d3_validation.rs` (60 lines)
> Independent numeric validation of the d=3 framework's OTHER predictions.
- **Functions**: `predictions`, `checks`

### `src/derrick.rs` (196 lines)
> Derrick scaling for the Frank-Oseen + Skyrme + Z3 energy.
- **Types**: `ScalingMinimum`
- **Functions**: `exponents`, `scaling_energy`, `virial`, `second_derivative`, `stationary_scale_squared`, `scaling_minimum`, `checks`

### `src/dynamical_flow.rs` (69 lines)
> Dynamical (RG-flow) test of the compact index framework's target values.
- **Functions**: `q_up_range`, `q_down_range`, `q_lepton_range`, `target_up`, `target_down`, `target_lepton`, `checks`

### `src/ew_from_planck.rs` (64 lines)
> The d=3 framework's weakest-scale prediction: electroweak scale from M_Pl.
- **Functions**: `d`, `beta`, `alpha_star`, `exponent`, `v_from_planck`, `m_higgs`, `checks`

### `src/exact_phase_labels.rs` (88 lines)
> Conditional WZW weight differences for selected phase targets.
- **Functions**: `delta_lepton_label`, `delta_down_label`, `delta_up_composite`, `up_braiding_level_su3`, `up_braiding_label_from_level`, `checks`

### `src/exceptional_nonsimply.rs` (112 lines)
> F4 and G2 WZW quantum dimensions with CORRECT non-simply-laced normalization.
- **Functions**: `g2_lengths`, `f4_lengths`, `g2_hvee`, `f4_hvee`, `g2_qdim`, `f4_qdim`, `checks`

### `src/exceptional_scan.rs` (99 lines)
> E6_k quantum dimensions: an extended-group test beyond SU(3)_k.
- **Functions**: `h_vee_e6`, `e6_qdim`, `e6_fund_qdim`, `required_a2`, `checks`

### `src/fft.rs` (153 lines)
> Minimal radix-2 FFT for power-of-two 3D grids. No external crates.
- **Functions**: `fft1d`, `fft3d`, `signed_k`

### `src/fib_significance.rs` (59 lines)
> Statistical significance of the A^2/delta ~ {3^2, 5^2, 8^2} pattern.
- **Functions**: `observed_a2_over_delta`, `observed_square_roots`, `null_rate_within_half_percent`, `null_rate_within_two_percent`, `null_median_error`, `joint_p_two_independent`, `checks`

### `src/fibonacci_phase.rs` (149 lines)
> New hypothesis: the Brannen phase is the amplitude divided by a squared
- **Functions**: `lepton_masses`, `fib_label`, `sector_masses`, `amplitude_squared`, `required_delta`, `predicted_delta`, `ratios`, `checks`

### `src/fluid.rs` (269 lines)
> Closed-form fluid / vortex / Marcus checks.
- **Functions**: `ermakov_fixed_point`, `gm_action`, `pinney_branch_b`, `pinney_det`, `vortex_core_velocity`, `vortex_core_q_zero`, `blob_loop_fraction`, `marcus_fwhm`, `koide`, `checks`

### `src/flux_quantization.rs` (72 lines)
> Candidate external physics to fix the Z3-circulant shape parameter p.
- **Functions**: `product_from_wilson`, `pdg_product`, `required_flux_quantum`, `checks`

### `src/flux_sector.rs` (74 lines)
> Flux-threaded Z3 rotor: H = (n-alpha)^2/(2I) + mu(1-cos 3phi).
- **Functions**: `flux_levels`, `q_levels`, `checks`

### `src/fn_scan.rs` (95 lines)
> Joint Froggatt-Nielsen charge scan: both hierarchy ratios at once.
- **Types**: `Assignment`
- **Functions**: `q_of`, `eps_for`, `scan`, `checks`

### `src/foot_geometry.rs` (85 lines)
> Peer-reviewed geometric validation of Koide's beta = 2/3 (Foot).
- **Functions**: `theta`, `foot_rhs`, `cos2`, `framework_theta_deg`, `checks`

### `src/froggatt.rs` (78 lines)
> Froggatt-Nielsen charge sector: masses m_i = eps^(2 q_i) for integer charges.
- **Functions**: `q_of_masses`, `eps_for_koide`, `ratios`, `checks`

### `src/gauge_flavor.rs` (126 lines)
> Gauge–flavor connection: the weak mixing angle fixes the amplitude scale.
- **Functions**: `g1_mz`, `g2_mz`, `sin2_theta_w`, `cot2_theta_w`, `a2_from_gauge`, `observed_a2`, `fib_label`, `delta_from_gauge`, `observed_delta`, `checks`

### `src/gauge_running.rs` (77 lines)
> Gauge-coupling running: a previously unaddressed sector.
- **Functions**: `couplings_mz`, `crossing_scales`, `q_start`, `q_end`, `delta_start`, `delta_end`, `d_box_su3_3`, `checks`

### `src/geometry.rs` (77 lines)
> Geometric characterization: Koide Q=2/3 means 45 degrees to (1,1,1).
- **Functions**: `q`, `cos2_theta`, `required_ratio`, `checks`

### `src/gj_relation.rs` (49 lines)
> Georgi-Jarlskog b-tau relation from the d=3 framework: m_b/m_tau = d*eta_QCD = 2.35.
- **Functions**: `d`, `predicted_ratio`, `eta_from_prediction`, `observed_ratio_mb_scale`, `observed_ratio_mz`, `checks`

### `src/gravity_relation.rs` (74 lines)
> Gauge-gravity numerical relation discovered in this session.
- **Functions**: `planck_mass_gev`, `reduced_planck_mass_gev`, `mu_gut`, `alpha_g`, `predicted_planck`, `relative_error`, `alpha_g_required`, `checks`

### `src/index_framework.rs` (136 lines)
> A compact single structure with one sector index n = 1, 2, 3:
- **Functions**: `fib`, `amplitude_squared`, `label`, `predicted_delta`, `predicted_q`, `predict_ratios`, `checks`

### `src/index_uncertainty.rs` (59 lines)
> Uncertainty-aware verdict for the single-index framework (index_framework.rs).
- **Functions**: `z_down_r1`, `z_up_r1`, `z_up_q`, `rel_sigma_up_r1`, `checks`

### `src/informational_geometry.rs` (80 lines)
> External candidate derivation: the d=3 identities that fix our constants.
- **Functions**: `dimension`, `beta`, `theta_h`, `cot_theta_h`, `phase_from_beta`, `sin2_theta_w`, `three_cot2_theta_w`, `checks`

### `src/jordan_spectrum.rs` (187 lines)
> First genuinely NON-CIRCULANT candidate: the exceptional Jordan spectrum.
- **Functions**: `jordan_delta`, `ratio_d`, `ratio_b`, `ratio_c`, `ratio_t`, `pdg_up`, `pdg_down`, `arithmetic_q`, `min_positive_q`, `nu_q_max`, `nu_target_ratios`, `nu_best_fit`, `checks`

### `src/lib.rs` (237 lines)
> fluid-universe: falsification-first research core, rewritten in Rust.
- **Types**: `Check`
- **Functions**: `new`, `all_checks`, `fingerprint`

### `src/linalg.rs` (123 lines)
> Small numerical kernels used by the physics modules. No external crates.
- **Functions**: `symmetric_eigenvalues`, `simpson`, `bisect`, `approx`

### `src/lorentz.rs` (225 lines)
> Linearised director dispersion and the emergent-metric question (claims
- **Functions**: `elastic_eigenvalues`, `residual_anisotropy`, `checks`

### `src/main.rs` (894 lines)
> CLI for the Rust fluid-universe core. Every subcommand is bounded and silent

### `src/mass_convention.rs` (121 lines)
> CORRECTION: the earlier "exact circulant no-go" was a mass-convention artifact.
- **Functions**: `up_masses_mz`, `down_masses_mz`, `q_from_masses`, `sqrt_ratios`, `implied_q`, `amplitude`, `required_delta`, `checks`

### `src/mass_spectrum.rs` (109 lines)
> Mass-spectrum sector: a Z3 rotor with a symmetry-breaking field.
- **Functions**: `rotor_levels`, `koide_from_spacings`, `koide_from_levels`, `checks`

### `src/modular.rs` (78 lines)
> Modular sector: singular moduli at Z3 CM points via theta series.
- **Functions**: `lambda`, `koide`, `checks`

### `src/monodromy.rs` (173 lines)
> WZW monodromy: the cube identity and the N=3 over-determination of Q.
- **Functions**: `h_box`, `quantum_dimension`, `quantum_dimension_nn`, `wzw_number`, `cube`, `cube_argument`, `q_from_fusion`, `overdetermination_gap`, `unique_n`, `amplitude_from_q`, `ratios`, `checks`

### `src/monodromy_block.rs` (199 lines)
> Rigorous SU(3)_3 conformal-block monodromy calculation.
- **Types**: `Complex`
- **Functions**: `primaries`, `s_matrix_element`, `h_weight`, `fusion`, `sphere_four_point_dimension`, `braiding_label`, `checks`

### `src/multi_radial_shell.rs` (318 lines)
> HO-3 double-radial-shell finite-core probe (claim 63, ADR-059).
- **Types**: `Report`
- **Functions**: `nodes`, `phase`, `phase_slip`, `phase_slip_margin`, `velocity`, `velocity_max`, `q_at_node`, `fd_q_at_node`, `singular_gradient_max`, `raw_norm_closed_form`, `raw_norm_numeric`, `verdict`, `run`, `checks`

### `src/natural_phase_labels.rs` (62 lines)
> Singlet-channel weight differences at three post-hoc selected levels.
- **Functions**: `d`, `singlet_label`, `levels`, `checks`

### `src/nosusy_unification.rs` (70 lines)
> No-SUSY unification test using the d=3 gravity relation as boundary.
- **Functions**: `inv_alpha_at`, `spread`, `min_spread_scale`, `alpha_grav`, `checks`

### `src/null_model.rs` (65 lines)
> Statistical null model for the "rational amplitude" regularity.
- **Functions**: `observed_rational_errors`, `null_hit_fraction`, `null_median_error`, `joint_chance_probability`, `checks`

### `src/orthogonal.rs` (83 lines)
> Orthogonal-polynomial sector: exact Koide point from Laguerre zeros.
- **Functions**: `e1_squared_over_e2`, `laguerre_minus_one_zeros`, `q_of`, `nonzero_ratio`, `checks`

### `src/peer_reviewed_37.rs` (75 lines)
> Peer-reviewed external support for the d=3 framework's key 3/13 relation.
- **Functions**: `m_w_gev`, `sin2_mz_loginov`, `observed_sin2_mz`, `checks`

### `src/phase_arithmetic.rs` (93 lines)
> Why the WZW identification has the right arithmetic form and G2 geometry does not.
- **Functions**: `delta_rational`, `delta_radians`, `is_rational_multiple_of_pi`, `wzw_h_rational_form`, `checks`

### `src/phase_interpolation.rs` (56 lines)
> A single d=3 phase rule eliminating the Fibonacci/gauge branch split.
- **Functions**: `d`, `delta`, `checks`

### `src/poisson.rs` (86 lines)
> Poisson/exponential statistics as a candidate for unit CV.
- **Functions**: `exponential_population_cv`, `q_from_population_cv`, `expected_q_three_exponentials`, `sample_statistics`, `checks`

### `src/potential_family.rs` (99 lines)
> Generalized Z3 potential family V_p(phi) = (1 - cos(3phi))^p.
- **Functions**: `levels`, `q_levels`, `checks`

### `src/potts_sector.rs` (97 lines)
> Z3 center sector: the 3-state clock/Potts transfer matrix.
- **Functions**: `transfer_ratio`, `critical_coupling`, `eigenvalues`, `q_signed`, `koide_coupling`, `checks`

### `src/puddle.rs` (83 lines)
> Puddle and film geometry: can it fix the depth required by the Koide ratio?
- **Functions**: `root_y`, `required_depth_over_lc`, `max_puddle_height_over_lc`, `checks`

### `src/quark_phases.rs` (84 lines)
> Independent quark-sector test of the SU(3)_3 / Brannen framework.
- **Functions**: `amplitude_from_q`, `ratios`, `up_target`, `down_target`, `joint_error`, `checks`

### `src/quark_rg.rs` (98 lines)
> Down-quark RG channel: does Q_D reach 2/3 at the unification scale?
- **Functions**: `koide`, `run`, `checks`

### `src/radial_shell.rs` (272 lines)
> HO-2 radial-shell finite-core probe (claim 62, ADR-058).
- **Types**: `Report`
- **Functions**: `core_radius`, `node_position`, `phase`, `phase_slip`, `phase_slip_margin`, `velocity`, `velocity_max`, `q_zero`, `fd_q_zero`, `singular_gradient_max`, `raw_norm_closed_form`, `raw_norm_numeric`, `verdict`, `run`, `checks`

### `src/rep_scan.rs` (150 lines)
> Systematic scan of SU(3)_k representations against every fermion sector.
- **Types**: `BestRep`
- **Functions**: `dimq`, `casimir`, `conformal_h`, `a2_from_q`, `closest_rep`, `ratios`, `checks`

### `src/reports.rs` (396 lines)
> Result regeneration for the Rust rewrite.
- **Types**: `GeneratedReport`
- **Functions**: `build_reports`, `write_reports`, `checks`

### `src/rg_sector.rs` (135 lines)
> One-loop SM renormalization-group sector for the circulant phase and ratio.
- **Functions**: `run_high_to_mz`, `phase`, `effective_ratio`, `checks`

### `src/rotor.rs` (230 lines)
> Quantum rotor H = L^2/(2I) + mu (1 - cos 3 phi) and its spectra.
- **Functions**: `sector_matrix`, `lowest_levels`, `free_levels`, `domain_wall_tension`, `domain_wall_tension_quadrature`, `checks`

### `src/scale_choice.rs` (67 lines)
> The gauge–flavor offset is a scale-choice artifact: use M_W, not M_Z.
- **Functions**: `a1`, `b1`, `a2`, `b2`, `three_cot2_at`, `checks`

### `src/sector_count.rs` (62 lines)
> Derivation of the number of sectors from d=3 (no measured input).
- **Functions**: `q`, `admissible`, `lower_bound`, `upper_bound`, `nu`, `checks`

### `src/shape_freedom.rs` (100 lines)
> Structural proof: the Z3-circulant symmetry CANNOT fix the root product p.
- **Functions**: `p_degenerate`, `q_of_p`, `pdg_spacing_ratio`, `pdg_root_product`, `checks`

### `src/simulation.rs` (955 lines)
> 3D field core: Hopf map, finite-difference energy/forces and relaxation.
- **Types**: `Field3`, `Energy`, `Metrics`
- **Functions**: `hopf_map`, `analytic_field`, `normalize_field`, `energy_terms`, `z3_order`, `hopf_charge_preimage`, `force_field`, `relax`, `z3_complex`, `z3_phase`, `phase_rotated_field`, `perturbed_field`, `hopf_charge_grid`, `hopf_charge_spectral`, `metrics`, `checks`

### `src/single_model_test.rs` (1098 lines)
> Single-WZW-model existence and bounded non-existence evidence.
- **Types**: `Witness`, `SearchSummary`, `InverseSearchSummary`, `MinimalDomainSummary`
- **Functions**: `fundamental_pair_center_allowed`, `fundamental_pair_weight_differences`, `fundamental_pair_monodromy_target_mask`, `phases`, `models`, `braiding_fraction`, `search`, `inverse_search`, `relaxed_inverse_search`, `lower_rank_inverse_search`, `minimal_level_inverse_search`, `minimal_witness_search`, `witness_fractions`, `witness_exact`, `all_witnesses_exact`, `conformal_weight`, `quantum_dimension`, `central_charge`, `sm_gauge_central_charge_upper_bound`, `sm_charge_compatible`, `su7_minimal_coset_central_charge`, `su7_block_sm_central_charge`, `su7_block_coset_central_charge`, `su7_hypercharge_branching`, `su7_weak_doublet_count`, `su7_subgroup_anomaly_coefficients`, `su7_cartan_cubic_anomaly`, `canonical_conformal_weights`, `canonical_quantum_dimensions`, `single_model_exists`, `checks`

### `src/single_phase_rule.rs` (80 lines)
> Clarification of the up-quark tension: it is an AMPLITUDE precision issue,
- **Functions**: `f_up`, `a2_framework`, `a2_measured`, `a2_best_fit`, `ratios`, `checks`

### `src/so10_gut.rs` (110 lines)
> SO(10) = D5 GUT test: mass parameters AND gauge unification.
- **Functions**: `h_vee_d5`, `so10_qdim`, `so10_casimir`, `mssm_unification_scale`, `mssm_alpha_g`, `checks`

### `src/spectra.rs` (217 lines)
> Mass-spectra Koide checks: neutrinos, quarks and PMNS angles.
- **Functions**: `q_from_masses`, `neutrino_masses`, `neutrino_q_max`, `pmns_angles_degrees`, `checks`

### `src/spinodal.rs` (125 lines)
> Tilted Z3 potential spinodal sector.
- **Functions**: `stationary`, `minima`, `q_from_curvatures`, `checks`

### `src/su2_monodromy.rs` (129 lines)
> Exact SU(2) fusion and full-monodromy witness with higher-spin fields.
- **Functions**: `fusion_allowed`, `reversed_monodromy_residue`, `lower_level_search`, `lower_spin_search`, `checks`

### `src/su3.rs` (84 lines)
> SU(3) representation sector: do representation dimensions reproduce Q=2/3?
- **Functions**: `dim`, `koide`, `closest`, `checks`

### `src/threshold_unification.rs` (48 lines)
> No-SUSY unification with one intermediate threshold: honest negative.
- **Functions**: `sm_spread_min`, `threshold_spread`, `threshold_mu_g`, `threshold_mu_i`, `checks`

### `src/topoguard_independent.rs` (151 lines)
> Independent local-energy finite-difference check of the guarded probe.
- **Types**: `IndependentProbe`
- **Functions**: `run_independent_probe`

### `src/topology_guard.rs` (128 lines)
> Conservative homotopy certificate for periodic sampled unit-vector fields.
- **Types**: `MonitoredRelaxation`
- **Functions**: `homotopy_hemisphere_margin`, `monitored_relaxation`, `checks`

### `src/two_component.rs` (136 lines)
> Two-component ansatz: a topological H=1 layer and a Z3-ordered layer.
- **Functions**: `z3_layer`, `mean_overlap`, `blended`, `checks`

### `src/unification.rs` (101 lines)
> The parameter-free condition a unified Z3/circulant model would have to satisfy.
- **Functions**: `required_r`, `r_for_q`, `delta_independence`, `bridge_gap`, `checks`

### `src/unified_model.rs` (237 lines)
> ONE falsifiable model: Z3-circulant charged-lepton mass matrix.
- **Functions**: `r_koide`, `root_product`, `delta_star`, `roots`, `q_signed`, `q_physical`, `symmetric`, `hierarchy_ratios`, `fit_p_to_ratio`, `ratios_of_cubic`, `held_out_tau_over_mu`, `held_out_spacing_from_mu_e`, `checks`

### `src/up_amplitude_gauge.rs` (77 lines)
> Up amplitude derived from the measured weak mixing angle to 0.016%.
- **Functions**: `a2_up_from_gauge`, `a2_up_from_q`, `ratios`, `checks`

### `src/up_phase_resolution.rs` (83 lines)
> Resolution of the persistent up-quark ratio tension using a d=3 phase.
- **Functions**: `d`, `beta`, `delta_lep`, `sin2_theta_w`, `delta_up_gauge`, `delta_up_chain`, `a2_up`, `ratios`, `z_chain`, `z_gauge`, `checks`

### `src/variation.rs` (127 lines)
> Exact reformulation: Q = 2/3 means CV = 1 for the three signed roots.
- **Functions**: `q`, `cv`, `q_from_cv`, `mean_cv_exponential`, `mean_cv_lognormal`, `checks`

### `src/verify.rs` (179 lines)
> Project rule G-001 requires per-claim agreement in two separate processes.
- **Types**: `Verification`
- **Functions**: `line`, `verify_in_process`, `compare`, `compare_audit_records`, `compare_expected_audit_records`, `verify_with`

### `src/vortex_unification.rs` (82 lines)
> Vortex-core and loop sector: does a pre-registered radius supply 1/sqrt(2)?
- **Functions**: `blob_fraction`, `radius_for_fraction`, `koide_radius_over_delta`, `velocity_ratio`, `checks`

### `src/water.rs` (183 lines)
> Shallow-water acoustic-horizon closed forms (claim 27, ADR-018).
- **Functions**: `sound_speed`, `horizon_radius`, `surface_gravity`, `hawking_temperature`, `capillary_length`, `capillary_ratio`, `checks`

### `src/water_mechanics.rs` (127 lines)
> Water mechanics: gravity-capillary dispersion and the shallow-water horizon.
- **Functions**: `capillary_length`, `phase_speed_squared`, `minimum_wave_number`, `energy_ratio_at_minimum`, `depth_for_ratio`, `horizon`, `checks`

### `src/water_unification.rs` (79 lines)
> Independent water fixation of h = 2 l_c.
- **Functions**: `min_phase_speed_squared`, `shallow_speed_squared`, `matching_depth`, `ratio_at_matching_depth`, `checks`

### `src/wzw.rs` (90 lines)
> Physics beyond the Z3-circulant symmetry: SU(3)_3 WZW conformal dimension.
- **Functions**: `casimir_su3`, `dual_coxeter_su3`, `level`, `h_fundamental`, `derived_delta`, `checks`

### `src/wzw_crosscheck.rs` (74 lines)
> Independent cross-check of the SU(3)_3 WZW framework against other sectors.
- **Functions**: `quantum_dimension_fundamental`, `q_from_quantum_dimension`, `amplitude_squared_from_q`, `checks`

### `src/wzw_phase_origin.rs` (72 lines)
> SU(3)_3 conformal-weight differences and candidate phase labels.
- **Functions**: `h_box`, `h`, `label`, `checks`

### `src/xy_z3.rs` (192 lines)
> 2D XY + Z3 centre model: a new topological class (plaquette winding).
- **Functions**: `energy`, `z3_order`, `winding_number`, `winding_density`, `seeded_vortex`, `relax_from`, `relax`, `checks`
