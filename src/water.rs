//! Shallow-water acoustic-horizon closed forms (claim 27, ADR-018).
//!
//! Dependency-free port of `theory/water_analogue.py`. For long-wavelength
//! waves over a radial drain, `c = sqrt(g h)` and `v_r = Q / (2 pi r h)`, so
//! the acoustic horizon sits at `r_H = Q / (2 pi h c)` with surface gravity
//! `kappa = c^2 / r_H` and analogue temperature `T_H = hbar kappa / (2 pi kB)`.
//! Capillary dispersion breaks the acoustic long-wavelength limit at short
//! scales, hence the capillary length `l_c = sqrt(gamma / (rho g))` and the
//! dimensionless comparison `l_c^2 / r_H^2`.
//!
//! Every function is pure, so the two-process audit in `verify` reproduces
//! each value bit-for-bit.

use crate::Check;

/// Standard gravity in m/s^2, as used by `theory/water_analogue.py`.
pub const GRAVITY: f64 = 9.81;
/// Reduced Planck constant in J s (CODATA 2018).
pub const HBAR: f64 = 1.054571817e-34;
/// Boltzmann constant in J/K (CODATA 2018, exact).
pub const BOLTZMANN: f64 = 1.380649e-23;

/// Shallow-water sound speed `c = sqrt(g h)`. Panics for non-positive inputs,
/// matching the `ValueError` guard in the Python source.
pub fn sound_speed(depth: f64, gravity: f64) -> f64 {
    assert!(depth > 0.0 && gravity > 0.0, "depth and gravity must be positive");
    (gravity * depth).sqrt()
}

/// Acoustic horizon radius `r_H = Q / (2 pi h sqrt(g h))`, where the radial
/// flow speed equals the sound speed.
pub fn horizon_radius(flow: f64, depth: f64, gravity: f64) -> f64 {
    assert!(flow > 0.0, "flow must be positive");
    let speed = sound_speed(depth, gravity);
    flow / (2.0 * std::f64::consts::PI * depth * speed)
}

/// Surface gravity of the acoustic horizon `kappa = c^2 / r_H`.
pub fn surface_gravity(flow: f64, depth: f64, gravity: f64) -> f64 {
    let radius = horizon_radius(flow, depth, gravity);
    let speed = sound_speed(depth, gravity);
    speed * speed / radius
}

/// Analogue Hawking temperature `T_H = hbar kappa / (2 pi k_B)`, in kelvin.
pub fn hawking_temperature(kappa: f64) -> f64 {
    assert!(kappa > 0.0, "kappa must be positive");
    HBAR * kappa / (2.0 * std::f64::consts::PI * BOLTZMANN)
}

/// Capillary length `l_c = sqrt(gamma / (rho g))`, the scale at which surface
/// tension overcomes gravity.
pub fn capillary_length(gamma: f64, density: f64, gravity: f64) -> f64 {
    assert!(
        gamma > 0.0 && density > 0.0 && gravity > 0.0,
        "gamma, density and gravity must be positive"
    );
    (gamma / (density * gravity)).sqrt()
}

/// Squared capillary length to horizon radius, `(length / radius)^2`. Values
/// below unity mark a horizon smaller than the capillary scale, where the
/// acoustic (long-wavelength) approximation ceases to hold.
pub fn capillary_ratio(radius: f64, length: f64) -> f64 {
    assert!(radius > 0.0 && length > 0.0, "radius and length must be positive");
    let ratio = length / radius;
    ratio * ratio
}

