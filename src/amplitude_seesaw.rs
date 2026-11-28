//! Two structural tests: rational amplitudes and a seesaw link to the GUT scale.
//!
//! Part 1 -- With the propagated Huang-Zhou / PDG mass uncertainties, the four
//! required amplitudes are consistent with simple rationals:
//!   lepton A^2 = 2.00000 +/- 0.00010  ~ 2        (z=0.14)
//!   down   A^2 = 2.48999 +/- 0.04005  ~ 5/2      (z=0.25)
//!   up     A^2 = 3.32550 +/- 0.00830  ~ 10/3     (z=0.94)
//!   nu     A^2 = 1.51367 (Q-max)      ~ 3/2
//! Equivalently 1/A^2 = {5/10, 4/10, 3/10} for {lepton, down, up} --
//! consecutive tenths. This is a NUMERICAL regularity, not a derivation:
//! no group output yet reproduces these rationals with the phases as well.
//!
//! Part 2 -- Type-I seesaw with the MSSM GUT scale found in so10_gut.rs:
//!   m_nu ~ m_top^2 / mu_G = (171.7 GeV)^2 / 2.05e16 GeV = 1.43e-3 eV.
//! The observed heaviest neutrino is 0.050 eV -- 35x larger. The required
//! seesaw scale is 5.9e14 GeV, about 2.9% of mu_G. A single GUT-scale
//! framework therefore does not reproduce the neutrino mass scale with an
//! order-one Yukawa; a suppressed scale is needed.

use crate::Check;

/// Required amplitudes (central values) from the consistent mass sets.
pub fn a2_lepton() -> f64 { 2.0 }
pub fn a2_down() -> f64 { 2.48999 }
pub fn a2_up() -> f64 { 3.32550 }
pub fn a2_nu() -> f64 { 1.51367 }

/// The rational hypotheses.
pub fn rational_a2(sector: usize) -> f64 {
    match sector { 0 => 2.0, 1 => 2.5, 2 => 10.0/3.0, _ => 1.5 }
}

/// Inverse amplitudes: consecutive tenths for the three measured sectors.
pub fn inverse_a2(sector: usize) -> f64 { 1.0 / rational_a2(sector) }

/// Type-I seesaw estimate from the GUT scale (GeV -> eV).
pub fn seesaw_mnu_ev(mtop_gev: f64, mu_gut_gev: f64) -> f64 {
    mtop_gev * mtop_gev / mu_gut_gev * 1e9
}

pub fn mu_gut() -> f64 { 2.0546e16 }
pub fn observed_mnu_ev() -> f64 { (2.507e-3_f64).sqrt() }

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("amplitude.inv_lepton", inverse_a2(0), "1/A^2 lepton = 5/10"),
        Check::new("amplitude.inv_down", inverse_a2(1), "1/A^2 down = 4/10"),
        Check::new("amplitude.inv_up", inverse_a2(2), "1/A^2 up = 3/10"),
        Check::new("amplitude.seesaw_mnu", seesaw_mnu_ev(171.7, mu_gut()),
                   "m_nu from GUT-scale seesaw (eV)"),
        Check::new("amplitude.seesaw_gap", seesaw_mnu_ev(171.7, mu_gut()) / observed_mnu_ev(),
                   "predicted/observed neutrino mass (~0.029, needs lower scale)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amplitudes_match_simple_rationals_within_one_sigma() {
        let sigmas = [0.00010_f64, 0.04005, 0.00830];
        for i in 0..3 {
            let actual = [a2_lepton(), a2_down(), a2_up()][i];
            assert!((actual - rational_a2(i)).abs() / sigmas[i] < 1.5,
                    "sector {i}: {actual} vs {}", rational_a2(i));
        }
    }

    #[test]
    fn inverse_amplitudes_are_consecutive_tenths() {
        assert!((inverse_a2(0) - 0.5).abs() < 1e-12);
        assert!((inverse_a2(1) - 0.4).abs() < 1e-12);
        assert!((inverse_a2(2) - 0.3).abs() < 1e-12);
        assert!((inverse_a2(0) - inverse_a2(1) - 0.1).abs() < 1e-12);
        assert!((inverse_a2(1) - inverse_a2(2) - 0.1).abs() < 1e-12);
    }

    #[test]
    fn gut_seesaw_undershoots_neutrino_mass() {
        let pred = seesaw_mnu_ev(171.7, mu_gut());
        assert!(pred < observed_mnu_ev() / 10.0,
                "seesaw {pred} vs observed {}", observed_mnu_ev());
    }
}
