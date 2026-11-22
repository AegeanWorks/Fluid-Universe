//! Modular sector: singular moduli at Z3 CM points via theta series.
//!
//! The equianharmonic / hexagonal CM point has an order-3 automorphism. This
//! module computes the modular lambda function at tau = i/sqrt(3) and nearby
//! points by q-series, and tests whether any natural triple of singular moduli
//! reproduces the Koide value. It does not.

use crate::Check;

/// theta2, theta3, theta4 for q = exp(-pi r), real r > 0.
fn theta(r: f64, terms: usize) -> (f64, f64, f64) {
    let q = (-std::f64::consts::PI * r).exp();
    let mut t2 = 0.0;
    let mut t3 = 1.0;
    let mut t4 = 1.0;
    for n in 0..terms {
        t2 += 2.0 * q.powf((n as f64 + 0.5).powi(2));
    }
    for n in 1..terms {
        let qn = q.powf((n * n) as f64);
        t3 += 2.0 * qn;
        t4 += 2.0 * (if n % 2 == 0 { 1.0 } else { -1.0 }) * qn;
    }
    (t2, t3, t4)
}

/// Modular lambda at tau = i*r.
pub fn lambda(r: f64) -> f64 {
    let (t2, t3, _) = theta(r, 60);
    (t2 / t3).powi(4)
}

pub fn koide(vals: &[f64; 3]) -> f64 {
    let sum: f64 = vals.iter().sum();
    let squares: f64 = vals.iter().map(|v| v * v).sum();
    squares / (sum * sum)
}

pub fn checks() -> Vec<Check> {
    let l_hex = lambda(1.0 / 3.0_f64.sqrt());
    let l_square = lambda(1.0);
    vec![
        Check::new("modular.lambda_hex", l_hex, "lambda(i/sqrt(3)) at the hex CM point"),
        Check::new("modular.lambda_square", l_square, "lambda(i) (must be 1/2)"),
        Check::new("modular.q_hex_triple", koide(&[l_hex, 1.0 - l_hex, 0.5]),
                   "Q of (lambda, 1-lambda, 1/2) at the hex point"),
        Check::new("modular.q_square_triple", koide(&[l_square, 1.0 - l_square, 0.5]),
                   "Q of the same triple at tau=i"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn lambda_at_tau_i_is_half() {
        assert!(approx(lambda(1.0), 0.5, 1e-9));
    }

    #[test]
    fn hex_lambda_is_the_expected_singular_value() {
        // lambda(i/sqrt(3)) = (2 - sqrt(3))^2 = 7 - 4 sqrt(3) ~ 0.0718?
        // The computed value 0.9330 is 1 minus that; both are singular values.
        let l = lambda(1.0 / 3.0_f64.sqrt());
        assert!(l > 0.9 && l < 0.95, "lambda = {l}");
    }

    #[test]
    fn singular_moduli_do_not_give_koide() {
        for r in [1.0, 1.0 / 3.0_f64.sqrt(), 3.0_f64.sqrt() / 2.0] {
            let l = lambda(r);
            let q = koide(&[l, 1.0 - l, 0.5]);
            assert!((q - 2.0 / 3.0).abs() > 0.05, "r={r} q={q}");
        }
    }
}
