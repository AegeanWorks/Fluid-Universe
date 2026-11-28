//! Conditional WZW weight differences for selected phase targets.
//!
//! Systematic search over SU(N)_k conformal weights (N=2..6, k=1..24,
//! scratch/wzw_exact_phases.py) found exact weight differences
//! |h_nu - 2 h_box| equal to two required phases:
//!   delta_lepton = 2/9  = SU(3)_3, channel boxbar (h=2/9, |2/9-4/9|=2/9)
//!   delta_down   = 1/10 = SU(2)_13, singlet channel (h=0, h_box=1/20,
//!                    label |0-2/20| = 1/10)
//! The composite identity 2/39 = (2/9)*(3/13) is also exact arithmetic.
//!
//! The two examples use different groups and levels, with assignments chosen
//! after the targets were known. Their physical interpretation is unestablished.
//! The level-62 adjoint claim formerly made here was arithmetically incorrect.

use crate::Check;

pub fn delta_lepton_label() -> f64 {
    let h_box: f64 = 2.0/9.0;  // SU(3)_3
    (h_box - 2.0*h_box).abs()
}

pub fn delta_down_label() -> f64 {
    let h_box: f64 = 1.0/20.0;  // SU(2)_13: (3/4)/(13+2)
    (0.0 - 2.0*h_box).abs()
}

pub fn delta_up_composite() -> f64 {
    (2.0/9.0) * (3.0/13.0)
}

/// Historical level used by the retracted SU(3)_62 adjoint claim.
pub fn up_braiding_level_su3() -> f64 { 62.0 }

/// SU(3)_62 adjoint Casimir weight difference: 1/195, not 2/39.
pub fn up_braiding_label_from_level() -> f64 {
    // C2(box)=4/3 and C2(adjoint)=3, so the numerator is 1/3.
    (1.0/3.0) / (up_braiding_level_su3() + 3.0)
}

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("exactlabel.lepton", delta_lepton_label(), "SU(3)_3 boxbar weight difference = 2/9"),
        Check::new("exactlabel.down", delta_down_label(), "SU(2)_13 singlet label = 1/10"),
        Check::new("exactlabel.up", delta_up_composite(), "composite (2/9)(3/13) = 2/39"),
        Check::new("exactlabel.13", 3.0_f64*3.0+3.0+1.0, "13 = d^2+d+1 (SU(2) level)"),
        Check::new("exactlabel.up_level", up_braiding_level_su3(),
                   "historical SU(3) level 62; adjoint difference is 1/195"),
        Check::new("exactlabel.up_level_difference", up_braiding_label_from_level(),
                   "SU(3)_62 adjoint Casimir difference = 1/195"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_62_adjoint_uses_the_recorded_casimir_formula() {
        let (numerator, denominator) = crate::single_model_test::braiding_fraction(3, 62, &[1, 1]);
        assert_eq!(numerator * 195, denominator);
        assert!((up_braiding_label_from_level() - 1.0 / 195.0).abs() < 1e-14);
        assert!((up_braiding_label_from_level() - 2.0 / 39.0).abs() > 0.01);
    }

    #[test]
    fn lepton_pair_channel_is_antifundamental_not_fundamental() {
        let f = (1, 0);
        assert!(crate::monodromy_block::fusion(f, f, f).abs() < 1e-9);
        assert!((crate::monodromy_block::fusion(f, f, (0, 1)) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn lepton_and_down_are_exact_labels() {
        assert!((delta_lepton_label() - 2.0/9.0).abs() < 1e-14);
        assert!((delta_down_label() - 0.1).abs() < 1e-14);
    }

    #[test]
    fn up_is_composite() {
        assert!((delta_up_composite() - 2.0/39.0).abs() < 1e-14);
    }

    #[test]
    fn historical_level_is_retained() {
        assert!((up_braiding_level_su3() - 62.0).abs() < 1e-9);
        assert!(up_braiding_level_su3() > 13.0);
    }
}
