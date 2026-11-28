//! Clarification of the up-quark tension: it is an AMPLITUDE precision issue,
//! not a phase-rule failure.
//!
//! Earlier (up_phase_resolution.rs) we resolved the +2.61 sigma up tension by
//! switching the up phase to the gauge relation delta_up = delta_lep sin^2tw
//! = 2/39, making the phase rule mixed. Here we show the single Fibonacci rule
//! delta = A^2/F^2 (F=8 for up) is actually CORRECT:
//!
//!   With A^2 = 3.32415 (best fit) and delta = A^2/64, the predicted up ratios
//!   are (22.4527, 16.4389) vs recorded (22.4514, 16.4738) -- errors 0.006%
//!   and 0.2%. That best-fit amplitude is within 0.04% of the measured
//!   A^2 = 6(Q_up - 1/3) = 3.32542.
//!
//! The framework's predicted A^2 = 10/3 = 3.33333 is 0.24% too high; because
//! the up first ratio sits near the positive-root boundary, that 0.24% is
//! amplified into a 25.6% ratio error. The gauge-derived amplitude at M_Z
//! (3 cot^2theta_W/3 = 3.32053) overshoots the other way (-7.4% ratio error),
//! bracketing the measured value. The single phase rule therefore survives;
//! what remains is predicting A^2_up to ~0.1% precision.
//!
//! HONEST CAVEAT: the best-fit A^2 is obtained from the measured ratios, so
//! this is a diagnostic, not a new prediction. It supersedes the claim that a
//! mixed phase rule is REQUIRED.

use crate::Check;
use crate::mass_convention::{q_from_masses, sqrt_ratios, up_masses_mz};
use std::f64::consts::PI;

pub fn f_up() -> f64 { 8.0 }
pub fn a2_framework() -> f64 { 10.0/3.0 }
pub fn a2_measured() -> f64 { 6.0*(q_from_masses(up_masses_mz()) - 1.0/3.0) }
pub fn a2_best_fit() -> f64 { 3.32415 }

pub fn ratios(a2: f64) -> (f64,f64) {
    let a=a2.sqrt(); let d=a2/(f_up()*f_up());
    let mut y=[0.0_f64;3];
    for k in 0..3 { y[k]=1.0+a*(d+2.0*PI*k as f64/3.0).cos(); }
    y.sort_by(|x,y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    (y[1]/y[0],y[2]/y[1])
}

pub fn checks() -> Vec<Check> {
    let obs=sqrt_ratios(up_masses_mz());
    let rf=ratios(a2_framework()); let rb=ratios(a2_best_fit());
    vec![
        Check::new("singlephase.a2_gap", a2_framework()/a2_measured()-1.0,
                   "framework vs measured up amplitude (~+0.0024)"),
        Check::new("singlephase.r1_framework_err", rf.0/obs.0-1.0,
                   "r1 error with framework A^2 (~0.256)"),
        Check::new("singlephase.r1_bestfit_err", rb.0/obs.0-1.0,
                   "r1 error with best-fit A^2 (~0.0001)"),
        Check::new("singlephase.bestfit_vs_measured", a2_best_fit()/a2_measured()-1.0,
                   "best-fit vs measured amplitude (~-0.0004)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amplitude_mismatch_is_small() {
        assert!((a2_framework()/a2_measured()-1.0).abs() < 0.005);
    }

    #[test]
    fn single_rule_with_best_fit_is_excellent() {
        let obs=sqrt_ratios(up_masses_mz());
        let r=ratios(a2_best_fit());
        assert!((r.0/obs.0-1.0).abs() < 0.01 && (r.1/obs.1-1.0).abs() < 0.01);
    }

    #[test]
    fn sensitivity_amplifies_small_amplitude_gap() {
        let obs=sqrt_ratios(up_masses_mz());
        let ef=(ratios(a2_framework()).0/obs.0-1.0).abs();
        let eb=(ratios(a2_best_fit()).0/obs.0-1.0).abs();
        assert!(ef > 10.0*eb, "amplitude gap amplified {}x", ef/eb.max(1e-9));
    }
}
