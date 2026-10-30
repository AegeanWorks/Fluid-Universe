# Codebase Map & Symbol Catalog 🗺️

> **AUTO-GENERATED & VERIFIED BY `scripts/update_knowledge_base.py`**
> This document provides an automated, up-to-date registry of all modules, classes, and exported functions across this repository.

---

## 📊 Codebase Metrics
- **Total Python Modules**: `116`
- **Total Classes**: `6`
- **Total Top-Level Functions**: `630`
- **Total Class Methods**: `4`
- **Total Source Lines**: `15753`

---

## 📦 Modules & Exported Symbols

### [`phenomenology/__init__.py`](phenomenology/__init__.py)
> *Phenomenology: PDG input, circulant fit, and one-loop SM running.*

**Size**: `1` lines (`72` bytes)

---

### [`phenomenology/coherent_fluid_report.py`](phenomenology/coherent_fluid_report.py)
> *Moving-center Gaussian reduction report (claim 23, ADR-014).*

**Size**: `157` lines (`5262` bytes)

**Functions:**
- `def _energy_drift(result)`
- `def _zero_crossings(times, values)`
- `def run_coherent_fluid_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/convergence_report.py`](phenomenology/convergence_report.py)
> *Resolution report for claim 15: N=64 versus N=96 at the same physical time.*

**Size**: `166` lines (`5655` bytes)

**Functions:**
- `def _run(n, deadline)`
- `def run_convergence_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/delta_search.py`](phenomenology/delta_search.py)
> *Search for a closed form of the circulant phase delta, with a stated prior.*

**Size**: `185` lines (`6508` bytes)

**Functions:**
- `def rational_candidates()`
- `def pi_candidates()`
- `def simple_candidates()`
- `def all_candidates()`
- `def search(delta, sigma)`
- `def run_delta_search(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/excited_fluid_report.py`](phenomenology/excited_fluid_report.py)
> *First-excited-state node report (claim 24, ADR-015).*

**Size**: `146` lines (`4771` bytes)

**Functions:**
- `def run_excited_fluid_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/extension_report.py`](phenomenology/extension_report.py)
> *Checkpointed N=96 extension for claim 15 (ADR-004).*

**Size**: `223` lines (`7402` bytes)

**Functions:**
- `def _checkpoint(chunk)`
- `def _write_npy_atomic(path, array)`
- `def _load_state()`
- `def _save_state(state)`
- `def run_next_chunk(write)`
- `def render_markdown(state)`
- `def _format_projection(steps, time_value)`
- `def main(argv)`

---

### [`phenomenology/fit.py`](phenomenology/fit.py)
> *PDG pole-mass fit of the Hermitian circulant parametrisation.*

**Size**: `228` lines (`8091` bytes)

**Classes:**
- `class PoleMass`

**Functions:**
- `def load_pole_masses(path)`
- `def _summary(values)`
- `def _circular_mean(angles)`
- `def fit_point(masses_MeV)`
- `def run_fit(n_mc, seed, write)`
- `def render_markdown(payload)`
- `def main()`

---

### [`phenomenology/gaussian_fluid_report.py`](phenomenology/gaussian_fluid_report.py)
> *Gaussian Madelung reduction report (claim 22, ADR-013).*

**Size**: `155` lines (`5293` bytes)

**Functions:**
- `def _period(result, width)`
- `def run_gaussian_fluid_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/hopf_report.py`](phenomenology/hopf_report.py)
> *Hopf estimator validation result payload.*

**Size**: `66` lines (`2420` bytes)

**Functions:**
- `def run_hopf_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/hopf_resolution_report.py`](phenomenology/hopf_resolution_report.py)
> *Hopf estimator resolution report.*

**Size**: `47` lines (`1509` bytes)

**Functions:**
- `def run_hopf_resolution_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/hopfion3d_n64_report.py`](phenomenology/hopfion3d_n64_report.py)
> *Targeted N=64 diagnostic for the 3D E2+E4+Z3 pilot.*

**Size**: `104` lines (`3726` bytes)

**Functions:**
- `def run_n64_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/hopfion3d_report.py`](phenomenology/hopfion3d_report.py)
> *3D pilot report for the E2+E4+Z3 model.*

**Size**: `83` lines (`3099` bytes)

**Functions:**
- `def run_hopfion3d_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/hopfion_report.py`](phenomenology/hopfion_report.py)
> *Static finite-energy H=1 configuration report.*

**Size**: `61` lines (`2215` bytes)

**Functions:**
- `def run_hopfion_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/hopfion_scale_report.py`](phenomenology/hopfion_scale_report.py)
> *Report for the damped scale-iteration candidate.*

**Size**: `71` lines (`2442` bytes)

**Functions:**
- `def run_scale_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/hydrogen_fluid_report.py`](phenomenology/hydrogen_fluid_report.py)
> *Hydrogen 1s Madelung reduction report (claim 25, ADR-016).*

**Size**: `128` lines (`4504` bytes)

**Functions:**
- `def run_hydrogen_fluid_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/hydrogen_vortex_report.py`](phenomenology/hydrogen_vortex_report.py)
> *Hydrogen 2p vortex identification report (claim 26, ADR-017).*

**Size**: `123` lines (`4305` bytes)

**Functions:**
- `def run_hydrogen_vortex_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/length_scan_report.py`](phenomenology/length_scan_report.py)
> *Box-length scan of the pure-FN N=64 candidate (ADR-007, claim 14b).*

**Size**: `183` lines (`6446` bytes)

**Functions:**
- `def run_length_scan(write)`
- `def render_markdown(result)`

---

### [`phenomenology/lv_bounds.py`](phenomenology/lv_bounds.py)
> *Residual anisotropy and Lorentz-violation bounds (claims 11-12).*

**Size**: `105` lines (`3912` bytes)

**Functions:**
- `def run_lorentz_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/madelung_report.py`](phenomenology/madelung_report.py)
> *Madelung fluid dictionary report (claim 20, ADR-011).*

**Size**: `160` lines (`5069` bytes)

**Functions:**
- `def _grid()`
- `def run_madelung_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/nelson_report.py`](phenomenology/nelson_report.py)
> *Nelson stochastic kinematics report (claim 21, ADR-012).*

**Size**: `136` lines (`4391` bytes)

**Functions:**
- `def run_nelson_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/neutrinos.py`](phenomenology/neutrinos.py)
> *Neutrino Koide ratio scan.*

**Size**: `242` lines (`8651` bytes)

**Classes:**
- `class NuFitParameters`

**Functions:**
- `def load_parameters(path)`
- `def masses_normal(m_light, dm21_sq, dm31_sq_no)` - *Return (m1, m2, m3) for normal ordering, m_light = m1.*
- `def masses_inverted(m_light, dm21_sq, dm31_sq_io)` - *Return (m1, m2, m3) for inverted ordering, m_light = m3.*
- `def q_nu(masses)`
- `def scan_ordering(ordering, params, m_light_max, points)`
- `def corner_max(ordering, params)` - *Maximum Q at m_light = 0 over the 3-sigma corners of the splittings.*
- `def run_neutrino_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/pmns.py`](phenomenology/pmns.py)
> *What a minimal circulant lepton sector predicts for PMNS mixing.*

**Size**: `143` lines (`5592` bytes)

**Functions:**
- `def dft_matrix()`
- `def angles_from_pmns(U)` - *Standard three-neutrino angles in degrees from |U|.*
- `def run_pmns_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/predictions.py`](phenomenology/predictions.py)
> *Search for a genuinely new prediction beyond Q=2/3 (claim 16).*

**Size**: `112` lines (`3965` bytes)

