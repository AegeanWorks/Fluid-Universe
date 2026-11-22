//! Puddle and film geometry: can it fix the depth required by the Koide ratio?
//!
//! Earlier water work found that E_cap/E_grav = 1/2 at the phase-speed minimum
//! requires h/l_c = 2.007084679. The large-puddle capillary surface has a
//! maximum height h = 2 l_c sin(theta/2), which cannot exceed 2 l_c. This module
//! checks the exact depth analytically and proves the geometric exclusion.

use crate::Check;
use crate::linalg::bisect;
use crate::water_mechanics::capillary_length;
use std::f64::consts::PI;

/// Right-hand side of sinh(y) = 3 y, whose positive root fixes the depth.
fn root_equation(y: f64) -> f64 {
    y.sinh() - 3.0 * y
}

/// Positive nonzero root of sinh(y) = 3y (y = 0 is trivial).
pub fn root_y() -> f64 {
    bisect(root_equation, 1.0, 5.0, 1e-14).unwrap_or(f64::NAN)
}

/// Exact depth/l_c at which q = k l_c = 1/sqrt(2) at the phase-speed minimum.
/// From the stationarity condition with q=1/sqrt2: sinh(2x)=6x, x=d/sqrt2.
pub fn required_depth_over_lc() -> f64 {
    // y = 2x and d = sqrt(2) x, so d = y / sqrt(2).
    root_y() / std::f64::consts::SQRT_2
}

/// Maximum height of a large two-dimensional puddle of water on a flat surface.
pub fn max_puddle_height_over_lc(contact_angle_rad: f64) -> f64 {
    2.0 * (contact_angle_rad / 2.0).sin().clamp(0.0, 1.0)
}

pub fn checks() -> Vec<Check> {
    let lc = capillary_length();
    let required = required_depth_over_lc();
    let puddle_max = max_puddle_height_over_lc(PI);
    vec![
        Check::new("puddle.root_y", root_y(), "positive root of sinh(y)=3y"),
        Check::new("puddle.required_depth", required,
                   "exact h/l_c for E_cap/E_grav = 1/2 at the phase-speed minimum"),
        Check::new("puddle.max_height_over_lc", puddle_max,
                   "maximum large-puddle height h/l_c (theta=180 deg)"),
        Check::new("puddle.exclusion_gap", required - puddle_max,
                   "required depth minus the geometric maximum; positive means impossible"),
        Check::new("puddle.required_depth_meters", required * lc, "required depth in metres"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn root_is_correct() {
        let y = root_y();
        assert!(approx(y.sinh(), 3.0 * y, 1e-10), "y={y}");
        assert!(y > 2.0 && y < 4.0);
    }

    #[test]
    fn exact_depth_matches_numeric_scan() {
        // The golden-section water scan found 2.007084679301.
        assert!(approx(required_depth_over_lc(), 2.007084679301, 1e-8));
    }

    #[test]
    fn puddle_cannot_reach_the_required_depth() {
        assert!(required_depth_over_lc() > 2.0);
        assert!(approx(max_puddle_height_over_lc(PI), 2.0, 1e-12));
        assert!(required_depth_over_lc() - max_puddle_height_over_lc(PI) > 0.007);
        for theta in [0.0, PI / 2.0, PI] {
            assert!(max_puddle_height_over_lc(theta) <= 2.0 + 1e-12);
        }
    }

    #[test]
    fn thin_film_is_below_the_capillary_length() {
        assert!(max_puddle_height_over_lc(PI / 3.0) < 1.0);
    }
}
