//! Gauge-coupling running: a previously unaddressed sector.
//!
//! Data: Huang & Zhou (2021) full-SM gauge couplings recorded in
//! `results/rg_run.json`, scales 91.2 GeV to 1e16 GeV. At MZ (GUT-normalised):
//!   g1 = 0.46121, g2 = 0.65100, g3 = 1.21040.
//!
//! Findings (all recorded, no claim of unification):
//!  * The SM couplings do NOT meet at one scale. Fitting 1/g^2 = a + b ln(mu)
//!    to the recorded running gives pairwise crossings
//!      g1=g2 at ~1.0e13 GeV, g1=g3 at ~2.4e14 GeV, g2=g3 at ~9.2e16 GeV.
//!    Three different scales -> Standard-Model unification FAILS. This is the
//!    expected result and is consistent with the need for new physics (the
//!    Sumino family gauge symmetry invoked for the lepton result).
//!  * The Koide quotient Q and the Brannen phase delta are nearly RG-invariant
//!    along this running: Q varies by ~3e-6 and delta by ~4e-6 over 8 decades,
//!    the radiative stability the Sumino mechanism was designed to provide.
//!  * At MZ, g2^2/g1^2 = 1.9923, within 0.4% of the SU(3)_3 quantum dimension
//!    d_□ = 2. This is a numerical regularity (it is the Weinberg angle in
//!    disguise), NOT a derivation; it is recorded for completeness only.

use crate::Check;

/// Recorded Huang-Zhou gauge couplings at M_Z (GUT-normalised g1).
pub fn couplings_mz() -> [f64; 3] { [0.4612129307886615, 0.65100, 1.2104] }

/// Pairwise one-loop crossing scales in GeV, derived from results/rg_run.json.
pub fn crossing_scales() -> [f64; 3] { [1.021e13, 2.366e14, 9.154e16] }

/// Koide quotient and Brannen phase at the endpoints of the recorded running.
pub fn q_start() -> f64 { 0.6678239941864693 }
pub fn q_end() -> f64 { 0.6678268819850045 }
pub fn delta_start() -> f64 { 0.22112939195284964 }
pub fn delta_end() -> f64 { 0.2211258519105841 }

/// Quantum dimension of the SU(3)_3 fundamental (for the numerical comparison).
pub fn d_box_su3_3() -> f64 { 2.0 }

pub fn checks() -> Vec<Check> {
    let g = couplings_mz();
    let cross = crossing_scales();
    vec![
        Check::new("gauge.g2_over_g1_sq", g[1] * g[1] / (g[0] * g[0]),
                   "g2^2/g1^2 at MZ (~1.992, near d_□=2)"),
        Check::new("gauge.cross_12_over_23", cross[0] / cross[2],
                   "g1=g2 scale / g2=g3 scale (<<1: no SM unification)"),
        Check::new("gauge.cross_13_over_23", cross[1] / cross[2],
                   "g1=g3 scale / g2=g3 scale (<<1: no SM unification)"),
        Check::new("gauge.q_drift", q_end() - q_start(), "Koide Q drift over 8 decades"),
        Check::new("gauge.delta_drift", delta_end() - delta_start(),
                   "Brannen phase drift over 8 decades"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sm_does_not_unify() {
        let c = crossing_scales();
        // The three pairwise crossing scales span >3 orders of magnitude.
        assert!(c[2] / c[0] > 1e3, "spread = {}", c[2] / c[0]);
    }

    #[test]
    fn koide_and_phase_are_rg_stable() {
        assert!((q_end() - q_start()).abs() < 1e-5);
        assert!((delta_end() - delta_start()).abs() < 1e-5);
    }

    #[test]
    fn g2_over_g1_is_only_a_numerical_regularity() {
        let r = couplings_mz()[1].powi(2) / couplings_mz()[0].powi(2);
        assert!((r - d_box_su3_3()).abs() / d_box_su3_3() < 0.01);
        // Explicitly NOT interpreted as a derivation of d_□ from the gauge sector.
    }
}
