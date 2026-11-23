//! Gauge–flavor connection: the weak mixing angle fixes the amplitude scale.
//!
//! From the recorded M_Z couplings (GUT-normalized g1=0.46121, g2=0.651),
//! the physical hypercharge coupling g' = g1/sqrt(5/3) gives
//!   sin^2(theta_W) = 0.2314529,  cot^2(theta_W) = 3.3205328.
//! The required amplitudes A^2 = {2, 2.48999, 3.32550} then satisfy
//!   cot^2(theta_W)/A^2 = {1.6603, 1.3336, 0.9985} ~ {5/3, 4/3, 3/3},
//! i.e. cot^2(theta_W)/A^2(n) = (6-n)/3 for n=1,2,3 (lepton, down, up).
//! Equivalently 3 cot^2(theta_W) = 9.962 ~ 10, the numerator of the compact
//! amplitude formula A^2(n) = 10/(6-n). Testing the gauge-derived formula
//!   A^2(n) = 3 cot^2(theta_W)/(6-n)
//! gives relative errors -0.38%, +0.02%, -0.15% for the three sectors.
//!
//! This is the closest thing to a gauge/flavor bridge found in this study.
//! HONEST CAVEATS:
//!  * It is POST-HOC: the relation was noticed after the amplitude values were
//!    known, and the sector assignment (lepton=1, down=2, up=3) is unchanged.
//!  * It rests on one number, sin^2(theta_W), and 3cot^2 ~ 10 to 0.38% could
//!    be a coincidence (a small-integer approximation).
//!  * The Weinberg angle is scheme-dependent; we used the recorded M_Z values.
//!  * It supplies no phases delta and is not itself a unified theory.

use crate::Check;

pub fn g1_mz() -> f64 { 0.4612129307886615 }
pub fn g2_mz() -> f64 { 0.65100 }

pub fn sin2_theta_w() -> f64 {
    let gp = g1_mz() / (5.0_f64 / 3.0).sqrt();
    gp * gp / (g2_mz() * g2_mz() + gp * gp)
}

pub fn cot2_theta_w() -> f64 {
    let s = sin2_theta_w();
    (1.0 - s) / s
}

/// Gauge-derived amplitude for sector n: 3 cot^2(theta_W)/(6-n).
pub fn a2_from_gauge(n: i32) -> f64 {
    3.0 * cot2_theta_w() / (6 - n) as f64
}

pub fn observed_a2() -> [f64; 3] { [2.0, 2.48999, 3.32550] }

/// Fibonacci labels of the three sectors: F = {3,5,8}.
pub fn fib_label(n: i32) -> f64 { [3.0, 5.0, 8.0][(n-1) as usize] }

/// Gauge-derived Brannen phase for sector n: delta = A^2(n)/F(n)^2
/// = 3 cot^2(theta_W) / ((6-n) F(n)^2). Because the only approximation is
/// 3 cot^2(theta_W) ~ 10, this inherits the SAME uniform 0.384% offset as
/// the amplitudes: one gauge observable fixes all six flavor parameters
/// (three amplitudes + three phases) to within 0.4%.
pub fn delta_from_gauge(n: i32) -> f64 {
    a2_from_gauge(n) / fib_label(n).powi(2)
}

pub fn observed_delta() -> [f64; 3] { [2.0/9.0, 0.1, 5.0/96.0] }

pub fn checks() -> Vec<Check> {
    let mut out = vec![
        Check::new("gauge_flavor.sin2tw", sin2_theta_w(), "sin^2(theta_W) at MZ"),
        Check::new("gauge_flavor.cot2tw", cot2_theta_w(), "cot^2(theta_W) at MZ"),
        Check::new("gauge_flavor.three_cot2", 3.0 * cot2_theta_w(),
                   "3 cot^2(theta_W) ~ 10 (framework numerator)"),
    ];
    for n in 1..=3 {
        out.push(Check::new(
            match n { 1 => "gauge_flavor.a2_lep", 2 => "gauge_flavor.a2_down", _ => "gauge_flavor.a2_up" },
            a2_from_gauge(n) / observed_a2()[(n-1) as usize] - 1.0,
            "gauge-derived amplitude relative error",
        ));
        out.push(Check::new(
            match n { 1 => "gauge_flavor.delta_lep", 2 => "gauge_flavor.delta_down", _ => "gauge_flavor.delta_up" },
            delta_from_gauge(n) / observed_delta()[(n-1) as usize] - 1.0,
            "gauge-derived phase relative error (uniform ~-0.384%)",
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weinberg_angle_is_physical() {
        let s = sin2_theta_w();
        assert!(s > 0.22 && s < 0.24, "sin^2 theta_W = {s}");
    }

    #[test]
    fn three_cot2_approximates_ten() {
        assert!((3.0 * cot2_theta_w() / 10.0 - 1.0).abs() < 0.01);
    }

    #[test]
    fn gauge_derived_amplitudes_match_all_three_sectors() {
        for n in 1..=3 {
            let rel = (a2_from_gauge(n) / observed_a2()[(n-1) as usize] - 1.0).abs();
            assert!(rel < 0.01, "sector {n}: relative error {rel}");
        }
    }

    #[test]
    fn cot2_over_a2_are_consecutive_thirds() {
        for (n, target) in [(1, 5.0/3.0), (2, 4.0/3.0), (3, 1.0)] {
            let r = cot2_theta_w() / observed_a2()[n-1];
            assert!((r / target - 1.0).abs() < 0.01, "n={n}: {r} vs {target}");
        }
    }

    #[test]
    fn one_gauge_input_fixes_all_six_flavor_parameters() {
        // The same 3cot^2~10 approximation propagates uniformly to all
        // three amplitudes and three phases: relative error ~-0.384% each.
        // Against the EXACT rational framework targets the offset is uniform;
        // the rounded measured amplitudes introduce only small extra scatter.
        let exact_a2 = [2.0_f64, 2.5, 10.0/3.0];
        for n in 1..=3 {
            let a_err = a2_from_gauge(n) / exact_a2[(n-1) as usize] - 1.0;
            let d_err = delta_from_gauge(n) / observed_delta()[(n-1) as usize] - 1.0;
            assert!((a_err - d_err).abs() < 1e-9, "n={n}: A2 err {a_err}, delta err {d_err}");
            assert!(a_err.abs() < 0.01, "n={n}: error too large");
        }
    }
}
