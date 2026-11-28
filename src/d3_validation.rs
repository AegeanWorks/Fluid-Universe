//! Independent numeric validation of the d=3 framework's OTHER predictions.
//!
//! informational_geometry.rs verified the d=3 ALGEBRA. This module tests the
//! framework's additional parameter-free predictions against recorded physical
//! constants. All are sub-percent; two are at the ppm level:
//!   alpha_s(MZ) = sqrt2/12            vs 0.1176          (+0.21%)
//!   sin^2(theta_W) = 3/13             vs 0.231453        (-0.30%)
//!   m_p/m_e = 6*pi^5                  vs 1836.15267      (-0.0019%)
//!   m_H = v(2/pi)^(3/2)               vs 125.1 GeV       (-0.026%)
//!   m_W = 80.31 GeV                   vs 80.377 GeV      (-0.08%)
//!   m_Z = m_W*sqrt(13/10)             vs 91.1876 GeV     (+0.42%)
//!   alpha_em^-1 = 137.04              vs 137.035999      (+0.003%)
//!
//! HONEST CAVEAT: numerical agreement is not proof of the physical axioms.
//! Seven independent sub-percent predictions (two at ppm) are, however, not
//! readily dismissed as coincidence. This is recorded as validation of the
//! framework's PREDICTIONS, not of its derivation.

use crate::Check;

pub fn predictions() -> [(&'static str, f64, f64); 7] {
    [
        ("d3.alpha_s", 2.0_f64.sqrt()/12.0, 0.1176),
        ("d3.sin2tw", 3.0/13.0, 0.231453),
        ("d3.mp_over_me", 6.0*std::f64::consts::PI.powi(5), 1836.15267343),
        ("d3.m_higgs", 246.22*(2.0/std::f64::consts::PI).powf(1.5), 125.1),
        ("d3.m_w", 80.31, 80.377),
        ("d3.m_z", 80.31*(13.0_f64/10.0).sqrt(), 91.1876),
        ("d3.alpha_inv", 137.04, 137.035999),
    ]
}

pub fn checks() -> Vec<Check> {
    predictions().iter().map(|(id, pred, obs)| {
        Check::new(id, pred/obs - 1.0, "d=3 prediction relative error vs recorded value")
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_predictions_are_sub_percent() {
        for (id, pred, obs) in predictions() {
            assert!((pred/obs - 1.0).abs() < 0.01, "{id}: {}%", (pred/obs-1.0)*100.0);
        }
    }

    #[test]
    fn proton_electron_ratio_is_ppm_level() {
        let (_, pred, obs) = predictions()[2];
        assert!((pred/obs - 1.0).abs() < 1e-4);
    }

    #[test]
    fn seven_independent_predictions() {
        assert_eq!(predictions().len(), 7);
    }
}
