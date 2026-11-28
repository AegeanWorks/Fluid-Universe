//! Orthogonal-polynomial sector: exact Koide point from Laguerre zeros.
//!
//! For generalized Laguerre L_3^(alpha), the three zeros x_i satisfy
//! e1 = 3(3+alpha), e2 = 3(3+alpha)(2+alpha), hence e1^2/e2 = 3(3+alpha)/(2+alpha).
//! The Koide condition Q=2/3 is equivalent to e1^2 = 6 e2, which fixes alpha=-1
//! exactly: L_3^(-1) has zeros {0, 3-sqrt(3), 3+sqrt(3)} and Q=2/3.
//! This is a genuine parameter-free occurrence of the Koide value. The module
//! also tests whether the resulting spectrum is physical (it is not: one zero).

use crate::Check;

/// Symmetric-function condition equivalent to Q=2/3 for three roots.
pub fn e1_squared_over_e2(alpha: f64) -> f64 {
    3.0 * (3.0 + alpha) / (2.0 + alpha)
}

/// Exact zeros of L_3^(-1): one zero and 3 +/- sqrt(3).
pub fn laguerre_minus_one_zeros() -> [f64; 3] {
    [0.0, 3.0 - 3.0_f64.sqrt(), 3.0 + 3.0_f64.sqrt()]
}

pub fn q_of(roots: &[f64; 3]) -> f64 {
    let sum: f64 = roots.iter().sum();
    let squares: f64 = roots.iter().map(|v| v * v).sum();
    squares / (sum * sum)
}

/// Ratio of the two nonzero roots: 2+sqrt(3).
pub fn nonzero_ratio() -> f64 {
    (3.0 + 3.0_f64.sqrt()) / (3.0 - 3.0_f64.sqrt())
}

pub fn checks() -> Vec<Check> {
    let roots = laguerre_minus_one_zeros();
    vec![
        Check::new("orthogonal.alpha_for_koide", -1.0, "alpha solving e1^2/e2=6"),
        Check::new("orthogonal.e1sq_over_e2", e1_squared_over_e2(-1.0),
                   "value at alpha=-1 (must be 6)"),
        Check::new("orthogonal.koide_q", q_of(&roots), "Q of the L_3^(-1) zeros"),
        Check::new("orthogonal.massless_state", roots[0], "the zero root (unphysical)"),
        Check::new("orthogonal.nonzero_ratio", nonzero_ratio(),
                   "ratio 2+sqrt(3) of the two nonzero roots"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn alpha_minus_one_is_exact() {
        assert!(approx(e1_squared_over_e2(-1.0), 6.0, 1e-14));
        assert!((e1_squared_over_e2(0.0) - 4.5).abs() < 1e-14);
        assert!((e1_squared_over_e2(1.0) - 4.0).abs() < 1e-14);
    }

    #[test]
    fn laguerre_zeros_give_exact_koide() {
        let roots = laguerre_minus_one_zeros();
        assert!(approx(q_of(&roots), 2.0 / 3.0, 1e-14));
        let e1: f64 = roots.iter().sum();
        let e2 = roots[0] * roots[1] + roots[0] * roots[2] + roots[1] * roots[2];
        assert!(approx(e1 * e1, 6.0 * e2, 1e-12));
    }

    #[test]
    fn spectrum_is_not_the_leptons() {
        let roots = laguerre_minus_one_zeros();
        assert!(roots[0].abs() < 1e-14, "massless state");
        assert!(approx(nonzero_ratio(), 2.0 + 3.0_f64.sqrt(), 1e-12));
        // Observed muon/tau root ratio is about sqrt(1776.93/105.66) ~ 4.10,
        // mass ratio ~16.8; 2+sqrt(3) ~ 3.73 is neither.
        assert!((nonzero_ratio() - 16.8).abs() > 10.0);
    }

    #[test]
    fn other_alphas_do_not_work() {
        for alpha in [0.0, 0.5, 1.0, 2.0, 3.0] {
            assert!((e1_squared_over_e2(alpha) - 6.0).abs() > 0.1);
        }
    }
}
