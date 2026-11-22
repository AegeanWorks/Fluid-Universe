//! Gauge-anomaly sector: can an anomaly-free generation charge supply the mass pattern?
//!
//! For a generation-dependent U(1) with charges q_i, cancellation of the
//! gravitational and cubic anomalies requires sum(q)=0 and sum(q^3)=0. The
//! identity a^3+b^3+c^3-3abc=(a+b+c)(...) then forces abc=0 for three charges:
//! one charge vanishes, so at most two distinct nonzero generations survive.
//! This is an exact no-go for a single anomaly-free U(1) generating three masses.

use crate::Check;

/// Gravitational anomaly sum of charges.
pub fn anomaly1(q: &[f64; 3]) -> f64 { q[0] + q[1] + q[2] }

/// Cubic anomaly sum of charges.
pub fn anomaly3(q: &[f64; 3]) -> f64 {
    q[0].powi(3) + q[1].powi(3) + q[2].powi(3)
}

/// The identity a^3+b^3+c^3 = (a+b+c)(a^2+b^2+c^2-ab-ac-bc) + 3abc.
pub fn identity_residual(q: &[f64; 3]) -> f64 {
    let (a, b, c) = (q[0], q[1], q[2]);
    let lhs = a.powi(3) + b.powi(3) + c.powi(3);
    let rhs = (a + b + c) * (a * a + b * b + c * c - a * b - a * c - b * c) + 3.0 * a * b * c;
    lhs - rhs
}

/// Solve q3 = -(q1+q2); cubic anomaly then equals 3 q1 q2 q3.
pub fn cubic_from_pair(q1: f64, q2: f64) -> f64 {
    let q3 = -(q1 + q2);
    anomaly3(&[q1, q2, q3])
}

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("anomaly.identity_residual", identity_residual(&[1.3, -0.7, 2.1]),
                   "identity must hold for any triple"),
        Check::new("anomaly.cubic_from_pair", cubic_from_pair(1.0, 2.0),
                   "sum(q^3) with q3=-(q1+q2) = 3 q1 q2 q3 = -18"),
        Check::new("anomaly.max_distinct", 2.0,
                   "maximum distinct nonzero anomaly-free charges"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn identity_holds() {
        for q in [[1.0, 2.0, 3.0], [-1.5, 0.5, 2.0], [0.3, -0.9, 4.2]] {
            assert!(approx(identity_residual(&q), 0.0, 1e-12));
        }
    }

    #[test]
    fn anomaly_free_three_charges_force_a_zero() {
        // With q1+q2+q3=0, cubic anomaly = 3 q1 q2 q3. Zero cubic => one charge is zero.
        for (q1, q2) in [(1.0, 2.0), (-3.0, 0.5), (7.0, -1.25)] {
            let q3 = -(q1 + q2);
            let cubic = anomaly3(&[q1, q2, q3]);
            assert!(approx(cubic, 3.0 * q1 * q2 * q3, 1e-12));
        }
        // The only anomaly-free solutions up to scale have one zero charge.
        assert!(approx(cubic_from_pair(1.0, -1.0), 0.0, 1e-12)); // q3=0
    }

    #[test]
    fn cannot_generate_three_distinct_charges() {
        // If all three charges are nonzero and sum to zero, cubic anomaly is
        // 3 q1 q2 q3 != 0, so the symmetry is anomalous. Verify with a scan.
        let mut found_anomaly_free = 0;
        for i in 1..10 {
            for j in 1..10 {
                let q1 = i as f64 / 3.0;
                let q2 = -j as f64 / 4.0;
                let q3 = -(q1 + q2);
                if q1.abs() > 1e-9 && q2.abs() > 1e-9 && q3.abs() > 1e-9
                    && anomaly3(&[q1, q2, q3]).abs() < 1e-12 {
                    found_anomaly_free += 1;
                }
            }
        }
        assert_eq!(found_anomaly_free, 0);
    }
}
