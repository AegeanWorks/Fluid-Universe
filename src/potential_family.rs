//! Generalized Z3 potential family V_p(phi) = (1 - cos(3phi))^p.
//!
//! p=1 is the standard Z3 wall with the exact tension T=(8/3)sqrt(mu). This
//! module asks whether another member of the family supplies the Koide ratio
//! from a parameter-free choice of p, rather than from a fitted coupling.

use crate::Check;
use crate::linalg::symmetric_eigenvalues;

/// Fourier coefficients of (1-cos 3phi)^p on the periodic rotor grid.
/// Only the diagonal and the first 3p harmonics are needed for the Hamiltonian.
fn fourier(p: f64, max_mode: usize) -> Vec<f64> {
    let samples = 4096usize;
    let mut coeff = vec![0.0f64; max_mode + 1];
    for index in 0..samples {
        let phi = 2.0 * std::f64::consts::PI * index as f64 / samples as f64;
        let value = (1.0 - (3.0 * phi).cos()).powf(p);
        for mode in 0..=max_mode {
            coeff[mode] += value * (3.0 * mode as f64 * phi).cos();
        }
    }
    for value in coeff.iter_mut() {
        *value *= 2.0 / samples as f64;
    }
    // The m=0 coefficient must not be doubled by the cosine transform.
    coeff[0] *= 0.5;
    coeff
}

/// Lowest three rotor levels for V_p, in units mu=1, I=1.
pub fn levels(p: f64, mu: f64, cutoff: usize) -> Vec<f64> {
    let modes = fourier(p, 3);
    let size = 2 * cutoff + 1;
    let mut matrix = vec![vec![0.0; size]; size];
    for row in 0..size {
        let n = row as isize - cutoff as isize;
        matrix[row][row] = (n * n) as f64 / 2.0 + mu * modes[0];
        for shift in 1..=3 {
            let strength = mu * modes[shift] / 2.0;
            let bound = cutoff as isize;
            for target in [n + 3 * shift as isize, n - 3 * shift as isize] {
                if (-bound..=bound).contains(&target) {
                    let col = (target + bound) as usize;
                    matrix[row][col] += strength;
                }
            }
        }
    }
    symmetric_eigenvalues(&matrix).into_iter().take(3).collect()
}

/// Level-based Koide Q with the project's documented clamp convention.
pub fn q_levels(levels: &[f64; 3]) -> f64 {
    let m = [levels[0].max(1e-12), levels[1].max(1e-12), levels[2].max(1e-12)];
    let sum: f64 = m.iter().sum();
    let roots: f64 = m.iter().map(|v| v.sqrt()).sum();
    sum / (roots * roots)
}

pub fn checks() -> Vec<Check> {
    let p1 = levels(1.0, 1.0, 40);
    vec![
        Check::new("potential_family.p1_e0", p1[0], "p=1 rotor ground level"),
        Check::new("potential_family.p1_q", q_levels(&[p1[0], p1[1], p1[2]]),
                   "Koide Q of the standard Z3 rotor"),
        Check::new("potential_family.q_at_half",
                   q_levels(&{ let v = levels(0.5, 1.0, 40); [v[0], v[1], v[2]] }),
                   "Koide Q at the half-power potential"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn standard_potential_matches_known_rotor() {
        let v = levels(1.0, 1.0, 40);
        assert!(approx(v[0], 0.89118615974, 1e-8), "{:?}", v);
        assert!((v[1] - v[2]).abs() < 1e-10);
    }

    #[test]
    fn half_power_potential_is_computed() {
        let v = levels(0.5, 1.0, 40);
        assert!(v.len() == 3 && v.iter().all(|x| x.is_finite()));
        assert!(v[0] < v[1] && v[1] <= v[2]);
    }

    #[test]
    fn q_is_bounded_over_the_family() {
        for p in [0.25, 0.5, 0.75, 1.0, 1.5, 2.0] {
            let v = levels(p, 1.0, 40);
            let q = q_levels(&[v[0], v[1], v[2]]);
            assert!((1.0 / 3.0 - 1e-9..=1.0 + 1e-9).contains(&q), "p={p} Q={q}");
        }
    }
}
