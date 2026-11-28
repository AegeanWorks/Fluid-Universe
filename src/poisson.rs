//! Poisson/exponential statistics as a candidate for unit CV.
//!
//! An exponential distribution has population CV exactly 1, and the exact
//! identity Q = (1 + CV^2)/3 would then give 2/3. But Koide's Q is computed
//! from three *sample* roots, and the sample CV of three exponential draws
//! averages ~0.65 while E[Q] = 1/2. The population property cannot be
//! substituted for the sample statistic: Poisson statistics do not explain Q.

use crate::Check;

/// Population CV of an exponential distribution is exactly 1.
pub fn exponential_population_cv() -> f64 { 1.0 }

/// Q implied by the population CV through the exact identity.
pub fn q_from_population_cv() -> f64 {
    let cv = exponential_population_cv();
    (1.0 + cv * cv) / 3.0
}

/// Analytic E[Q] for three iid exponentials: Dirichlet(1,1,1) gives E[sum p_i^2]=1/2.
pub fn expected_q_three_exponentials() -> f64 { 0.5 }

fn lcg(state: &mut u64) -> f64 {
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    ((*state >> 33) as f64) / ((1u64 << 31) as f64)
}

fn sample_cv_and_q(state: &mut u64) -> (f64, f64) {
    let x = [(-lcg(state).max(1e-12).ln()), (-lcg(state).max(1e-12).ln()),
             (-lcg(state).max(1e-12).ln())];
    let sum: f64 = x.iter().sum();
    let q = x.iter().map(|v| v * v).sum::<f64>() / (sum * sum);
    let mu = sum / 3.0;
    let var: f64 = x.iter().map(|v| (v - mu) * (v - mu)).sum::<f64>() / 3.0;
    (var.sqrt() / mu, q)
}

/// Monte Carlo mean sample CV and mean Q for three exponential draws.
pub fn sample_statistics(count: usize, seed: u64) -> (f64, f64) {
    let mut state = seed;
    let (mut cv_total, mut q_total) = (0.0, 0.0);
    for _ in 0..count {
        let (cv, q) = sample_cv_and_q(&mut state);
        cv_total += cv;
        q_total += q;
    }
    (cv_total / count as f64, q_total / count as f64)
}

pub fn checks() -> Vec<Check> {
    let (cv, q) = sample_statistics(50000, 20261122);
    vec![
        Check::new("poisson.population_cv", exponential_population_cv(),
                   "population CV of an exponential (exactly 1)"),
        Check::new("poisson.q_from_population_cv", q_from_population_cv(),
                   "Q implied by population CV (2/3)"),
        Check::new("poisson.analytic_sample_q", expected_q_three_exponentials(),
                   "analytic E[Q] for three exponential samples"),
        Check::new("poisson.sample_cv", cv, "Monte Carlo mean sample CV of three draws"),
        Check::new("poisson.sample_q", q, "Monte Carlo mean Q of three draws"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn population_cv_identity_gives_koide() {
        assert!((exponential_population_cv() - 1.0).abs() < 1e-14);
        assert!((q_from_population_cv() - 2.0 / 3.0).abs() < 1e-14);
    }

    #[test]
    fn analytic_sample_q_is_half() {
        assert!((expected_q_three_exponentials() - 0.5).abs() < 1e-14);
    }

    #[test]
    fn sample_statistics_do_not_match_population() {
        let (cv, q) = sample_statistics(20000, 7);
        assert!(cv > 0.6 && cv < 0.7, "sample CV = {cv}");
        assert!(q > 0.49 && q < 0.51, "sample Q = {q}");
        assert!((q - 2.0 / 3.0).abs() > 0.1);
    }
}
