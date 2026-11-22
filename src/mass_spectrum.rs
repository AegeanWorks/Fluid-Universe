//! Mass-spectrum sector: a Z3 rotor with a symmetry-breaking field.
//!
//! The unbroken rotor has exact E1 = E2 degeneracy, so it cannot produce three
//! distinct masses. Adding h cos(phi) breaks the residual symmetry and splits the
//! levels. This module asks whether the splitting can supply the parameter-free
//! Koide condition Q = 2/3, or whether Q=2/3 only occurs at a fitted value of h.

use crate::Check;
use crate::linalg::symmetric_eigenvalues;

/// Lowest `count` levels of H = n^2/(2I) + mu(1-cos 3phi) + h cos phi
/// in the Fourier basis |n>, |n| <= cutoff.
pub fn rotor_levels(i: f64, mu: f64, h: f64, cutoff: usize, count: usize) -> Vec<f64> {
    assert!(i > 0.0 && cutoff >= 4 && count >= 1);
    let size = 2 * cutoff + 1;
    let mut matrix = vec![vec![0.0; size]; size];
    for row in 0..size {
        let n = row as isize - cutoff as isize;
        matrix[row][row] = (n * n) as f64 / (2.0 * i) + mu;
        for (shift, strength) in [(3isize, -0.5 * mu), (-3isize, -0.5 * mu),
                                  (1isize, 0.5 * h), (-1isize, 0.5 * h)] {
            let target = n + shift;
            let bound = cutoff as isize;
            if (-bound..=bound).contains(&target) {
                let col = (target + cutoff as isize) as usize;
                matrix[row][col] += strength;
            }
        }
    }
    symmetric_eigenvalues(&matrix).into_iter().take(count).collect()
}

/// Koide ratio of the three spacings above the ground level (physical, non-negative).
pub fn koide_from_spacings(levels: &[f64; 3]) -> f64 {
    let m = [0.0f64, (levels[1] - levels[0]).max(0.0), (levels[2] - levels[0]).max(0.0)];
    let sum: f64 = m.iter().sum();
    let roots: f64 = m.iter().map(|v| v.sqrt()).sum();
    if roots > 0.0 { sum / (roots * roots) } else { 1.0 / 3.0 }
}

/// Koide ratio of three positive "masses" derived from level heights.
pub fn koide_from_levels(levels: &[f64; 3]) -> f64 {
    let m = [levels[0].max(1e-12), levels[1].max(1e-12), levels[2].max(1e-12)];
    let sum: f64 = m.iter().sum();
    let roots: f64 = m.iter().map(|v| v.sqrt()).sum();
    sum / (roots * roots)
}

pub fn checks() -> Vec<Check> {
    let degenerate = rotor_levels(1.0, 1.0, 0.0, 40, 3);
    let split = rotor_levels(1.0, 1.0, 0.5, 40, 3);
    vec![
        Check::new("mass_spectrum.rotor_e0", degenerate[0], "Z3 rotor ground level, cutoff 40"),
        Check::new("mass_spectrum.degeneracy_gap", (degenerate[1] - degenerate[2]).abs(),
                   "exact E1-E2 degeneracy at h=0"),
        Check::new("mass_spectrum.split_gap", split[2] - split[1], "E1-E2 splitting at h=0.5"),
        Check::new("mass_spectrum.koide_at_zero", koide_from_levels(&[
            degenerate[0], degenerate[1], degenerate[2]]), "Koide Q of unbroken levels"),
        Check::new("mass_spectrum.koide_spacings_at_zero", koide_from_spacings(&[
            degenerate[0], degenerate[1], degenerate[2]]), "Koide Q of level spacings"),
        Check::new("mass_spectrum.h_wall_tension", 8.0 / 3.0,
                   "wall-tension field h=(8/3)sqrt(mu)/sqrt(mu) candidate"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn unbroken_rotor_is_degenerate() {
        let levels = rotor_levels(1.0, 1.0, 0.0, 40, 3);
        assert!(approx(levels[0], 0.89118615974, 1e-8), "{:?}", levels);
        assert!((levels[1] - levels[2]).abs() < 1e-10);
    }

    #[test]
    fn breaking_splits_the_doublet() {
        let levels = rotor_levels(1.0, 1.0, 0.5, 40, 3);
        assert!(levels[2] - levels[1] > 0.1, "{:?}", levels);
    }

    #[test]
    fn wall_tension_field_is_a_near_miss_not_an_identity() {
        let levels = rotor_levels(1.0, 1.0, 8.0 / 3.0, 80, 3);
        let q = koide_from_levels(&[levels[0], levels[1], levels[2]]);
        assert!(q < 2.0 / 3.0 && q > 0.65, "Q(8/3)={q}");
        assert!((q - 2.0 / 3.0).abs() < 0.01, "near-miss must stay below 1%");
        // A fitted crossing exists, but it is not exactly the wall-tension constant.
        assert!((2.680588438_f64 - 8.0 / 3.0).abs() > 1e-3);
    }

    #[test]
    fn spacings_definition_is_bounded() {
        let levels = rotor_levels(1.0, 1.0, 8.0 / 3.0, 60, 3);
        let q = koide_from_spacings(&[levels[0], levels[1], levels[2]]);
        assert!((1.0 / 3.0..=1.0).contains(&q));
    }

    #[test]
    fn koide_ratio_is_bounded() {
        for h in [0.0, 0.1, 0.5, 1.0, 2.0] {
            let levels = rotor_levels(1.0, 1.0, h, 40, 3);
            let q = koide_from_levels(&[levels[0], levels[1], levels[2]]);
            assert!((1.0 / 3.0..=1.0).contains(&q), "h={h} Q={q}");
        }
    }
}
