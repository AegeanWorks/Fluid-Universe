//! Derrick scaling for the Frank-Oseen + Skyrme + Z3 energy.
//!
//! Under x -> lambda x in d dimensions the three pieces of the energy scale as
//!     E2 -> lambda^(d-2) E2,   E4 -> lambda^(d-4) E4,   V -> lambda^d V.
//! For d = 3 this is E(lambda) = lambda E2 + E4/lambda + lambda^3 V, and a
//! stationary point obeys the virial relation E2 - E4 + 3V = 0 after rescaling.
//! The curvature 2(E4 + 3V) then decides whether it is a scaling minimum.

use crate::Check;

/// Scaling exponents [gradient, Skyrme, potential] for d spatial dimensions.
pub fn exponents(d: i32) -> [i32; 3] {
    assert!(d >= 1, "d must be positive");
    [d - 2, d - 4, d]
}

/// E(lambda) = lambda^(d-2) E2 + lambda^(d-4) E4 + lambda^d V.
pub fn scaling_energy(lambda: f64, e2: f64, e4: f64, v: f64, d: i32) -> f64 {
    assert!(lambda > 0.0, "lambda must be positive");
    let [g, s, p] = exponents(d);
    lambda.powi(g) * e2 + lambda.powi(s) * e4 + lambda.powi(p) * v
}

/// dE/dlambda at lambda = 1: (d-2) E2 + (d-4) E4 + d V.
pub fn virial(e2: f64, e4: f64, v: f64, d: i32) -> f64 {
    let [g, s, p] = exponents(d);
    f64::from(g) * e2 + f64::from(s) * e4 + f64::from(p) * v
}

/// d^2E/dlambda^2 at lambda = 1: (d-2)(d-3) E2 + (d-4)(d-5) E4 + d(d-1) V.
pub fn second_derivative(e2: f64, e4: f64, v: f64, d: i32) -> f64 {
    let [g, s, p] = exponents(d);
    f64::from(g) * f64::from(g - 1) * e2
        + f64::from(s) * f64::from(s - 1) * e4
        + f64::from(p) * f64::from(p - 1) * v
}

/// Closed-form lambda^2 at the d = 3 stationary point, or None if none exists.
///
/// Solves 3 V y^2 + E2 y - E4 = 0 with y = lambda^2. Invalid inputs (E2 <= 0
/// or V < 0) are reported as None, as is a negative discriminant.
pub fn stationary_scale_squared(e2: f64, e4: f64, v: f64) -> Option<f64> {
    if e2 <= 0.0 || v < 0.0 {
        return None;
    }
    if v == 0.0 {
        return if e4 > 0.0 { Some(e4 / e2) } else { None };
    }
    let discriminant = e2 * e2 + 12.0 * v * e4;
    if discriminant < 0.0 {
        return None;
    }
    let y = (-e2 + discriminant.sqrt()) / (6.0 * v);
    if y > 0.0 { Some(y) } else { None }
}

/// Characterisation of the d = 3 scaling minimum.
#[derive(Clone, Debug, PartialEq)]
pub struct ScalingMinimum {
    pub lambda: f64,
    pub energy: f64,
    pub curvature: f64,
    pub is_minimum: bool,
    pub virial_at_one: f64,
}

/// Locate and characterise the d = 3 stationary scale, if it exists.
pub fn scaling_minimum(e2: f64, e4: f64, v: f64) -> Option<ScalingMinimum> {
    let y = stationary_scale_squared(e2, e4, v)?;
    let lambda = y.sqrt();
    let curvature = second_derivative(e2, e4, v, 3);
    Some(ScalingMinimum {
        lambda,
        energy: scaling_energy(lambda, e2, e4, v, 3),
        curvature,
        is_minimum: curvature > 0.0,
        virial_at_one: virial(e2, e4, v, 3),
    })
}

