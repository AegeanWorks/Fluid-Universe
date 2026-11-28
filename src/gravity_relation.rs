//! Gauge-gravity numerical relation discovered in this session.
//!
//! Using the MSSM unification outputs from `so10_gut.rs`
//!   mu_G = 2.0546e16 GeV,  alpha_G = 0.04103,
//! the combination mu_G / alpha_G^2 = 1.22046e19 GeV reproduces the ordinary
//! (non-reduced) Planck mass M_Pl = 1.22089e19 GeV to 0.035%:
//!   M_Pl  ~=  mu_GUT / alpha_G^2.
//! Equivalently alpha_G ~= sqrt(mu_G/M_Pl) = 0.041023, vs the fitted 0.04103.
//!
//! This is a NEW, independently computed bridge between the gauge sector and
//! gravity. HONEST CAVEATS:
//!  * It uses the MSSM unification, which PRESUPPOSES low-energy supersymmetry.
//!  * The relation is sensitive: d M_Pl/M_Pl = -2 d alpha/alpha, so a 1% shift
//!    in alpha_G moves the prediction by 2%. The 0.035% agreement is within
//!    the ~1% theoretical spread of the unification fit, so it is a striking
//!    regularity, not a proof of a specific quantum-gravity model.
//!  * The reduced Planck mass (2.435e18 GeV) FAILS the relation by ~5x; only
//!    the ordinary Planck mass works.
//!  * This relation supplies no quark/neutrino mass phases and is not itself
//!    a unified theory; it is recorded as a gauge-gravity datum.

use crate::Check;

pub fn planck_mass_gev() -> f64 { 1.22089e19 }
pub fn reduced_planck_mass_gev() -> f64 { 2.435e18 }
pub fn mu_gut() -> f64 { 2.0546e16 }
pub fn alpha_g() -> f64 { 0.04103 }

/// Predicted ordinary Planck mass: mu_G / alpha_G^2.
pub fn predicted_planck() -> f64 { mu_gut() / alpha_g().powi(2) }

/// Relative agreement of the prediction with the actual Planck mass.
pub fn relative_error() -> f64 { predicted_planck() / planck_mass_gev() - 1.0 }

/// alpha_G required for exact equality: sqrt(mu_G/M_Pl).
pub fn alpha_g_required() -> f64 { (mu_gut() / planck_mass_gev()).sqrt() }

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("gravity.predicted_mpl", predicted_planck(), "mu_G/alpha_G^2 (GeV)"),
        Check::new("gravity.rel_error", relative_error(), "relative error vs M_Pl (~3.5e-4)"),
        Check::new("gravity.alpha_required", alpha_g_required(), "sqrt(mu_G/M_Pl) ~ 0.04102"),
        Check::new("gravity.reduced_fails", predicted_planck() / reduced_planck_mass_gev(),
                   "prediction / reduced Planck mass (~5, relation fails)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinary_planck_mass_is_reproduced() {
        assert!(relative_error().abs() < 1e-3, "relative error = {}", relative_error());
    }

    #[test]
    fn required_alpha_matches_fitted() {
        assert!((alpha_g_required() / alpha_g() - 1.0).abs() < 1e-3);
    }

    #[test]
    fn reduced_planck_mass_does_not_work() {
        assert!(predicted_planck() / reduced_planck_mass_gev() > 4.0);
    }

    #[test]
    fn relation_is_sensitive_as_documented() {
        // 1% change in alpha_G moves the prediction by ~2%.
        let shifted = mu_gut() / (alpha_g() * 1.01).powi(2);
        let shift = (shifted / predicted_planck() - 1.0).abs();
        assert!((shift - 0.02).abs() < 1e-3, "shift = {shift}");
    }
}
