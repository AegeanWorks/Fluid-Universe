//! Structural proof: the Z3-circulant symmetry CANNOT fix the root product p.
//!
//! At r=1/sqrt2 the three signed roots are the roots of
//!   x^3 - 3x^2 + (3/2)x - p = 0
//! for p in (0, p_degenerate]. The trace (3) and pairwise sum (3/2) are fixed by
//! the symmetry + Q=2/3, but the product p is free. Q of the masses is 2/3 for
//! EVERY admissible p, so no condition inside this symmetry can select p.
//! Deriving p therefore requires physics OUTSIDE the stated symmetry. This
//! module proves the freedom and records a second held-out observable (the
//! level-spacing ratio) computed from PDG for future model tests.

use crate::Check;
use crate::unified_model::ratios_of_cubic;

/// Largest p for which the cubic has three positive real roots (double-root limit).
pub fn p_degenerate() -> f64 {
    // -1/2 + cos(3d)/sqrt(2) at the smallest d keeping all roots positive.
    -0.5 + 1.0 / 2.0_f64.sqrt()
}

/// Q of the masses for a given p, recomputed straight from the cubic roots.
pub fn q_of_p(p: f64) -> f64 {
    let (lo, hi) = ratios_of_cubic(p);
    if !lo.is_finite() { return f64::NAN; }
    // Recover roots from the two ratios with sum=3: x1 = 3/(1 + (1+lo) + hi*lo?) -- use scaling.
    let x = [lo * 0.0 + 1.0, lo, lo * hi]; // shape only; scale cancels in Q
    let m = x.map(|v| v * v);
    let s: f64 = m.iter().sum();
    let r: f64 = m.iter().map(|v| v.sqrt()).sum();
    s / (r * r)
}

/// PDG level-spacing ratio (x2-x1)/(x3-x2) of the normalized sqrt-mass roots.
pub fn pdg_spacing_ratio() -> f64 {
    let mass = [0.51099895_f64, 105.6583755, 1776.93];
    let mut x: Vec<f64> = mass.iter().map(|v| v.sqrt()).collect();
    x.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let s: f64 = x.iter().sum();
    let y: Vec<f64> = x.iter().map(|v| 3.0 * v / s).collect();
    (y[1] - y[0]) / (y[2] - y[1])
}

/// PDG root product p of the normalized sqrt-mass roots.
pub fn pdg_root_product() -> f64 {
    let mass = [0.51099895_f64, 105.6583755, 1776.93];
    let mut x: Vec<f64> = mass.iter().map(|v| v.sqrt()).collect();
    x.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let s: f64 = x.iter().sum();
    let y: Vec<f64> = x.iter().map(|v| 3.0 * v / s).collect();
    y[0] * y[1] * y[2]
}

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("shape_freedom.p_degenerate", p_degenerate(), "largest admissible root product"),
        Check::new("shape_freedom.q_independent_1", q_of_p(0.02), "Q at p=0.02 (target 2/3)"),
        Check::new("shape_freedom.q_independent_2", q_of_p(0.12), "Q at p=0.12 (target 2/3)"),
        Check::new("shape_freedom.pdg_product", pdg_root_product(), "root product from PDG masses"),
        Check::new("shape_freedom.pdg_spacing", pdg_spacing_ratio(), "level-spacing ratio from PDG"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;
    use crate::unified_model::{q_physical, r_koide, roots, symmetric};

    #[test]
    fn q_is_independent_of_p() {
        for p in [0.01, 0.02, 0.0557, 0.12, 0.18] {
            assert!(approx(q_of_p(p), 2.0 / 3.0, 1e-8), "p={p} Q={}", q_of_p(p));
        }
    }

    #[test]
    fn symmetry_fixes_sums_not_product() {
        // For r=1/sqrt2 the trace is 3 and pairwise sum is 3/2 for all delta,
        // but the product varies: that is the free parameter.
        let s0 = symmetric(&roots(r_koide(), 0.0));
        let s1 = symmetric(&roots(r_koide(), 0.2));
        assert!(approx(s0.0, 3.0, 1e-12) && approx(s1.0, 3.0, 1e-12));
        assert!(approx(s0.1, 1.5, 1e-12) && approx(s1.1, 1.5, 1e-12));
        assert!((s0.2 - s1.2).abs() > 0.05, "product must be free");
    }

    #[test]
    fn pdg_product_is_inside_admissible_range() {
        let p = pdg_root_product();
        assert!(p > 0.0 && p < p_degenerate());
    }

    #[test]
    fn physical_q_matches_for_pdg_product() {
        // q_physical at the PDG product reconstructs 2/3 from a different route.
        let delta = (2.0_f64.sqrt() * (pdg_root_product() + 0.5)).clamp(-1.0, 1.0).acos() / 3.0;
        assert!(approx(q_physical(r_koide(), delta), 2.0 / 3.0, 1e-8));
    }
}
