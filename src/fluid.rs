//! Closed-form fluid / vortex / Marcus checks.
//!
//! Dependency-free port of the numeric result lines referenced by formulas
//! F22, F27, F33, F38, F40, F44 and F45 in `theory/formulas.md`. Everything in
//! here is an algebraic closed form: no quadrature, no solver, no Python and
//! no external crate. Each public function is pure so that the two-process
//! audit in `verify` can reproduce every value bit-for-bit.
//!
//! Conventions (the "recorded point" used by the checks):
//! `hbar = m = omega = 1`, hence diffusivity `nu = hbar / (2m) = 0.5` and the
//! Ermakov variance `sigma^2 = nu / omega = 0.5`.

use crate::Check;

/// Boltzmann constant in eV/K (CODATA 2018), as used by the water-Marcus scans.
const KB_EV_PER_K: f64 = 8.617333262145e-5;

/// Ermakov / Gross-Madelung Ritz fixed point: variance `sigma^2 = nu / omega`
/// (F22-adjacent, F33, F35). At the recorded point this is `0.5`.
pub fn ermakov_fixed_point(nu: f64, omega: f64) -> f64 {
    nu / omega
}

/// Gross-Madelung Gaussian width action `J_G(s2) = -m nu^2/(2 s2) - m omega^2 s2/2`
/// (F33). It is maximised at `s2 = nu / omega`; the maximum value is `-m nu omega`.
pub fn gm_action(s2: f64, m: f64, nu: f64, omega: f64) -> f64 {
    -m * nu * nu / (2.0 * s2) - m * omega * omega * s2 / 2.0
}

/// The two Pinney `B` coefficients of the pinned-endpoint quadratic at the
/// recorded point (`a0 = 0.5`, i.e. the monic quadratic carries `-1`):
/// `B = (-1 +/- sqrt(disc)) / 2`, returned positive branch first (F38/F40).
///
/// For `disc = 3` this gives `B1 = 0.3660254037844386` (admissible) and
/// `B2 = -1.3660254037844386` (the killed sibling with `s^2 < 0` mid-interval).
pub fn pinney_branch_b(disc: f64) -> [f64; 2] {
    let s = disc.sqrt();
    [(s - 1.0) / 2.0, -(1.0 + s) / 2.0]
}

/// Pinney determinant `a*c - b*b` (the first argument, named `ac` in the
/// contract, is the coefficient `a`). It equals `-nu^2` for the Yasue / c<0
/// bridge and `+nu^2` for the G-M bridge (F38, F40).
pub fn pinney_det(ac: f64, b: f64, c: f64) -> f64 {
    ac * c - b * b
}

/// Finite-core HO-1 velocity `v = -(hbar/m) eps / (x^2 + eps^2)` (F44).
/// Its magnitude is capped at `hbar/(m eps)`.
pub fn vortex_core_velocity(hbar_over_m: f64, eps: f64, x: f64) -> f64 {
    -(hbar_over_m) * eps / (x * x + eps * eps)
}

/// Analytic quantum potential at the HO-1 core centre `x = 0` (F44):
/// `Q(0) = -hbar^2/(2m) * (1/eps^2 - 1/s^2)`, finite for `eps > 0`.
pub fn vortex_core_q_zero(hbar: f64, m: f64, eps: f64, s: f64) -> f64 {
    -(hbar * hbar) / (2.0 * m) * (1.0 / (eps * eps) - 1.0 / (s * s))
}

/// 2p core-threading blob loop fraction `r^2 / (r^2 + delta^2)` (F45).
/// With `r = delta` it is `0.5`, with `r = 2 delta` it is `0.8`, with
/// `r = 20 delta` it is `400/401 ~ 0.997506`.
pub fn blob_loop_fraction(r: f64, delta: f64) -> f64 {
    let rr = r * r;
    rr / (rr + delta * delta)
}

/// Fixed-lambda classical Marcus FWHM in eV (F27, F28):
/// `2 sqrt(2 ln 2) sqrt(2 lambda kB T)`.
pub fn marcus_fwhm(lambda_ev: f64, temp_k: f64) -> f64 {
    2.0 * (2.0_f64.ln() * 2.0).sqrt() * (2.0 * lambda_ev * KB_EV_PER_K * temp_k).sqrt()
}

