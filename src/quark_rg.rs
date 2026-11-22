//! Down-quark RG channel: does Q_D reach 2/3 at the unification scale?
//!
//! One-loop SM running of the three down-type Yukawas plus the top Yukawa and
//! gauge couplings. The Koide ratio of the three down masses is tracked from
//! M_Z to 10^16 GeV.

use crate::Check;
use std::f64::consts::PI;

type State = [f64; 7];

const C: f64 = 1.0 / (16.0 * PI * PI);

fn deriv(s: &State) -> State {
    let [yd, ys, yb, yt, g1, g2, g3] = s;
    let t = yt * yt + yd * yd + ys * ys + yb * yb;
    let common = t - 8.0 * g3 * g3 - g1 * g1 / 6.0 - 2.25 * g2 * g2;
    [
        C * yd * (1.5 * yd * yd + common),
        C * ys * (1.5 * ys * ys + common),
        C * yb * (1.5 * yb * yb + common),
        C * yt * (3.0 * yt * yt + 0.5 * t - 8.0 * g3 * g3
                  - 17.0 / 12.0 * g1 * g1 - 2.25 * g2 * g2),
        C * (41.0 / 6.0) * g1 * g1 * g1,
        C * (-19.0 / 6.0) * g2 * g2 * g2,
        C * (-7.0) * g3 * g3 * g3,
    ]
}

fn add(s: &State, k: &State, factor: f64) -> State {
    let mut out = *s;
    for i in 0..7 {
        out[i] += factor * k[i];
    }
    out
}

/// Koide ratio of three masses (the common Yukawa scale cancels).
pub fn koide(m: &[f64; 3]) -> f64 {
    let sum: f64 = m.iter().sum();
    let roots: f64 = m.iter().map(|v| v.sqrt()).sum();
    sum / (roots * roots)
}

/// Integrate from M_Z to 10^16 GeV and return (Q at MZ, Q at 1e16, ratios at 1e16).
pub fn run() -> (f64, f64, f64, f64) {
    let v = 174.0;
    let mut s: State = [0.00418 / v, 0.0934 / v, 4.18 / v, 0.96, 0.36, 0.65, 1.20];
    let q_start = koide(&[s[0], s[1], s[2]]);
    let steps = 4000usize;
    let dt = (1e16_f64.ln() - 91.1876_f64.ln()) / steps as f64;
    for _ in 0..steps {
        let k1 = deriv(&s);
        let k2 = deriv(&add(&s, &k1, dt / 2.0));
        let k3 = deriv(&add(&s, &k2, dt / 2.0));
        let k4 = deriv(&add(&s, &k3, dt));
        s = add(&s, &k1, dt / 6.0);
        for i in 0..7 {
            s[i] += dt / 6.0 * (2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
        }
    }
    (q_start, koide(&[s[0], s[1], s[2]]), s[1] / s[0], s[2] / s[0])
}

pub fn checks() -> Vec<Check> {
    let (q_mz, q_gut, r21, r31) = run();
    vec![
        Check::new("quark_rg.q_at_mz", q_mz, "down-quark Koide ratio at MZ"),
        Check::new("quark_rg.q_at_gut", q_gut, "down-quark Koide ratio at 1e16 GeV"),
        Check::new("quark_rg.drift", (q_gut - q_mz).abs(), "RG drift of the down-quark Koide ratio"),
        Check::new("quark_rg.ratio_21", r21, "m_s/m_d at the GUT scale"),
        Check::new("quark_rg.ratio_31", r31, "m_b/m_d at the GUT scale"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_does_not_reach_two_thirds() {
        let (q_mz, q_gut, _, _) = run();
        assert!(q_mz > 0.7 && q_mz < 0.76, "Q(MZ)={q_mz}");
        assert!(q_gut > 0.7 && q_gut < 0.76, "Q(GUT)={q_gut}");
        assert!((q_gut - 2.0 / 3.0).abs() > 0.05);
    }

    #[test]
    fn ratios_are_positive_and_ordered() {
        let (_, _, r21, r31) = run();
        assert!(r21 > 1.0 && r31 > r21);
    }

    #[test]
    fn koide_of_equal_masses_is_one_third() {
        assert!((koide(&[1.0, 1.0, 1.0]) - 1.0 / 3.0).abs() < 1e-12);
    }
}
