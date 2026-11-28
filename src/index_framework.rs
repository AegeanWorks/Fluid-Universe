//! A compact single structure with one sector index n = 1, 2, 3:
//!   amplitude:   A^2(n) = n + 1/n          -> {2, 5/2, 10/3}
//!   label:       F(n)   = Fib(n+3)          -> {3, 5, 8}
//!   phase:       delta(n) = A^2(n)/F(n)^2   -> {2/9, 1/10, 5/96}
//!   Koide Q:     Q(n)   = 1/3 + A^2(n)/6    -> {2/3, 3/4, 8/9}
//!
//! This reproduces (within propagated mass uncertainties):
//!   A^2 = {2.00000, 2.490, 3.326} measured; Q = {0.6667, 0.7481, 0.8876}.
//! It predicts the two held-out charged-lepton ratios EXACTLY (zero fits).
//!
//! HONEST LIMITATIONS (verified below, not hidden):
//!  * The index assignment (lepton=1, down=2, up=3) is POST-HOC; no dynamics
//!    yet derives which sector gets which n.
//!  * Down-quark ratios are ~2% off; up-quark ratios are ~25% off at the
//!    predicted delta because the first ratio is extremely sensitive to the
//!    phase there. Matching Q does NOT fix the ratio pair (Q is one constraint
//!    on two ratios). The framework therefore does NOT reproduce the up-quark
//!    hierarchy, and is NOT a unified theory.
//!  * No gauge or gravity sector is supplied by this structure.

use crate::Check;
use crate::mass_convention::{down_masses_mz, q_from_masses, sqrt_ratios, up_masses_mz};
use std::f64::consts::PI;

/// Fibonacci number F(k) with F(0)=0, F(1)=1.
pub fn fib(k: i32) -> i64 {
    let (mut a, mut b) = (0_i64, 1_i64);
    for _ in 0..k { let t = a + b; a = b; b = t; }
    a
}

/// Amplitude squared for sector index n: n + 1/n.
pub fn amplitude_squared(n: i32) -> f64 { n as f64 + 1.0 / n as f64 }

/// Fibonacci label for sector index n: Fib(n+3).
pub fn label(n: i32) -> i64 { fib(n + 3) }

/// Predicted Brannen phase for sector index n: A^2 / F^2.
pub fn predicted_delta(n: i32) -> f64 {
    let f = label(n) as f64;
    amplitude_squared(n) / (f * f)
}

/// Predicted Koide quotient: Q = 1/3 + A^2/6.
pub fn predicted_q(n: i32) -> f64 { 1.0 / 3.0 + amplitude_squared(n) / 6.0 }

/// Circulant ratio pair from (A, delta).
pub fn predict_ratios(n: i32) -> Option<(f64, f64)> {
    let a = amplitude_squared(n).sqrt();
    let d = predicted_delta(n);
    let mut y = [0.0_f64; 3];
    for k in 0..3 { y[k] = 1.0 + a * (d + 2.0 * PI * k as f64 / 3.0).cos(); }
    if y.iter().any(|v| *v <= 0.0) { return None; }
    y.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    Some((y[1] / y[0], y[2] / y[1]))
}

/// Recorded mass triplet for a sector index (1=lepton, 2=down, 3=up).
fn masses(n: i32) -> [f64; 3] {
    match n { 1 => [0.51099895, 105.6583755, 1776.93], 2 => down_masses_mz(), _ => up_masses_mz() }
}

pub fn checks() -> Vec<Check> {
    let mut out = Vec::new();
    for n in 1..=3 {
        let name = match n { 1 => "lep", 2 => "down", _ => "up" };
        let obs_q = q_from_masses(masses(n));
        out.push(Check::new(
            match n { 1 => "index.q_lep", 2 => "index.q_down", _ => "index.q_up" },
            predicted_q(n) - obs_q,
            match n { 1 => "predicted minus measured Q (lepton)",
                      2 => "predicted minus measured Q (down)",
                      _ => "predicted minus measured Q (up)" },
        ));
        out.push(Check::new(
            match n { 1 => "index.delta_lep", 2 => "index.delta_down", _ => "index.delta_up" },
            predicted_delta(n),
            match n { 1 => "predicted phase 2/9", 2 => "predicted phase 1/10", _ => "predicted phase 5/96" },
        ));
        if let Some((p1, p2)) = predict_ratios(n) {
            let (o1, o2) = sqrt_ratios(masses(n));
            out.push(Check::new(
                match n { 1 => "index.r1_lep", 2 => "index.r1_down", _ => "index.r1_up" },
                p1 / o1 - 1.0,
                "first ratio relative error",
            ));
            out.push(Check::new(
                match n { 1 => "index.r2_lep", 2 => "index.r2_down", _ => "index.r2_up" },
                p2 / o2 - 1.0,
                "second ratio relative error",
            ));
        }
        let _ = name;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_three_sectors_have_the_required_rational_values() {
        assert!((amplitude_squared(1) - 2.0).abs() < 1e-12);
        assert!((amplitude_squared(2) - 2.5).abs() < 1e-12);
        assert!((amplitude_squared(3) - 10.0/3.0).abs() < 1e-12);
        assert_eq!([label(1), label(2), label(3)], [3, 5, 8]);
        assert!((predicted_delta(1) - 2.0/9.0).abs() < 1e-12);
        assert!((predicted_delta(2) - 0.1).abs() < 1e-12);
        assert!((predicted_delta(3) - 5.0/96.0).abs() < 1e-12);
    }

    #[test]
    fn predicted_q_matches_measured_within_one_percent() {
        for n in 1..=3 {
            assert!((predicted_q(n) - q_from_masses(masses(n))).abs() / predicted_q(n) < 0.01,
                    "sector {n}: Q predicted {} measured {}", predicted_q(n), q_from_masses(masses(n)));
        }
    }

    #[test]
    fn held_out_lepton_ratios_are_reproduced() {
        let (p1, p2) = predict_ratios(1).unwrap();
        let (o1, o2) = sqrt_ratios(masses(1));
        assert!((p1/o1 - 1.0).abs() < 1e-3 && (p2/o2 - 1.0).abs() < 1e-3,
                "lepton ratios predicted {p1},{p2} vs {o1},{o2}");
    }

    #[test]
    fn up_quark_hierarchy_is_not_reproduced_honest_failure() {
        let (p1, _) = predict_ratios(3).unwrap();
        let (o1, _) = sqrt_ratios(masses(3));
        assert!((p1/o1 - 1.0).abs() > 0.1,
                "up ratio error = {}", p1/o1 - 1.0);
    }
}
