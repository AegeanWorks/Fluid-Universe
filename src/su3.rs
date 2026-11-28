//! SU(3) representation sector: do representation dimensions reproduce Q=2/3?
//!
//! This is a *candidate numerology scan*, not a derivation: with three free
//! Dynkin labels (p,q) the dimension triple has three degrees of freedom, so a
//! one-parameter condition like Q=2/3 can always be approximately satisfied.
//! The module records the closest hit and the fact that it is not exact.

use crate::Check;

/// Dimension of the SU(3) irrep with Dynkin labels (p,q).
pub fn dim(p: usize, q: usize) -> f64 {
    ((p + 1) * (q + 1) * (p + q + 2) / 2) as f64
}

pub fn koide(vals: &[f64; 3]) -> f64 {
    let sum: f64 = vals.iter().sum();
    let roots: f64 = vals.iter().map(|v| v.sqrt()).sum();
    sum / (roots * roots)
}

/// Closest dimension triple among reps with p,q < limit, distinct dimensions.
pub fn closest(limit: usize) -> ([f64; 3], f64) {
    let mut best: Option<([f64; 3], f64)> = None;
    for p1 in 0..limit {
        for q1 in 0..limit {
            if p1 + q1 == 0 { continue; }
            let d1 = dim(p1, q1);
            for p2 in p1..limit {
                for q2 in 0..limit {
                    if p2 + q2 == 0 { continue; }
                    let d2 = dim(p2, q2);
                    for p3 in p2..limit {
                        for q3 in 0..limit {
                            if p3 + q3 == 0 { continue; }
                            let d3 = dim(p3, q3);
                            if d1 == d2 || d2 == d3 || d1 == d3 { continue; }
                            let q = koide(&[d1, d2, d3]);
                            let gap = (q - 2.0 / 3.0).abs();
                            if best.as_ref().map(|b| gap < b.1).unwrap_or(true) {
                                best = Some(([d1, d2, d3], gap));
                            }
                        }
                    }
                }
            }
        }
    }
    best.unwrap_or(([0.0, 0.0, 0.0], 1.0))
}

pub fn checks() -> Vec<Check> {
    let (triple, gap) = closest(9);
    vec![
        Check::new("su3.closest_dims_1", triple[0], "first SU(3) dimension in the closest triple"),
        Check::new("su3.closest_dims_2", triple[1], "second SU(3) dimension"),
        Check::new("su3.closest_dims_3", triple[2], "third SU(3) dimension"),
        Check::new("su3.closest_q", koide(&triple), "Koide Q of the closest dimension triple"),
        Check::new("su3.closest_gap", gap, "absolute deviation from 2/3 (nonzero)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fundamental_and_adjoint_dimensions() {
        assert!((dim(1, 0) - 3.0).abs() < 1e-12);
        assert!((dim(1, 1) - 8.0).abs() < 1e-12);
        assert!((dim(0, 2) - 6.0).abs() < 1e-12);
    }

    #[test]
    fn closest_hit_is_near_but_not_exact() {
        let (triple, gap) = closest(8);
        assert!(gap > 1e-6, "a fit is not an identity, gap = {gap}");
        assert!((koide(&triple) - 2.0 / 3.0).abs() < 0.01);
    }

    #[test]
    fn koide_of_equal_dimensions_is_one_third() {
        assert!((koide(&[8.0, 8.0, 8.0]) - 1.0 / 3.0).abs() < 1e-12);
    }
}
