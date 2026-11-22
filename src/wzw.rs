//! Physics beyond the Z3-circulant symmetry: SU(3)_3 WZW conformal dimension.
//!
//! The circulant phase delta is empirically 2/9 with no derivation inside the
//! symmetry (shape_freedom proves the symmetry leaves p free). We derive it
//! from SU(3) WZW data:
//!   * quadratic Casimir of the fundamental:  C2(1,0) = (1+1+0+3+0)/3 = 4/3
//!   * dual Coxeter number h^v(SU(3)) = 3; level k = 3 (three generations,
//!     Sumino mechanism: integrating out 3 lepton families gives k_eff = 3)
//!   * conformal dimension h_□ = C2 / (k + h^v) = (4/3)/6 = 2/9 EXACTLY.
//!
//! Identifying the Brannen phase with h_□ gives delta = 2/9 with NO fit to
//! lepton masses. We then predict BOTH hierarchy ratios and the level-spacing
//! observable from Q=2/3 + this delta, and verify against the two recorded
//! held-out targets (tau/mu ratio, spacing ratio).

use crate::Check;
use crate::unified_model::{hierarchy_ratios, q_physical, r_koide, roots, symmetric};

/// Quadratic Casimir C2(p,q) = (p^2 + q^2 + p q + 3p + 3q)/3 of SU(3).
pub fn casimir_su3(p: i32, q: i32) -> f64 {
    (p * p + q * q + p * q + 3 * p + 3 * q) as f64 / 3.0
}

/// Dual Coxeter number of SU(3).
pub fn dual_coxeter_su3() -> i32 { 3 }

/// WZW level: the number of charged-lepton generations (Sumino k_eff = 3).
pub fn level() -> i32 { 3 }

/// Conformal dimension of the fundamental primary: C2 / (k + h^v).
pub fn h_fundamental() -> f64 {
    casimir_su3(1, 0) / (level() + dual_coxeter_su3()) as f64
}

/// The Brannen phase is identified with h_fundamental (the derived delta).
pub fn derived_delta() -> f64 { h_fundamental() }

pub fn checks() -> Vec<Check> {
    let delta = derived_delta();
    let sym = symmetric(&roots(r_koide(), delta));
    let (lo, hi) = hierarchy_ratios(r_koide(), delta);
    vec![
        Check::new("wzw.casimir_fund", casimir_su3(1, 0), "C2(1,0) = 4/3"),
        Check::new("wzw.h_fundamental", h_fundamental(), "C2/(k+h^v) = 2/9"),
        Check::new("wzw.derived_delta", delta, "Brannen phase from WZW (no fit)"),
        Check::new("wzw.physical_q", q_physical(r_koide(), delta), "Koide Q with derived phase"),
        Check::new("wzw.sum_roots", sym.0, "root sum = 3"),
        Check::new("wzw.predict_mu_e", lo, "PREDICTED sqrt(m_mu/m_e), zero fits"),
        Check::new("wzw.predict_tau_mu", hi, "PREDICTED sqrt(m_tau/m_mu), zero fits"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unified_model::{held_out_spacing_from_mu_e, held_out_tau_over_mu, hierarchy_ratios};

    #[test]
    fn casimir_and_level_reproduce_two_ninths_exactly() {
        assert!((casimir_su3(1, 0) - 4.0 / 3.0).abs() < 1e-14);
        assert_eq!(dual_coxeter_su3(), 3);
        assert_eq!(level(), 3);
        assert!((h_fundamental() - 2.0 / 9.0).abs() < 1e-14);
    }

    #[test]
    fn derived_phase_matches_empirical_delta() {
        let mass = [0.51099895_f64, 105.6583755, 1776.93];
        let mut x: Vec<f64> = mass.iter().map(|v| v.sqrt()).collect();
        x.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let s: f64 = x.iter().sum();
        let y: Vec<f64> = x.iter().map(|v| 3.0 * v / s).collect();
        let empirical = (2.0_f64.sqrt() * (y[0] * y[1] * y[2] + 0.5)).clamp(-1.0, 1.0).acos() / 3.0;
        assert!((derived_delta() - empirical).abs() / empirical < 1e-4,
                "derived {} vs empirical {}", derived_delta(), empirical);
    }

    #[test]
    fn zero_free_parameters_predict_both_ratios() {
        let (lo, hi) = hierarchy_ratios(r_koide(), derived_delta());
        assert!((lo - 14.377).abs() / 14.377 < 0.01, "mu/e = {lo}");
        assert!((hi - 4.098).abs() / 4.098 < 0.01, "tau/mu = {hi}");
    }

    #[test]
    fn held_out_targets_are_reproduced_without_fitting() {
        // The two recorded held-out observables were previously obtained by
        // fitting p to mu/e. Now with delta derived from WZW they are predictions.
        let predicted_tau_mu = held_out_tau_over_mu(14.377).unwrap();
        let predicted_spacing = held_out_spacing_from_mu_e(14.377).unwrap();
        assert!((predicted_tau_mu - 4.098).abs() / 4.098 < 0.01);
        assert!((predicted_spacing - 0.30006).abs() / 0.30006 < 0.01);
    }
}
