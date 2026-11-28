//! Georgi-Jarlskog b-tau relation from the d=3 framework: m_b/m_tau = d*eta_QCD = 2.35.
//!
//! The framework predicts m_b/m_tau = d * eta_QCD with eta_QCD = 0.7833...,
//! giving 2.35 for d=3. Tested against recorded masses:
//!   m_b(m_b)/m_tau(pole) = 4.18/1.77693 = 2.35237 -> +0.10%
//!   m_b(M_Z)/m_tau(M_Z)  = 2.839/1.72856 = 1.64241 -> -30%
//! The agreement holds at the m_b(m_b) scale (the conventional GJ comparison
//! scale), not at M_Z. This is a GUT-scale Yukawa relation tested at low energy.
//!
//! HONEST CAVEATS:
//!  * This is a classic GUT relation (Georgi-Jarlskog, SU(5)); the d=3 framework
//!    reproduces it, but it is not unique to d=3.
//!  * It is scale-dependent: 0.1% at m_b(m_b), -30% at M_Z. We record the
//!    conventional-scale comparison, not a scale-invariant prediction.
//!  * eta_QCD = 0.7833 is fixed from the prediction, not independently derived.

use crate::Check;

pub fn d() -> f64 { 3.0 }
pub fn predicted_ratio() -> f64 { 2.35 }
pub fn eta_from_prediction() -> f64 { predicted_ratio()/d() }
pub fn observed_ratio_mb_scale() -> f64 { 4.18/1.77693 }
pub fn observed_ratio_mz() -> f64 { 2.839/1.72856 }

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("gj.eta", eta_from_prediction(), "eta_QCD = 2.35/3 = 0.7833"),
        Check::new("gj.ratio_obs", observed_ratio_mb_scale(), "m_b(m_b)/m_tau(pole) = 2.352"),
        Check::new("gj.err_mb_scale", observed_ratio_mb_scale()/predicted_ratio()-1.0,
                   "GJ error at m_b scale (~+0.001)"),
        Check::new("gj.err_mz", observed_ratio_mz()/predicted_ratio()-1.0,
                   "GJ error at M_Z (~-0.30, scale dependent)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gj_holds_at_mb_scale() {
        assert!((observed_ratio_mb_scale()/predicted_ratio()-1.0).abs() < 0.005);
    }

    #[test]
    fn gj_fails_at_mz_scale() {
        assert!((observed_ratio_mz()/predicted_ratio()-1.0).abs() > 0.2);
    }
}