**Functions:**
- `def run_prediction_audit(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/production_report.py`](phenomenology/production_report.py)
> *Production claim-15 report: resolved Hopf charge and Z3 order.*

**Size**: `218` lines (`7903` bytes)

**Functions:**
- `def run_production_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/provenance.py`](phenomenology/provenance.py)
> *Small reproducibility helpers: git state, versions, config hashes.*

**Size**: `80` lines (`2510` bytes)

**Functions:**
- `def git_state()`
- `def package_versions()`
- `def config_hash(payload)`
- `def metadata(config)`
- `def write_text_atomic(path, text)` - *Write a generated file atomically (temp file + fsync + replace).*
- `def write_json(path, payload)`

---

### [`phenomenology/purefn_stability_report.py`](phenomenology/purefn_stability_report.py)
> *Pure-Faddeev-Niemi N=64 candidate Hessian (ADR-005, claims 14b/14d).*

**Size**: `156` lines (`5252` bytes)

**Functions:**
- `def run_purefn_check(write)`
- `def render_markdown(result)`

---

### [`phenomenology/quarks.py`](phenomenology/quarks.py)
> *Quark Koide ratios from the full-SM MS-bar masses of Huang & Zhou (2021).*

**Size**: `154` lines (`5293` bytes)

**Classes:**
- `class QuarkRow`

**Functions:**
- `def load_rows(path)`
- `def q_up(row)`
- `def q_down(row)`
- `def run_quark_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/rg.py`](phenomenology/rg.py)
> *One-loop Standard-Model running of charged-lepton Yukawa couplings.*

**Size**: `495` lines (`17563` bytes)

**Classes:**
- `class InitialState`
  - Method `.vector()`

**Functions:**
- `def yukawa_from_mass(mass_GeV, v_GeV)`
- `def initial_state(masses_GeV, m_top_GeV, m_bottom_GeV)`
- `def _split(state)` - *Return y_e, y_mu, y_tau, y_t, y_b, g1, g2, g3 from log-state.*
- `def rhs(_t, state)` - *One-loop SM RHS for log(yukawas) and log(gauge couplings).*
- `def _rhs_ratio_only(_t, state)` - *RHS with only the generation-dependent (3/2) y_k^2 term.*
- `def run_system(initial, scale_max, rtol, atol)`
- `def run_ratio_only(initial, scale_max)`
- `def observables_from_yukawas(y)` - *Q, r, delta and masses from the three positive Yukawa eigenvalues.*
- `def convert_pole_to_msbar_explicit(m_pole_GeV, mu_GeV, alpha_mu)` - *Published one-pass QED conversion used by Xing & Zhang (2006).*
- `def convert_pole_to_msbar(m_pole_GeV, mu_GeV, alpha_mu)` - *Self-consistent one-loop QED pole-to-MSbar conversion by fixed point.*
- `def _literature_check(sol)`
- `def run_rg(write)`
- `def render_markdown(payload)`
- `def main()`

---

### [`phenomenology/rotor_potential_report.py`](phenomenology/rotor_potential_report.py)
> *Real Z3 rotor potential scan report (claim 19, ADR-010).*

**Size**: `77` lines (`2516` bytes)

**Functions:**
- `def run_rotor_potential_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/run_all.py`](phenomenology/run_all.py)
> *Regenerate every result file from scratch.*

**Size**: `526` lines (`24710` bytes)

**Functions:**
- `def rotor_payload()`
- `def render_rotor_markdown(payload)`
- `def extra_summary(delta, fo, derrick)`
- `def negatives_summary(topology, reduction, predictions)`
- `def lorentz_summary(lorentz)`
- `def sim_summary(sim)`
- `def hopf_summary(hopf)`
- `def hopfion_summary(payload)`
- `def hopfion3d_summary(payload)`
- `def hopf_resolution_summary(payload)`
- `def build_summary(fit, rg, rotor, neutrino, quarks, pmns)`
- `def main()`

---

### [`phenomenology/saddle_escape_report.py`](phenomenology/saddle_escape_report.py)
> *Saddle-escape search from the pure-FN N=64 candidate (ADR-006, claim 14b).*

**Size**: `205` lines (`7056` bytes)

**Functions:**
- `def _relax(field, steps, deadline)`
- `def run_escape_check(write)`
- `def render_markdown(result)`

---

### [`phenomenology/sim_report.py`](phenomenology/sim_report.py)
> *Result payload for the 1D simulation baseline.*

**Size**: `162` lines (`6286` bytes)

**Functions:**
- `def run_1d_baseline(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/stability_report.py`](phenomenology/stability_report.py)
> *Report for the validated tangent-space Hessian of the scale candidate.*

**Size**: `87` lines (`3358` bytes)

**Functions:**
- `def run_stability_report(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/temporal_report.py`](phenomenology/temporal_report.py)
> *Temporal-check report for claim 15: N=64 at two timesteps, same physical time.*

**Size**: `159` lines (`5349` bytes)

**Functions:**
- `def _run(steps, dt, deadline)`
- `def run_temporal_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/uv_checks.py`](phenomenology/uv_checks.py)
> *Reproducible UV-track checks: Frank-Oseen modes and Derrick scaling.*

**Size**: `187` lines (`7228` bytes)

**Functions:**
- `def run_frank_oseen_check(write)`
- `def render_frank_oseen(payload)`
- `def run_derrick_check(write)`
- `def render_derrick(payload)`

---

### [`phenomenology/uv_negatives.py`](phenomenology/uv_negatives.py)
> *Result payloads for the topology, reduction, and prediction negatives.*

**Size**: `63` lines (`2201` bytes)

**Functions:**
- `def run_topology_check(write)`
- `def run_reduction_check(write)`

---

### [`phenomenology/vortex_cascade_report.py`](phenomenology/vortex_cascade_report.py)
> *One-parameter vortex hierarchy report (claim 18, ADR-009).*

**Size**: `88` lines (`3170` bytes)

**Functions:**
- `def load_lepton_masses()`
- `def run_cascade_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/vortex_spectrum_report.py`](phenomenology/vortex_spectrum_report.py)
> *Vortex-knot mass spectrum report (claim 17, ADR-008).*

**Size**: `110` lines (`3790` bytes)

**Functions:**
- `def load_lepton_masses()`
- `def run_vortex_spectrum(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/water_analogue_report.py`](phenomenology/water_analogue_report.py)
> *Shallow-water acoustic horizon report (claim 27, ADR-018).*

**Size**: `109` lines (`3749` bytes)

**Functions:**
- `def run_water_analogue_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/water_dispersive_report.py`](phenomenology/water_dispersive_report.py)
> *Dispersive shallow-water horizon report (claim 29, ADR-020).*

**Size**: `108` lines (`3597` bytes)

**Functions:**
- `def run_water_dispersive_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/water_rays_report.py`](phenomenology/water_rays_report.py)
> *Radial ray report at the shallow-water horizon (claim 30, ADR-021).*

**Size**: `159` lines (`5144` bytes)

**Functions:**
- `def run_water_rays_check(write)`
- `def render_markdown(payload)`

---

### [`phenomenology/water_validity_report.py`](phenomenology/water_validity_report.py)
> *Capillary validity report for the shallow-water horizon (claim 28, ADR-019).*

**Size**: `116` lines (`4314` bytes)

**Functions:**
- `def _row(flow_rate)`
- `def run_water_validity_check(write)`
- `def render_markdown(payload)`

---

### [`sims/__init__.py`](sims/__init__.py)
> *Simulation package: validated 1D core and bounded 3D diagnostics.*

**Size**: `1` lines (`72` bytes)

---

### [`sims/convergence.py`](sims/convergence.py)
> *Resolution comparison for the claim-15 production scan (ADR-002).*

