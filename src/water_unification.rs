//! Independent water fixation of h = 2 l_c.
//!
//! Deep-water gravity-capillary phase speed c(k)^2 = g/k + (sigma/rho) k has a
//! minimum c_min^2 = 2 sqrt(g sigma/rho). The shallow-water speed is c_s^2 = g h.
//! Setting c_s = c_min gives h = 2 sqrt(sigma/(rho g)) = 2 l_c exactly: a depth
//! fixed by water alone, with no reference to the Koide relation. This module
//! then evaluates the capillary/gravity energy ratio at that depth and asks how
//! close it is to the Koide value 1/2.

use crate::Check;
use crate::water_mechanics::{G, RHO, SIGMA, capillary_length, energy_ratio_at_minimum, minimum_wave_number};

/// Minimum phase speed squared of the deep-water gravity-capillary relation.
pub fn min_phase_speed_squared() -> f64 {
    2.0 * (G * SIGMA / RHO).sqrt()
}

/// Shallow-water phase speed squared at depth h.
pub fn shallow_speed_squared(h: f64) -> f64 {
    G * h
}

/// Depth matching the shallow-water speed to the capillary-gravity minimum.
pub fn matching_depth() -> f64 {
    min_phase_speed_squared() / G
}

/// Capillary/gravity energy ratio at the phase-speed minimum of the matching depth.
pub fn ratio_at_matching_depth() -> f64 {
    energy_ratio_at_minimum(matching_depth())
}

pub fn checks() -> Vec<Check> {
    let lc = capillary_length();
    let depth = matching_depth();
    let ratio = ratio_at_matching_depth();
    let k = minimum_wave_number(depth);
    vec![
        Check::new("water_unification.min_speed_squared", min_phase_speed_squared(),
                   "min_k (g/k + sigma k/rho)"),
        Check::new("water_unification.matching_depth_over_lc", depth / lc,
                   "depth where shallow speed equals the capillary-gravity minimum"),
        Check::new("water_unification.ratio_at_match", ratio,
                   "E_cap/E_grav at the phase-speed minimum of that depth"),
        Check::new("water_unification.ratio_deviation", (ratio - 0.5).abs(),
                   "distance of the ratio from the Koide value 1/2"),
        Check::new("water_unification.k_lc_at_match", k * lc,
                   "dimensionless minimum wave number at the matching depth"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn matching_depth_is_exactly_two_capillary_lengths() {
        assert!(approx(matching_depth(), 2.0 * capillary_length(), 1e-14));
    }

    #[test]
    fn speeds_match_at_that_depth() {
        let h = matching_depth();
        assert!(approx(shallow_speed_squared(h), min_phase_speed_squared(), 1e-12));
    }

    #[test]
    fn ratio_at_match_is_close_but_not_equal_to_half() {
        let ratio = ratio_at_matching_depth();
        assert!(ratio > 0.48 && ratio < 0.5, "ratio = {ratio}");
        assert!((ratio - 0.5).abs() > 1e-3, "must stay a near-miss, ratio = {ratio}");
    }

    #[test]
    fn deep_water_limit_is_unity() {
        assert!((energy_ratio_at_minimum(1e4 * capillary_length()) - 1.0).abs() < 1e-3);
    }
}
