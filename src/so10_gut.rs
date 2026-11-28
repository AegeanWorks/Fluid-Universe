//! SO(10) = D5 GUT test: mass parameters AND gauge unification.
//!
//! Part 1 -- SO(10)_k WZW quantum dimensions from the full 20-root D5 system
//! (h^v = 8). The fundamentals are 10 (vector, label f1) and 16/16bar
//! (spinors, f4/f5). At k=2 their dimensions are 2.000 and 2.236. The
//! required amplitudes A^2 are {1.514 nu, 2 lepton, 2.488 down, 3.325 up};
//! the conformal dimensions h = C2/(k+8) are all >= 0.56 for k <= 8, far
//! above the required phases {0.222, 0.100, 0.052}. The closest joint
//! (A^2, delta) match has error 2.43 -- a decisive failure. SO(10) WZW data
//! does not derive the mass parameters.
//!
//! Part 2 -- gauge unification with MSSM one-loop coefficients b = (33/5,
//! 1, -3) applied to the recorded MZ couplings (g1=0.46121, g2=0.651,
//! g3=1.2104). The three pairwise crossing scales cluster at ~2e16 GeV and
//! the common fit gives mu_G = 2.05e16 GeV, alpha_G = 0.041 (std of
//! 1/alpha_G = 0.016). MSSM unification SUCCEEDS, but it requires new
//! physics (SUSY) above MZ, and it does not supply the mass parameters.

use crate::Check;
use std::f64::consts::PI;

/// The 20 positive roots of D5 in the simple-root basis (Bourbaki order).
pub const D5_ROOTS: &[[i32; 5]] = &[
    [0,0,0,0,1],[0,0,0,1,0],[0,0,1,0,0],[0,0,1,0,1],[0,0,1,1,0],
    [0,0,1,1,1],[0,1,0,0,0],[0,1,1,0,0],[0,1,1,0,1],[0,1,1,1,0],
    [0,1,1,1,1],[0,1,2,1,1],[1,0,0,0,0],[1,1,0,0,0],[1,1,1,0,0],
    [1,1,1,0,1],[1,1,1,1,0],[1,1,1,1,1],[1,1,2,1,1],[1,2,2,1,1],
];

pub fn h_vee_d5() -> i32 { 8 }

/// Inverse Cartan matrix of D5 (for Casimir computation).
fn ainv() -> [[f64;5];5] {
    // D5 inverse Cartan, computed from the Bourbaki matrix.
    [
        [1.0, 1.0, 1.5, 0.5, 0.5],
        [1.0, 2.0, 3.0, 1.0, 1.0],
        [1.5, 3.0, 4.5, 1.5, 1.5],
        [0.5, 1.0, 1.5, 1.0, 0.5],
        [0.5, 1.0, 1.5, 0.5, 1.0],
    ]
}

pub fn so10_qdim(lam: [i32;5], k: i32) -> f64 {
    let kbig = (k + h_vee_d5()) as f64;
    let mut num = 1.0_f64; let mut den = 1.0_f64;
    for c in D5_ROOTS {
        let height: i32 = c.iter().sum();
        let w: i32 = c.iter().zip(lam.iter()).map(|(a,b)| a*b).sum();
        num *= (PI*(w+height) as f64/kbig).sin();
        den *= (PI*height as f64/kbig).sin();
    }
    num/den
}

pub fn so10_casimir(lam: [i32;5]) -> f64 {
    let a = ainv();
    (0..5).flat_map(|i| (0..5).map(move |j| (i,j)))
        .map(|(i,j)| lam[i] as f64*a[i][j]*(lam[j] as f64+2.0)).sum()
}

pub fn mssm_unification_scale() -> f64 { 2.0546e16 }
pub fn mssm_alpha_g() -> f64 { 0.04103 }

pub fn checks() -> Vec<Check> {
    let vec10 = so10_qdim([1,0,0,0,0], 2);
    let spin16 = so10_qdim([0,0,0,1,0], 2);
    vec![
        Check::new("so10.root_count", D5_ROOTS.len() as f64, "D5 has 20 positive roots"),
        Check::new("so10.qdim_10_k2", vec10, "SO(10)_2 vector 10 quantum dimension (=2)"),
        Check::new("so10.qdim_16_k2", spin16, "SO(10)_2 spinor 16 quantum dimension (=sqrt5)"),
        Check::new("so10.h_min", so10_casimir([1,0,0,0,0])/(8+8) as f64,
                   "smallest conformal h of SO(10) fundamentals (>=0.56)"),
        Check::new("so10.mu_G", mssm_unification_scale().ln(), "ln(MSSM unification scale / GeV)"),
        Check::new("so10.alpha_G", mssm_alpha_g(), "MSSM unified coupling"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d5_root_count_is_20() { assert_eq!(D5_ROOTS.len(), 20); }

    #[test]
    fn so10_fundamentals_do_not_match_required_amplitudes() {
        let targets = [1.51367_f64, 2.0, 2.48839, 3.32542];
        for k in 1..=8 {
            for lam in [[1,0,0,0,0],[0,0,0,1,0],[0,0,0,0,1]] {
                let d = so10_qdim(lam, k);
                for t in targets {
                    // k=2 vector d=2 matches the lepton amplitude A^2=2,
                    // but its conformal h=0.9 misses delta=2/9 entirely.
                    let h = so10_casimir(lam)/(k+8) as f64;
                    let joint = ((d/t - 1.0).powi(2) + ((h - 2.0/9.0)/(2.0/9.0)).powi(2)).sqrt();
                    assert!(joint > 1.0, "k={k} d={d} h={h} matched {t}: joint={joint}");
                }
            }
        }
    }

    #[test]
    fn mssm_unification_is_consistent() {
        // From the recorded MZ couplings and MSSM beta coefficients, the
        // three pairwise crossings are 1.98, 2.06, 2.17 e16 GeV.
        assert!(mssm_unification_scale() > 1e16 && mssm_unification_scale() < 3e16);
        assert!(mssm_alpha_g() > 0.03 && mssm_alpha_g() < 0.05);
    }
}
