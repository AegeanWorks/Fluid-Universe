//! A single d=3 phase rule eliminating the Fibonacci/gauge branch split.
//!
//! Systematic search over simple ratios of d=3 building blocks
//! (scratch/single_rule_search.py) found NO block-ratio rule reproducing the
//! three phases. However, one parameter-free formula in d and n does:
//!   delta(n) = (d-1) / [ (d+1) n^2 - n + d(d-1) ]
//! For d=3 this gives delta(1)=2/9, delta(2)=1/10, delta(3)=2/39 exactly --
//! the charged-lepton, down and up phases from the d=3-only synthesis, with a
//! SINGLE rule (no branch choice).
//!
//! HONEST CAVEAT (critical): this is an INTERPOLATING quadratic fitted to the
//! three known phase values -- any three points can be fit by a quadratic, so
//! this is post-hoc curve-fitting, NOT a first-principles derivation. It is
//! recorded because it removes the branch split in a parameter-free way, but
//! it does not satisfy the demand for a non-post-hoc rule. The physical origin
//! of the phase pattern remains unexplained.
//!
//! The rule also predicts delta(4)=1/33, but n=4 is excluded by Q>1
//! (sector_count.rs), so this extrapolation has no physical sector.

use crate::Check;

pub fn d() -> i64 { 3 }
pub fn delta(n: i64) -> f64 {
    let d = d() as f64;
    (d - 1.0) / ((d + 1.0) * n as f64 * n as f64 - n as f64 + d * (d - 1.0))
}

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("phaseinterp.d1", delta(1), "delta(1) = 2/9 (lepton)"),
        Check::new("phaseinterp.d2", delta(2), "delta(2) = 1/10 (down)"),
        Check::new("phaseinterp.d3", delta(3), "delta(3) = 2/39 (up)"),
        Check::new("phaseinterp.d4", delta(4), "delta(4) = 1/33 (n=4 excluded)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_rule_reproduces_all_three_phases() {
        assert!((delta(1) - 2.0/9.0).abs() < 1e-14);
        assert!((delta(2) - 0.1).abs() < 1e-14);
        assert!((delta(3) - 2.0/39.0).abs() < 1e-14);
    }

    #[test]
    fn no_branch_split() {
        // One formula covers all sectors; no Fibonacci/gauge choice remains.
        assert_eq!(delta(1), 2.0/9.0);
        assert_eq!(delta(2), 0.1);
        assert_eq!(delta(3), 2.0/39.0);
    }
}
