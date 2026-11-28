//! Independent verification of the d=3 framework's spectral (SUSY-QM) axioms.
//!
//! Source: verify_spectrum.py of the informational-geometry repository. We do
//! not endorse the physical interpretation; we check the mathematical claims:
//!   s(s-1) = beta,  s = (1+sqrt(1+4beta))/2        (beta = (d-1)/d)
//!   beta/2 - s^2/4 - (s-1)^2/4 = -1/4               for all d = 2..8
//!   d/du log(cosh^-s(u/2) e^{(s-1)u/2}) = -W(u)     with
//!       W(u) = (s/2)tanh(u/2) - (s-1)/2             (A psi_S = 0)
//!   W(u_max) -> 1/2 exactly for u_max -> infinity
//!       u_max = pi/alpha*,  alpha* = sqrt2/(6pi) -> u_max = 3 sqrt2 pi^2
//!   V_c(u) = (beta/4)(1-tanh(u/2))^2 >= 0            (no negative potential)
//!
//! These identities are SUSY-QM algebra, valid for any beta with s(s-1)=beta;
//! they show the spectral sector is internally consistent, not that d=3 is
//! physically correct. Recorded as validation of mathematical self-consistency.

use crate::Check;

pub fn beta(d: f64) -> f64 { (d - 1.0) / d }
pub fn s_from_beta(beta: f64) -> f64 { 0.5 * (1.0 + (1.0 + 4.0*beta).sqrt()) }
pub fn w(u: f64, s: f64) -> f64 { (s/2.0)*(u/2.0).tanh() - (s-1.0)/2.0 }
pub fn potential(u: f64, beta: f64) -> f64 {
    let t = (u/2.0).tanh();
    beta/4.0 * (1.0-t)*(1.0-t)
}
pub fn alpha_star() -> f64 { 2.0_f64.sqrt()/(6.0*std::f64::consts::PI) }
pub fn u_max() -> f64 { std::f64::consts::PI / alpha_star() }

/// Analytic log-derivative of psi_S at u: -(s/2)tanh(u/2) + (s-1)/2 = -W(u).
pub fn log_derivative_psi_s(u: f64, s: f64) -> f64 {
    -(s/2.0)*(u/2.0).tanh() + (s-1.0)/2.0
}

pub fn checks() -> Vec<Check> {
    let b = beta(3.0);
    let s = s_from_beta(b);
    vec![
        Check::new("d3spec.s_s_minus_1", s*(s-1.0), "s(s-1) = beta = 2/3"),
        Check::new("d3spec.eigenvalue_id",
                   b/2.0 - s*s/4.0 - (s-1.0)*(s-1.0)/4.0,
                   "beta/2 - s^2/4 - (s-1)^2/4 = -1/4"),
        Check::new("d3spec.logderiv_identity",
                   log_derivative_psi_s(1.234, s) + w(1.234, s),
                   "d/du log psi_S + W(u) = 0 at u=1.234"),
        Check::new("d3spec.kappa_star", w(u_max(), s),
                   "W(u_max) -> 1/2 (u_max = 3 sqrt2 pi^2)"),
        Check::new("d3spec.potential_min", (0..=40).map(|i| {
            potential(-20.0 + i as f64, b)
        }).fold(f64::INFINITY, f64::min), "V_c >= 0 on sample grid"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s_solves_the_quadratic() {
        let b = beta(3.0);
        assert!((b - 2.0/3.0).abs() < 1e-14);
        let s = s_from_beta(b);
        assert!((s*(s-1.0) - b).abs() < 1e-14);
    }

    #[test]
    fn eigenvalue_identity_holds_for_all_d() {
        for d in 2..=8 {
            let b = beta(d as f64);
            let s = s_from_beta(b);
            let val = b/2.0 - s*s/4.0 - (s-1.0)*(s-1.0)/4.0;
            assert!((val + 0.25).abs() < 1e-12, "d={d}: {val}");
        }
    }

    #[test]
    fn a_psi_s_zero_and_kappa_star() {
        let b = beta(3.0);
        let s = s_from_beta(b);
        for u in [-10.0, -1.0, 0.5, 1.234, 7.0] {
            assert!((log_derivative_psi_s(u, s) + w(u, s)).abs() < 1e-14);
        }
        assert!((w(u_max(), s) - 0.5).abs() < 1e-12, "kappa* = {}", w(u_max(), s));
    }

    #[test]
    fn potential_is_nonnegative() {
        let b = beta(3.0);
        for i in -200..=200 {
            assert!(potential(i as f64/10.0, b) >= -1e-15);
        }
    }
}
