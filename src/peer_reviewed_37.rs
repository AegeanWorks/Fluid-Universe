//! Peer-reviewed external support for the d=3 framework's key 3/13 relation.
//!
//! Source: E.K. Loginov, "Standard Model gauge coupling unification",
//! arXiv:1109.2052v3 (published; journal ref in the paper). Independently of
//! the d=3 informational-geometry framework, this paper derives the Weinberg
//! angle 3/13 as a gauge-coupling FIXED POINT in a Spin(8)⋊S₃ framework with
//! non-canonical generator normalization:
//!   eq (22): sin^2(theta)_mu = 3/13 - 110 alpha(mu)/(39 pi) ln(M0/mu)
//!   eq (28): M_W/M0 = sqrt(13 pi alpha0/3),  M0 = v = 246.2204 GeV
//!
//! We verified the paper's algebra numerically:
//!   eq (28) -> M_W = 80.38414 GeV (paper: 80.3841 GeV)  [5-digit agreement]
//!   eq (22) -> sin^2(MZ) = 0.2238 (paper: 0.2229) with the stated inputs.
//! The 3/13 value itself matches the recorded MZ Weinberg angle to 0.30%.
//!
//! HONEST CAVEATS:
//!  * Loginov's low-energy prediction (0.2229) is ~3.7% below the observed
//!    0.2312; only the FIXED-POINT value is 3/13. The d=3 framework instead
//!    claims 3/13 directly at MZ (0.3% agreement).
//!  * The two frameworks (Spin(8)⋊S₃ vs informational geometry) are distinct;
//!    this is independent convergence on the number 3/13, not mutual proof.
//!  * A separate Zenodo claim deriving 3/13 from SU(3)_3 x SU(2)_2 modular
//!    tensor categories has been WITHDRAWN by its author; we do not rely on it.

use crate::Check;

pub const M0: f64 = 246.2204;
pub const ALPHA0_INV: f64 = 127.726;
pub const FOUR_THIRTEEN: f64 = 3.0 / 13.0;

/// Loginov eq (28): M_W = M0 sqrt(13 pi alpha0 / 3), in GeV.
pub fn m_w_gev() -> f64 {
    let a0 = 1.0 / ALPHA0_INV;
    M0 * (13.0 * std::f64::consts::PI * a0 / 3.0).sqrt()
}

/// Loginov eq (22) low-energy prediction with inputs from the paper.
pub fn sin2_mz_loginov() -> f64 {
    let mu = 91.1876_f64;
    let alpha_inv = 127.937_f64;
    FOUR_THIRTEEN - 110.0/(39.0*std::f64::consts::PI) / alpha_inv * (M0/mu).ln()
}

pub fn observed_sin2_mz() -> f64 { 0.231453 }

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("peer37.fixed_point", FOUR_THIRTEEN, "Loginov fixed point 3/13"),
        Check::new("peer37.mw", m_w_gev(), "Loginov eq(28) M_W (GeV, paper 80.3841)"),
        Check::new("peer37.eq22", sin2_mz_loginov(), "Loginov eq(22) sin^2(MZ) (paper 0.2229)"),
        Check::new("peer37.fixed_vs_obs", FOUR_THIRTEEN/observed_sin2_mz() - 1.0,
                   "3/13 vs recorded MZ value (~-0.003)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loginov_mw_formula_reproduced() {
        assert!((m_w_gev() - 80.3841).abs() < 1e-3, "M_W = {}", m_w_gev());
    }

    #[test]
    fn loginov_eq22_reproduced() {
        assert!((sin2_mz_loginov() - 0.2229).abs() < 2e-3,
                "eq22 = {}", sin2_mz_loginov());
    }

    #[test]
    fn three_thirteenths_matches_recorded_within_half_percent() {
        assert!((FOUR_THIRTEEN/observed_sin2_mz() - 1.0).abs() < 0.005);
    }
}
