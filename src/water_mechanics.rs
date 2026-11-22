//! Water mechanics: gravity-capillary dispersion and the shallow-water horizon.
//!
//! For a water wave omega^2 = (g k + (sigma/rho) k^3) tanh(k h). The gravitational
//! and capillary contributions to the restoring force are g k and (sigma/rho) k^3.
//! Their ratio at the phase-speed minimum is a dimensionless function only of
//! h / l_c, with l_c = sqrt(sigma/(rho g)) the capillary length. This module asks
//! whether a depth fixed by water itself makes that ratio 1/2, i.e. supplies
//! r = sqrt(E_cap / E_grav) = 1/sqrt(2) without fitting.

use crate::Check;

pub const G: f64 = 9.81;
pub const SIGMA: f64 = 0.0728;
pub const RHO: f64 = 998.0;

pub fn capillary_length() -> f64 {
    (SIGMA / (RHO * G)).sqrt()
}

/// Squared phase speed of a gravity-capillary wave of wave number k in depth h.
pub fn phase_speed_squared(k: f64, h: f64) -> f64 {
    if k <= 0.0 { return 0.0; }
    (G / k + SIGMA * k / RHO) * (k * h).tanh()
}

/// Wave number at the minimum phase speed (golden-section search on log k).
pub fn minimum_wave_number(h: f64) -> f64 {
    let lc = capillary_length();
    let k_c = 1.0 / lc;
    let mut lo = k_c * 1e-4;
    let mut hi = k_c * 1e2;
    let phi = (5.0_f64.sqrt() - 1.0) / 2.0;
    for _ in 0..200 {
        let a = hi - phi * (hi - lo);
        let b = lo + phi * (hi - lo);
        if phase_speed_squared(a, h) < phase_speed_squared(b, h) {
            hi = b;
        } else {
            lo = a;
        }
    }
    0.5 * (lo + hi)
}

/// E_cap / E_grav at the phase-speed minimum, equal to (k l_c)^2.
pub fn energy_ratio_at_minimum(h: f64) -> f64 {
    let k = minimum_wave_number(h);
    let lc = capillary_length();
    (k * lc) * (k * lc)
}

/// Depth at which the minimum-phase-speed energy ratio takes a target value.
pub fn depth_for_ratio(target: f64) -> f64 {
    let lc = capillary_length();
    let mut lo = 1e-6 * lc;
    let mut hi = 1e3 * lc;
    for _ in 0..200 {
        let mid = (lo * hi).sqrt();
        if energy_ratio_at_minimum(mid) < target { lo = mid; } else { hi = mid; }
    }
    (lo * hi).sqrt()
}

/// Shallow-water horizon quantities (long-wavelength, non-dispersive).
pub fn horizon(flow_rate: f64, depth: f64) -> (f64, f64, f64, f64) {
    let c = (G * depth).sqrt();
    let radius = flow_rate / (2.0 * std::f64::consts::PI * depth * c);
    let kappa = c * c / radius;
    let hawking = 1.054571817e-34 * kappa
        / (2.0 * std::f64::consts::PI * 1.380649e-23);
    (c, radius, kappa, hawking)
}

pub fn checks() -> Vec<Check> {
    let lc = capillary_length();
    let depth = depth_for_ratio(0.5);
    let ratio = energy_ratio_at_minimum(depth);
    let (c, radius, kappa, hawking) = horizon(1e-3, 0.1);
    vec![
        Check::new("water_mechanics.capillary_length", lc, "sqrt(sigma/(rho g))"),
        Check::new("water_mechanics.depth_for_half_ratio", depth / lc,
                   "depth/l_c where E_cap/E_grav = 1/2 at the phase-speed minimum"),
        Check::new("water_mechanics.ratio_at_that_depth", ratio,
                   "restored energy ratio (target 1/2)"),
        Check::new("water_mechanics.horizon_radius", radius, "r_H for Q=1e-3, h=0.1"),
        Check::new("water_mechanics.surface_gravity", kappa, "kappa = c^2/r_H"),
        Check::new("water_mechanics.hawking_temperature", hawking, "analogue T_H"),
        Check::new("water_mechanics.sound_speed", c, "sqrt(g h)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capillary_length_is_physical() {
        assert!((capillary_length() - 2.7268793e-3).abs() < 1e-9);
    }

    #[test]
    fn deep_water_minimum_has_equal_energy_shares() {
        let deep = 1e4 * capillary_length();
        assert!((energy_ratio_at_minimum(deep) - 1.0).abs() < 1e-3);
    }

    #[test]
    fn shallow_water_minimum_is_gravity_dominated() {
        let shallow = 1e-3 * capillary_length();
        assert!(energy_ratio_at_minimum(shallow) < 1e-3);
    }

    #[test]
    fn half_ratio_depth_is_determined() {
        let depth = depth_for_ratio(0.5);
        assert!((energy_ratio_at_minimum(depth) - 0.5).abs() < 1e-6);
        assert!(depth.is_finite() && depth > 0.0);
    }

    #[test]
    fn horizon_reproduces_known_numbers() {
        let (c, radius, kappa, _) = horizon(1e-3, 0.1);
        assert!((c - 0.990454441).abs() < 1e-8);
        assert!((radius - 1.606888076e-3).abs() < 1e-11);
        assert!((kappa - 610.4967825).abs() < 1e-5);
    }
}
