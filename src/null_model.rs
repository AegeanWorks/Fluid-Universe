//! Statistical null model for the "rational amplitude" regularity.
//!
//! Earlier work highlighted that the required amplitudes are close to simple
//! rationals (A^2 = {2, 5/2, 10/3}, i.e. 1/A^2 = {1/2, 2/5, 3/10}). This module
//! tests whether that is statistically surprising using a null model of
//! 200,000 random log-uniform mass triplets with quark-like hierarchies
//! (scratch/null_model.py, seed 20261123).
//!
//! Result: the pattern is NOT significant.
//!  * 51.3% of random triplets have A^2 within 0.3% of SOME rational with
//!    denominator <= 12 (rationals are dense enough that this is common).
//!  * Median rational error for a random triplet: 0.288%.
//!  * The three observed deviations (0.000%, 0.402%, 0.236%) are behind the
//!    10th percentile of the null (0.049%) for two of the three sectors.
//!  * Approximating independence, P(all three equally "rational" by chance)
//!    ~ 0.135 -- not even 2 sigma.
//!
//! HONEST CONCLUSION: the rational-amplitude observation is an artifact of
//! rational density, not evidence for a discrete underlying principle. It is
//! retained in the record as a debunked regularity, alongside the cube identity
//! and gauge-gravity relation which have their own (separate) caveats.

use crate::Check;

/// Observed relative deviation of each amplitude from its nearest simple rational.
pub fn observed_rational_errors() -> [f64; 3] { [0.0, 0.00402, 0.00236] }

/// Null-model fraction of random triplets within 0.3% of a simple rational.
pub fn null_hit_fraction() -> f64 { 0.51303 }

/// Null-model median rational error.
pub fn null_median_error() -> f64 { 0.00288 }

/// Approximate probability that all three sectors are this "rational" by chance.
pub fn joint_chance_probability() -> f64 { 0.135 }

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("null.hit_fraction", null_hit_fraction(),
                   "51.3% of random triplets are this close to a rational"),
        Check::new("null.median_error", null_median_error(), "null median rational error"),
        Check::new("null.joint_p", joint_chance_probability(),
                   "P(all three rational by chance) ~ 0.135 (not significant)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rational_pattern_is_not_significant() {
        assert!(null_hit_fraction() > 0.5, "rational closeness is common in the null");
        assert!(joint_chance_probability() > 0.05, "joint pattern is not at 2-sigma");
    }

    #[test]
    fn observed_errors_are_not_all_extreme() {
        let errors = observed_rational_errors();
        assert!(errors[1] > null_median_error() * 0.5,
                "down amplitude rational error is not exceptional");
        assert!(errors[2] > null_median_error() * 0.5,
                "up amplitude rational error is not exceptional");
    }
}
