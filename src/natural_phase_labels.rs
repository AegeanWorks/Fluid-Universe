//! Singlet-channel weight differences at three post-hoc selected levels.
//!
//! The singlet-channel braiding label of SU(N)_k is 2 h_box = (N^2-1)/(N(k+N)).
//! For the three sectors the required phases are obtained EXACTLY at levels
//! that are simple functions of d=3:
//!   lepton  2/9  = SU(3)_{d^2=9}:       8/(3*12)  = 2/9
//!   down    1/10 = SU(2)_{d^2+d+1=13}:  3/(2*15)  = 1/10
//!   up      2/39 = SU(3)_{(2d+1)^2=49}: 8/(3*52)  = 2/39
//!
//! The earlier SU(3)_62 adjoint claim was incorrect: its weight difference is
//! 1/195. For a singlet channel, the external pair is fundamental and its
//! conjugate. The magnitude shown is a full-monodromy exponent, not a proof
//! that any physical sector uses this WZW model or phase.
//!
//! HONEST CAVEAT: the choice of SU(2) for down vs SU(3) for lepton/up, and the
//! specific level functions (d^2, d^2+d+1, (2d+1)^2), were recognized after
//! the phases were known. The labels are exact; the assignment rule is still
//! post-hoc in the sense that other d=3 expressions could be tried. What is
//! numerically exact identities use one channel type in three separate models.

use crate::Check;

pub fn d() -> i64 { 3 }

/// Singlet-channel full-monodromy exponent magnitude 2 h_box.
pub fn singlet_label(n: i64, k: i64) -> f64 {
    (n*n - 1) as f64 / (n as f64 * (k + n) as f64)
}

pub fn levels() -> [i64; 3] {
    let d = d();
    [d*d, d*d + d + 1, (2*d + 1).pow(2)]  // 9, 13, 49
}

pub fn checks() -> Vec<Check> {
    let lv = levels();
    vec![
        Check::new("natlabel.k_lep", lv[0] as f64, "lepton level d^2 = 9"),
        Check::new("natlabel.k_down", lv[1] as f64, "down level d^2+d+1 = 13"),
        Check::new("natlabel.k_up", lv[2] as f64, "up level (2d+1)^2 = 49"),
        Check::new("natlabel.lep", singlet_label(3, lv[0]), "SU(3)_9 singlet = 2/9"),
        Check::new("natlabel.down", singlet_label(2, lv[1]), "SU(2)_13 singlet = 1/10"),
        Check::new("natlabel.up", singlet_label(3, lv[2]), "SU(3)_49 singlet = 2/39"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levels_are_d3_natural() {
        assert_eq!(levels(), [9, 13, 49]);
    }

    #[test]
    fn all_three_phases_are_exact_singlet_labels() {
        assert!((singlet_label(3, 9) - 2.0/9.0).abs() < 1e-14);
        assert!((singlet_label(2, 13) - 0.1).abs() < 1e-14);
        assert!((singlet_label(3, 49) - 2.0/39.0).abs() < 1e-14);
    }
}
