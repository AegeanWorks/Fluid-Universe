//! Geometric characterization: Koide Q=2/3 means 45 degrees to (1,1,1).
//!
//! For any positive triple x, cos^2(theta) = 1/(3Q) where theta is the angle
//! between x and (1,1,1). Hence Q=2/3 iff cos^2(theta)=1/2, i.e. theta=45deg.
//! The physical lepton roots sit at 44.9999deg. This module records the exact
//! identity and tests whether simple lattice vectors supply the 45deg direction
//! parameter-free. They do not: the required third component is 4+3 sqrt(2).

use crate::Check;

pub fn q(x: &[f64; 3]) -> f64 {
    let sum: f64 = x.iter().sum();
    let squares: f64 = x.iter().map(|v| v * v).sum();
    squares / (sum * sum)
}

pub fn cos2_theta(x: &[f64; 3]) -> f64 {
    let sum: f64 = x.iter().sum();
    let squares: f64 = x.iter().map(|v| v * v).sum();
    sum * sum / (3.0 * squares)
}

/// Third component r making (1,1,r) satisfy Q=2/3: r = 4 + 3 sqrt(2).
pub fn required_ratio() -> f64 {
    4.0 + 3.0 * 2.0_f64.sqrt()
}

pub fn checks() -> Vec<Check> {
    let leptons = [0.51099895_f64.sqrt(), 105.6583755_f64.sqrt(), 1776.93_f64.sqrt()];
    let theta = cos2_theta(&leptons).sqrt().acos().to_degrees();
    vec![
        Check::new("geometry.lepton_angle_deg", theta,
                   "angle of the lepton root vector to (1,1,1)"),
        Check::new("geometry.lepton_cos2", cos2_theta(&leptons),
                   "cos^2 of that angle (target 1/2)"),
        Check::new("geometry.required_ratio", required_ratio(),
                   "r for Q(1,1,r)=2/3"),
        Check::new("geometry.q_at_required", q(&[1.0, 1.0, required_ratio()]),
                   "Q of (1,1,r_required)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn identity_holds() {
        for x in [[1.0, 2.0, 3.0], [0.5, 10.0, 100.0], [1.0, 1.0, 8.2426]] {
            assert!(approx(cos2_theta(&x), 1.0 / (3.0 * q(&x)), 1e-12));
        }
    }

    #[test]
    fn koide_is_45_degrees() {
        let x = [1.0, 1.0, required_ratio()];
        assert!(approx(q(&x), 2.0 / 3.0, 1e-12));
        assert!(approx(cos2_theta(&x), 0.5, 1e-12));
        assert!(approx(cos2_theta(&x).sqrt().acos().to_degrees(), 45.0, 1e-9));
    }

    #[test]
    fn leptons_are_near_45_degrees() {
        let leptons = [0.51099895_f64.sqrt(), 105.6583755_f64.sqrt(), 1776.93_f64.sqrt()];
        let theta = cos2_theta(&leptons).sqrt().acos().to_degrees();
        assert!((theta - 45.0).abs() < 0.001, "theta = {theta}");
    }

    #[test]
    fn required_ratio_is_not_a_small_lattice_component() {
        let r = required_ratio();
        for small in [1.0, 2.0, 3.0, 2.0_f64.sqrt(), 4.0] {
            assert!((r - small).abs() > 1.0);
        }
    }
}