/// Local Koide ratio `sum(m) / sum(sqrt(m))^2`, kept here so the fluid checks
/// are self-contained (no dependency on the `circulant` module).
pub fn koide(masses: &[f64]) -> f64 {
    let sum: f64 = masses.iter().sum();
    let root_sum: f64 = masses.iter().map(|m| m.sqrt()).sum();
    sum / (root_sum * root_sum)
}

/// Reproducible result lines for this module, with stable ids (the order is
/// part of the audit fingerprint).
pub fn checks() -> Vec<Check> {
    // Recorded point: hbar = m = omega = 1, nu = 0.5, sigma^2 = 0.5.
    let nu = 0.5;
    let omega = 1.0;
    let s2 = ermakov_fixed_point(nu, omega);
    let [b1, b2] = pinney_branch_b(3.0);
    // Yasue Pinney coefficients at the recorded endpoints: A = s0^2 = 0.5,
    // C = (B^2 - nu^2) / A gives A*C - B^2 = -nu^2 = -0.25.
    let a = 0.5;
    let c = (b1 * b1 - nu * nu) / a;
    let delta = 0.1; // eps0 = delta0 = 0.1 in the pre-registered units.
    let lambda_ev = 1.6171325440878093;
    let masses = [0.51099895, 105.6583755, 1776.93];

    vec![
        Check::new(
            "fluid.ermakov_fixed_point",
            s2,
            "variance sigma^2 = nu/omega at nu=0.5, omega=1",
        ),
        Check::new(
            "fluid.gm_action_at_fixed_point",
            gm_action(s2, 1.0, nu, omega),
            "J_G(sigma^2*) = -m*nu*omega at the Ermakov fixed point",
        ),
        Check::new(
            "fluid.pinney_branch_b1",
            b1,
            "admissible Yasue branch B1 = (-1+sqrt(3))/2",
        ),
        Check::new(
            "fluid.pinney_branch_b2",
            b2,
            "killed sibling B2 = (-1-sqrt(3))/2",
        ),
        Check::new(
            "fluid.pinney_det_yasue",
            pinney_det(a, b1, c),
            "A*C - B^2 = -nu^2 for the c<0 Pinney bridge",
        ),
        Check::new(
            "fluid.vortex_core_velocity_max",
            vortex_core_velocity(1.0, delta, 0.0),
            "max finite core speed -(hbar/m)/eps at x=0, eps=0.1",
        ),
        Check::new(
            "fluid.vortex_core_q_zero",
            vortex_core_q_zero(1.0, 1.0, delta, 1.0),
            "analytic Q(0) = -0.5*(1/eps^2 - 1/s^2) at eps=0.1, s=1",
        ),
        Check::new(
            "fluid.blob_loop_fraction_delta",
            blob_loop_fraction(delta, delta),
            "blob loop fraction at r=delta",
        ),
        Check::new(
            "fluid.blob_loop_fraction_2delta",
            blob_loop_fraction(2.0 * delta, delta),
            "blob loop fraction at r=2*delta",
        ),
        Check::new(
            "fluid.blob_loop_fraction_20delta",
            blob_loop_fraction(20.0 * delta, delta),
            "blob loop fraction at r=20*delta",
        ),
        Check::new(
            "fluid.marcus_fwhm_300k",
            marcus_fwhm(lambda_ev, 300.0),
            "fixed-lambda Marcus FWHM (eV) at 300 K",
        ),
        Check::new(
            "fluid.koide_physical",
            koide(&masses),
            "Koide ratio for (m_e, m_mu, m_tau) in MeV",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn ermakov_fixed_point_and_gm_action_extremum() {
        let nu = 0.5;
        let omega = 1.0;
        let s2 = ermakov_fixed_point(nu, omega);
        assert!(approx(s2, 0.5, 1e-12));
        // The G-M action is stationary (and maximal) at the Ermakov variance.
        let at = gm_action(s2, 1.0, nu, omega);
        assert!(approx(at, -0.5, 1e-12));
        assert!(gm_action(0.4, 1.0, nu, omega) < at);
        assert!(gm_action(0.6, 1.0, nu, omega) < at);
        // Scaling: with omega = 2 and m = 2 the location is still nu/omega.
        assert!(approx(ermakov_fixed_point(nu, 2.0), 0.25, 1e-12));
    }

    #[test]
    fn pinney_branches_match_recorded_reference() {
        let [b1, b2] = pinney_branch_b(3.0);
        assert!(approx(b1, 0.3660254037844386, 1e-14));
        assert!(approx(b2, -1.3660254037844386, 1e-14));
        assert!(b1 > b2); // positive branch first, as the contract requires.
        assert!(approx(b1 - b2, 3.0_f64.sqrt(), 1e-14));
    }

    #[test]
    fn pinney_det_is_minus_nu_squared_for_yasue() {
        let nu = 0.5;
        let a = 0.5;
        let b = pinney_branch_b(3.0)[0];
        let c = (b * b - nu * nu) / a;
        assert!(approx(pinney_det(a, b, c), -nu * nu, 1e-12));
        // The G-M bridge uses c = +nu^2 instead.
        let c_gm = (b * b + nu * nu) / a;
        assert!(approx(pinney_det(a, b, c_gm), nu * nu, 1e-12));
    }

    #[test]
    fn vortex_core_velocity_is_even_and_capped() {
        let eps = 0.1;
        let v0 = vortex_core_velocity(1.0, eps, 0.0);
        assert!(approx(v0, -10.0, 1e-12));
        assert!(approx(vortex_core_velocity(1.0, eps, -eps), v0 / 2.0, 1e-12));
        assert!(approx(vortex_core_velocity(1.0, eps, eps), v0 / 2.0, 1e-12));
        assert!(v0.abs() <= 1.0 / eps + 1e-12);
        assert!(approx(vortex_core_velocity(2.0, eps, 0.0), -20.0, 1e-12));
    }

    #[test]
    fn vortex_core_q_zero_matches_closed_form() {
        let q = vortex_core_q_zero(1.0, 1.0, 0.1, 1.0);
        assert!(approx(q, -49.5, 1e-12));
        // As s -> infinity the -1/s^2 term vanishes.
        assert!(approx(vortex_core_q_zero(1.0, 1.0, 0.1, 1e6), -50.0, 1e-6));
    }

    #[test]
    fn blob_loop_fractions_match_frozen_values() {
        let d = 0.1;
        assert!(approx(blob_loop_fraction(d, d), 0.5, 1e-14));
        assert!(approx(blob_loop_fraction(2.0 * d, d), 0.8, 1e-14));
        assert!(approx(blob_loop_fraction(20.0 * d, d), 400.0 / 401.0, 1e-14));
        assert!(blob_loop_fraction(20.0 * d, d) > blob_loop_fraction(2.0 * d, d));
    }

    #[test]
    fn marcus_fwhm_reference_and_sqrt_t_scaling() {
        let lam = 1.6171325440878093;
        let w = marcus_fwhm(lam, 300.0);
        assert!(approx(w, 0.6809145309595547, 1e-10));
        assert!(approx(marcus_fwhm(lam, 600.0) / w, 2.0_f64.sqrt(), 1e-12));
        assert_eq!(marcus_fwhm(lam, 0.0), 0.0);
    }

    #[test]
    fn koide_matches_two_thirds_and_single_value() {
        let masses = [0.51099895, 105.6583755, 1776.93];
        assert!(approx(koide(&masses), 2.0 / 3.0, 1e-4));
        assert!(approx(koide(&[1.0, 1.0, 1.0]), 1.0 / 3.0, 1e-14));
    }

    #[test]
    fn checks_are_stable_unique_and_match_references() {
        let first = checks();
        let second = checks();
        assert_eq!(first.len(), second.len());
        let mut ids = std::collections::HashSet::new();
        for (a, b) in first.iter().zip(second.iter()) {
            assert_eq!(a.id, b.id);
            assert_eq!(a.value.to_bits(), b.value.to_bits());
            assert_eq!(a.detail, b.detail);
            assert!(a.id.starts_with("fluid."));
            assert!(ids.insert(a.id));
        }
        let by_id = |id: &str| first.iter().find(|c| c.id == id).unwrap().value;
        assert!(approx(by_id("fluid.ermakov_fixed_point"), 0.5, 1e-12));
        assert!(approx(by_id("fluid.pinney_branch_b1"), 0.3660254037844386, 1e-14));
        assert!(approx(by_id("fluid.pinney_branch_b2"), -1.3660254037844386, 1e-14));
        assert!(approx(by_id("fluid.pinney_det_yasue"), -0.25, 1e-12));
        assert!(approx(by_id("fluid.blob_loop_fraction_delta"), 0.5, 1e-14));
        assert!(approx(by_id("fluid.blob_loop_fraction_2delta"), 0.8, 1e-14));
        assert!(approx(by_id("fluid.marcus_fwhm_300k"), 0.6809145309595547, 1e-10));
    }
}
