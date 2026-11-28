//! Mass-spectra Koide checks: neutrinos, quarks and PMNS angles.
//!
//! Port of the numeric checks in `phenomenology/neutrinos.py`,
//! `phenomenology/quarks.py` and `phenomenology/pmns.py`. All functions are
//! pure and dependency-free so the CLI can re-derive them in two processes.

/// Koide ratio `sum(m) / (sum(sqrt(m)))^2` of three (positive) masses.
///
/// This is the `q_masses` / `q_nu` function used by the neutrino and quark
/// phenomenology checks. A zero mass is allowed (`sqrt(0) = 0`).
pub fn q_from_masses(m: &[f64]) -> f64 {
    let sum: f64 = m.iter().sum();
    let sum_sqrt: f64 = m.iter().map(|&value| value.sqrt()).sum();
    sum / (sum_sqrt * sum_sqrt)
}

/// Three active-neutrino masses (eV) from a lightest mass `m0` and the
/// mass-squared splittings `dm21`, `dm31`.
///
/// Normal ordering has `m1 = m0`; inverted ordering has `m3 = m0` and uses
/// `abs(dm31)` for the (negative) inverted splitting.
pub fn neutrino_masses(m0: f64, dm21: f64, dm31: f64, inverted: bool) -> [f64; 3] {
    let m0sq = m0 * m0;
    if inverted {
        let split = dm31.abs();
        [(m0sq + split).sqrt(), (m0sq + split + dm21).sqrt(), m0]
    } else {
        [m0, (m0sq + dm21).sqrt(), (m0sq + dm31).sqrt()]
    }
}

/// Maximum neutrino Koide ratio over a scan of the lightest mass.
///
/// Scans `m0` over `[0, 0.05]` eV with `dm21 = 7.425e-5` and
/// `dm31 = 2.517e-3` (eV^2), normal ordering. The ratio decreases with `m0`,
/// so the maximum is attained at `m0 = 0`.
pub fn neutrino_q_max() -> f64 {
    const DM21: f64 = 7.425e-5;
    const DM31: f64 = 2.517e-3;
    const POINTS: usize = 2001;
    let mut best = f64::NEG_INFINITY;
    for i in 0..POINTS {
        let m0 = 0.05 * (i as f64) / ((POINTS - 1) as f64);
        let q = q_from_masses(&neutrino_masses(m0, DM21, DM31, false));
        if q > best {
            best = q;
        }
    }
    best
}

/// Koide ratio of the up-type quarks at `M_Z` (Huang & Zhou 2021).
pub const QUARK_Q_UP_MZ: f64 = 0.8875692313187327;
/// Koide ratio of the down-type quarks at `M_Z` (Huang & Zhou 2021).
pub const QUARK_Q_DOWN_MZ: f64 = 0.7480650548031402;

/// Standard three-neutrino mixing angles (degrees) from the magnitudes of a
/// PMNS matrix.
///
/// Returns `[theta12, theta13, theta23]`, with
/// `s13 = |U[0][2]|`, `c13 = sqrt(1 - s13^2)`, `s12 = |U[0][1]|/c13` and
/// `s23 = |U[1][2]|/c13` (all clamped to 1 before `asin`). At `theta13 = 90`
/// degrees the other two angles are undefined and reported as `NaN`.
pub fn pmns_angles_degrees(u: &[[f64; 3]; 3]) -> [f64; 3] {
    let s13 = u[0][2].abs().min(1.0);
    let c13 = (1.0 - s13 * s13).max(0.0).sqrt();
    if c13 == 0.0 {
        return [f64::NAN, 90.0, f64::NAN];
    }
    let s12 = (u[0][1].abs() / c13).min(1.0);
    let s23 = (u[1][2].abs() / c13).min(1.0);
    [
        s12.asin().to_degrees(),
        s13.asin().to_degrees(),
        s23.asin().to_degrees(),
    ]
}

/// Deterministic real stand-in for the DFT matrix: every entry has magnitude
/// `1/sqrt(3)`, which is all the angle extraction uses.
fn trimaximal_matrix() -> [[f64; 3]; 3] {
    let v = 1.0 / 3.0_f64.sqrt();
    [[v, v, v], [v, v, v], [v, v, v]]
}