**Size**: `218` lines (`8429` bytes)

**Functions:**
- `def _require_same_problem(low, high)`
- `def compare_scans(low, high)` - *Compare a coarse and a fine production payload at the same physical time.*
- `def compare_temporal(coarse, fine)` - *Compare two N=64 payloads at the same physical time but different dt.*
- `def summarize_trend(rows, z3_target, force_tol)` - *Summarise a multi-chunk trend and project the two claim-15 thresholds.*

---

### [`sims/core.py`](sims/core.py)
> *Validated 1D core for the Z3 sine-Gordon sector.*

**Size**: `210` lines (`6823` bytes)

**Functions:**
- `def grid(length, n)`
- `def potential(phi, mu)`
- `def potential_prime(phi, mu)`
- `def potential_second(phi, mu)`
- `def analytic_kink(x, mu, x0)`
- `def kink_tension(mu)`
- `def energy(phi, dx, mu)`
- `def force(phi, dx, mu)` - *- (1/dx) dE/dphi on interior nodes; boundaries are fixed.*
- `def gradient_error(phi, dx, mu, eps)` - *Max relative error between the analytic force and a central difference.*
- `def gradient_flow(phi, dx, mu, dt, steps)` - *Explicit Euler flow dphi/dt = force; boundaries fixed.*
- `def relax_kink(phi, dx, mu, tolerance)` - *Minimise the discrete energy on interior nodes (L-BFGS-B).*
- `def hessian(phi, dx, mu)` - *Hessian of E/dx on interior nodes (Dirichlet boundaries).*
- `def kink_convergence(mu, length, sizes)`
- `def rotor_crosscheck()`

---

### [`sims/core3d.py`](sims/core3d.py)
> *3D JAX pilot: E2 + Skyrme E4 + a Z3 azimuthal potential.*

**Size**: `221` lines (`7762` bytes)

**Functions:**
- `def normalize(field)`
- `def energy_terms(field, dx, mu, K, kappa)`
- `def energy(field, dx, K, kappa, mu)`
- `def z3_order(field)`
- `def relax(field, dx, mu, steps, dt, K, kappa, fix_boundary)`
- `def gradient_check(n, length, mu, eps)`
- `def convergence_scan(n, length, mu, steps_values, dt)` - *Show that the N=32 pilot is not converged in step count.*
- `def pilot_scan(n, length, mu_values, steps, dt)`

---

### [`sims/production.py`](sims/production.py)
> *Production scan for claim 15: E2 + Skyrme E4 + Z3 potential.*

**Size**: `299` lines (`11517` bytes)

**Functions:**
- `def spectral_upsample(field, factor)` - *Band-limited upsampling of a periodic real field by an integer factor.*
- `def field_metrics(field, dx, mu, upsample_factor)` - *Energy, topology and smoothness of one field.*
- `def run_stage(field, dx, mu, steps, dt, deadline)` - *Relax for ``steps`` steps and record the smallest pre-normalization norm.*
- `def run_production(n, length, mu, steps, dt, seed, perturbation, upsample_factor, chunk, max_wall_seconds, initial_field, return_field)` - *Run one deterministic production scan and apply the acceptance criteria.*

---

### [`sims/scale_iteration.py`](sims/scale_iteration.py)
> *Damped scale iteration toward a stationary Faddeev-Niemi Hopfion.*

**Size**: `95` lines (`3482` bytes)

**Functions:**
- `def rescale_field(field, dx, length, scale)`
- `def run_scale_iteration(n, length, iterations, steps, dt)`

---

### [`sims/stability.py`](sims/stability.py)
> *Tangent-space Hessian of the 3D Faddeev-Niemi + Z3 energy.*

**Size**: `381` lines (`13687` bytes)

**Functions:**
- `def tangent_basis(field)` - *Return two orthonormal tangent vectors at every interior site.*
- `def build_candidate(n, length, iterations, steps, dt)` - *Damped scale iteration, returning the snapshot with the smallest |virial|.*
- `def _dense_tangent_hessian(field, dx, e1, e2)` - *Dense Hessian of the energy restricted to the tangent basis (N<=8 only).*
- `def tangent_operator(field, dx, e1, e2)` - *Matrix-free Hessian on the two-dimensional tangent space of each site.*
- `def validate_operator(n, length, steps, dt, k)` - *Compare the matrix-free operator with the dense tangent Hessian.*
- `def hessian_spectrum(field, dx, k, e1, e2)` - *Low-lying eigenvalues of the tangent-space Hessian.*
- `def classify_escape(attempts, note)` - *Summarise a saddle-escape search without promoting beyond one grid.*
- `def hessian_modes(field, dx, k, e1, e2)` - *Lowest tangent eigenpairs, with each eigenvector mapped back to a field.*
- `def normalize_mode(mode)` - *Scale a tangent mode to unit RMS over the interior.*
- `def perturb_along_mode(field, mode, amplitude)` - *Add a normalized tangent mode and renormalize every site.*
- `def classify_candidate(validation, spectrum)` - *Classify one candidate's Hessian without overstating the result.*
- `def run_stability_check(n, length, iterations, steps, dt, k, validate_n)` - *Build the scale-stationary candidate and compute its tangent Hessian.*

---

### [`tests/__init__.py`](tests/__init__.py)
**Size**: `0` lines (`0` bytes)

---

### [`tests/test_circulant.py`](tests/test_circulant.py)
> *Tests for claims 1-2: circulant eigenvalues, Q identity, Hermiticity, DFT vectors.*

**Size**: `163` lines (`5124` bytes)

**Functions:**
- `def test_symbolic_eigenvalues()`
- `def test_hermitian_not_symmetric()`
- `def test_dft_eigenvectors()`
- `def test_q_identity_symbolic()`
- `def test_q_identity_numeric()`
- `def test_extract_roundtrip()`
- `def test_koide_condition()`
- `def test_sign_caveat()`
- `def test_bound_half_is_false()`
- `def test_fit_masses_exact()`
- `def test_koide_tau_prediction()`
- `def test_property_q_identity(a, c, delta)`
- `def test_property_fit(m1, m2, m3)`

---

### [`tests/test_coherent_fluid.py`](tests/test_coherent_fluid.py)
> *Moving-center Gaussian reduction tests (claim 23).*

**Size**: `64` lines (`1887` bytes)

**Functions:**
- `def test_center_follows_the_classical_oscillator()`
- `def test_width_stays_at_the_fixed_point()`
- `def test_displaced_width_oscillates_with_energy_conservation()`
- `def test_invalid_inputs_are_rejected()`

---

### [`tests/test_convergence.py`](tests/test_convergence.py)
> *Resolution-comparison tests for claim 15 (ADR-002).*

**Size**: `216` lines (`6911` bytes)

**Functions:**
- `def _scan(n, *, h, h_up, z3, force, bond)`
- `def test_compare_scans_accepts_a_converging_pair()`
- `def test_compare_scans_flags_a_native_H_regression()`
- `def test_compare_scans_flags_force_or_bond_regression()`
- `def test_compare_scans_requires_the_same_physical_problem()`
- `def test_compare_scans_never_promotes_the_claim()`
- `def _temporal(n, steps, dt, *, h, h_up, z3, force, bond)`
- `def test_compare_temporal_accepts_a_converged_pair()`
- `def test_compare_temporal_flags_a_Z3_difference()`
- `def test_compare_temporal_requires_the_same_physical_time()`
- `def test_compare_temporal_requires_the_same_resolution()`
- `def test_compare_temporal_never_promotes_the_claim()`
- `def _trend_row(steps, t, z3, force)`
- `def test_summarize_trend_projects_the_thresholds()`
- `def test_summarize_trend_handles_saturation_and_no_projection()`
- `def test_summarize_trend_requires_two_rows()`

