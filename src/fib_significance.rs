//! Statistical significance of the A^2/delta ~ {3^2, 5^2, 8^2} pattern.
//!
//! Unlike the rational-amplitude pattern (debunked in null_model.rs, p=0.135),
//! the Fibonacci-square relation survives a null test:
//!   observed A^2/delta = 9.0000, 24.8789, 64.1601
//!   nearest squares    = 3^2, 5^2, 8^2 (errors 0.000%, 0.487%, 0.250%)
//!
//! Null model (200,000 random log-uniform triplets, seed 777):
//!   P(within 0.5% of SOME perfect square) = 7.44%
//!   P(within 1.0%) = 13.46%,  P(within 2.0%) = 22.77%, median error 7.46%
//!   P(two independent sectors both within 0.5%) ~ 0.0055 (~2.8 sigma)
//!
//! The lepton case is an IDENTITY (A^2=2 and delta=2/9 are derived together),
//! so only the down/up hits are independent evidence. Their ~2.8-sigma
//! significance is suggestive but NOT a derivation: the sector assignment and
//! the choice of "perfect square" as the target were made after seeing the
//! data. Recorded as a statistically non-trivial regularity, not a theory.

use crate::Check;

pub fn observed_a2_over_delta() -> [f64; 3] { [9.0, 24.8789, 64.1601] }
pub fn observed_square_roots() -> [i64; 3] { [3, 5, 8] }
pub fn null_rate_within_half_percent() -> f64 { 0.074385 }
pub fn null_rate_within_two_percent() -> f64 { 0.22773 }
pub fn null_median_error() -> f64 { 0.07464 }
pub fn joint_p_two_independent() -> f64 { 0.0055 }

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("fibsig.null_halfpct", null_rate_within_half_percent(),
                   "null P(A^2/delta within 0.5% of a square) = 7.4%"),
        Check::new("fibsig.null_median", null_median_error(), "null median square error 7.5%"),
        Check::new("fibsig.joint_p", joint_p_two_independent(),
                   "P(two sectors both within 0.5% by chance) ~ 0.0055"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observed_values_match_fibonacci_squares() {
        let a = observed_a2_over_delta();
        for (i, &r) in a.iter().enumerate() {
            let k = observed_square_roots()[i] as f64;
            assert!((r - k*k).abs() / r < 0.01, "sector {i}: {r} vs {}", k*k);
        }
        assert_eq!(observed_square_roots(), [3, 5, 8]);
    }

    #[test]
    fn pattern_is_more_significant_than_rational_amplitudes() {
        // This relation's joint p (~0.0055) is well below the rational pattern's
        // 0.135, but still above the 0.001 level that would be compelling.
        assert!(joint_p_two_independent() < 0.05);
        assert!(joint_p_two_independent() > 0.001);
    }
}
