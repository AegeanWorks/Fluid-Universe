//! Exact reformulation: Q = 2/3 means CV = 1 for the three signed roots.
//!
//! For any three positive numbers x1,x2,x3 with mean mu and population standard
//! deviation sigma, one has Q = (1 + (sigma/mu)^2)/3. Therefore Koide's value is
//! equivalent to sigma = mu (unit coefficient of variation). This module verifies
//! the identity, measures CV for the physical lepton roots, and scans whether any
//! parameter-free distribution of three samples supplies CV = 1 without fitting.

use crate::Check;

/// Koide Q of three positive numbers.
pub fn q(x: &[f64; 3]) -> f64 {
    let sum: f64 = x.iter().sum();
    let squares: f64 = x.iter().map(|v| v * v).sum();
    squares / (sum * sum)
}

/// Population coefficient of variation sigma/mu of three positive numbers.
pub fn cv(x: &[f64; 3]) -> f64 {
    let mu: f64 = x.iter().sum::<f64>() / 3.0;
    if mu <= 0.0 { return f64::NAN; }
    let variance: f64 = x.iter().map(|v| (v - mu) * (v - mu)).sum::<f64>() / 3.0;
    variance.sqrt() / mu
}

/// The identity Q = (1 + CV^2)/3, with CV computed from the same triple.
pub fn q_from_cv(x: &[f64; 3]) -> f64 {
    (1.0 + cv(x) * cv(x)) / 3.0
}

/// Deterministic LCG for distribution scans; no external RNG crate.
fn lcg(state: &mut u64) -> f64 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    ((*state >> 33) as f64) / ((1u64 << 31) as f64)
}

/// Mean sample CV over `count` triples drawn from a unit exponential distribution.
pub fn mean_cv_exponential(count: usize, seed: u64) -> f64 {
    let mut state = seed;
    let mut total = 0.0;
    for _ in 0..count {
        let mut x = [0.0; 3];
        for value in x.iter_mut() {
            let u = lcg(&mut state).max(1e-12);
            *value = -u.ln();
        }
        total += cv(&x);
    }
    total / count as f64
}

/// Mean sample CV over `count` triples from a unit-mean lognormal with shape s.
pub fn mean_cv_lognormal(shape: f64, count: usize, seed: u64) -> f64 {
    let mut state = seed;
    let mut total = 0.0;
    for _ in 0..count {
        let mut x = [0.0; 3];
        for (index, value) in x.iter_mut().enumerate() {
            let u1 = lcg(&mut state).max(1e-12);
            let u2 = lcg(&mut state);
            let z = (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos();
            let log_mu = -0.5 * shape * shape;
            if index == 0 { /* keep the shape parameter explicit below */ }
            *value = (log_mu + shape * z).exp();
        }
        total += cv(&x);
    }
    total / count as f64
}

pub fn checks() -> Vec<Check> {
    let leptons = [0.51099895_f64.sqrt(), 105.6583755_f64.sqrt(), 1776.93_f64.sqrt()];
    let exp_cv = mean_cv_exponential(20000, 20261122);
    vec![
        Check::new("variation.lepton_cv", cv(&leptons),
                   "population CV of the three physical signed roots"),
        Check::new("variation.lepton_q_identity", q_from_cv(&leptons),
                   "Q reconstructed from CV (must equal measured Q)"),
        Check::new("variation.lepton_q", q(&leptons), "direct Koide Q"),
        Check::new("variation.exponential_mean_cv", exp_cv,
                   "mean sample CV of three unit-exponential draws"),
        Check::new("variation.lognormal_shape1_cv", mean_cv_lognormal(1.0, 20000, 7),
                   "mean sample CV of a unit-mean lognormal, shape 1"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn identity_holds_for_arbitrary_triples() {
        for x in [[1.0, 2.0, 3.0], [0.5, 10.0, 100.0], [1.0, 1.0, 1.0],
                  [2.414, 0.293, 0.293]] {
            assert!(approx(q(&x), q_from_cv(&x), 1e-12), "x={x:?}");
        }
    }

    #[test]
    fn koide_point_is_unit_cv() {
        let r = 2.0_f64.powf(-0.5);
        let roots = [1.0 + 2.0 * r, 1.0 - r, 1.0 - r];
        assert!(approx(q(&roots), 2.0 / 3.0, 1e-12));
        assert!(approx(cv(&roots), 1.0, 1e-12));
    }

    #[test]
    fn physical_leptons_have_near_unit_cv() {
        let leptons = [0.51099895_f64.sqrt(), 105.6583755_f64.sqrt(), 1776.93_f64.sqrt()];
        assert!((cv(&leptons) - 1.0).abs() < 1e-4, "CV = {}", cv(&leptons));
    }

    #[test]
    fn exponential_population_cv_is_one_but_sample_cv_is_not() {
        let sample = mean_cv_exponential(20000, 123);
        assert!(sample > 0.5 && sample < 1.2, "sample CV = {sample}");
        assert!((sample - 1.0).abs() > 0.01, "no exact unit sample CV");
    }

    #[test]
    fn lognormal_cv_depends_on_shape() {
        let a = mean_cv_lognormal(0.5, 5000, 1);
        let b = mean_cv_lognormal(1.5, 5000, 1);
        assert!(b > a);
    }
}
