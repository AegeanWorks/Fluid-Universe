//! Test of the proposed derivation: run the d=3 fixed point 3/13 to M_Z.
//!
//! Proposal: d=3 gives sin^2(theta_W) = 3/13 at a fixed point; run it to M_Z
//! with the recorded one-loop slopes (1/g_i^2 = b_i + a_i ln mu) and check
//! whether sin^2(M_Z) = 0.23122 (PDG MS-bar).
//!
//! Result (honest): the fixed point sin^2=3/13 occurs at mu* = 79.50 GeV,
//! within 1.1% of M_W = 80.379 GeV (a genuine d=3 prediction of the W scale).
//! Running from mu* up to M_Z gives sin^2(M_Z) = 0.23145, NOT the PDG MS-bar
//! 0.23122 (0.10% mismatch). The recorded g1,g2 themselves give 0.23145, so
//! the discrepancy reflects the scheme/input difference between the project's
//! couplings and the PDG MS-bar extraction, not a failure of the d=3 value.
//!
//! VERDICT: the proposed derivation does NOT reproduce 0.23122 with the
//! recorded running; it does predict mu* ~= M_W. A^2_up from the PDG MS-bar
//! value (up_amplitude_gauge.rs, 0.016% precision) therefore remains a
//! gauge-flavor relation using a measured input, not a pure d=3 derivation.

use crate::Check;

pub const A1: f64 = -0.05193;
pub const B1: f64 = 4.9354;
pub const A2: f64 = 0.04011;
pub const B2: f64 = 2.1786;
pub const M_W: f64 = 80.379;
pub const M_Z: f64 = 91.1876;
pub const FIXED: f64 = 3.0/13.0;
pub const PDG_MSBAR: f64 = 0.23122;

pub fn sin2_at(mu: f64) -> f64 {
    let t=mu.ln();
    let g1sq=1.0/(B1+A1*t); let g2sq=1.0/(B2+A2*t);
    (3.0/5.0*g1sq)/(g2sq+3.0/5.0*g1sq)
}

/// mu* where sin^2 = 3/13, found by bisection at runtime.
pub fn fixed_point_scale() -> f64 {
    let (mut lo,mut hi)=(10.0_f64,200.0_f64);
    for _ in 0..200 {
        let mid=(lo+hi)/2.0;
        if sin2_at(mid)>FIXED { hi=mid; } else { lo=mid; }
    }
    (lo+hi)/2.0
}

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("d3run.mu_star", fixed_point_scale(), "scale where sin2=3/13 (GeV)"),
        Check::new("d3run.mu_over_mw", fixed_point_scale()/M_W, "mu*/M_W (~0.989)"),
        Check::new("d3run.sin2_mz", sin2_at(M_Z), "sin2(MZ) from running the fixed point"),
        Check::new("d3run.vs_pdg", sin2_at(M_Z)/PDG_MSBAR - 1.0,
                   "running result vs PDG MS-bar (~+0.001)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_point_is_near_w_mass() {
        let mu=fixed_point_scale();
        assert!((mu/M_W-1.0).abs()<0.02, "mu*={mu}");
    }

    #[test]
    fn running_does_not_reproduce_pdg_msbar() {
        // Honest negative: the proposed derivation misses PDG by ~0.1%.
        let err=(sin2_at(M_Z)/PDG_MSBAR-1.0).abs();
        assert!(err>5e-4 && err<2e-3, "err={err}");
    }
}
