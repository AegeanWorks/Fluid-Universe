//! Candidate external physics to fix the Z3-circulant shape parameter p.
//!
//! The circulant phase delta is physically a 3-site ring flux: the gauge-
//! invariant Wilson loop is exp(3 i delta). We test whether discrete flux
//! quantization or a natural value of cos(3 delta) fixes the empirical p
//! without fitting to lepton masses. Result: none does; each is a near-miss.

use crate::Check;
use std::f64::consts::PI;

/// Root product implied by a candidate Wilson-loop cosine: p = cos(3d)/sqrt2 - 1/2.
pub fn product_from_wilson(cos3d: f64) -> f64 {
    cos3d / 2.0_f64.sqrt() - 0.5
}

/// Product of the normalized sqrt-mass roots from PDG masses.
pub fn pdg_product() -> f64 {
    let mass = [0.51099895_f64, 105.6583755, 1776.93];
    let mut x: Vec<f64> = mass.iter().map(|v| v.sqrt()).collect();
    x.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let s: f64 = x.iter().sum();
    let y: Vec<f64> = x.iter().map(|v| 3.0 * v / s).collect();
    y[0] * y[1] * y[2]
}

/// Z_N flux quantization: needs 3 delta = 2 pi k / N with integer N.
/// Returns the required N for k=1 and whether it is an integer.
pub fn required_flux_quantum() -> (f64, bool) {
    let p = pdg_product();
    let cos3d = 2.0_f64.sqrt() * (p + 0.5);
    let flux = cos3d.acos(); // 3 delta in radians, principal branch
    let n = 2.0 * PI / flux; // k=1
    (n, (n - n.round()).abs() < 1e-6)
}

pub fn checks() -> Vec<Check> {
    let p = pdg_product();
    vec![
        Check::new("flux.pdg_product", p, "root product from PDG masses"),
        Check::new("flux.wilson_cosine", 2.0_f64.sqrt() * (p + 0.5), "cos(3 delta) from PDG"),
        Check::new("flux.candidate_pi_over_4", product_from_wilson(PI / 4.0),
                   "p if cos(3delta)=pi/4 (near-miss)"),
        Check::new("flux.required_ZN", required_flux_quantum().0,
                   "Z_N needed for exact flux quantization (not integer)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pdg_product_is_small_positive() {
        let p = pdg_product();
        assert!(p > 0.05 && p < 0.06, "p = {p}");
    }

    #[test]
    fn flux_quantization_is_not_an_integer() {
        let (n, integer) = required_flux_quantum();
        assert!(!integer, "required N = {n} is not an integer");
        // It is close to 3 pi ~ 9.42, not a quantization value.
        assert!((n - 3.0 * PI).abs() < 0.1, "N = {n}");
    }

    #[test]
    fn pi_over_4_is_only_a_near_miss() {
        let candidate = product_from_wilson(PI / 4.0);
        let gap = (candidate - pdg_product()).abs() / pdg_product();
        assert!(gap > 0.001 && gap < 0.02, "relative gap = {gap}");
    }
}
