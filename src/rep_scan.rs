//! Systematic scan of SU(3)_k representations against every fermion sector.
//!
//! The WZW/Koide framework fixes the amplitude by the quantum dimension,
//! A^2 = d_□, and the phase by the conformal dimension, delta = h = C2/(k+h^v).
//! Here we ask: is there ANY integrable SU(3)_k representation whose quantum
//! dimension matches the amplitude required by a measured sector Q?
//!
//!   d_q(p,q;k) = sin(pi(p+1)/K) sin(pi(q+1)/K) sin(pi(p+q+2)/K)
//!                / [ sin(pi/K)^2 sin(2pi/K)],   K = k+3,  p+q <= k.
//!   C2(p,q)    = (p^2 + q^2 + p q + 3p + 3q)/3,  h = C2/(k+3).
//!   A^2 required by Q:  A^2 = 6 (Q - 1/3).
//!
//! Result (k = 1..40, all integrable reps): the charged leptons have an EXACT
//! match, d_q(1,0;3) = 2 (A^2 = 2). Every other sector only has NEAR misses:
//!   down  Q=0.748065 -> A^2=2.4884, best d_q=2.5321 (1,0;6), 1.8% high
//!   up    Q=0.887569 -> A^2=3.3254, best d_q=3.4142 (2,0;5), 2.7% high
//!   nu    Q=0.585612 -> A^2=1.5137, best d_q=1.6180 (1,0;2), 6.9% high
//! and the conformal phases of those near-match reps give catastrophic mass
//! ratios (down predicts r1=43 vs 4.7; up gives no positive spectrum).
//! Within SU(3)_k there is therefore NO representation that fits quarks or
//! neutrinos; the quantum-dimension mechanism is unique to the leptons.

use crate::Check;
use std::f64::consts::PI;

/// Quantum dimension of the integrable SU(3)_k rep (p,q), K = k+3.
pub fn dimq(p: i64, q: i64, k: i64) -> f64 {
    let kbig = (k + 3) as f64;
    let s = |x: f64| (PI * x / kbig).sin();
    s((p + 1) as f64) * s((q + 1) as f64) * s((p + q + 2) as f64)
        / (s(1.0) * s(1.0) * s(2.0))
}

/// Quadratic Casimir C2(p,q) of SU(3).
pub fn casimir(p: i64, q: i64) -> f64 {
    (p * p + q * q + p * q + 3 * p + 3 * q) as f64 / 3.0
}

/// Conformal dimension h = C2/(k + h^v), h^v(SU(3)) = 3.
pub fn conformal_h(p: i64, q: i64, k: i64) -> f64 {
    casimir(p, q) / (k + 3) as f64
}

/// Amplitude squared required by a measured Koide quotient Q.
pub fn a2_from_q(q: f64) -> f64 { 6.0 * (q - 1.0 / 3.0) }

#[derive(Clone, Copy, Debug)]
pub struct BestRep {
    pub err: f64,
    pub d: f64,
    pub p: i64,
    pub q: i64,
    pub k: i64,
    pub h: f64,
}

/// Closest integrable rep to a target amplitude squared, scanning k=1..=kmax.
pub fn closest_rep(target_a2: f64, kmax: i64) -> BestRep {
    let mut best = BestRep { err: f64::INFINITY, d: 0.0, p: 0, q: 0, k: 0, h: 0.0 };
    for k in 1..=kmax {
        for p in 0..=k {
            for q in 0..=(k - p) {
                let d = dimq(p, q, k);
                let err = (d - target_a2).abs();
                if err < best.err {
                    best = BestRep { err, d, p, q, k, h: conformal_h(p, q, k) };
                }
            }
        }
    }
    best
}

/// Two sqrt-mass ratios of the Brannen circulant at amplitude A and phase d.
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
    let lepton = closest_rep(a2_from_q(2.0 / 3.0), 40);
    let down = closest_rep(a2_from_q(0.7480650548031402), 40);
    let up = closest_rep(a2_from_q(0.8875692313187327), 40);
    let nu = closest_rep(a2_from_q(0.5856120093687199), 40);

    // Predictions using each near-match rep's own quantum dimension and phase.
    let down_pred = ratios(down.d.sqrt(), down.h);
    let up_pred = ratios(up.d.sqrt(), up.h);

    vec![
        Check::new("rep_scan.lepton_d", lepton.d, "exact d_q(1,0;3)=2 for leptons"),
        Check::new("rep_scan.lepton_err", lepton.err, "lepton amplitude error (zero)"),
        Check::new("rep_scan.down_err", down.err, "closest rep error to down A^2"),
        Check::new("rep_scan.up_err", up.err, "closest rep error to up A^2"),
        Check::new("rep_scan.nu_err", nu.err, "closest rep error to neutrino A^2"),
        Check::new("rep_scan.down_pred_r1",
                   down_pred.map(|r| r.0).unwrap_or(-1.0),
                   "down rep predicted r1 (obs 4.717; -1=no spectrum)"),
        Check::new("rep_scan.up_pred_valid",
                   if up_pred.is_some() { 1.0 } else { 0.0 },
                   "up rep gives a positive spectrum? (0 = no)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantum_dimension_formula_is_exact_for_fundamental() {
        assert!((dimq(1, 0, 3) - 2.0).abs() < 1e-12);
        assert!((dimq(0, 1, 3) - 2.0).abs() < 1e-12);
    }

    #[test]
    fn leptons_have_an_exact_rep_match() {
        let b = closest_rep(a2_from_q(2.0 / 3.0), 40);
        assert!(b.err < 1e-12, "err={} rep=({},{},{})", b.err, b.p, b.q, b.k);
        assert!((b.d - 2.0).abs() < 1e-12);
        assert!((b.h - 2.0 / 9.0).abs() < 1e-12);
    }

    #[test]
    fn quarks_and_neutrinos_have_no_exact_rep_match() {
        for (name, q) in [
            ("down", 0.7480650548031402_f64),
            ("up", 0.8875692313187327),
            ("nu", 0.5856120093687199),
        ] {
            let b = closest_rep(a2_from_q(q), 40);
            assert!(b.err > 1e-3, "{name} unexpectedly matched: err={}", b.err);
        }
    }

    #[test]
    fn near_match_reps_fail_the_mass_ratios() {
        let down = closest_rep(a2_from_q(0.7480650548031402), 40);
        let d = ratios(down.d.sqrt(), down.h).unwrap();
        // predicted first ratio ~43 vs observed ~4.7
        assert!((d.0 - 4.717).abs() / 4.717 > 1.0, "down r1={}", d.0);

        let up = closest_rep(a2_from_q(0.8875692313187327), 40);
        assert!(ratios(up.d.sqrt(), up.h).is_none(), "up rep should give no positive spectrum");
    }
}
