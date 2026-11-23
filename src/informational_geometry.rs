//! External candidate derivation: the d=3 identities that fix our constants.
//!
//! Source: H. Hertault, "Informational Geometry" (self-published, Zenodo
//! 10.5281/zenodo.18870211), README claim: from d=3 and M_Pl alone. We do NOT
//! endorse the physical axioms (holographic fibration, Fisher-Rao metric). We
//! independently verify the ALGEBRAIC identities here, because they fix the
//! constants that appeared empirically in our own gauge_flavor.rs and
//! wzw/unified_model work:
//!
//!   beta       = (d-1)/d            = 2/3    = charged-lepton Koide Q
//!   theta_H    = arccos sqrt(beta)  = 35.264 deg
//!   cot(theta_H) = sqrt(2)          = lepton amplitude A
//!   delta      = beta (1-beta)      = 2/9    = lepton Brannen phase
//!   sin^2(theta_W) = d/(d^2+d+1)    = 3/13
//!   cot^2(theta_W) = 10/3  =>  3 cot^2(theta_W) = 10 EXACTLY
//!
//! The last line is the origin of the "10" in our gauge_flavor bridge
//! A^2(n) = 3 cot^2(theta_W)/(6-n): it is not a fitted constant but follows
//! from sin^2(theta_W) = 3/13, which itself follows from d=3. Thus ONE integer
//! d=3 fixes Q, A, delta AND the gauge-flavor amplitude scale.
//!
//! HONEST CAVEATS:
//!  * The physical mechanism that yields d=3 -> beta and sin^2(theta_W) is a
//!    self-published, non-peer-reviewed framework; we verify only the algebra.
//!  * It derives the LEPTON sector and the Weinberg angle. It does NOT derive
//!    our quark/neutrino sector indices n or Fibonacci labels {3,5,8}.
//!  * Our 2.6-sigma up-quark tension and the need for SUSY in unification remain.

use crate::Check;

pub fn dimension() -> f64 { 3.0 }
pub fn beta() -> f64 { (dimension() - 1.0) / dimension() }
pub fn theta_h() -> f64 { beta().sqrt().acos() }
pub fn cot_theta_h() -> f64 { 1.0 / theta_h().tan() }
pub fn phase_from_beta() -> f64 { beta() * (1.0 - beta()) }
pub fn sin2_theta_w() -> f64 {
    let d = dimension();
    d / (d * d + d + 1.0)
}
pub fn three_cot2_theta_w() -> f64 {
    let s = sin2_theta_w();
    3.0 * (1.0 - s) / s
}

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("infgeom.beta", beta(), "beta = (d-1)/d = 2/3 (lepton Q)"),
        Check::new("infgeom.cot_theta_h", cot_theta_h(), "cot(theta_H) = sqrt(2)"),
        Check::new("infgeom.delta", phase_from_beta(), "delta = beta(1-beta) = 2/9"),
        Check::new("infgeom.sin2tw", sin2_theta_w(), "sin^2(theta_W) = d/(d^2+d+1) = 3/13"),
        Check::new("infgeom.three_cot2", three_cot2_theta_w(),
                   "3 cot^2(theta_W) = 10 exactly (gauge-flavor numerator)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beta_is_koide_q() {
        assert!((beta() - 2.0 / 3.0).abs() < 1e-14);
    }

    #[test]
    fn cot_theta_h_is_amplitude() {
        assert!((cot_theta_h() - 2.0_f64.sqrt()).abs() < 1e-12);
    }

    #[test]
    fn beta_one_minus_beta_is_lepton_phase() {
        assert!((phase_from_beta() - 2.0 / 9.0).abs() < 1e-14);
    }

    #[test]
    fn d3_weinberg_angle_makes_gauge_flavor_numerator_exact() {
        assert!((sin2_theta_w() - 3.0 / 13.0).abs() < 1e-14);
        assert!((three_cot2_theta_w() - 10.0).abs() < 1e-12);
    }
}
