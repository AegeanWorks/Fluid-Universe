//! Dynamical (RG-flow) test of the compact index framework's target values.
//!
//! The static framework predicts Q_up=8/9 and Q_down=3/4. A genuine dynamical
//! principle would make these RG fixed points (attractors in the UV or IR).
//! Using the recorded Huang-Zhou running of the quark Koide quotients from
//! M_Z to 10^12 GeV (`results/quarks.json`):
//!   Q_up:   0.8875692 at MZ -> 0.8925537 at 1e12  (drift +0.0049845)
//!   Q_down: 0.7480651 at MZ -> 0.7385352 at 1e12  (drift -0.0095298)
//!   distance of Q_up   to 8/9: 0.00132 -> 0.00366  (GROWS)
//!   distance of Q_down to 3/4: 0.00193 -> 0.01146  (GROWS)
//! Both quotients flow AWAY from the compact values as the scale increases;
//! they are neither UV nor IR attractors. The lepton Q is nearly RG-invariant
//! (distance to 2/3 ~ 0.00116 at both ends), consistent with the Sumino
//! stability mechanism, but its offset from 2/3 is fixed, not attracted.
//!
//! VERDICT: the compact targets are not dynamical fixed points of the observed
//! running. The agreement is closest at M_Z, supporting the interpretation as
//! a low-energy numerical regularity rather than a dynamical principle.

use crate::Check;

pub fn q_up_range() -> [f64; 2] { [0.8875692, 0.8925537] }
pub fn q_down_range() -> [f64; 2] { [0.7480651, 0.7385352] }
pub fn q_lepton_range() -> [f64; 2] { [0.667823994, 0.667826882] }

pub fn target_up() -> f64 { 8.0 / 9.0 }
pub fn target_down() -> f64 { 3.0 / 4.0 }
pub fn target_lepton() -> f64 { 2.0 / 3.0 }

pub fn checks() -> Vec<Check> {
    let up = q_up_range();
    let down = q_down_range();
    let lep = q_lepton_range();
    vec![
        Check::new("flow.up_dist_start", (up[0] - target_up()).abs(),
                   "Q_up distance to 8/9 at MZ"),
        Check::new("flow.up_dist_end", (up[1] - target_up()).abs(),
                   "Q_up distance to 8/9 at 1e12 GeV (grows)"),
        Check::new("flow.down_dist_start", (down[0] - target_down()).abs(),
                   "Q_down distance to 3/4 at MZ"),
        Check::new("flow.down_dist_end", (down[1] - target_down()).abs(),
                   "Q_down distance to 3/4 at 1e12 GeV (grows)"),
        Check::new("flow.lepton_drift", lep[1] - lep[0],
                   "lepton Q drift over 8 decades (Sumino stability)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quark_targets_are_not_attractors() {
        let up = q_up_range();
        let down = q_down_range();
        assert!((up[1] - target_up()).abs() > (up[0] - target_up()).abs(),
                "Q_up must flow away from 8/9");
        assert!((down[1] - target_down()).abs() > (down[0] - target_down()).abs(),
                "Q_down must flow away from 3/4");
    }

    #[test]
    fn lepton_q_is_rg_stable_but_offset() {
        let lep = q_lepton_range();
        assert!((lep[1] - lep[0]).abs() < 1e-5, "lepton Q should be nearly invariant");
        assert!((lep[0] - target_lepton()).abs() > 1e-4,
                "lepton Q is offset from 2/3 at the pole-mass scale");
    }
}