---

### [`tests/test_delta_search.py`](tests/test_delta_search.py)
> *Tests for the delta closed-form search (claim 9).*

**Size**: `31` lines (`1000` bytes)

**Functions:**
- `def test_two_ninths_is_in_the_prior()`
- `def test_all_candidates_are_in_range_and_unique()`
- `def test_search_finds_two_ninths_and_does_not_promote()`

---

### [`tests/test_derrick.py`](tests/test_derrick.py)
> *Derrick scaling tests for claim 14.*

**Size**: `86` lines (`2596` bytes)

**Functions:**
- `def test_exponents_in_three_dimensions()`
- `def test_virial_relation()`
- `def test_scaling_energy_derivative_matches_virial()`
- `def test_zero_potential_minimizer()`
- `def test_positive_potential_minimizer_is_stable()`
- `def test_no_skyrme_means_no_scaling_minimum()`
- `def test_negative_skyrme_is_unbounded()`
- `def test_positive_skyrme_region_and_no_existence_claim()`
- `def test_symbolic_virial_identity()`

---

### [`tests/test_excited_fluid.py`](tests/test_excited_fluid.py)
> *First-excited-state node tests (claim 24).*

**Size**: `69` lines (`1834` bytes)

**Functions:**
- `def _grid(n, length)`
- `def test_density_is_normalized()`
- `def test_regular_quantum_pressure()`
- `def test_phase_slip_and_gradient()`
- `def test_phase_gradient_scales_as_inverse_dx()`
- `def test_invalid_inputs_are_rejected()`

---

### [`tests/test_fit.py`](tests/test_fit.py)
> *Regression tests for the PDG pole-mass fit.*

**Size**: `59` lines (`2198` bytes)

**Functions:**
- `def test_pdg_input_values()`
- `def test_fit_regression_numbers()`
- `def test_fit_is_exact_by_construction()`
- `def test_monte_carlo_is_reproducible()`
- `def test_two_thirds_is_inside_one_sigma_of_pole_fit()`
- `def test_tau_prediction_consistency()`

---

### [`tests/test_frank_oseen.py`](tests/test_frank_oseen.py)
> *Frank-Oseen mode and dispersion checks (claim 10).*

**Size**: `62` lines (`1945` bytes)

**Functions:**
- `def test_symbolic_hessian_matches_numeric_matrix()`
- `def test_one_constant_is_isotropic()`
- `def test_axis_dispersions_for_unequal_constants()`
- `def test_anisotropy_zero_for_equal_constants_positive_for_unequal()`
- `def test_mode_count_and_no_photon()`
- `def test_equal_constant_dispersion_formula()`

---

### [`tests/test_gaussian_fluid.py`](tests/test_gaussian_fluid.py)
> *Gaussian Madelung reduction tests (claim 22).*

**Size**: `73` lines (`2054` bytes)

**Functions:**
- `def test_fixed_point_is_the_quantum_width()`
- `def test_effective_potential_has_a_minimum_at_the_quantum_width()`
- `def test_ermakov_oscillates_around_the_fixed_point()`
- `def test_energy_is_conserved()`
- `def test_width_scaling()`
- `def test_invalid_inputs_are_rejected()`

---

### [`tests/test_hopf.py`](tests/test_hopf.py)
> *Hopf-charge estimator tests (claim 15, baseline 4).*

**Size**: `56` lines (`1968` bytes)

**Functions:**
- `def test_hopf_map_is_unit_and_boundary_condition()`
- `def test_preimage_linking_converges_to_one()`
- `def test_reflection_flips_sign_and_rotation_shear_preserve()`
- `def test_grid_integral_matches_known_answer()`

---

### [`tests/test_hopf_resolution.py`](tests/test_hopf_resolution.py)
> *Hopf grid-resolution tests.*

**Size**: `21` lines (`861` bytes)

**Functions:**
- `def test_resolution_converges_and_flags_coarse_grids()`

---

### [`tests/test_hopfion.py`](tests/test_hopfion.py)
> *Static finite-energy H=1 configuration tests (claim 14b).*

**Size**: `41` lines (`1453` bytes)

**Functions:**
- `def test_energy_is_positive_and_h1()`
- `def test_scaling_virial_and_minimum()`
- `def test_convergence_and_no_stability_claim()`

---

### [`tests/test_hopfion3d.py`](tests/test_hopfion3d.py)
> *3D E2+E4+Z3 pilot tests (claim 15).*

**Size**: `48` lines (`1428` bytes)

**Functions:**
- `def test_gradient_matches_finite_difference()`
- `def test_z3_order_limits()`
- `def test_energy_terms_are_positive()`
- `def test_small_pilot_no_coexistence()`

---

### [`tests/test_hydrogen_fluid.py`](tests/test_hydrogen_fluid.py)
> *Hydrogen 1s Madelung reduction tests (claim 25).*

**Size**: `56` lines (`1467` bytes)

**Functions:**
- `def test_bohr_radius_atomic_units()`
- `def test_density_is_normalized()`
- `def test_force_balance_vanishes_at_the_bohr_radius()`
- `def test_force_balance_is_nonzero_at_the_wrong_radius()`
- `def test_si_values()`
- `def test_invalid_inputs_are_rejected()`

---

### [`tests/test_hydrogen_vortex.py`](tests/test_hydrogen_vortex.py)
> *Hydrogen 2p vortex tests (claim 26).*

**Size**: `48` lines (`1367` bytes)

**Functions:**
- `def test_radial_probability_is_normalized()`
- `def test_most_probable_radius_is_four_bohr_radii()`
- `def test_circulation_is_quantized()`
- `def test_azimuthal_velocity_and_angular_momentum()`
- `def test_invalid_inputs_are_rejected()`

---

### [`tests/test_lorentz.py`](tests/test_lorentz.py)
> *Residual anisotropy and LV-bound tests (claims 11-12).*

**Size**: `57` lines (`1999` bytes)

**Functions:**
- `def test_equal_K_is_isotropic()`
- `def test_unequal_K_has_order_one_anisotropy()`
- `def test_bounds_are_marked_not_directly_applicable()`
- `def test_conditional_bound_excludes_order_one_anisotropy()`
- `def test_verdict_downgrades_claims_11_and_12()`

---

### [`tests/test_madelung.py`](tests/test_madelung.py)
> *Madelung fluid dictionary tests (claim 20).*

**Size**: `95` lines (`2779` bytes)

**Functions:**
- `def _grid(n, length)`
- `def test_free_gaussian_density_matches_exact()`
- `def test_coherent_state_density_matches_exact()`
- `def test_quantum_potential_of_a_gaussian()`
- `def test_exact_gaussian_satisfies_the_euler_equation()`
- `def test_fluid_residuals_are_small()`
- `def test_invalid_inputs_are_rejected()`

---

### [`tests/test_nelson.py`](tests/test_nelson.py)
> *Nelson stochastic kinematics tests (claim 21).*

**Size**: `61` lines (`1770` bytes)

**Functions:**
- `def test_ground_state_width()`
- `def test_ou_stationary_variance()`
- `def test_ou_histogram_matches_ground_state()`
- `def test_free_packet_drift_at_zero()`
- `def test_free_packet_histogram_matches_exact()`
- `def test_invalid_inputs_are_rejected()`

---

### [`tests/test_neutrinos.py`](tests/test_neutrinos.py)
> *Neutrino Q_nu scan tests (claim 7a).*

**Size**: `65` lines (`2049` bytes)

