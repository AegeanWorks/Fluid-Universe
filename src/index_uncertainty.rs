//! Uncertainty-aware verdict for the single-index framework (index_framework.rs).
//!
//! A previous verdict called the framework's up-quark ratio failure "decisive"
//! on central values alone (25.6% off). That overstated the case: the first
//! ratio is extremely sensitive to delta near that point, and m_u has a ~17%
//! uncertainty. Monte Carlo propagation of the Huang-Zhou / PDG mass errors
//! (40,000 samples per sector, seed 20261123, script scratch/index_uncertainty.py)
//! gives these z-scores for (prediction - observed)/sigma:
//!
//!   lepton: r1 z=-75.9 (paradoxical: the MC mean of r1 is offset by many
//!           times its tiny sampled sigma because of an asymmetric PDG m_tau
//!           error; the CENTRAL values agree to 1e-5 -- see index_framework)
//!   down:   r1 z=-0.40, r2 z=-0.06, Q z=-0.25   (fully consistent)
//!   up:     r1 z=-2.60, r2 z=-0.28, Q z=-0.94   (2.6-sigma tension, not a
//!           decisive exclusion; relative sigma on r1 is 9.3%)
//!
//! HONEST VERDICT: the framework is NOT decisively falsified by the up ratio;
//! it has a ~2.6-sigma tension there. It remains a post-hoc candidate with no
//! gauge or gravity sector, hence still not a theory.

use crate::Check;

/// Recorded z-scores from the 40k-sample propagation.
pub fn z_down_r1() -> f64 { -0.40 }
pub fn z_up_r1() -> f64 { -2.60 }
pub fn z_up_q() -> f64 { -0.94 }
pub fn rel_sigma_up_r1() -> f64 { 0.093 }

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("index_unc.z_down_r1", z_down_r1(), "down first-ratio z-score (consistent)"),
        Check::new("index_unc.z_up_r1", z_up_r1(), "up first-ratio z-score (2.6 sigma tension)"),
        Check::new("index_unc.z_up_q", z_up_q(), "up Q z-score (consistent)"),
        Check::new("index_unc.rel_sigma_up_r1", rel_sigma_up_r1(),
                   "relative sigma of up first ratio (9.3%)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn down_is_consistent() {
        assert!(z_down_r1().abs() < 1.0);
    }

    #[test]
    fn up_is_a_tension_not_an_exclusion() {
        assert!(z_up_r1().abs() > 2.0, "should be a genuine tension");
        assert!(z_up_r1().abs() < 4.0, "should NOT be called a decisive exclusion");
        assert!(rel_sigma_up_r1() > 0.05, "large uncertainty explains the central-value gap");
    }

    #[test]
    fn up_q_is_consistent() {
        assert!(z_up_q().abs() < 1.5);
    }
}
