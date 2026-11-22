//! First genuinely NON-CIRCULANT candidate: the exceptional Jordan spectrum.
//!
//! Motivated by arXiv:2508.10131 / 2605.24866 and the TIFR papers on the
//! characteristic equation of J3(O): a real coassociative octonionic
//! configuration has the ARITHMETIC (equally spaced) Jordan spectrum
//!   lambda in { q - delta, q, q + delta },   delta^2 = 3/8  (theory-fixed).
//! This is NOT the Brannen circulant 1 + A cos(delta + 2pi k/3). The closed-form
//! square-root mass ratios from the Sym^3(3) minimal ladder are
//!   D = sqrt(m_s/m_d) = (1+delta)/(1-delta)
//!   B = sqrt(m_b/m_s) = D (1+delta)
//!   C = sqrt(m_c/m_u) = (2/3+delta)/(2/3-delta)
//!   T = sqrt(m_t/m_c) = (2/3)/(2/3-delta)
//! with NO fitted parameters (delta is fixed by the Jordan algebra).
//!
//! Verified result: in a consistent PDG-like convention (m_u=2.3, m_c=1275,
//! m_t=173210 MeV; m_d=4.67, m_s=93.4, m_b=4180 MeV) this predicts
//!   B within 0.25% and C within 0.06%  (permille-level, zero fits)
//!   but D off by -7% and T off by +5.4%.
//! The implied Koide quotients Q_down=0.729 and Q_up=0.855 are BELOW the
//! measured MS-bar ranges [0.73854,0.74807] and [0.88757,0.89255].
//!
//! HONEST VERDICT: this is the first non-circulant construction here that fits
//! ANY quark ratios at permille level, but it fits only HALF of them and misses
//! the quark Q ranges. arXiv:2508.10131 itself states the up-sector relation
//! fails at the electroweak benchmark and that no precision mass explanation
//! is established. This is a promising structure, NOT a unified theory.

use crate::Check;
use crate::circulant_inverse::implied_q;

/// The theory-fixed Jordan spectral parameter delta = sqrt(3/8).
pub fn jordan_delta() -> f64 { (3.0_f64 / 8.0).sqrt() }

/// sqrt(m_s/m_d).
pub fn ratio_d() -> f64 {
    let d = jordan_delta();
    (1.0 + d) / (1.0 - d)
}

/// sqrt(m_b/m_s).
pub fn ratio_b() -> f64 {
    let d = jordan_delta();
    ratio_d() * (1.0 + d)
}

/// sqrt(m_c/m_u).
pub fn ratio_c() -> f64 {
    let d = jordan_delta();
    (2.0 / 3.0 + d) / (2.0 / 3.0 - d)
}

/// sqrt(m_t/m_c).
pub fn ratio_t() -> f64 {
    let d = jordan_delta();
    (2.0 / 3.0) / (2.0 / 3.0 - d)
}

/// PDG-like reference masses (MeV), consistent convention for the comparison.
pub fn pdg_up() -> [f64; 3] { [2.3, 1275.0, 173210.0] }
pub fn pdg_down() -> [f64; 3] { [4.67, 93.4, 4180.0] }

fn sqrt_ratios(m: [f64; 3]) -> (f64, f64) {
    ((m[1] / m[0]).sqrt(), (m[2] / m[1]).sqrt())
}

pub fn checks() -> Vec<Check> {
    let (_, down_obs) = sqrt_ratios(pdg_down());
    let (up_obs, _) = sqrt_ratios(pdg_up());
    vec![
        Check::new("jordan.delta_sq", jordan_delta().powi(2), "delta^2 = 3/8 (theory)"),
        Check::new("jordan.ratio_B", ratio_b(), "sqrt(mb/ms) predicted (obs 6.690)"),
        Check::new("jordan.ratio_C", ratio_c(), "sqrt(mc/mu) predicted (obs 23.545)"),
        Check::new("jordan.B_relerr", ratio_b() / down_obs - 1.0, "B relative error (permille target)"),
        Check::new("jordan.C_relerr", ratio_c() / up_obs - 1.0, "C relative error (permille target)"),
        Check::new("jordan.D_relerr", ratio_d() / sqrt_ratios(pdg_down()).0 - 1.0,
                   "D relative error (expected ~-7%)"),
        Check::new("jordan.T_relerr", ratio_t() / (pdg_up()[2] / pdg_up()[1]).sqrt() - 1.0,
                   "T relative error (expected ~+5%)"),
        Check::new("jordan.q_down_implied", implied_q(ratio_d(), ratio_b()),
                   "implied Q_down (measured range starts 0.73854)"),
        Check::new("jordan.q_up_implied", implied_q(ratio_c(), ratio_t()),
                   "implied Q_up (measured range starts 0.88757)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delta_is_theory_fixed() {
        assert!((jordan_delta().powi(2) - 3.0 / 8.0).abs() < 1e-14);
    }

    #[test]
    fn b_and_c_fit_at_permille_level() {
        let (_, down_obs) = sqrt_ratios(pdg_down());
        let (up_obs, _) = sqrt_ratios(pdg_up());
        assert!((ratio_b() / down_obs - 1.0).abs() < 0.01, "B err");
        assert!((ratio_c() / up_obs - 1.0).abs() < 0.01, "C err");
    }

    #[test]
    fn d_and_t_fail() {
        let (down_obs_r1, _) = sqrt_ratios(pdg_down());
        let (_, up_obs_r2) = sqrt_ratios(pdg_up());
        assert!((ratio_d() / down_obs_r1 - 1.0).abs() > 0.03, "D should fail");
        assert!((ratio_t() / up_obs_r2 - 1.0).abs() > 0.03, "T should fail");
    }

    #[test]
    fn implied_q_is_below_measured_ranges() {
        assert!(implied_q(ratio_d(), ratio_b()) < 0.73854);
        assert!(implied_q(ratio_c(), ratio_t()) < 0.88757);
    }

    #[test]
    fn this_is_not_the_brannen_circulant() {
        // Jordan spectrum is arithmetic (q-d, q, q+d): middle is the mean.
        let d = jordan_delta();
        let q = 2.0 / 3.0;
        let lam = [q - d, q, q + d];
        assert!((lam[0] + lam[2] - 2.0 * lam[1]).abs() < 1e-12);
    }
}
