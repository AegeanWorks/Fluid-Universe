//! Up amplitude derived from the measured weak mixing angle to 0.016%.
//!
//! The d=3 fixed point gives cot^2(theta_W)=10/3 exactly, but the observed
//! up amplitude is 3.32542, 0.24% below that rounded ideal. Using the PDG
//! MS-bar value sin^2(theta_W)(M_Z)=0.23122 instead:
//!   A^2_up = cot^2(theta_W) = (1-s2)/s2 = 3.324885
//! which matches the amplitude required by the recorded Q_up to 0.016% --
//! far better than 0.1% precision. With the SINGLE Fibonacci phase rule
//! delta = A^2/F^2 (F=8), predicted up ratios are (22.822, 16.447) vs
//! recorded (22.451, 16.474): errors 1.65% and 0.16%, i.e. ~0.18 sigma under
//! the 9.2% propagated mass uncertainty. The predicted Q is 0.887481 vs
//! recorded 0.887569 (0.01%).
//!
//! This resolves the amplitude-precision problem flagged in single_phase_rule.rs
//! with an INDEPENDENT experimental input (the Weinberg angle), and keeps a
//! single phase rule across all sectors.
//!
//! HONEST CAVEAT: A^2_up here comes from a MEASURED gauge coupling, not from
//! d=3 alone (the d=3 fixed point gives the 0.24%-high ideal 10/3). It is a
//! gauge-flavor numerical relation, not a first-principles derivation.

use crate::Check;
use crate::mass_convention::{q_from_masses, sqrt_ratios, up_masses_mz};
use std::f64::consts::PI;

pub const SIN2TW_MSBAR_MZ: f64 = 0.23122;  // PDG MS-bar

pub fn a2_up_from_gauge() -> f64 {
    (1.0 - SIN2TW_MSBAR_MZ) / SIN2TW_MSBAR_MZ
}
pub fn a2_up_from_q() -> f64 {
    6.0 * (q_from_masses(up_masses_mz()) - 1.0/3.0)
}

pub fn ratios(a2: f64) -> (f64, f64) {
    let a=a2.sqrt(); let d=a2/64.0;
    let mut y=[0.0_f64;3];
    for k in 0..3 { y[k]=1.0+a*(d+2.0*PI*k as f64/3.0).cos(); }
    y.sort_by(|x,y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    (y[1]/y[0],y[2]/y[1])
}

pub fn checks() -> Vec<Check> {
    let obs=sqrt_ratios(up_masses_mz());
    let r=ratios(a2_up_from_gauge());
    vec![
        Check::new("upgauge.a2", a2_up_from_gauge(), "cot^2(theta_W) = 3.324885"),
        Check::new("upgauge.a2_err", a2_up_from_gauge()/a2_up_from_q()-1.0,
                   "amplitude error vs recorded (~-0.00016)"),
        Check::new("upgauge.q_err", (1.0/3.0+a2_up_from_gauge()/6.0)/q_from_masses(up_masses_mz())-1.0,
                   "predicted Q error (~-0.0001)"),
        Check::new("upgauge.r1_err", r.0/obs.0-1.0, "r1 error (~0.0165)"),
        Check::new("upgauge.r2_err", r.1/obs.1-1.0, "r2 error (~-0.0016)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amplitude_precision_is_better_than_one_promille() {
        assert!((a2_up_from_gauge()/a2_up_from_q()-1.0).abs() < 0.001);
    }

    #[test]
    fn q_prediction_is_very_precise() {
        assert!(((1.0/3.0+a2_up_from_gauge()/6.0)/q_from_masses(up_masses_mz())-1.0).abs() < 0.001);
    }

    #[test]
    fn ratios_with_single_phase_rule_are_good() {
        let obs=sqrt_ratios(up_masses_mz());
        let r=ratios(a2_up_from_gauge());
        assert!((r.0/obs.0-1.0).abs() < 0.02 && (r.1/obs.1-1.0).abs() < 0.01);
    }
}