**Functions:**
- `def test_normal_ordering_mass_formula()`
- `def test_inverted_ordering_mass_formula()`
- `def test_q_nu_regression_values()`
- `def test_scan_never_reaches_two_thirds()`
- `def test_three_sigma_corners_stay_below_two_thirds()`
- `def test_q_nu_matches_generic_mass_ratio()`

---

### [`tests/test_pmns.py`](tests/test_pmns.py)
> *Minimal circulant PMNS branches (claim 7b).*

**Size**: `32` lines (`1168` bytes)

**Functions:**
- `def test_identity_has_no_mixing()`
- `def test_dft_is_trimaximal()`
- `def test_both_branches_excluded_and_claim_ill_posed()`

---

### [`tests/test_predictions.py`](tests/test_predictions.py)
> *New-prediction audit tests (claim 16).*

**Size**: `20` lines (`680` bytes)

**Functions:**
- `def test_candidate_table_is_complete()`
- `def test_no_new_prediction_is_found()`

---

### [`tests/test_production.py`](tests/test_production.py)
> *Production-scan smoke tests (claim 15).*

**Size**: `107` lines (`3173` bytes)

**Functions:**
- `def test_spectral_upsample_preserves_analytic_charge()`
- `def test_run_stage_matches_relax()`
- `def test_production_verdict_shape()`
- `def test_production_enforces_wall_clock_limit()`
- `def test_run_production_resumes_from_a_field_exactly()`
- `def test_run_production_validates_initial_field_and_hides_it_by_default()`

---

### [`tests/test_purefn_stability.py`](tests/test_purefn_stability.py)
> *Pure-FN candidate Hessian classification tests (claims 14b/14d).*

**Size**: `36` lines (`1199` bytes)

**Functions:**
- `def test_classify_candidate_accepts_a_local_minimum()`
- `def test_classify_candidate_rejects_a_saddle()`
- `def test_classify_candidate_requires_a_validated_operator()`

---

### [`tests/test_quarks.py`](tests/test_quarks.py)
> *Quark Koide ratios from full-SM MS-bar masses (claim 8).*

**Size**: `38` lines (`1174` bytes)

**Functions:**
- `def test_data_has_six_scales()`
- `def test_mz_regression_values()`
- `def test_direct_extension_falsified()`
- `def test_ranges_are_scale_dependent()`

---

### [`tests/test_reduction.py`](tests/test_reduction.py)
> *Dimensional-reduction and moment-of-inertia tests (claim 6b).*

**Size**: `52` lines (`1635` bytes)

**Functions:**
- `def test_wall_tension_and_bound()`
- `def test_moment_convergence_criterion()`
- `def test_numeric_moment_log_and_power()`
- `def test_verdict_downgrades_exact_reduction()`

---

### [`tests/test_rg.py`](tests/test_rg.py)
> *Regression tests and cross-checks for the one-loop SM lepton running.*

**Size**: `127` lines (`4749` bytes)

**Functions:**
- `def payload()`
- `def test_rhs_returns_log_derivatives()`
- `def test_initial_state_sanity()`
- `def test_published_conversion_reproduced()`
- `def test_q_of_converted_pole_masses_matches_literature(payload)`
- `def test_msbar_q_at_mz_is_not_two_thirds(payload)`
- `def test_exact_invariance_is_false(payload)`
- `def test_drift_is_small(payload)`
- `def test_shape_only_approximation_is_close_but_not_exact(payload)`
- `def test_high_scale_literature_agreement(payload)`
- `def test_observables_use_sqrt_masses()`

---

### [`tests/test_rotor.py`](tests/test_rotor.py)
> *Rotor, sector-degeneracy, and domain-wall tests (claim 5).*

**Size**: `117` lines (`4427` bytes)

**Functions:**
- `def test_hamiltonian_is_symmetric_and_banded()`
- `def test_sector_degeneracy_is_exact()`
- `def test_free_rotor_limit()`
- `def test_truncation_convergence()`
- `def test_fourier_and_grid_agree_in_continuum()`
- `def test_both_map_signs_give_two_distinct_roots()`
- `def test_naive_map_cannot_give_three_distinct_masses()`
- `def test_domain_wall_tension_analytic_and_quadrature()`
- `def test_energy_scales_with_mu()`

---

### [`tests/test_rotor_potential.py`](tests/test_rotor_potential.py)
> *Real Z3 rotor potential tests (claim 19).*

**Size**: `46` lines (`1389` bytes)

**Functions:**
- `def test_zero_flux_real_potential_is_degenerate()`
- `def test_fractional_flux_splits_the_sectors()`
- `def test_scan_is_falsified()`
- `def test_invalid_inputs_are_rejected()`
- `def test_second_harmonic_does_not_force_a_large_spread()`

---

### [`tests/test_saddle_escape.py`](tests/test_saddle_escape.py)
> *Saddle-escape mode tests (claim 14b/ADR-006).*

**Size**: `69` lines (`2263` bytes)

**Functions:**
- `def test_hessian_modes_returns_a_tangent_field()`
- `def test_perturb_along_mode_preserves_norm_and_boundary()`
- `def test_perturb_along_mode_rejects_invalid_inputs()`
- `def test_classify_escape_finds_a_local_minimum()`
- `def test_classify_escape_keeps_the_claim_unverified()`

---

### [`tests/test_scale_iteration.py`](tests/test_scale_iteration.py)
> *Scale-iteration candidate tests.*

**Size**: `31` lines (`1143` bytes)

**Functions:**
- `def test_rescale_preserves_unit_norm_and_boundary()`
- `def test_smoke_scale_iteration()`

---

### [`tests/test_sim_core.py`](tests/test_sim_core.py)
> *1D simulation-core baselines (claim 15, validation only).*

**Size**: `87` lines (`2704` bytes)

**Functions:**
- `def test_analytic_kink_limits_and_tension()`
- `def test_kink_energy_converges_quadratically()`
- `def test_gradient_matches_finite_difference()`
- `def test_gradient_flow_decreases_energy()`
- `def test_relaxed_hessian_is_symmetric_and_stable()`
- `def test_rotor_fourier_and_grid_agree()`
- `def test_report_makes_no_production_claim()`

---

### [`tests/test_stability.py`](tests/test_stability.py)
> *Tangent-space Hessian tests (claim 14d).*

**Size**: `46` lines (`1579` bytes)

**Functions:**
- `def test_tangent_basis_is_orthonormal()`
- `def test_dense_validation_passes()`
- `def test_spectrum_uses_tangent_dimension()`

---

### [`tests/test_topology.py`](tests/test_topology.py)
> *Nematic topology tests (claim 13).*

**Size**: `33` lines (`1148` bytes)

**Functions:**
- `def test_homotopy_groups_are_recorded()`
- `def test_no_z3_factor_in_rp2()`
- `def test_standard_extensions_do_not_derive_z3()`
- `def test_verdict_and_json_shape()`

---

### [`tests/test_vortex_cascade.py`](tests/test_vortex_cascade.py)
> *One-parameter vortex hierarchy tests (claim 18).*

**Size**: `62` lines (`1845` bytes)

**Functions:**
- `def test_power_law_koide_root()`
- `def test_geometric_cascade_is_falsified()`
- `def test_power_law_muon_fit_is_falsified()`
- `def test_all_one_parameter_hierarchies_fail()`
- `def test_invalid_inputs_are_rejected()`

---

### [`tests/test_vortex_particles.py`](tests/test_vortex_particles.py)
> *Vortex-knot mass hypothesis tests (claim 17).*

**Size**: `50` lines (`1417` bytes)

**Functions:**
- `def test_torus_knot_shapes_and_closure()`
- `def test_koide_q_known_values()`
- `def test_simple_vortex_spectrum_is_falsified()`
- `def test_invalid_inputs_are_rejected()`

---

