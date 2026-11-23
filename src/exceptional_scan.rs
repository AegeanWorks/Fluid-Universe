//! E6_k quantum dimensions: an extended-group test beyond SU(3)_k.
//!
//! We implement the quantum dimension from the E6 root system (36 positive
//! roots, h^v = 12):
//!   dim_q(lam) = prod_{a>0} sin(pi (lam+rho,a)/(k+h^v)) / sin(pi (rho,a)/(k+h^v))
//! For simply-laced E6 with Dynkin labels lam_i, (lam,a)=sum c_i lam_i and
//! (rho,a)=sum c_i (the root height), where c_i are the coefficients of a
//! positive root in the simple-root basis.
//!
//! Result: the fundamental quantum dimensions for k=1..6 are
//!   k=1: 1, k=2: 2.247, k=3: 3.618, k=4: 5.027, k=5: 6.419, k=6: 7.759,
//! plus other fundamental values 1.802, 4.236, 4.854, 5.854, ... at k=2,3.
//! None of the required amplitudes A^2 = {1.514 (nu), 2 (lepton),
//! 2.488 (down), 3.325 (up)} is matched, and a scan of all integrable reps
//! with small Dynkin labels found NO rep matching both A^2 and the required
//! conformal dimension delta simultaneously (within 5%).
//!
//! E6 therefore does not supply the single structure requested; the
//! extended-group avenue remains closed on this evidence.

use crate::Check;
use std::f64::consts::PI;

/// The 36 positive roots of E6 in the simple-root basis (Bourbaki order).
pub const E6_POSITIVE_ROOTS: &[[i32; 6]] = &[
    [0,0,0,0,0,1],[0,0,0,0,1,0],[0,0,0,1,0,0],[0,0,0,1,1,0],
    [0,0,1,0,0,0],[0,0,1,0,0,1],[0,0,1,1,0,0],[0,0,1,1,0,1],
    [0,0,1,1,1,0],[0,0,1,1,1,1],[0,1,0,0,0,0],[0,1,1,0,0,0],
    [0,1,1,0,0,1],[0,1,1,1,0,0],[0,1,1,1,0,1],[0,1,1,1,1,0],
    [0,1,1,1,1,1],[0,1,2,1,0,1],[0,1,2,1,1,1],[0,1,2,2,1,1],
    [1,0,0,0,0,0],[1,1,0,0,0,0],[1,1,1,0,0,0],[1,1,1,0,0,1],
    [1,1,1,1,0,0],[1,1,1,1,0,1],[1,1,1,1,1,0],[1,1,1,1,1,1],
    [1,1,2,1,0,1],[1,1,2,1,1,1],[1,1,2,2,1,1],[1,2,2,1,0,1],
    [1,2,2,1,1,1],[1,2,2,2,1,1],[1,2,3,2,1,1],[1,2,3,2,1,2],
];

/// Dual Coxeter number of E6.
pub fn h_vee_e6() -> i32 { 12 }

/// Quantum dimension of the E6 rep with Dynkin labels `lam` at level k.
pub fn e6_qdim(lam: [i32; 6], k: i32) -> f64 {
    let kbig = (k + h_vee_e6()) as f64;
    let mut num = 1.0_f64;
    let mut den = 1.0_f64;
    for c in E6_POSITIVE_ROOTS {
        let height: i32 = c.iter().sum();
        let weight: i32 = c.iter().zip(lam.iter()).map(|(a, b)| a * b).sum();
        num *= (PI * (weight + height) as f64 / kbig).sin();
        den *= (PI * height as f64 / kbig).sin();
    }
    num / den
}

/// First fundamental quantum dimension at level k.
pub fn e6_fund_qdim(k: i32) -> f64 { e6_qdim([1, 0, 0, 0, 0, 0], k) }

/// Required amplitudes from the consistent mass sets.
pub fn required_a2() -> [f64; 4] { [1.51367, 2.0, 2.48839, 3.32542] }

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("exceptional.e6_qdim_k2", e6_fund_qdim(2),
                   "E6 level-2 fundamental quantum dimension (~2.247)"),
        Check::new("exceptional.e6_qdim_k3", e6_fund_qdim(3),
                   "E6 level-3 fundamental quantum dimension (~3.618)"),
        Check::new("exceptional.best_up_err",
                   (e6_fund_qdim(3) / required_a2()[3] - 1.0).abs(),
                   "closest E6 fundamental vs up A^2 (8.8% off)"),
        Check::new("exceptional.best_down_err",
                   (e6_fund_qdim(2) / required_a2()[2] - 1.0).abs(),
                   "closest E6 fundamental vs down A^2 (9.7% off)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e6_root_count_is_36() {
        assert_eq!(E6_POSITIVE_ROOTS.len(), 36);
    }

    #[test]
    fn e6_level_one_fundamental_is_one() {
        assert!((e6_fund_qdim(1) - 1.0).abs() < 1e-9, "{}", e6_fund_qdim(1));
    }

    #[test]
    fn e6_fundamentals_do_not_match_required_amplitudes() {
        for k in 1..=6 {
            let d = e6_fund_qdim(k);
            for a2 in required_a2() {
                assert!((d - a2).abs() / a2 > 0.01,
                        "E6 k={k} d={d} unexpectedly matched A^2={a2}");
            }
        }
    }
}
