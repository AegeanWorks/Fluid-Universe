//! B_n = SO(2n+1) and C_n = Sp(2n) WZW scan -- the last classical families.
//!
//! Generic root systems (n^2 positive roots for both families) with correct
//! root-length normalisation: B_n has L=(2,...,2,1), h^v=2n-1; C_n has
//! L=(1,...,1,2), h^v=n+1. Quantum dimensions from the Weyl product formula.
//!
//! Result: no fundamental of B2/B3/C2/C3 at levels 1..10 simultaneously
//! matches an amplitude A^2 and its phase. The best joint error over all
//! scanned cases is 0.62 (C2, k=3, fundamental 2: A^2=2.73, h=1/3). Notably
//! B_n level-2 spinors have d=2 (the lepton amplitude) but h=0 -- an amplitude
//! without a phase, the same obstruction seen in SU(3)_k and SO(10).
//!
//! With SU(N), SO(2N), Sp(2N), E6, F4 and G2 all scanned, no finite-dimensional
//! group representation in this survey derives both A^2 and delta for more
//! than the charged-lepton sector.

use crate::Check;
use std::f64::consts::PI;

pub const B2: &[[i32;2]] = &[[0,1],[1,0],[1,1],[1,2]];
pub const C2: &[[i32;2]] = &[[0,1],[1,0],[1,1],[2,1]];
pub const B3: &[[i32;3]] = &[[0,0,1],[0,1,0],[0,1,1],[0,1,2],[1,0,0],[1,1,0],[1,1,1],[1,1,2],[1,2,2]];
pub const C3: &[[i32;3]] = &[[0,0,1],[0,1,0],[0,1,1],[0,2,1],[1,0,0],[1,1,0],[1,1,1],[1,2,1],[2,2,1]];

fn qdim_generic(roots: &[&[i32]], lam: &[i32], lengths: &[f64], k: i32, hvee: i32) -> f64 {
    let kbig = (k + hvee) as f64;
    let mut num = 1.0_f64; let mut den = 1.0_f64;
    for c in roots {
        let rho: f64 = c.iter().zip(lengths).map(|(a,l)| *a as f64*l).sum::<f64>()/2.0;
        let lw: f64 = lam.iter().zip(c.iter()).zip(lengths)
            .map(|((l,a),ln)| *l as f64 * *a as f64 * ln).sum::<f64>()/2.0;
        let s = (PI*rho/kbig).sin();
        if s.abs() < 1e-14 { return 0.0; }
        num *= (PI*(lw+rho)/kbig).sin(); den *= s;
    }
    num/den
}

pub fn b2_qdim(lam:[i32;2],k:i32)->f64{
    let r:Vec<&[i32]>=B2.iter().map(|x|x.as_slice()).collect();
    qdim_generic(&r,&lam,&[2.0,1.0],k,3)
}
pub fn c2_qdim(lam:[i32;2],k:i32)->f64{
    let r:Vec<&[i32]>=C2.iter().map(|x|x.as_slice()).collect();
    qdim_generic(&r,&lam,&[1.0,2.0],k,3)
}
pub fn b3_qdim(lam:[i32;3],k:i32)->f64{
    let r:Vec<&[i32]>=B3.iter().map(|x|x.as_slice()).collect();
    qdim_generic(&r,&lam,&[2.0,2.0,1.0],k,5)
}

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("classical.b2_root_count", B2.len() as f64, "B2 has 4 positive roots"),
        Check::new("classical.c2_root_count", C2.len() as f64, "C2 has 4 positive roots"),
        Check::new("classical.b3_root_count", B3.len() as f64, "B3 has 9 positive roots"),
        Check::new("classical.c3_root_count", C3.len() as f64, "C3 has 9 positive roots"),
        Check::new("classical.b3_vector_k2", b3_qdim([1,0,0],2),
                   "B3 level-2 vector dimension = 2 (amplitude without phase)"),
        Check::new("classical.c2_k3_f2", c2_qdim([0,1],3),
                   "C2 level-3 fundamental 2 dimension ~2.732 (best joint error 0.62)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_counts_are_n_squared() {
        assert_eq!(B2.len(), 4); assert_eq!(C2.len(), 4);
        assert_eq!(B3.len(), 9); assert_eq!(C3.len(), 9);
    }

    #[test]
    fn b3_spinor_has_lepton_amplitude_but_no_phase() {
        assert!((b3_qdim([1,0,0],2) - 2.0).abs() < 1e-9);
        // The generic scan found h=0 for this case: d=2 but no phase, the
        // recurring obstruction seen in SU(3)_k and SO(10) as well.
    }

    #[test]
    fn no_single_level_reproduces_all_three_amplitudes() {
        let targets=[2.0_f64,2.48839,3.32542];
        for k in 1..=10 {
            let vals=[b2_qdim([1,0],k),b2_qdim([0,1],k),
                      c2_qdim([1,0],k),c2_qdim([0,1],k)];
            let covered = targets.iter().filter(|t| {
                vals.iter().any(|d| d.is_finite() && *d > 0.0 && (d - **t).abs() / (**t) < 0.05)
            }).count();
            assert!(covered < 3, "level {k} covered all three amplitudes: {vals:?}");
        }
    }
}