### [`tests/test_water_analogue.py`](tests/test_water_analogue.py)
> *Shallow-water acoustic horizon tests (claim 27).*

**Size**: `66` lines (`1852` bytes)

**Functions:**
- `def test_sound_speed()`
- `def test_horizon_satisfies_v_equals_c()`
- `def test_metric_sign_changes_across_the_horizon()`
- `def test_surface_gravity_and_temperature()`
- `def test_lab_scales()`
- `def test_invalid_inputs_are_rejected()`

---

### [`tests/test_water_dispersive.py`](tests/test_water_dispersive.py)
> *Dispersive shallow-water horizon tests (claim 29).*

**Size**: `57` lines (`1517` bytes)

**Functions:**
- `def test_low_k_dispersion_and_group_velocity()`
- `def test_low_frequency_horizon_approaches_the_long_wavelength_value()`
- `def test_high_frequency_horizon_is_shifted_outward()`
- `def test_capillary_frequency()`
- `def test_horizon_wavenumber_solves_the_dispersion_condition()`
- `def test_invalid_inputs_are_rejected()`

---

### [`tests/test_water_rays.py`](tests/test_water_rays.py)
> *Radial ray tests at the water horizon (claim 30).*

**Size**: `69` lines (`2022` bytes)

**Functions:**
- `def test_outgoing_speed_changes_sign_at_the_horizon()`
- `def test_analytic_ray_time()`
- `def test_numerical_ray_time_matches_analytic()`
- `def test_peeling_rate_is_the_surface_gravity()`
- `def test_invalid_inputs_are_rejected()`

---

### [`tests/test_water_validity.py`](tests/test_water_validity.py)
> *Capillary validity tests for the shallow-water horizon (claim 28).*

**Size**: `46` lines (`1296` bytes)

**Functions:**
- `def test_capillary_length_of_water()`
- `def test_lab_example_is_capillary_dominated()`
- `def test_larger_flow_reaches_the_long_wavelength_regime()`
- `def test_minimum_flow_for_ten_capillary_lengths()`
- `def test_invalid_inputs_are_rejected()`

---

### [`theory/__init__.py`](theory/__init__.py)
> *Exact theory utilities for the Z3/Koide core.*

**Size**: `1` lines (`52` bytes)

---

### [`theory/circulant.py`](theory/circulant.py)
> *Exact algebra of the Hermitian 3x3 circulant mass matrix.*

**Size**: `172` lines (`5563` bytes)

**Functions:**
- `def circulant_matrix(a, c, delta)` - *Return the Hermitian circulant M = circ(a, c e^{i delta}, c e^{-i delta}).*
- `def eigenvalues(a, c, delta)` - *Signed eigenvalues of the Hermitian circulant, in the k = 0,1,2 order.*
- `def dft_vectors()` - *Columns are the three DFT eigenvectors v_k, each normalised to unit norm.*
- `def q_signed(x)` - *Q for signed square roots: sum(x^2) / (sum(x))^2.*
- `def q_masses(m)` - *Physical Koide Q for positive masses.*
- `def q_physical_from_roots(x)` - *Physical Q when x are signed square roots: sum(x^2) / (sum(|x|))^2.*
- `def q_from_ac(a, c)` - *The algebraic identity Q = 1/3 + 2 c^2 / (3 a^2).*
- `def extract_ac_delta(x)` - *Inverse DFT: return (a, c, delta) from three signed roots x_k.*
- `def fit_masses(m)` - *Fit (a, c, delta) to positive masses and return Q, r, delta, residuals.*
- `def koide_tau_prediction(m_e, m_mu)` - *Solve Q=2/3 for m_tau given m_e and m_mu.*
- `def _sympy_symbols()`
- `def circulant_symbolic()` - *Return (M, a, c, delta) as sympy objects.*
- `def eigenvalues_symbolic()`
- `def q_identity_symbolic()` - *Return the expression Q_signed - (1/3 + 2c^2/(3a^2)); simplifies to 0.*

---

### [`theory/derrick.py`](theory/derrick.py)
> *Derrick scaling for the exact Frank-Oseen + Skyrme + Z3 energy.*

**Size**: `128` lines (`4684` bytes)

**Functions:**
- `def exponents(d)`
- `def scaling_energy(lambda_, e2, e4, potential, d)`
- `def virial(e2, e4, potential, d)`
- `def second_derivative(e2, e4, potential, d)`
- `def stationary_scale_squared(e2, e4, potential, d)` - *Return lambda^2 at the stationary point for d=3, or None if none exists.*
- `def scaling_minimum(e2, e4, potential)`
- `def stable_region(k, kappa, mu)` - *Necessary conditions for a stable scaling soliton in d=3.*
- `def symbolic_virial()` - *Verify d/dlambda E(lambda) at lambda=1 equals the virial expression.*

---

### [`theory/excited_fluid.py`](theory/excited_fluid.py)
> *First excited state in the Madelung fluid (claim 24, ADR-015).*

**Size**: `79` lines (`2609` bytes)

**Functions:**
- `def _sigma(omega, hbar, mass)`
- `def excited_density(x, omega, hbar, mass)`
- `def excited_wavefunction(x, omega, hbar, mass)`
- `def excited_quantum_pressure_regular(x, omega, hbar, mass)` - *Regular part of Q away from the node; at x=0 the limit is returned.*
- `def phase_slip(psi)`
- `def phase_gradient_max(psi, dx)`
- `def scaling_slope(dx_values, maxima)`

---

### [`theory/frank_oseen.py`](theory/frank_oseen.py)
> *Linearised Frank-Oseen director modes.*

**Size**: `143` lines (`4577` bytes)

**Functions:**
- `def elastic_matrix(kx, ky, kz, k1, k2, k3)` - *2x2 elastic matrix for the two transverse director components.*
- `def dispersion(kx, ky, kz, k1, k2, k3, rho)` - *omega^2 for the two transverse modes; rho is the effective inertia.*
- `def equal_constant_dispersion(k, K, rho)` - *omega for K1=K2=K3=K: omega = sqrt(K/rho) |k|.*
- `def anisotropy(direction, k1, k2, k3, rho)` - *Fractional split of the two omega^2 values at unit |k|.*
- `def modes()` - *Mode counting and the photon question.*
- `def symbolic_elastic_matrix()` - *Sympy Hessian of the quadratic form; equals the numeric matrix.*
- `def symbolic_matches_numeric()`

---

### [`theory/gaussian_fluid.py`](theory/gaussian_fluid.py)
> *Gaussian reduction of the Madelung fluid (claim 22, ADR-013).*

**Size**: `204` lines (`5881` bytes)

**Functions:**
- `def quantum_width(omega, hbar, mass)`
- `def effective_potential(sigma, omega, hbar, mass)`
- `def energy(sigma, sigma_dot, omega, hbar, mass)`
- `def ermakov_rhs(sigma, sigma_dot, omega, hbar, mass)`
- `def simulate(sigma0, sigma_dot0, omega, duration, hbar, mass, dt)`
- `def coherent_rhs(state, omega, hbar, mass)` - *RHS for (x0, v0, sigma, sigma_dot) of the displaced Gaussian.*
- `def coherent_energy(state, omega, hbar, mass)`
- `def simulate_coherent(x0, v0, sigma0, sigma_dot0, omega, duration, hbar, mass, dt)`

---

### [`theory/hopf.py`](theory/hopf.py)
> *Hopf-charge estimators for the analytic H=1 map.*

**Size**: `287` lines (`10923` bytes)

