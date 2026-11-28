//! The parameter-free condition a unified Z3/circulant model would have to satisfy.
//!
//! This module does not claim a unified theory. It derives the one number that any
//! Z3/circulant bridge must reproduce, and tests whether the simulated fluid
//! observables supply it without fitting to lepton masses.

use crate::Check;
use crate::circulant::{q_from_ac, q_masses};

/// The Koide point fixes r = |c|/a = 1/sqrt(2), independent of the phase delta.
pub fn required_r() -> f64 {
    2.0_f64.powf(-0.5)
}

/// Inverse of Q_signed = 1/3 + 2r^2/3 for Q in [1/3, 1].
pub fn r_for_q(q: f64) -> Option<f64> {
    let value = ((q - 1.0 / 3.0) * 1.5).sqrt();
    if value.is_finite() && (1.0 / 3.0..=1.0 + 1e-12).contains(&q) {
        Some(value)
    } else {
        None
    }
}

/// Maximum variation of the signed Koide Q over a dense delta scan at fixed r.
/// It is zero to machine precision: the phase cannot fix the mass ratio.
pub fn delta_independence(r: f64, samples: usize) -> f64 {
    let mut low = f64::INFINITY;
    let mut high = f64::NEG_INFINITY;
    for step in 0..samples {
        let delta = 2.0 * std::f64::consts::PI * step as f64 / samples as f64;
        let roots = [
            1.0 + 2.0 * r * delta.cos(),
            1.0 + 2.0 * r * (delta + 2.0 * std::f64::consts::PI / 3.0).cos(),
            1.0 + 2.0 * r * (delta + 4.0 * std::f64::consts::PI / 3.0).cos(),
        ];
        let q = q_signed(&roots);
        low = low.min(q);
        high = high.max(q);
    }
    high - low
}

fn q_signed(x: &[f64; 3]) -> f64 {
    let sum: f64 = x.iter().sum();
    let squares: f64 = x.iter().map(|v| v * v).sum();
    squares / (sum * sum)
}

/// Distance from a fluid observable to the required parameter-free ratio.
pub fn bridge_gap(observable: f64) -> f64 {
    (observable - required_r()).abs()
}

pub fn checks() -> Vec<Check> {
    let r = required_r();
    vec![
        Check::new("unification.required_r", r, "r=|c|/a forced by Q=2/3"),
        Check::new("unification.q_at_required_r", q_from_ac(1.0, r),
                   "Koide Q evaluated at the forced ratio"),
        Check::new("unification.inverse_r", r_for_q(2.0 / 3.0).unwrap_or(f64::NAN),
                   "inverse map Q -> r"),
        Check::new("unification.delta_independence", delta_independence(r, 4096),
                   "max Q variation over a dense delta scan at fixed r"),
        Check::new("unification.physical_q", q_masses(&[0.51099895, 105.6583755, 1776.93]),
                   "physical Q for PDG pole masses"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn required_ratio_reproduces_koide() {
        assert!(approx(required_r(), 0.7071067811865476, 1e-14));
        assert!(approx(q_from_ac(1.0, required_r()), 2.0 / 3.0, 1e-14));
    }

    #[test]
    fn inverse_map_is_consistent() {
        let value = r_for_q(2.0 / 3.0).unwrap();
        assert!(approx(value, required_r(), 1e-12));
        assert!(r_for_q(0.2).is_none());
    }

    #[test]
    fn phase_cannot_fix_the_ratio() {
        assert!(delta_independence(required_r(), 2048) < 1e-12);
        assert!(delta_independence(0.9, 2048) < 1e-12);
    }

    #[test]
    fn fluid_observables_are_not_the_required_ratio() {
        // Values from the n=16 coupling scan; none coincides with 1/sqrt(2).
        for z3 in [0.281731683_f64, 0.366481080, 0.999836025] {
            assert!(bridge_gap(z3) > 0.2);
        }
    }
}
