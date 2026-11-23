//! New hypothesis: the Brannen phase is the amplitude divided by a squared
//! discrete label, delta = A^2 / F^2, with F = {3, 5, 8} (consecutive Fibonacci
//! numbers) for {charged leptons, down quarks, up quarks}.
//!
//! Observed (consistent Huang-Zhou MZ masses for quarks, PDG for leptons):
//!   lepton  A^2 = 2.0000, delta = 0.222225, A^2/delta = 9.00  -> F=3 (exact)
//!   down    A^2 = 2.4884, delta = 0.0999,   A^2/delta = 24.88 -> F=5 (vs 25)
//!   up      A^2 = 3.3254, delta = 0.0518,   A^2/delta = 64.16 -> F=8 (vs 64)
//!
//! HONEST STATUS: this is a POST-HOC pattern, not a derivation. The labels
//! {3,5,8} were selected because they match the three central values; the
//! relation does not reproduce the quark amplitudes A^2 from group data (the
//! exhaustive SU(3)_k scan in rep_scan.rs already failed to do so). Under
//! propagated Huang-Zhou mass uncertainties, delta_down = 0.0999 +/- 0.0060
//! and delta_up = 0.0518 +/- 0.0008, so 1/10 and 2/39 (the literature value)
//! are both within ~1 sigma. The central-value agreement is therefore NOT
//! statistically decisive. We record the hypothesis and its test, not a claim.
//!
//! The module also CORRECTS earlier overstatements: literature phases 2/27,
//! 4/27 were called "falsified" on central values alone; with uncertainties
//! they are merely disfavoured (1/9 is ~1.9 sigma off, 4/27 much further).

use crate::Check;
use crate::mass_convention::{down_masses_mz, q_from_masses, sqrt_ratios, up_masses_mz};
use std::f64::consts::PI;

/// Lepton (PDG pole) masses in GeV.
pub fn lepton_masses() -> [f64; 3] { [0.51099895, 105.6583755, 1776.93] }

/// Hypothesised Fibonacci label per sector.
pub fn fib_label(sector: usize) -> i64 { [3, 5, 8][sector] }

pub fn sector_masses(sector: usize) -> [f64; 3] {
    match sector { 0 => lepton_masses(), 1 => down_masses_mz(), _ => up_masses_mz() }
}

/// Amplitude squared implied by a sector's Koide quotient.
pub fn amplitude_squared(m: [f64; 3]) -> f64 { 6.0 * (q_from_masses(m) - 1.0 / 3.0) }

/// The required (empirical) Brannen phase from a consistent mass triplet.
pub fn required_delta(m: [f64; 3]) -> f64 {
    let a2 = amplitude_squared(m);
    let a = a2.sqrt();
    let mut s: Vec<f64> = m.iter().map(|v| v.sqrt()).collect();
    s.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    let total: f64 = s.iter().sum();
    let y: Vec<f64> = s.iter().map(|v| 3.0 * v / total).collect();
    let s3: f64 = y.iter().map(|v| v * v * v).sum();
    let cos3 = (4.0 / (3.0 * a * a2)) * (s3 - 3.0 - 9.0 * a2 / 2.0);
    cos3.clamp(-1.0, 1.0).acos() / 3.0
}

/// Predicted phase from the hypothesis: delta = A^2 / F^2.
pub fn predicted_delta(sector: usize) -> f64 {
    let f = fib_label(sector) as f64;
    amplitude_squared(sector_masses(sector)) / (f * f)
}

/// Circulant ratio pair at amplitude A and phase d.
pub fn ratios(a: f64, d: f64) -> Option<(f64, f64)> {
    let mut y = [0.0_f64; 3];
    for k in 0..3 {
        y[k] = 1.0 + a * (d + 2.0 * PI * k as f64 / 3.0).cos();
    }
    if y.iter().any(|v| *v <= 0.0) { return None; }
    y.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    Some((y[1] / y[0], y[2] / y[1]))
}

pub fn checks() -> Vec<Check> {
    let mut out = Vec::new();
    for (sector, name) in ["lepton", "down", "up"].iter().enumerate() {
        let m = sector_masses(sector);
        let req = required_delta(m);
        let pred = predicted_delta(sector);
        out.push(Check::new(
            match sector { 0 => "fib.phase_lepton", 1 => "fib.phase_down", _ => "fib.phase_up" },
            pred,
            match sector {
                0 => "delta=A^2/3^2 (lepton, exact)",
                1 => "delta=A^2/5^2 (down hypothesis)",
                _ => "delta=A^2/8^2 (up hypothesis)",
            },
        ));
        out.push(Check::new(
            match sector { 0 => "fib.err_lepton", 1 => "fib.err_down", _ => "fib.err_up" },
            pred - req,
            match sector { 0 => "lepton", 1 => "down", _ => "up" },
        ));
        let (obs_r1, obs_r2) = sqrt_ratios(m);
        let a = amplitude_squared(m).sqrt();
        if let Some((p1, p2)) = ratios(a, pred) {
            out.push(Check::new(
                match sector { 0 => "fib.r1_lepton", 1 => "fib.r1_down", _ => "fib.r1_up" },
                p1 / obs_r1 - 1.0,
                "first ratio relative error",
            ));
            out.push(Check::new(
                match sector { 0 => "fib.r2_lepton", 1 => "fib.r2_down", _ => "fib.r2_up" },
                p2 / obs_r2 - 1.0,
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
    fn fibonacci_labels_are_consecutive() {
        assert_eq!(fib_label(0), 3);
        assert_eq!(fib_label(1), 5);
        assert_eq!(fib_label(2), 8);
        assert_eq!(fib_label(0) + fib_label(1), fib_label(2)); // 3 + 5 = 8
    }

    #[test]
    fn required_times_f_squared_approaches_amplitude_squared() {
        // A^2/delta should be near F^2 for each sector.
        for sector in 0..3 {
            let m = sector_masses(sector);
            let a2 = amplitude_squared(m);
            let req = required_delta(m);
            let f2 = (fib_label(sector) * fib_label(sector)) as f64;
            let ratio = a2 / req;
            assert!((ratio / f2 - 1.0).abs() < 0.01,
                    "sector {sector}: A^2/delta={ratio} vs F^2={f2}");
        }
    }

    #[test]
    fn lepton_prediction_is_essentially_exact() {
        let err = predicted_delta(0) - required_delta(lepton_masses());
        assert!(err.abs() < 1e-4, "lepton delta error = {err}");
    }

    #[test]
    fn literature_phases_are_disfavoured_not_decisively_falsified() {
        // With Huang-Zhou uncertainties delta_up = 0.0518 +/- 0.0008; the
        // literature value 2/39 = 0.05128 is within ~1 sigma. We record this
        // rather than repeating the earlier overstated "falsified" verdict.
        let up_delta = required_delta(up_masses_mz());
        assert!((up_delta - 2.0 / 39.0).abs() / up_delta < 0.02);
    }
}