**Functions:**
- `def hopf_map(x, y, z)`
- `def _target_of_quaternion(q)`
- `def _jacobian(q)`
- `def fiber_curve(base, samples, dphi)` - *Trace one fiber on S^3 by integrating the null space of dn, then stereographically project.*
- `def linking_number(curve_a, curve_b)` - *Gauss linking integral for two closed curves.*
- `def _bases()`
- `def preimage_charge(samples)`
- `def target_pair_charge(samples)`
- `def reflection_charge(samples)`
- `def rotation_charge(samples)`
- `def shear_charge(samples)`
- `def compactified_field(n, length, reflect)` - *Analytic map with a smooth vacuum window at the box boundary.*
- `def spectral_derivatives(field, dx)` - *Return [d_x field, d_y field, d_z field] with spectral derivatives.*
- `def chern_simons_charge(field, dx)` - *Normalized Chern-Simons charge for a compactified unit-vector field.*
- `def grid_charge(n, length, reflect)` - *Normalized grid Chern-Simons integral for the analytic H=1 map.*
- `def resolution_study(sizes, length)` - *Grid Chern-Simons charge vs resolution for the analytic H=1 map.*
- `def validate()`

---

### [`theory/hopfion.py`](theory/hopfion.py)
> *Finite-energy H=1 configuration for the Faddeev-Niemi energy.*

**Size**: `79` lines (`2916` bytes)

**Functions:**
- `def energies(n, length, K, kappa)`
- `def scaling_study(sizes, length)`

---

### [`theory/hydrogen_fluid.py`](theory/hydrogen_fluid.py)
> *Hydrogen 1s Madelung reduction (claim 25, ADR-016).*

**Size**: `95` lines (`2630` bytes)

**Functions:**
- `def bohr_radius(hbar, mass, coupling)`
- `def ground_energy(radius, hbar, mass)`
- `def hydrogen_density(r, radius)`
- `def quantum_pressure(r, radius, hbar, mass)`
- `def coulomb_potential(r, coupling)`
- `def force_balance(r, radius, hbar, mass, coupling)` - *V + Q - mu; zero when radius is the Bohr radius.*
- `def si_values()`

---

### [`theory/hydrogen_vortex.py`](theory/hydrogen_vortex.py)
> *Hydrogen 2p vortex identification (claim 26, ADR-017).*

**Size**: `79` lines (`2246` bytes)

**Functions:**
- `def radial_2p(r, radius)` - *Hydrogen 2p radial function R_21(r), normalized as int r^2 R^2 dr = 1.*
- `def radial_probability(r, radius)`
- `def most_probable_radius(radius)`
- `def azimuthal_velocity(r, theta, m, hbar, mass)`
- `def circulation(m, hbar, mass)`
- `def loop_circulation(r, theta, m, hbar, mass, samples)`
- `def angular_momentum(m, hbar)`

---

### [`theory/lorentz.py`](theory/lorentz.py)
> *Linearised director dispersion and the emergent-metric question (claims 11-12).*

**Size**: `185` lines (`6318` bytes)

**Functions:**
- `def elastic_matrix(k, k1, k2, k3)`
- `def speeds(direction, k1, k2, k3, rho)`
- `def residual_anisotropy(k1, k2, k3, rho, n_samples, seed)` - *Sample directions on the sphere and return the max fractional speed split.*
- `def axis_speeds(k1, k2, k3, rho)`
- `def verdict(k1, k2, k3)`
- `def conditional_bound(k1, k2, k3)`

---

### [`theory/madelung.py`](theory/madelung.py)
> *Madelung fluid dictionary for the Schrodinger equation (claim 20, ADR-011).*

**Size**: `194` lines (`6131` bytes)

**Functions:**
- `def _wave_numbers(n, dx)`
- `def _derivative(values, k)`
- `def split_step(psi, potential, dt, steps, dx, hbar, mass)` - *Strang split-step integration of the equivalent Schrodinger equation.*
- `def fluid_fields(psi, dx, hbar, mass)` - *Return density, velocity and quantum pressure of a wavefunction.*
- `def free_gaussian_density(x, t, sigma, hbar, mass)`
- `def coherent_state_density(x, t, x0, omega, hbar, mass)`
- `def continuity_residual(psi0, psi1, dt, dx, hbar, mass)`
- `def free_gaussian_fields(x, t, sigma, hbar, mass)` - *Exact density, velocity and quantum pressure of a free Gaussian.*
- `def euler_residual_fields(rho0, v0, q0, rho1, v1, q1, dt, dx, potential, mass, density_floor)` - *Relative Euler residual on the core where the density is resolved.*

---

### [`theory/nelson.py`](theory/nelson.py)
> *Nelson stochastic kinematics for the quantum fluid (claim 21, ADR-012).*

**Size**: `122` lines (`4032` bytes)

**Functions:**
- `def ground_state_width(omega, hbar, mass)`
- `def ground_state_density(x, omega, hbar, mass)`
- `def osmotic_velocity(rho, dx, hbar, mass)`
- `def simulate_ou(particles, omega, hbar, mass, dt, steps, seed)` - *Euler-Maruyama for dX = -omega X dt + sqrt(hbar/m) dW.*
- `def free_packet_drift(x, t, sigma, hbar, mass)` - *Forward drift b_+ = v + D d_x ln(rho) for the free Gaussian.*
- `def simulate_free_packet(particles, t, sigma, hbar, mass, steps, seed)`
- `def histogram_density(samples, bins, span)`
- `def l1_error(density, exact, width)`

---

### [`theory/reduction.py`](theory/reduction.py)
> *Dimensional reduction and the effective moment of inertia (claim 6b).*

**Size**: `118` lines (`4335` bytes)

**Functions:**
- `def wall_tension(mu)`
- `def wall_lower_bound_2d(mu, length_y)`
- `def moment_power(alpha)` - *Power of r in the 2D moment integrand for rho ~ r^-alpha.*
- `def moment_finite(alpha)`
- `def moment_log_divergent(alpha)`
- `def moment_numeric(alpha, r_min, r_max)` - *2*pi int rho r^3 dr for rho = r^-alpha on [r_min, r_max].*
- `def moment_profile(profile, r_max, points)` - *Numerical 2*pi int_0^rmax profile(r) r^3 dr; profile must be regular at 0.*
- `def verdict(stated_alpha)`
- `def render_markdown(payload)`

---

### [`theory/rotor.py`](theory/rotor.py)
> *Quantum rotor H = L^2/(2I) + mu (1 - cos 3 phi) and its spectra.*

**Size**: `175` lines (`6058` bytes)

**Classes:**
- `class RotorSpectrum`
  - Method `.lowest(sector, count)`

**Functions:**
- `def sector_matrix(I, mu, sector, n_cutoff)` - *Dense Hamiltonian in the truncated sector n = sector + 3j, |n| <= n_cutoff.*
- `def spectrum(I, mu, n_cutoff)` - *Low-lying levels in each of the three Fourier sectors.*
- `def convergence(I, mu, cutoffs)`
- `def free_rotor_levels(sector, count, I)` - *Exact mu=0 levels n^2/(2I) in sector n = sector + 3j.*
- `def grid_spectrum(I, mu, n_grid, count)` - *Finite-difference cross-check on a periodic grid phi_j = 2 pi j / N.*
- `def mode_map(sector_energies)` - *The prompt's naive map a=(l0+2 l1)/3, c=(l1-l0)/3.*
- `def implied_roots(sector_energies)` - *Roots generated by the circulant map from three sector levels.*
- `def parity_obstruction()` - *State the exact degeneracy argument and return it as data.*
- `def domain_wall_tension(mu)` - *Analytic tension T = (8/3) sqrt(mu) of the 1-cos(3 phi) wall.*
- `def domain_wall_tension_quad(mu)` - *Numerical quadrature of int_0^{2pi/3} sqrt(2 mu (1-cos 3 phi)) dphi.*

---

