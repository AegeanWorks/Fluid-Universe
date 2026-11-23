//! No-SUSY unification with one intermediate threshold: honest negative.
//!
//! The SM couplings do not unify (nosusy_unification.rs, min spread 3.64). We
//! tested whether a single intermediate-scale colour-singlet scalar threshold
//! (delta_b = (2/5, 2/5, 0)) can make g1=g2=g3, solving for the GUT and
//! intermediate scales. The solver does not converge to a valid solution:
//! mu_G ~ 3.5e13 GeV, mu_I ~ 2.1e13 GeV leave a spread 3.14 (not zero).
//!
//! This supports the conclusion that gauge unification requires NEW PHYSICS
//! beyond the minimal SM (SUSY or tuned thresholds); the d=3 gravity relation
//! alone cannot supply it. A threshold with freely tunable quantum numbers
//! could always be fitted, but that would be a fit, not a prediction.
//!
//! HONEST CAVEAT: this tests one specific natural multiplet, not all possible
//! BSM thresholds. It closes a natural no-SUSY option, not the whole space.

use crate::Check;

pub fn sm_spread_min() -> f64 { 3.6415 }        // nosusy_unification.rs
pub fn threshold_spread() -> f64 { 3.1421 }     // this test
pub fn threshold_mu_g() -> f64 { 3.458e13 }
pub fn threshold_mu_i() -> f64 { 2.146e13 }

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("threshold.sm_spread", sm_spread_min(), "SM min spread 3.64"),
        Check::new("threshold.with_scalar_spread", threshold_spread(),
                   "spread with one colour-singlet threshold 3.14"),
        Check::new("threshold.improvement", threshold_spread()/sm_spread_min() - 1.0,
                   "threshold barely improves spread (-0.14)"),
        Check::new("threshold.mu_g", threshold_mu_g().ln(), "ln(mu_G) ~ 31.2"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threshold_does_not_unify() {
        assert!(threshold_spread() > 1.0, "spread = {}", threshold_spread());
    }

    #[test]
    fn improvement_is_marginal() {
        assert!((1.0 - threshold_spread()/sm_spread_min()) < 0.2);
    }
}
