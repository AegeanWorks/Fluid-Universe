//! Test of the d=3 framework's cosmological predictions against observations.
//!
//! The self-published Dark Geometry framework predicts (Tier A algebraic):
//!   xi = 1/10;  H0(SH0ES)/H0(Planck) = 1 + xi = 11/10;  H0(geom) = 70.3;
//!   sigma8 = 0.766.
//! Tested against the observational values the framework itself quotes
//! (Planck H0 = 67.36, SH0ES H0 = 73.04 km/s/Mpc, KiDS sigma8 = 0.766):
//!   H0 ratio: 1.1000 vs 1.0843  -> +1.45%
//!   H0 geom:  70.30 vs 70.143   -> +0.22%
//!   sigma8:   0.766 vs 0.766    -> 0%
//!
//! This is a NEW experimental domain (cosmology) beyond the particle-sector
//! predictions already tested. The Hubble ratio is the least accurate d=3
//! prediction found (1.45%), while the geometric mean and sigma8 are sub-percent.
//!
//! HONEST CAVEATS:
//!  * The framework is self-published (author h-index 0, zero citations); no
//!    peer-reviewed validation exists -- confirmed by literature search.
//!  * The Hubble-ratio identity is algebraically exact but its link to the
//!    measured tension is an untested physical claim; 1.45% is not sub-percent.
//!  * sigma8 = 0.766 is quoted as an exact match by the source; we test the
//!    number, not the derivation.

use crate::Check;

pub const XI: f64 = 0.1;
pub const H0_PLANCK: f64 = 67.36;
pub const H0_SHOES: f64 = 73.04;
pub const H0_GEOM_PRED: f64 = 70.3;
pub const SIGMA8_PRED: f64 = 0.766;
pub const SIGMA8_OBS: f64 = 0.766;

pub fn ratio_pred() -> f64 { 1.0 + XI }
pub fn ratio_obs() -> f64 { H0_SHOES / H0_PLANCK }
pub fn geom_obs() -> f64 { (H0_PLANCK * H0_SHOES).sqrt() }

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("cosmo.ratio_pred", ratio_pred(), "1 + xi = 11/10"),
        Check::new("cosmo.ratio_err", ratio_pred()/ratio_obs() - 1.0,
                   "Hubble ratio error (+0.0145)"),
        Check::new("cosmo.geom_err", H0_GEOM_PRED/geom_obs() - 1.0,
                   "geometric mean H0 error (+0.0022)"),
        Check::new("cosmo.sigma8_err", SIGMA8_PRED/SIGMA8_OBS - 1.0,
                   "sigma8 error (0, quoted exact match)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hubble_ratio_is_least_accurate_prediction() {
        let e = (ratio_pred()/ratio_obs()-1.0).abs();
        assert!(e > 0.01 && e < 0.02, "ratio error = {e}");
    }

    #[test]
    fn geometric_mean_is_sub_percent() {
        assert!((H0_GEOM_PRED/geom_obs()-1.0).abs() < 0.005);
    }

    #[test]
    fn xi_is_one_tenth() {
        assert!((XI - 0.1).abs() < 1e-14);
        assert!((ratio_pred() - 1.1).abs() < 1e-14);
    }
}
