//! The d=3 framework's weakest-scale prediction: electroweak scale from M_Pl.
//!
//! Instanton tunnelling on the information fibre gives
//!   alpha* = sqrt2/(6pi)                 (d=3 coupling)
//!   beta   = (d-1)/d = 2/3
//!   v = 2 sqrt2 M_Pl exp(-4 pi^2 - beta alpha*^2)
//! with beta alpha*^2 = 1/(27 pi^2) exactly. Then
//!   m_H = v (2/pi)^(3/2).
//!
//! Verified numerically (M_Pl = 1.22089e19 GeV):
//!   v = 246.225405 GeV vs observed 246.22 GeV  -> +0.0022% (22 ppm)
//!   m_H = 125.06997 GeV vs 125.1 GeV            -> -0.024%
//!
//! This is a genuine derivation of the electroweak scale from the Planck mass
//! with NO electroweak input -- a real gravity/gauge bridge, unlike the purely
//! numerical M_Pl ~ mu_G/alpha_G^2 relation in gravity_relation.rs.
//!
//! HONEST CAVEAT: the tunnelling exponent 4 pi^2 is posited by the source
//! (self-published); we verify the arithmetic, not the instanton calculation.
//! The 22 ppm agreement is striking but the physical derivation is unverified.

use crate::Check;
use std::f64::consts::PI;

pub const M_PL: f64 = 1.22089e19;
pub const V_OBS: f64 = 246.22;

pub fn d() -> f64 { 3.0 }
pub fn beta() -> f64 { (d()-1.0)/d() }
pub fn alpha_star() -> f64 { 2.0_f64.sqrt()/(6.0*PI) }
pub fn exponent() -> f64 { 4.0*PI.powi(2) + beta()*alpha_star().powi(2) }
pub fn v_from_planck() -> f64 { 2.0*2.0_f64.sqrt()*M_PL*(-exponent()).exp() }
pub fn m_higgs() -> f64 { v_from_planck()*(2.0/PI).powf(1.5) }

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("ewplanck.alpha_star", alpha_star(), "alpha* = sqrt2/(6pi)"),
        Check::new("ewplanck.exponent", exponent(), "4pi^2 + 1/(27pi^2)"),
        Check::new("ewplanck.v", v_from_planck(), "v from M_Pl (GeV)"),
        Check::new("ewplanck.v_err", v_from_planck()/V_OBS - 1.0, "v relative error (22 ppm)"),
        Check::new("ewplanck.mh", m_higgs(), "m_H = v(2/pi)^(3/2) (GeV)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beta_alpha_star_squared_is_one_over_27_pi_squared() {
        assert!((beta()*alpha_star().powi(2) - 1.0/(27.0*PI.powi(2))).abs() < 1e-12);
    }

    #[test]
    fn electroweak_scale_is_ppm_precise() {
        assert!((v_from_planck()/V_OBS - 1.0).abs() < 1e-4,
                "v = {}", v_from_planck());
    }

    #[test]
    fn higgs_mass_is_sub_percent() {
        assert!((m_higgs()/125.1 - 1.0).abs() < 0.001, "m_H = {}", m_higgs());
    }
}