/// Reproducible result lines for this module, with stable ids (the order is
/// part of the audit fingerprint). Recorded point: `h = 0.1 m`, `Q = 1e-3
/// m^3/s`, `g = 9.81 m/s^2`, water at room temperature.
pub fn checks() -> Vec<Check> {
    let depth = 0.1;
    let flow = 1e-3;
    let speed = sound_speed(depth, GRAVITY);
    let radius = horizon_radius(flow, depth, GRAVITY);
    let kappa = surface_gravity(flow, depth, GRAVITY);
    let length = capillary_length(0.0728, 998.0, GRAVITY);

    vec![
        Check::new(
            "water.sound_speed",
            speed,
            "c = sqrt(g h) at h=0.1 m, g=9.81 m/s^2",
        ),
        Check::new(
            "water.horizon_radius",
            radius,
            "r_H = Q / (2 pi h c) at Q=1e-3, h=0.1",
        ),
        Check::new(
            "water.surface_gravity",
            kappa,
            "kappa = c^2 / r_H at the recorded horizon",
        ),
        Check::new(
            "water.hawking_temperature",
            hawking_temperature(kappa),
            "T_H = hbar kappa / (2 pi kB) in kelvin",
        ),
        Check::new(
            "water.capillary_length",
            length,
            "l_c = sqrt(gamma/(rho g)) for water (gamma=0.0728, rho=998)",
        ),
        Check::new(
            "water.capillary_ratio",
            capillary_ratio(radius, length),
            "(l_c / r_H)^2: capillary versus acoustic-horizon scale",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn sound_speed_matches_shallow_water_reference() {
        let c = sound_speed(0.1, GRAVITY);
        assert!(approx(c, 0.9904544411531507, 1e-14));
        // c^2 = g h exactly, and c scales as sqrt(h).
        assert!(approx(c * c, 0.981, 1e-14));
        assert!(approx(sound_speed(0.4, GRAVITY) / c, 2.0, 1e-14));
    }

    #[test]
    fn horizon_and_surface_gravity_match_reference() {
        let r = horizon_radius(1e-3, 0.1, GRAVITY);
        assert!(approx(r, 1.6068880756049408e-3, 1e-14));
        let k = surface_gravity(1e-3, 0.1, GRAVITY);
        assert!(approx(k, 610.4967825034645, 1e-12));
        // kappa = c^2 / r_H and r_H is linear in the flow rate.
        assert!(approx(k, 0.981 / r, 1e-12));
        assert!(approx(horizon_radius(2e-3, 0.1, GRAVITY), 2.0 * r, 1e-14));
    }

    #[test]
    fn hawking_temperature_matches_reference() {
        let k = surface_gravity(1e-3, 0.1, GRAVITY);
        let t = hawking_temperature(k);
        assert!(approx(t, 7.421580272821032e-10, 1e-12));
        // The temperature is linear in kappa.
        assert!(approx(hawking_temperature(2.0 * k), 2.0 * t, 1e-14));
        assert_eq!(hawking_temperature(f64::INFINITY), f64::INFINITY);
    }

    #[test]
    fn capillary_length_and_ratio_match_reference() {
        let l = capillary_length(0.0728, 998.0, GRAVITY);
        assert!(approx(l, 2.726879300973213e-3, 1e-14));
        let r = horizon_radius(1e-3, 0.1, GRAVITY);
        assert!(approx(capillary_ratio(r, l), 2.8797883783539286, 1e-12));
        // The ratio is unity when the two scales coincide and is symmetric
        // under simultaneous rescaling.
        assert!(approx(capillary_ratio(l, l), 1.0, 1e-14));
        assert!(approx(capillary_ratio(10.0 * r, 10.0 * l), capillary_ratio(r, l), 1e-12));
    }

    #[test]
    fn checks_are_stable_unique_and_match_references() {
        let first = checks();
        let second = checks();
        assert_eq!(first.len(), 6);
        let mut ids = std::collections::HashSet::new();
        for (a, b) in first.iter().zip(second.iter()) {
            assert_eq!(a.id, b.id);
            assert_eq!(a.value.to_bits(), b.value.to_bits());
            assert_eq!(a.detail, b.detail);
            assert!(a.id.starts_with("water."));
            assert!(ids.insert(a.id));
        }
        let by_id = |id: &str| first.iter().find(|c| c.id == id).unwrap().value;
        assert!(approx(by_id("water.sound_speed"), 0.9904544411531507, 1e-14));
        assert!(approx(by_id("water.horizon_radius"), 1.6068880756049408e-3, 1e-14));
        assert!(approx(by_id("water.surface_gravity"), 610.4967825034645, 1e-12));
        assert!(approx(by_id("water.hawking_temperature"), 7.421580272821032e-10, 1e-12));
        assert!(approx(by_id("water.capillary_length"), 2.726879300973213e-3, 1e-14));
        assert!(approx(by_id("water.capillary_ratio"), 2.8797883783539286, 1e-12));
    }
}
