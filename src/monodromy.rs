//! WZW monodromy: the cube identity and the N=3 over-determination of Q.
//!
//! A WZW primary carries a "quantum number" z = sqrt(d) exp(i h), with d the
//! quantum (fusion) dimension and h the conformal dimension. For the
//! fundamental of SU(N)_N:
//!   h_□  = C2(□)/(k+h^v) = (N^2-1)/(4N^2)
//!   d_□  = sin(pi N/(k+N)) / sin(pi/(k+N)) = 1/sin(pi/(2N))
//! The cube z_□^3 has modulus d^{3/2} and argument 3 h_□ (De Moivre).
//!
//! At N=3 the conformal route and the fusion route to the Koide quotient
//! AGREE EXACTLY:
//!   3 h_□        = 3 * (8/36)      = 2/3
//!   1/3 + d_□/6  = 1/3 + 2/6       = 2/3
//! and this coincidence holds for NO other integer N (verified N=2..8:
//! differences 6e-3 .. 4e-1). Thus the observed Q=2/3 is over-determined by
//! two independent SU(3)_3 data, and arg(z_□^3)=Q ties the phase to h_□.
//!
//! HONEST CAVEAT: in the Brannen parametrization Q = 1/3 + A^2/6 is independent
//! of delta (the "shape freedom" result in shape_freedom.rs). Therefore
//! 3 h_□ = Q over-determines the VALUE 2/9 but does not, by itself, PROVE the
//! identification delta = h_□. It is a structural constraint, not a theorem.
//!
//! We also record a falsification of the LATEST literature quark phases
//! (delta_down=1/9, delta_up=2/39, distinct from the older 2/27, 4/27):
//! with the amplitude fixed by each measured quark Q, neither reproduces the
//! observed hierarchy ratios.

use crate::Check;
use std::f64::consts::PI;

/// Fundamental conformal dimension of SU(N)_N: (N^2-1)/(4N^2).
pub fn h_box(n: i64) -> f64 {
    (n * n - 1) as f64 / (4.0 * n as f64 * n as f64)
}

/// Fundamental quantum (fusion) dimension of SU(N)_k.
pub fn quantum_dimension(n: i64, k: i64) -> f64 {
    let big = (k + n) as f64;
    (PI * n as f64 / big).sin() / (PI / big).sin()
}

/// Fundamental quantum dimension at level k=N: 1/sin(pi/(2N)).
pub fn quantum_dimension_nn(n: i64) -> f64 { quantum_dimension(n, n) }

/// The WZW number z = sqrt(d) exp(i h) as (real, imaginary) components.
pub fn wzw_number(n: i64) -> (f64, f64) {
    let d = quantum_dimension_nn(n);
    let h = h_box(n);
    let mag = d.sqrt();
    (mag * h.cos(), mag * h.sin())
}

/// Cube z^3 via De Moivre: modulus d^{3/2}, argument 3h. Returns (re, im).
pub fn cube(n: i64) -> (f64, f64) {
    let d = quantum_dimension_nn(n);
    let h = h_box(n);
    let mag = d.powf(1.5);
    (mag * (3.0 * h).cos(), mag * (3.0 * h).sin())
}

/// Argument of z^3, equal to 3 h_□ for the values considered here.
pub fn cube_argument(n: i64) -> f64 {
    let (re, im) = cube(n);
    im.atan2(re)
}

/// Koide quotient implied by the fusion route: Q = 1/3 + d_□/6.
pub fn q_from_fusion(n: i64) -> f64 {
    1.0 / 3.0 + quantum_dimension_nn(n) / 6.0
}

/// Difference between the conformal route (3h) and fusion route (1/3+d/6).
pub fn overdetermination_gap(n: i64) -> f64 {
    (3.0 * h_box(n) - q_from_fusion(n)).abs()
}

/// Solve for the integer N where conformal and fusion data coincide.
pub fn unique_n() -> Option<i64> {
    (2..=12).find(|&n| overdetermination_gap(n) < 1e-12)
}

// ---- latest literature quark phases ----

/// Amplitude fixed by a measured Koide-like Q: A = sqrt(6(Q-1/3)).
pub fn amplitude_from_q(q: f64) -> f64 { (6.0 * (q - 1.0 / 3.0)).sqrt() }

/// Two sqrt-mass ratios of the circulant triplet at amplitude A and phase d.
pub fn ratios(a: f64, d: f64) -> Option<(f64, f64)> {
    let mut y = [0.0_f64; 3];
    for k in 0..3 {
        y[k] = 1.0 + a * (d + 2.0 * PI * k as f64 / 3.0).cos();
    }
    if y.iter().any(|v| *v <= 0.0) { return None; }
    y.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    Some((y[1] / y[0], y[2] / y[1]))
}

