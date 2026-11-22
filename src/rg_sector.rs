//! One-loop SM renormalization-group sector for the circulant phase and ratio.
//!
//! The SU(3)-symmetric construction fixes |c|/a=1/sqrt(2) at a high scale. This
//! module asks whether the RG supplies the second parameter-free number, delta,
//! by driving a symmetric high-scale phase to the empirically preferred 2/9.

use crate::Check;
use std::f64::consts::PI;

type State = [f64; 7]; // y_e, y_mu, y_tau, y_t, g1, g2, g3

const C: f64 = 1.0 / (16.0 * PI * PI);

fn deriv(s: &State) -> State {
    let [ye, ymu, ytau, yt, g1, g2, g3] = s;
    let t = yt * yt + ye * ye + ymu * ymu + ytau * ytau;
    let common = t - 2.25 * g2 * g2 - 3.75 * g1 * g1;
    [
        C * ye * (1.5 * ye * ye + common),
        C * ymu * (1.5 * ymu * ymu + common),
        C * ytau * (1.5 * ytau * ytau + common),
        C * yt * (3.0 * yt * yt + 0.5 * t - 8.0 * g3 * g3
                  - 17.0 / 12.0 * g1 * g1 - 2.25 * g2 * g2),
        C * (41.0 / 6.0) * g1 * g1 * g1,
        C * (-19.0 / 6.0) * g2 * g2 * g2,
        C * (-7.0) * g3 * g3 * g3,
    ]
}

fn advance(s: &State, step: f64) -> State {
    let shifted = |k: &State| -> State {
        let mut out = *s;
        for i in 0..7 {
            out[i] += step * k[i];
        }
        out
    };
    let k1 = deriv(s);
    let k2 = deriv(&shifted(&k1));
    let k3 = deriv(&shifted(&k2));
    let k4 = deriv(&shifted(&k3));
    let mut out = *s;
    for i in 0..7 {
        out[i] += step / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
    }
    out
}

/// Run the one-loop system from 10^16 GeV down to M_Z, RK4 with fixed steps.
pub fn run_high_to_mz(delta0: f64, ratio: f64) -> State {
    let roots = [0usize, 1, 2].map(|k| 1.0 + 2.0 * ratio * (delta0 + 2.0 * PI * k as f64 / 3.0).cos());
    let squared = roots.map(|v| v.max(1e-6) * v.max(1e-6));
    let scale = 2.7770900226649226e-6 / squared[0];
    let mut state: State = [squared[0] * scale, squared[1] * scale, squared[2] * scale,
                            0.90, 0.50, 0.50, 0.40];
    let steps = 2000usize;
    let step = (91.1876_f64.ln() - 1e16_f64.ln()) / steps as f64;
    for _ in 0..steps {
        state = advance(&state, step);
    }
    state
}

/// Circulant phase of the three Yukawa square roots, wrapped to [0, 2pi).
pub fn phase(state: &State) -> f64 {
    let x = [state[0].sqrt(), state[1].sqrt(), state[2].sqrt()];
    let mut re = 0.0;
    let mut im = 0.0;
    for k in 0..3 {
        let angle = -2.0 * PI * k as f64 / 3.0;
        re += x[k] * angle.cos();
        im += x[k] * angle.sin();
    }
    let angle = im.atan2(re).rem_euclid(2.0 * PI);
    if angle > 2.0 * PI - 1e-12 { 0.0 } else { angle }
}

/// Effective ratio |c|/a recovered from the running Yukawas.
pub fn effective_ratio(state: &State) -> f64 {
    let x = [state[0].sqrt(), state[1].sqrt(), state[2].sqrt()];
    let mean = (x[0] + x[1] + x[2]) / 3.0;
    let mut re = 0.0;
    let mut im = 0.0;
    for k in 0..3 {
        let angle = -2.0 * PI * k as f64 / 3.0;
        re += x[k] * angle.cos();
        im += x[k] * angle.sin();
    }
    ((re * re + im * im).sqrt() / 3.0 / mean).abs()
}

pub fn checks() -> Vec<Check> {
    let r = 2.0_f64.powf(-0.5);
    let state = run_high_to_mz(2.0 / 9.0, r);
    vec![
        Check::new("rg.delta_at_mz", phase(&state), "MZ phase for delta0=2/9 at 1e16 GeV"),
        Check::new("rg.ratio_at_mz", effective_ratio(&state), "|c|/a at MZ"),
        Check::new("rg.delta_drift", (phase(&state) - 2.0 / 9.0).abs(),
                   "absolute RG drift of the Koide phase"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn symmetric_phase_is_nearly_running_invariant() {
        let state = run_high_to_mz(2.0 / 9.0, 2.0_f64.powf(-0.5));
        assert!((phase(&state) - 2.0 / 9.0).abs() < 1e-8, "phase = {}", phase(&state));
    }

    #[test]
    fn ratio_stays_near_the_symmetric_value() {
        let state = run_high_to_mz(0.1, 2.0_f64.powf(-0.5));
        let ratio = effective_ratio(&state);
        assert!((ratio - 2.0_f64.powf(-0.5)).abs() < 1e-3, "ratio = {ratio}");
        assert!(ratio.is_finite() && ratio > 0.0);
    }

    #[test]
    fn no_phase_attractor_at_small_delta() {
        for delta in [0.05_f64, 0.1, 2.0 / 9.0] {
            let state = run_high_to_mz(delta, 2.0_f64.powf(-0.5));
            assert!(approx(phase(&state), delta, 1e-7));
        }
    }

    #[test]
    fn large_phase_does_run() {
        let state = run_high_to_mz(0.4, 2.0_f64.powf(-0.5));
        assert!((phase(&state) - 0.4).abs() > 1e-3);
    }
}
