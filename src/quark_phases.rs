//! Independent quark-sector test of the SU(3)_3 / Brannen framework.
//!
//! Literature assigns quark phases 2/27 (up) and 4/27 (down), with the
//! amplitude fixed by the measured Koide-like Q rather than by sqrt(2).
//! We test the literature fixed phases against the hierarchy ratios from the
//! SAME Huang-Zhou MZ mass set that defines the recorded Q values (see
//! mass_convention.rs). The literature phases fail. The earlier version of
//! this module used a different, simplified mass set; its reported error
//! values were artifacts of that mixed convention and are corrected here.

use crate::Check;
use std::f64::consts::PI;

/// Amplitude required by a measured Koide-like Q (all-positive roots).
pub fn amplitude_from_q(q: f64) -> f64 { (6.0 * (q - 1.0 / 3.0)).sqrt() }

/// The two sqrt-mass ratios of the circulant triplet at amplitude A and phase d.
pub fn ratios(a: f64, d: f64) -> Option<(f64, f64)> {
    let mut y = [0.0; 3];
    for k in 0..3 {
        y[k] = 1.0 + a * (d + 2.0 * PI * k as f64 / 3.0).cos();
    }
    if y.iter().any(|v| *v <= 0.0) { return None; }
    y.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    Some((y[1] / y[0], y[2] / y[1]))
}

/// Target ratios from the Huang-Zhou MZ masses that define the recorded Q.
pub fn up_target() -> (f64, f64) {
    crate::mass_convention::sqrt_ratios(crate::mass_convention::up_masses_mz())
}

pub fn down_target() -> (f64, f64) {
    crate::mass_convention::sqrt_ratios(crate::mass_convention::down_masses_mz())
}

/// Joint relative error of a predicted ratio pair against a target.
pub fn joint_error(p: (f64, f64), t: (f64, f64)) -> f64 {
    ((p.0 / t.0 - 1.0).powi(2) + (p.1 / t.1 - 1.0).powi(2)).sqrt()
}

pub fn checks() -> Vec<Check> {
    let a_up = amplitude_from_q(0.8875692313187327);
    let a_down = amplitude_from_q(0.7480650548031402);
    let lit_up = ratios(a_up, 2.0 / 27.0);
    let lit_down = ratios(a_down, 4.0 / 27.0);
    vec![
        Check::new("quark_phases.up_amplitude", a_up, "amplitude from measured up Q"),
        Check::new("quark_phases.down_amplitude", a_down, "amplitude from measured down Q"),
        // A non-physical (negative-root) triplet is reported as error = -1,
        // a finite sentinel meaning "no positive spectrum", not a numeric result.
        Check::new("quark_phases.lit_up_err",
                   lit_up.map(|r| joint_error(r, up_target())).unwrap_or(-1.0),
                   "joint error of literature phase 2/27 for up quarks (-1 = no positive spectrum)"),
        Check::new("quark_phases.lit_down_err",
                   lit_down.map(|r| joint_error(r, down_target())).unwrap_or(-1.0),
                   "joint error of literature phase 4/27 for down quarks"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amplitude_from_q_is_consistent() {
        assert!((amplitude_from_q(2.0 / 3.0) - 2.0_f64.sqrt()).abs() < 1e-12);
    }

    #[test]
    fn up_literature_phase_gives_no_positive_triplet() {
        // With A~1.82 the phase 2/27 makes the middle root negative: no physical masses.
        let a = amplitude_from_q(0.8875692313187327);
        assert!(ratios(a, 2.0 / 27.0).is_none());
    }

    #[test]
    fn down_literature_phase_fails_the_hierarchy() {
        let a = amplitude_from_q(0.7480650548031402);
        let r = ratios(a, 4.0 / 27.0).unwrap();
        let e = joint_error(r, down_target());
        assert!(e > 0.05, "down joint error = {e}");
    }
}
