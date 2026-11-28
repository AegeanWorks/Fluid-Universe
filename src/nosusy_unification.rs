//! No-SUSY unification test using the d=3 gravity relation as boundary.
//!
//! Proposal: replace the SUSY assumption with the d=3 gauge-gravity relation
//! M_Pl = mu_G/alpha_G^2, i.e. alpha_G = sqrt(mu_G/M_Pl), and ask whether the
//! SM (non-SUSY) one-loop running of g1,g2,g3 delivers all three couplings
//! equal to alpha_G at some common scale.
//!
//! Result (honest negative): with SM beta coefficients b=(41/10,-19/6,-7),
//! the three inverse couplings never meet: the minimum spread across scales
//! is ~3.64 (at mu ~ 2.3e14 GeV). The gravity-predicted 1/alpha_G also does
//! not coincide with the common value. Thus the d=3 gravity relation does
//! NOT replace SUSY for gauge unification; the earlier MSSM unification
//! (mu_G ~ 2e16 GeV, so10_gut.rs) still requires the SUSY assumption.
//!
//! This is recorded as a closed alternative, not a step toward a theory.

use crate::Check;

pub const M_Z: f64 = 91.1876;
pub const M_PL: f64 = 1.22089e19;
pub const INV_ALPHA_MZ: [f64;3] = [59.075, 29.652, 8.577];
pub const B_SM: [f64;3] = [41.0/10.0, -19.0/6.0, -7.0];

pub fn inv_alpha_at(mu: f64) -> [f64;3] {
    let t=(mu/M_Z).ln();
    let mut out=[0.0_f64;3];
    for i in 0..3 { out[i]=INV_ALPHA_MZ[i] - B_SM[i]/(2.0*std::f64::consts::PI)*t; }
    out
}

pub fn spread(mu: f64) -> f64 {
    let v=inv_alpha_at(mu);
    v.iter().copied().fold(f64::NEG_INFINITY, f64::max)
      - v.iter().copied().fold(f64::INFINITY, f64::min)
}

pub fn min_spread_scale() -> f64 { 2.344e14 }
pub fn alpha_grav(mu: f64) -> f64 { (mu/M_PL).sqrt() }

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("nosusy.min_spread", spread(min_spread_scale()),
                   "minimum SM coupling spread (~3.64, no unification)"),
        Check::new("nosusy.spread_ratio", spread(min_spread_scale())/INV_ALPHA_MZ[0],
                   "spread relative to 1/alpha_1 (~0.06)"),
        Check::new("nosusy.alpha_grav", alpha_grav(2.0e16),
                   "gravity alpha_G at 2e16 GeV (~0.004, not 0.041)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sm_couplings_do_not_unify() {
        assert!(spread(min_spread_scale()) > 1.0, "spread = {}", spread(min_spread_scale()));
    }

    #[test]
    fn gravity_alpha_is_wrong_scale() {
        // The d=3 relation M_Pl=mu_G/alpha_G^2 with mu_G=2e16 gives
        // alpha_G ~ sqrt(2e16/1.22e19) = 0.0405, but the couplings at that
        // scale sit near 1/alpha ~ 38-46 (alpha ~ 0.022-0.026), not 0.041.
        let a=alpha_grav(2.0e16);
        assert!(a>0.03 && a<0.05);
        let v=inv_alpha_at(2.0e16);
        assert!(1.0/v[0].max(1.0) < a, "couplings do not match gravity value");
    }
}
