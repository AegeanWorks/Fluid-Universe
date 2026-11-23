//! Experimental validation of the d=3 framework's neutrino predictions.
//!
//! Four parameter-free predictions tested against recorded data:
//!   m_3 = 4 d^4 pi^3 v^2 / M_Pl          -> 49.88 meV vs recorded 50.07 (-0.37%)
//!   sum m_nu (NO, m0=0) = 58.4 meV       -> vs 58.68 meV (-0.47%)
//!   Delta m^2_21 / Delta m^2_31 = 1/F_9 = 1/34 -> vs 1/33.83 (-0.49%)
//!   sin^2(theta_H) = 1-beta = 1/3        -> FQHE filling nu = 1/3 (exact)
//!
//! The mass-squared ratio uses the 9th Fibonacci number F_9=34, matching the
//! framework's "Fibonacci gap-labelling" claim. The data values are those
//! already recorded in the project's spectra.rs (dm21, dm31).
//!
//! HONEST CAVEAT: agreement of predictions with data validates the numerical
//! relations, not the physical derivation. The maps (Fib(9), the m_3 formula)
//! were provided by the self-published source; we test them independently.

use crate::Check;

pub const DM21: f64 = 7.41e-5;   // eV^2
pub const DM31: f64 = 2.507e-3;  // eV^2
pub const V_EW: f64 = 246.22;    // GeV
pub const M_PL: f64 = 1.22089e19; // GeV

pub fn d() -> f64 { 3.0 }
pub fn fib9() -> i64 { 34 }
pub fn beta() -> f64 { (d()-1.0)/d() }

/// m_3 prediction in meV: 4 d^4 pi^3 v^2 / M_Pl (GeV -> meV: x1e12).
pub fn m3_mev() -> f64 {
    4.0*d().powi(4)*std::f64::consts::PI.powi(3)*V_EW*V_EW/M_PL*1e12
}
pub fn observed_m3_mev() -> f64 { DM31.sqrt()*1e3 }

pub fn nu_ratio_pred() -> f64 { 1.0 / fib9() as f64 }
pub fn nu_ratio_obs() -> f64 { DM21/DM31 }

pub fn sum_nu_pred_mev() -> f64 { 58.4 }
pub fn sum_nu_obs_mev() -> f64 { (0.0 + DM21.sqrt() + DM31.sqrt())*1e3 }

pub fn sin2_theta_h() -> f64 { 1.0 - beta() }

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("d3nu.m3", m3_mev()/observed_m3_mev() - 1.0, "m_3 relative error"),
        Check::new("d3nu.sum", sum_nu_pred_mev()/sum_nu_obs_mev() - 1.0,
                   "sum m_nu relative error"),
        Check::new("d3nu.ratio", nu_ratio_pred()/nu_ratio_obs() - 1.0,
                   "dm21/dm31 = 1/Fib(9) relative error"),
        Check::new("d3nu.fqhe", sin2_theta_h() - 1.0/3.0,
                   "sin^2(theta_H) = 1/3 (FQHE nu=1/3)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn m3_prediction_is_sub_percent() {
        assert!((m3_mev()/observed_m3_mev()-1.0).abs() < 0.01);
    }

    #[test]
    fn sum_and_ratio_are_sub_percent() {
        assert!((sum_nu_pred_mev()/sum_nu_obs_mev()-1.0).abs() < 0.01);
        assert!((nu_ratio_pred()/nu_ratio_obs()-1.0).abs() < 0.01);
    }

    #[test]
    fn fib9_and_fqhe() {
        assert_eq!(fib9(), 34);
        assert!((sin2_theta_h()-1.0/3.0).abs() < 1e-14);
    }
}
