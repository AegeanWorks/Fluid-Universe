//! The gauge–flavor offset is a scale-choice artifact: use M_W, not M_Z.
//!
//! In gauge_flavor.rs the relation 3 cot^2(theta_W) ~ 10 holds to 0.384% when
//! the weak angle is evaluated at M_Z. Fitting the recorded one-loop running
//! of g1, g2 (results/rg_run.json) gives 1/g_i^2 = b_i + a_i ln(mu) with
//!   a1 = -0.05193, b1 = 4.9354 ;  a2 = +0.04011, b2 = 2.1786.
//! The exact condition 3 cot^2(theta_W) = 10 (i.e. sin^2 = 3/13) occurs at
//!   mu = 79.47 GeV,
//! which is within 1.1% of the W-boson mass M_W = 80.379 GeV. Evaluating at
//! M_W itself gives 3 cot^2(theta_W) = 9.9968, only 0.032% from 10 -- twelve
//! times closer than at M_Z. The framework's constant 10 is therefore the
//! W-scale value of 3 cot^2(theta_W), and the 0.384% residual is largely the
//! difference between M_Z and M_W.
//!
//! HONEST CAVEAT: the one-loop fit is an extrapolation ~10 GeV below M_Z; the
//! quantitative 0.032% is fit-dependent. The qualitative finding -- that the
//! offset shrinks by an order of magnitude at the W scale -- is robust.

use crate::Check;

pub const M_Z: f64 = 91.1876;
pub const M_W: f64 = 80.379;
pub const MU_EXACT: f64 = 79.4661;

pub fn a1() -> f64 { -0.05193 }
pub fn b1() -> f64 { 4.9354 }
pub fn a2() -> f64 { 0.04011 }
pub fn b2() -> f64 { 2.1786 }

pub fn three_cot2_at(mu: f64) -> f64 {
    let t = mu.ln();
    let g1sq = 1.0 / (b1() + a1() * t);
    let g2sq = 1.0 / (b2() + a2() * t);
    let s = (3.0 / 5.0 * g1sq) / (g2sq + 3.0 / 5.0 * g1sq);
    3.0 * (1.0 - s) / s
}

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("scale.at_mz", three_cot2_at(M_Z), "3 cot^2 at M_Z (~9.961)"),
        Check::new("scale.at_mw", three_cot2_at(M_W), "3 cot^2 at M_W (~9.997)"),
        Check::new("scale.exact_mu", MU_EXACT, "scale where 3 cot^2 = 10 exactly (GeV)"),
        Check::new("scale.mw_err", three_cot2_at(M_W) / 10.0 - 1.0,
                   "relative error at M_W (~-0.00032)"),
        Check::new("scale.exact_over_mw", MU_EXACT / M_W,
                   "exact scale / M_W (~0.989)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn w_scale_is_much_closer_to_ten() {
        let err_mz = (three_cot2_at(M_Z) / 10.0 - 1.0).abs();
        let err_mw = (three_cot2_at(M_W) / 10.0 - 1.0).abs();
        assert!(err_mw < err_mz / 5.0, "M_W error {err_mw} vs M_Z {err_mz}");
        assert!(err_mw < 0.001, "M_W error should be <0.1%");
    }

    #[test]
    fn exact_scale_is_near_w_mass() {
        assert!((MU_EXACT / M_W - 1.0).abs() < 0.02);
        assert!((three_cot2_at(MU_EXACT) - 10.0).abs() < 1e-6);
    }
}
