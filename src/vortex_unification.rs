//! Vortex-core and loop sector: does a pre-registered radius supply 1/sqrt(2)?
//!
//! The blob profile u_theta = Gamma/(2 pi) * r/(r^2 + delta^2) has a loop
//! circulation fraction f(r) = r^2/(r^2 + delta^2). The Koide value 2/3 occurs
//! at r = sqrt(2) delta. This module checks whether that radius coincides with
//! any length fixed by the model (core scale delta_0 = 0.1 a0, the 2p most
//! probable radius 4 a0, or the 5 a0 radial mean), without fitting.

use crate::Check;

/// Loop circulation fraction of the blob profile.
pub fn blob_fraction(r: f64, delta: f64) -> f64 {
    r * r / (r * r + delta * delta)
}

/// Radius at which the blob fraction reaches a target value in (0,1).
pub fn radius_for_fraction(target: f64, delta: f64) -> f64 {
    delta * (target / (1.0 - target)).sqrt()
}

/// Radius fixed by the Koide value 2/3, in units of the core delta.
pub fn koide_radius_over_delta() -> f64 {
    radius_for_fraction(2.0 / 3.0, 1.0)
}

/// Core velocity maximum relative to the singular control at the same radius:
/// v_blob/v_singular = f(r), so the Koide value is again r = sqrt(2) delta.
pub fn velocity_ratio(r: f64, delta: f64) -> f64 {
    blob_fraction(r, delta)
}

pub fn checks() -> Vec<Check> {
    let delta0 = 0.1; // pre-registered core in units of a0
    let r_koide = koide_radius_over_delta() * delta0;
    let r_most_probable = 4.0; // 2p radial maximum in a0
    let r_radial_mean = 5.0; // <r> for 2p = (a0/2)(3n^2 - l(l+1)) = 5 a0
    vec![
        Check::new("vortex_unification.koide_radius_over_delta", koide_radius_over_delta(),
                   "r/delta where the blob loop fraction equals 2/3"),
        Check::new("vortex_unification.koide_radius_in_a0", r_koide,
                   "same radius in a0 for delta0=0.1 a0"),
        Check::new("vortex_unification.ratio_to_most_probable", r_koide / r_most_probable,
                   "Koide radius divided by the 2p most-probable radius"),
        Check::new("vortex_unification.ratio_to_radial_mean", r_koide / r_radial_mean,
                   "Koide radius divided by the 2p radial mean"),
        Check::new("vortex_unification.fraction_at_2delta", blob_fraction(2.0, 1.0),
                   "frozen blob fraction at r=2 delta (must be 0.8)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn koide_radius_is_sqrt_two_delta() {
        assert!(approx(koide_radius_over_delta(), 2.0_f64.sqrt(), 1e-14));
    }

    #[test]
    fn fraction_and_velocity_ratio_agree() {
        for r in [0.1, 0.5, 1.0, 2.0, 10.0] {
            assert!(approx(blob_fraction(r, 1.0), velocity_ratio(r, 1.0), 1e-14));
        }
    }

    #[test]
    fn no_pre_registered_radius_matches() {
        let r = koide_radius_over_delta() * 0.1;
        for fixed in [0.1_f64, 1.0, 4.0, 6.0_f64.sqrt()] {
            assert!((r - fixed).abs() > 1e-3 * fixed.max(1.0), "r={r} fixed={fixed}");
        }
    }

    #[test]
    fn frozen_fractions_are_reproduced() {
        assert!(approx(blob_fraction(1.0, 1.0), 0.5, 1e-14));
        assert!(approx(blob_fraction(2.0, 1.0), 0.8, 1e-14));
        assert!(approx(blob_fraction(20.0, 1.0), 400.0 / 401.0, 1e-12));
    }
}