fn joint_error(p: (f64, f64), t: (f64, f64)) -> f64 {
    ((p.0 / t.0 - 1.0).powi(2) + (p.1 / t.1 - 1.0).powi(2)).sqrt()
}

pub fn checks() -> Vec<Check> {
    let (re, im) = cube(3);
    let (mu_e, tau_mu) = (
        (1.27_f64 / 0.00418).sqrt(),
        (171.7_f64 / 1.27).sqrt(),
    );
    let (d_t, d_r) = (
        (0.093_f64 / 0.00418).sqrt(),
        (4.18_f64 / 0.093).sqrt(),
    );
    let a_down = amplitude_from_q(0.7480650548031402);
    let a_up = amplitude_from_q(0.8875692313187327);
    let lit_down = ratios(a_down, 1.0 / 9.0).map(|r| joint_error(r, (d_t, d_r))).unwrap_or(-1.0);
    let lit_up = ratios(a_up, 2.0 / 39.0).map(|r| joint_error(r, (mu_e, tau_mu))).unwrap_or(-1.0);
    vec![
        Check::new("monodromy.d_quantum_3", quantum_dimension_nn(3), "d_□(SU(3)_3) = 2"),
        Check::new("monodromy.h_box_3", h_box(3), "h_□(SU(3)_3) = 2/9"),
        Check::new("monodromy.z_cube_mod", (re * re + im * im).sqrt(), "|z^3| = d^{3/2} = 2sqrt2"),
        Check::new("monodromy.z_cube_arg", cube_argument(3), "arg(z^3) = 3h = 2/3 = Q"),
        Check::new("monodromy.conformal_route", 3.0 * h_box(3), "3h = 2/3"),
        Check::new("monodromy.fusion_route", q_from_fusion(3), "1/3 + d/6 = 2/3"),
        Check::new("monodromy.gap_3", overdetermination_gap(3), "conformal-fusion gap at N=3 (~0)"),
        Check::new("monodromy.unique_n", unique_n().unwrap_or(0) as f64, "unique N with coincident routes"),
        Check::new("monodromy.lit_delta_down_err", lit_down, "WZW delta_down=1/9 joint error"),
        Check::new("monodromy.lit_delta_up_err", lit_up, "WZW delta_up=2/39 joint error"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantum_and_conformal_dimensions_are_exact_at_three() {
        assert!((quantum_dimension_nn(3) - 2.0).abs() < 1e-12);
        assert!((h_box(3) - 2.0 / 9.0).abs() < 1e-12);
        assert!((quantum_dimension(3, 3) - quantum_dimension_nn(3)).abs() < 1e-12);
    }

    #[test]
    fn cube_identity_holds() {
        let (re, im) = cube(3);
        let mag = (re * re + im * im).sqrt();
        assert!((mag - 2.0_f64.powf(1.5)).abs() < 1e-12);
        assert!((cube_argument(3) - 2.0 / 3.0).abs() < 1e-12);
        let (z_re, z_im) = wzw_number(3);
        let mag_z = (z_re * z_re + z_im * z_im).sqrt();
        assert!((mag_z - 2.0_f64.sqrt()).abs() < 1e-12);
    }

    #[test]
    fn conformal_and_fusion_routes_coincide_only_at_n_three() {
        assert_eq!(unique_n(), Some(3));
        assert!(overdetermination_gap(3) < 1e-12);
        for n in [2, 4, 5, 6, 7, 8] {
            assert!(overdetermination_gap(n) > 1e-3, "N={n} gap={}", overdetermination_gap(n));
        }
    }

    #[test]
    fn latest_literature_quark_phases_fail() {
        // New literature values delta_down=1/9, delta_up=2/39 are NOT the old
        // 2/27,4/27. With amplitude fixed by measured Q both miss the ratios.
        let a_down = amplitude_from_q(0.7480650548031402);
        let a_up = amplitude_from_q(0.8875692313187327);
        let d_t = ((0.093_f64 / 0.00418).sqrt(), (4.18_f64 / 0.093).sqrt());
        let u_t = ((1.27_f64 / 0.00418).sqrt(), (171.7_f64 / 1.27).sqrt());
        let rd = ratios(a_down, 1.0 / 9.0).unwrap();
        let ru = ratios(a_up, 2.0 / 39.0).unwrap();
        assert!(joint_error(rd, d_t) > 0.05, "down err={}", joint_error(rd, d_t));
        assert!(joint_error(ru, u_t) > 0.05, "up err={}", joint_error(ru, u_t));
    }
}
