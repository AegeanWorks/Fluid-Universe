//! F4 and G2 WZW quantum dimensions with CORRECT non-simply-laced normalization.
//!
//! A previous attempt used inconsistent Cartan/root-length conventions and
//! failed to terminate. Here A[i][j] = 2(alpha_i,alpha_j)/L_i with consistency
//! A[i][j]*L_i = A[j][i]*L_j, and for a positive root a = sum c_i alpha_i:
//!   (rho,a) = sum c_i L_i / 2,  (lam,a) = sum lam_i c_i L_i / 2.
//!
//! With this normalization the root counts are correct (G2: 6, F4: 24).
//! Fundamental quantum dimensions for k=1..12 were computed; the closest
//! matches to the required amplitudes are:
//!   G2 k=1 f2 = 1.618 (golden ratio) vs nu A^2=1.514  -> 6.9% off
//!   F4 k=2 f1 = 1.919 vs lepton A^2=2                -> 4.0% off
//!   F4 k=2 f3 = 3.229 vs up A^2=3.325                 -> 2.9% off
//! No fundamental matches the down amplitude 2.488 within 20%.
//! The non-simply-laced exceptional avenue therefore also fails to provide a
//! single structure deriving all four amplitudes.

use crate::Check;
use std::f64::consts::PI;

/// G2 positive roots (Bourbaki, first root long L=2, second short L=2/3).
pub const G2_ROOTS: &[[i32; 2]] = &[
    [0,1],[1,0],[1,1],[1,2],[1,3],[2,3],
];

/// F4 positive roots (first two long L=2, last two short L=1).
pub const F4_ROOTS: &[[i32; 4]] = &[
    [0,0,0,1],[0,0,1,0],[0,0,1,1],[0,1,0,0],[0,1,1,0],[0,1,1,1],
    [0,1,2,0],[0,1,2,1],[0,1,2,2],[1,0,0,0],[1,1,0,0],[1,1,1,0],
    [1,1,1,1],[1,1,2,0],[1,1,2,1],[1,1,2,2],[1,2,2,0],[1,2,2,1],
    [1,2,2,2],[1,2,3,1],[1,2,3,2],[1,2,4,2],[1,3,4,2],[2,3,4,2],
];

/// Root lengths squared (long = 2).
pub fn g2_lengths() -> [f64; 2] { [2.0, 2.0 / 3.0] }
pub fn f4_lengths() -> [f64; 4] { [2.0, 2.0, 1.0, 1.0] }

pub fn g2_hvee() -> i32 { 4 }
pub fn f4_hvee() -> i32 { 9 }

/// Quantum dimension for a non-simply-laced algebra from its positive roots.
fn qdim_generic(roots: &[&[i32]], lam: &[i32], lengths: &[f64], k: i32, hvee: i32) -> f64 {
    let kbig = (k + hvee) as f64;
    let mut num = 1.0_f64;
    let mut den = 1.0_f64;
    for c in roots {
        let rho_a: f64 = c.iter().zip(lengths).map(|(a, l)| *a as f64 * l).sum::<f64>() / 2.0;
        let lam_a: f64 = lam.iter().zip(c.iter()).zip(lengths)
            .map(|((l, a), len)| *l as f64 * *a as f64 * len).sum::<f64>() / 2.0;
        num *= (PI * (lam_a + rho_a) / kbig).sin();
        den *= (PI * rho_a / kbig).sin();
    }
    num / den
}

pub fn g2_qdim(lam: [i32; 2], k: i32) -> f64 {
    let refs: Vec<&[i32]> = G2_ROOTS.iter().map(|r| r.as_slice()).collect();
    qdim_generic(&refs, &lam, &g2_lengths(), k, g2_hvee())
}

pub fn f4_qdim(lam: [i32; 4], k: i32) -> f64 {
    let refs: Vec<&[i32]> = F4_ROOTS.iter().map(|r| r.as_slice()).collect();
    qdim_generic(&refs, &lam, &f4_lengths(), k, f4_hvee())
}

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("nonsimply.g2_root_count", G2_ROOTS.len() as f64, "G2 has 6 positive roots"),
        Check::new("nonsimply.f4_root_count", F4_ROOTS.len() as f64, "F4 has 24 positive roots"),
        Check::new("nonsimply.g2_k1_f2", g2_qdim([0, 1], 1), "G2 k=1 f2 = phi ~ 1.618"),
        Check::new("nonsimply.f4_k2_f1", f4_qdim([1, 0, 0, 0], 2), "F4 k=2 f1 ~ 1.919"),
        Check::new("nonsimply.f4_k2_f3", f4_qdim([0, 0, 1, 0], 2), "F4 k=2 f3 ~ 3.229"),
        Check::new("nonsimply.up_best_err",
                   (f4_qdim([0, 0, 1, 0], 2) / 3.32542 - 1.0).abs(),
                   "closest exceptional match to up A^2 (2.9% off)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_counts_are_correct() {
        assert_eq!(G2_ROOTS.len(), 6);
        assert_eq!(F4_ROOTS.len(), 24);
    }

    #[test]
    fn level_one_fundamentals_reproduce_golden_ratio() {
        assert!((g2_qdim([0, 1], 1) - 1.6180339887).abs() < 1e-6);
        assert!((f4_qdim([0, 0, 0, 1], 1) - 1.6180339887).abs() < 1e-6);
    }

    #[test]
    fn no_fundamental_matches_required_amplitudes_exactly() {
        let targets = [1.51367_f64, 2.0, 2.48839, 3.32542];
        for k in 1..=12 {
            let g2 = [g2_qdim([1, 0], k), g2_qdim([0, 1], k)];
            let f4 = [
                f4_qdim([1, 0, 0, 0], k), f4_qdim([0, 1, 0, 0], k),
                f4_qdim([0, 0, 1, 0], k), f4_qdim([0, 0, 0, 1], k),
            ];
            for d in g2.iter().chain(f4.iter()).filter(|v| v.is_finite() && **v > 0.0) {
                for t in targets {
                    assert!((*d - t).abs() / t > 0.01,
                            "k={k} d={d} unexpectedly matched {t}");
                }
            }
        }
    }
}