### [`theory/rotor_potential.py`](theory/rotor_potential.py)
> *Real Z3 rotor potential scan (claim 19, ADR-010).*

**Size**: `120` lines (`4022` bytes)

**Functions:**
- `def sector_matrix(mu, nu, flux, sector, n_cutoff, inertia)`
- `def sector_energies(mu, nu, flux, n_cutoff, inertia)`
- `def scan(mu, nus, fluxes, n_cutoff)`

---

### [`theory/topology.py`](theory/topology.py)
> *Topology of a nematic director and the Z3 question.*

**Size**: `113` lines (`4186` bytes)

**Functions:**
- `def has_z3_factor()` - *True only if one of the listed homotopy groups has a Z3 factor.*
- `def direct_result()`
- `def extensions()`
- `def verdict()`
- `def render_markdown(payload)`

---

### [`theory/vortex_cascade.py`](theory/vortex_cascade.py)
> *One-parameter vortex/eddy hierarchy test (claim 18, ADR-009).*

**Size**: `126` lines (`4406` bytes)

**Functions:**
- `def geometric_spectrum(ratio, generations)`
- `def power_law_spectrum(exponent, generations)`
- `def solve_power_for_koide(target, lower, upper)` - *Bisection for the power p with Koide Q = target.*
- `def _summary(name, observed, predicted, parameter)`
- `def run_hypotheses(observed)`

---

### [`theory/vortex_particles.py`](theory/vortex_particles.py)
> *Thin-filament vortex-knot mass hypothesis (claim 17, ADR-008).*

**Size**: `136` lines (`5174` bytes)

**Functions:**
- `def torus_knot(n, radius, tube, samples)` - *Closed T(2,n) curve and its chord vectors.*
- `def knot_length(points)`
- `def biot_savart_energy(points, core)` - *Regularized thin-filament energy proxy at a fixed core radius.*
- `def koide_q(values)`
- `def mass_hypothesis(observed, knots, core, relative_tolerance, koide_tolerance)` - *Test m proportional to the knot energy against the observed masses.*

---

### [`theory/water_analogue.py`](theory/water_analogue.py)
> *Shallow-water acoustic horizon (claim 27, ADR-018).*

**Size**: `145` lines (`4294` bytes)

**Functions:**
- `def sound_speed(depth, gravity)`
- `def flow_velocity(r, flow_rate, depth, gravity)`
- `def horizon_radius(flow_rate, depth, gravity)`
- `def surface_gravity(flow_rate, depth, gravity)`
- `def hawking_temperature(kappa)`
- `def acoustic_metric(r, flow_rate, depth, gravity)`
- `def lab_scales(depth, flow_rate, gravity)`
- `def capillary_length(gamma, density, gravity)`
- `def capillary_ratio(horizon_radius_value, length)`
- `def long_wavelength_min_flow(depth, gravity, gamma, density, ratio)`
- `def validity(depth, flow_rate, gamma, density, gravity, ratio)`

---

### [`theory/water_dispersive.py`](theory/water_dispersive.py)
> *Dispersive shallow-water horizon (claim 29, ADR-020).*

**Size**: `118` lines (`3466` bytes)

**Functions:**
- `def dispersion(k, depth, gamma, density, gravity)`
- `def group_velocity(k, depth, gamma, density, gravity)`
- `def horizon_wavenumber(omega, depth, gamma, density, gravity, maximum)`
- `def dispersive_horizon(omega, flow_rate, depth, gamma, density, gravity)`
- `def capillary_frequency(depth, gamma, density, gravity)`

---

### [`theory/water_rays.py`](theory/water_rays.py)
> *Radial rays at the shallow-water acoustic horizon (claim 30, ADR-021).*

**Size**: `82` lines (`2545` bytes)

**Functions:**
- `def outgoing_speed(r, flow_rate, depth, gravity)`
- `def ingoing_speed(r, flow_rate, depth, gravity)`
- `def peeling_rate(flow_rate, depth, gravity)`
- `def analytic_outgoing_time(r, r0, horizon, speed)`
- `def integrate_outgoing(r0, r_end, flow_rate, depth, gravity, dt)`

---

### [`verification/__init__.py`](verification/__init__.py)
> *Independent verification package. See audit.py.*

**Size**: `1` lines (`54` bytes)

---

### [`verification/audit.py`](verification/audit.py)
> *Independent verification of every claim marked PROVEN, VERIFIED, or NUMERIC.*

**Size**: `2137` lines (`84716` bytes)

**Classes:**
- `class Audit`
  - Method `.record(claim, passed, detail, expected, observed)`
  - Property `.failed`

**Functions:**
- `def circulant(a, c, delta)`
- `def roots_from_ac(a, c, delta)`
- `def q_from_roots(x)`
- `def q_from_masses(m)`
- `def q_from_ac(a, c)`
- `def fit_linear(x)` - *Independent least-squares/inverse of x_k = a + 2u cos(theta_k) - 2v sin(theta_k).*
- `def load_results(name)`
- `def provenance()` - *Minimal, independent provenance block (no project imports).*
- `def write_atomic(path, text)`
- `def audit_algebra(audit)`
- `def audit_fit(audit)`
- `def y0_vector()`
- `def independent_rhs(_t, s)`
- `def independent_solution(method)`
- `def q_from_yukawas(y)`
- `def audit_rg(audit)`
- `def rotor_levels(I, mu, sector, n_max)`
- `def audit_rotor(audit)`
- `def audit_neutrinos_quarks(audit)`
- `def audit_delta_modes_derrick(audit)`
- `def audit_negatives(audit)`
- `def audit_lorentz(audit)`
- `def audit_hopf(audit)`
- `def audit_hopfion3d(audit)`
- `def audit_hopf_resolution(audit)`
- `def audit_n64(audit)`
- `def audit_scale(audit)`
- `def audit_stability(audit)`
- `def audit_purefn(audit)`
- `def audit_escape(audit)`
- `def audit_lengthscan(audit)`
- `def audit_vortex(audit)`
- `def audit_cascade(audit)`
- `def audit_rotor_potential(audit)`
- `def audit_madelung(audit)`
- `def audit_nelson(audit)`
- `def audit_gaussian_fluid(audit)`
- `def audit_coherent_fluid(audit)`
- `def audit_excited_fluid(audit)`
- `def audit_hydrogen_fluid(audit)`
- `def audit_hydrogen_vortex(audit)`
- `def audit_water_analogue(audit)`
- `def audit_water_validity(audit)`
- `def audit_water_dispersive(audit)`
- `def audit_water_rays(audit)`
- `def audit_production(audit)`
- `def audit_convergence(audit)`
- `def audit_temporal(audit)`
- `def audit_extension(audit)`
- `def audit_docs(audit)`
- `def run(write, tag)`
- `def render(payload)`
- `def main()`

---

### [`verification/compare.py`](verification/compare.py)
> *Compare two independent verification runs.*

**Size**: `222` lines (`8203` bytes)

**Functions:**
- `def _load(tag)`
- `def _digest(payload)`
- `def compare(tags)`
- `def _write(name, text)`
- `def render(payload)`
- `def main()`

---

### [`verification/fingerprint.py`](verification/fingerprint.py)
> *Stable fingerprints for the two-run claim audit.*

**Size**: `155` lines (`4027` bytes)

**Functions:**
- `def _source_files()`
- `def _digest(files)`
- `def source_fingerprint()`
- `def results_fingerprint()`
- `def fingerprints()`

---

### [`verification/verify.py`](verification/verify.py)
> *Run the full claim-verification workflow in separate processes.*

**Size**: `66` lines (`2089` bytes)

**Functions:**
- `def _run(module, *args)`
- `def main()`

---