/// Reproducible result lines for this module (ids are part of the audit fingerprint).
pub fn checks() -> Vec<crate::Check> {
    const E2: f64 = 233.635;
    const E4: f64 = 159.262;

    let y = stationary_scale_squared(E2, E4, 0.0).unwrap_or(f64::NAN);
    let lambda = y.sqrt();
    let minimum = scaling_minimum(E2, E4, 0.0);
    let h = 1e-6;
    let slope = (scaling_energy(lambda + h, E2, E4, 0.0, 3)
        - scaling_energy(lambda - h, E2, E4, 0.0, 3))
        / (2.0 * h);
    let [g, s, p] = exponents(3);
    let field = |pick: fn(&ScalingMinimum) -> f64| -> f64 {
        minimum.as_ref().map_or(f64::NAN, pick)
    };

    vec![
        Check::new("derrick.exp_gradient_d3", f64::from(g), "d-2"),
        Check::new("derrick.exp_skyrme_d3", f64::from(s), "d-4"),
        Check::new("derrick.exp_potential_d3", f64::from(p), "d"),
        Check::new("derrick.lambda_squared", y, "E4/E2 at V=0"),
        Check::new("derrick.lambda_star", lambda, "sqrt(E4/E2) at V=0"),
        Check::new("derrick.energy_min", field(|m| m.energy), "2 sqrt(E2 E4) at V=0"),
        Check::new("derrick.curvature", field(|m| m.curvature), "2(E4+3V)"),
        Check::new("derrick.virial_at_one", field(|m| m.virial_at_one), "E2-E4+3V"),
        Check::new("derrick.virial_at_stationary", slope, "dE/dlambda at lambda*"),
        Check::new(
            "derrick.is_minimum",
            if minimum.as_ref().is_some_and(|m| m.is_minimum) { 1.0 } else { 0.0 },
            "curvature > 0",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::{approx, bisect};

    const E2: f64 = 233.635;
    const E4: f64 = 159.262;

    #[test]
    fn exponents_match_contract() {
        assert_eq!(exponents(3), [1, -1, 3]);
        assert_eq!(exponents(4), [2, 0, 4]);
    }

    #[test]
    fn derivative_at_one_equals_virial() {
        for d in [3, 4, 5] {
            let h = 1e-6;
            let slope = (scaling_energy(1.0 + h, 3.0, 7.0, 11.0, d)
                - scaling_energy(1.0 - h, 3.0, 7.0, 11.0, d))
                / (2.0 * h);
            let v = virial(3.0, 7.0, 11.0, d);
            assert!(approx(slope, v, 1e-6), "d={} slope={} virial={}", d, slope, v);
        }
    }

    #[test]
    fn stationary_scale_matches_hopfion_reference() {
        let y = stationary_scale_squared(E2, E4, 0.0).unwrap();
        assert!(approx(y, E4 / E2, 1e-12));
        let m = scaling_minimum(E2, E4, 0.0).unwrap();
        assert!(approx(m.lambda, 0.8256323619, 1e-6), "lambda = {}", m.lambda);
        assert!(approx(m.lambda, y.sqrt(), 1e-12));
        assert!(approx(m.energy, 385.793252741, 1e-6), "E_min = {}", m.energy);
        assert!(m.is_minimum);
    }

    #[test]
    fn virial_vanishes_at_stationary_scale() {
        let lambda = stationary_scale_squared(E2, E4, 0.0).unwrap().sqrt();
        let h = 1e-6;
        let slope = (scaling_energy(lambda + h, E2, E4, 0.0, 3)
            - scaling_energy(lambda - h, E2, E4, 0.0, 3))
            / (2.0 * h);
        assert!(slope.abs() < 1e-6, "dE/dlambda(lambda*) = {}", slope);
        // Independent root search (V=0: dE/dlambda = E2 - E4/lambda^2).
        let bisected = bisect(|l| E2 - E4 / (l * l), 0.5, 1.0, 1e-12).unwrap();
        assert!(approx(bisected, lambda, 1e-9), "bisect = {}, closed form = {}", bisected, lambda);
    }

    #[test]
    fn minimum_fields_are_consistent() {
        let m = scaling_minimum(E2, E4, 0.0).unwrap();
        assert!(approx(m.virial_at_one, E2 - E4, 1e-12));
        assert!(approx(m.curvature, 2.0 * E4, 1e-12));
        assert!(approx(second_derivative(E2, E4, 0.0, 3), 2.0 * E4, 1e-12));
        assert!(approx(scaling_energy(1.0, E2, E4, 0.0, 3), E2 + E4, 1e-12));
    }

    #[test]
    fn stationary_point_conditions() {
        assert!(stationary_scale_squared(1.0, 0.0, 0.0).is_none());
        assert!(stationary_scale_squared(1.0, -1.0, 0.0).is_none());
        assert!(stationary_scale_squared(1.0, -1.0, 1.0).is_none()); // disc < 0
        assert!(stationary_scale_squared(0.0, 1.0, 0.0).is_none());
        let y = stationary_scale_squared(1.0, 1.0, 1.0).unwrap();
        assert!(approx(y, (-1.0 + 13.0_f64.sqrt()) / 6.0, 1e-12));
        assert!(scaling_minimum(1.0, 1.0, 1.0).unwrap().is_minimum);
    }

    #[test]
    fn checks_have_stable_unique_ids() {
        let rows = checks();
        assert!(rows.len() >= 4);
        let mut ids: Vec<&str> = rows.iter().map(|c| c.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), rows.len());
        assert!(rows.iter().all(|c| c.value.is_finite()));
    }
}
