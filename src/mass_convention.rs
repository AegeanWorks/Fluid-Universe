//! CORRECTION: the earlier "exact circulant no-go" was a mass-convention artifact.
//!
//! The recorded quark Koide quotients Q_up=0.8875692, Q_down=0.7480651 were
//! computed from the Huang & Zhou (2021) full-SM running masses at M_Z. The
//! modules `circulant_inverse`, `quark_phases` and `monodromy` originally took
//! their hierarchy ratios from a DIFFERENT, simplified mass set
//! (m_u=4.18 MeV, m_c=1.27 GeV, m_t=171.7 GeV; m_d=4.18 MeV, m_s=93.4 MeV,
//! m_b=4.18 GeV), then compared the implied Q against the Huang-Zhou Q. That
//! mixed two conventions and produced a spurious gap (0.0411 for up).
//!
//! With ONE consistent mass set (Huang-Zhou 2021, full SM, M_Z):
//!   up   masses (GeV) = {1.23e-3, 0.620, 168.26}
//!   down masses (GeV) = {2.67e-3, 53.16e-3, 2.839}
//! the Koide quotient implied by the two hierarchy ratios is EXACTLY the
//! recorded Q (identity of the Brannen reparametrization). The no-go is
//! RETRACTED as a physical result; it was an arithmetic inconsistency.
//!
//! What SURVIVES: the SU(3)_k quantum-dimension scan (no rep matches the quark
//! A^2) and the literature-phase failures. With consistent ratios the required
//! Brannen phases are delta_up ~= 0.0518 and delta_down ~= 0.1000, neither of
//! which is the SU(3)_3 conformal dimension 2/9. The charged-lepton derivation
//! is unaffected: its Q and ratios use one consistent PDG pole-mass set.

use crate::Check;

/// Huang & Zhou (2021), full SM, M_Z running masses in GeV.
pub fn up_masses_mz() -> [f64; 3] { [1.23e-3, 0.620, 168.26] }
pub fn down_masses_mz() -> [f64; 3] { [2.67e-3, 53.16e-3, 2.839] }

/// Koide quotient Q = sum(m) / (sum sqrt(m))^2.
pub fn q_from_masses(m: [f64; 3]) -> f64 {
    let s: f64 = m.iter().map(|v| v.sqrt()).sum();
    m.iter().sum::<f64>() / (s * s)
}

/// Sorted sqrt-mass hierarchy ratios.
pub fn sqrt_ratios(m: [f64; 3]) -> (f64, f64) {
    let mut s: Vec<f64> = m.iter().map(|v| v.sqrt()).collect();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    (s[1] / s[0], s[2] / s[1])
}

/// Q implied by two hierarchy ratios using the sum rule sum(sqrt m)=3.
/// This is an identity: for any three positive masses it returns q_from_masses.
pub fn implied_q(r1: f64, r2: f64) -> f64 {
    let y1 = 3.0 / (1.0 + r1 + r1 * r2);
    let y = [y1, r1 * y1, r1 * r2 * y1];
    y.iter().map(|v| v * v).sum::<f64>() / 9.0
}

/// Required Brannen amplitude from Q.
pub fn amplitude(q: f64) -> f64 { (6.0 * (q - 1.0 / 3.0)).sqrt() }

/// Required Brannen phase from the normalized roots of a consistent mass set.
pub fn required_delta(m: [f64; 3]) -> f64 {
    let q = q_from_masses(m);
    let a = amplitude(q);
    let mut s: Vec<f64> = m.iter().map(|v| v.sqrt()).collect();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let total: f64 = s.iter().sum();
    let y: Vec<f64> = s.iter().map(|v| 3.0 * v / total).collect();
    let s3: f64 = y.iter().map(|v| v * v * v).sum();
    let a2 = a * a;
    let cos3 = (4.0 / (3.0 * a * a2)) * (s3 - 3.0 - 9.0 * a2 / 2.0);
    cos3.clamp(-1.0, 1.0).acos() / 3.0
}

pub fn checks() -> Vec<Check> {
    let up = up_masses_mz();
    let down = down_masses_mz();
    let (ur1, ur2) = sqrt_ratios(up);
    let (dr1, dr2) = sqrt_ratios(down);
    vec![
        Check::new("mass_convention.q_up", q_from_masses(up),
                   "Huang-Zhou MZ up Q = recorded 0.8875692"),
        Check::new("mass_convention.q_down", q_from_masses(down),
                   "Huang-Zhou MZ down Q = recorded 0.7480651"),
        Check::new("mass_convention.up_gap", implied_q(ur1, ur2) - q_from_masses(up),
                   "corrected implied-minus-measured up Q gap (~0, no-go retracted)"),
        Check::new("mass_convention.down_gap", implied_q(dr1, dr2) - q_from_masses(down),
                   "corrected implied-minus-measured down Q gap (~0)"),
        Check::new("mass_convention.delta_up", required_delta(up),
                   "required up Brannen phase from consistent masses"),
        Check::new("mass_convention.delta_down", required_delta(down),
                   "required down Brannen phase from consistent masses"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consistent_masses_reproduce_recorded_q() {
        assert!((q_from_masses(up_masses_mz()) - 0.8875692313187327).abs() < 1e-9);
        assert!((q_from_masses(down_masses_mz()) - 0.7480650548031402).abs() < 1e-9);
    }

    #[test]
    fn implied_q_is_an_identity_for_consistent_masses() {
        for m in [up_masses_mz(), down_masses_mz()] {
            let (r1, r2) = sqrt_ratios(m);
            assert!((implied_q(r1, r2) - q_from_masses(m)).abs() < 1e-12);
        }
    }

    #[test]
    fn the_old_gap_was_a_convention_artifact() {
        // The old modules used m_u=4.18 MeV, m_c=1.27 GeV, m_t=171.7 GeV,
        // whose Q is 0.8465, not the recorded 0.8876.
        let old_up = [0.00418_f64, 1.27, 171.7];
        assert!((q_from_masses(old_up) - 0.8875692313187327).abs() > 0.04);
    }

    #[test]
    fn required_phases_are_not_the_lepton_phase() {
        let lepton = 2.0 / 9.0;
        assert!((required_delta(up_masses_mz()) - lepton).abs() > 0.1);
        assert!((required_delta(down_masses_mz()) - lepton).abs() > 0.1);
    }
}
