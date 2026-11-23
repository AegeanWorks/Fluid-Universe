//! Genuine first-principles phase derivation from SU(3)_3 conformal-block data.
//!
//! Unlike the admitted post-hoc quadratic interpolation, these are exact WZW
//! braiding labels. For SU(3)_3 the integrable primaries have conformal weights
//! h = C2(p,q)/(k+3) with C2=(p^2+q^2+pq+3p+3q)/3. The braiding label of a
//! four-□ conformal block in intermediate channel nu is |h_nu - 2 h_box|.
//! The distinct nonzero labels are:
//!   nu = □bar (0,1):      |2/9 - 4/9| = 2/9
//!   nu = Sym^2□ (2,0):    |5/9 - 4/9| = 1/9
//!   nu = adjoint (1,1):   |1/2 - 4/9| = 1/18
//!   (plus 4/9, 5/9 for other channels)
//!
//! The three smallest labels form a clean halving sequence
//!   {2/9, 1/9, 1/18} = {4/18, 2/18, 1/18}.
//! Compare with the phases required by recorded data:
//!   lepton 2/9 = label 2/9        EXACT
//!   down   ~1/10 vs label 1/9     +11% (the label is 0.1111)
//!   up     ~2/39 vs label 1/18    +8%  (the label is 0.0556)
//!
//! HONEST CAVEAT: the labels are rigorously derived from SU(3)_3 modular data
//! (no fitting), but they reproduce the quark phases only at the ~10% level,
//! NOT to the precision of the d=3-only synthesis. The lepton match is exact.
//! This is the closest genuine first-principles phase origin found; it does not
//! eliminate the post-hoc elements for quarks.

use crate::Check;

pub fn h_box() -> f64 { 2.0/9.0 }

/// Conformal weight C2(p,q)/6 of an SU(3)_3 primary.
pub fn h(p: i32, q: i32) -> f64 {
    let c2 = (p*p + q*q + p*q + 3*p + 3*q) as f64 / 3.0;
    c2 / 6.0
}

/// Braiding label |h_nu - 2 h_box|.
pub fn label(p: i32, q: i32) -> f64 { (h(p,q) - 2.0*h_box()).abs() }

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("wzworig.h_lepton", label(0,1), "lepton label = 2/9 (exact)"),
        Check::new("wzworig.h_down", label(2,0), "down braiding label = 1/9"),
        Check::new("wzworig.h_up", label(1,1), "up braiding label = 1/18"),
        Check::new("wzworig.down_vs_required", label(2,0)/0.1 - 1.0,
                   "down label vs required phase (~+0.11)"),
        Check::new("wzworig.up_vs_required", label(1,1)/(2.0/39.0) - 1.0,
                   "up label vs required phase (~+0.08)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lepton_label_is_exact() {
        assert!((label(0,1) - 2.0/9.0).abs() < 1e-14);
        assert!((label(1,0) - 2.0/9.0).abs() < 1e-14);
    }

    #[test]
    fn halving_sequence() {
        assert!((label(0,1) - 2.0*label(2,0)).abs() < 1e-12);
        assert!((label(2,0) - 2.0*label(1,1)).abs() < 1e-12);
    }

    #[test]
    fn quark_labels_are_within_15_percent() {
        assert!((label(2,0)/0.1 - 1.0).abs() < 0.15);
        assert!((label(1,1)/(2.0/39.0) - 1.0).abs() < 0.15);
    }
}