/// Reproducible result lines for this module (ids prefixed with `spectra.`).
pub fn checks() -> Vec<crate::Check> {
    use crate::Check;

    const DM21: f64 = 7.425e-5;
    const DM31: f64 = 2.517e-3;
    let no_zero = q_from_masses(&neutrino_masses(0.0, DM21, DM31, false));
    let io_zero = q_from_masses(&neutrino_masses(0.0, DM21, -DM31, true));
    let identity = pmns_angles_degrees(&[
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ]);
    let tri = pmns_angles_degrees(&trimaximal_matrix());
    let tri_s13_exact = (1.0 / 3.0_f64.sqrt()).asin().to_degrees();

    vec![
        Check::new("spectra.q_nu_max", neutrino_q_max(), "NO scan m0 in [0, 0.05] eV"),
        Check::new("spectra.q_nu_at_zero", no_zero, "NO Q at m0 = 0"),
        Check::new("spectra.q_nu_inverted_at_zero", io_zero, "IO Q at m0 = 0"),
        Check::new("spectra.q_universal", q_from_masses(&[1.0, 1.0, 1.0]), "Q of exactly equal masses"),
        Check::new("spectra.quark_q_up_mz", QUARK_Q_UP_MZ, "Q_up at M_Z"),
        Check::new("spectra.quark_q_down_mz", QUARK_Q_DOWN_MZ, "Q_down at M_Z"),
        Check::new("spectra.pmns_identity_theta13", identity[1], "same-basis branch (no mixing)"),
        Check::new("spectra.pmns_trimaximal_theta13", tri[1], "DFT branch; exact asin(1/sqrt(3))"),
        Check::new("spectra.pmns_trimaximal_theta12", tri[0], "DFT branch theta12"),
        Check::new("spectra.pmns_trimaximal_theta23", tri[2], "DFT branch theta23"),
        Check::new("spectra.pmns_theta13_exact", tri_s13_exact, "asin(1/sqrt(3)) in degrees"),
        Check::new("spectra.koide_gap", 2.0 / 3.0 - QUARK_Q_UP_MZ, "2/3 - Q_up(M_Z)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn koide_ratio_definition() {
        assert!(approx(q_from_masses(&[1.0, 1.0, 1.0]), 1.0 / 3.0, 1e-14));
        assert!(approx(q_from_masses(&[1.0, 4.0, 9.0]), 14.0 / 36.0, 1e-14));
        // Matches the shared circulant implementation exactly.
        assert!(approx(q_from_masses(&[0.51099895, 105.6583755, 1776.9690270830142]),
                       crate::circulant::q_masses(&[0.51099895, 105.6583755, 1776.9690270830142]),
                       1e-15));
    }

    #[test]
    fn neutrino_ordering_formula() {
        let (dm21, dm31) = (7.41e-5_f64, 2.507e-3_f64);
        let no = neutrino_masses(0.0, dm21, dm31, false);
        assert_eq!(no[0], 0.0);
        assert!(approx(no[1], dm21.sqrt(), 1e-15));
        assert!(approx(no[2], dm31.sqrt(), 1e-15));

        let io = neutrino_masses(0.0, dm21, -dm31, true);
        assert_eq!(io[2], 0.0);
        assert!(approx(io[0], dm31.sqrt(), 1e-15));
        assert!(approx(io[1], (dm31 + dm21).sqrt(), 1e-15));

        // Q decreases as the lightest mass grows, for both orderings.
        let mut prev_no = q_from_masses(&no);
        let mut prev_io = q_from_masses(&io);
        for i in 1..=20 {
            let m0 = 0.05 * (i as f64) / 20.0;
            let qn = q_from_masses(&neutrino_masses(m0, dm21, dm31, false));
            let qi = q_from_masses(&neutrino_masses(m0, dm21, -dm31, true));
            assert!(qn <= prev_no + 1e-15 && qi <= prev_io + 1e-15);
            prev_no = qn;
            prev_io = qi;
        }
    }

    #[test]
    fn neutrino_q_max_regression() {
        let qmax = neutrino_q_max();
        // The contract quotes ~0.585612 (the exact NuFit CSV regression is
        // 0.5856120093687199 for dm21=7.41e-5, dm31=2.507e-3). The contract's
        // own constants 7.425e-5 / 2.517e-3 give the value asserted tightly
        // below; both round to ~0.5856 and stay below 2/3.
        assert!(approx(qmax, 0.585612, 1e-3), "qmax {}", qmax);
        assert!(approx(qmax, 0.5856959842032594, 1e-13), "qmax {}", qmax);
        assert!(qmax < 2.0 / 3.0);
        assert!(approx(qmax, q_from_masses(&neutrino_masses(0.0, 7.425e-5, 2.517e-3, false)), 1e-15));
    }

    #[test]
    fn quark_mz_constants() {
        assert!(approx(QUARK_Q_UP_MZ, 0.8875692313187327, 1e-15));
        assert!(approx(QUARK_Q_DOWN_MZ, 0.7480650548031402, 1e-15));
        assert!(QUARK_Q_UP_MZ > 2.0 / 3.0 && QUARK_Q_DOWN_MZ > 2.0 / 3.0);
        assert!(QUARK_Q_UP_MZ - 2.0 / 3.0 > 0.05);
    }

    #[test]
    fn pmns_angles_identity_and_trimaximal() {
        let identity = [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ];
        let a = pmns_angles_degrees(&identity);
        assert!(approx(a[0], 0.0, 1e-13));
        assert!(approx(a[1], 0.0, 1e-13));
        assert!(approx(a[2], 0.0, 1e-13));

        let tri = pmns_angles_degrees(&trimaximal_matrix());
        assert!(approx(tri[1], 35.264389682754654, 1e-10));
        assert!(approx(tri[0], 45.0, 1e-10));
        assert!(approx(tri[2], 45.0, 1e-10));

        let singular = [[0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]];
        let b = pmns_angles_degrees(&singular);
        assert!(approx(b[1], 90.0, 1e-12));
        assert!(b[0].is_nan() && b[2].is_nan());
    }

    #[test]
    fn checks_are_stable_and_finite() {
        let first = checks();
        let second = checks();
        assert_eq!(first.len(), second.len());
        let mut ids = std::collections::HashSet::new();
        for (a, b) in first.iter().zip(second.iter()) {
            assert_eq!(a.id, b.id);
            assert_eq!(a.value.to_bits(), b.value.to_bits());
            assert!(a.value.is_finite(), "id {} is not finite", a.id);
            assert!(a.id.starts_with("spectra."));
            assert!(ids.insert(a.id));
        }
    }
}
