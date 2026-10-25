# Codebase Map & Symbol Catalog 🗺️

> **AUTO-GENERATED & VERIFIED BY `scripts/update_knowledge_base.py`**
> This document provides an automated, up-to-date registry of all modules, classes, and exported functions across this repository.

---

## 📊 Codebase Metrics
- **Total Python Modules**: `19`
- **Total Classes**: `4`
- **Total Top-Level Functions**: `129`
- **Total Class Methods**: `4`
- **Total Source Lines**: `2934`

---

## 📦 Modules & Exported Symbols

### [`phenomenology/__init__.py`](phenomenology/__init__.py)
> *Phenomenology: PDG input, circulant fit, and one-loop SM running.*

**Size**: `1` lines (`72` bytes)

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

### [`phenomenology/run_all.py`](phenomenology/run_all.py)
> *Regenerate every result file from scratch.*

**Size**: `214` lines (`8715` bytes)

**Functions:**
- `def rotor_payload()`
- `def render_rotor_markdown(payload)`
- `def build_summary(fit, rg, rotor)`
- `def main()`

---

### [`sims/__init__.py`](sims/__init__.py)
> *Simulation package placeholder. No validated core exists yet; see README.md.*

**Size**: `1` lines (`83` bytes)

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

### [`verification/__init__.py`](verification/__init__.py)
> *Independent verification package. See audit.py.*

**Size**: `1` lines (`54` bytes)

---

### [`verification/audit.py`](verification/audit.py)
> *Independent verification of every claim marked PROVEN, VERIFIED, or NUMERIC.*

**Size**: `742` lines (`28357` bytes)

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
- `def audit_docs(audit)`
- `def run(write, tag)`
- `def render(payload)`
- `def main()`

---

### [`verification/compare.py`](verification/compare.py)
> *Compare two independent verification runs.*

**Size**: `215` lines (`8113` bytes)

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

**Size**: `77` lines (`2093` bytes)

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
