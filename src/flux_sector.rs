//! Flux-threaded Z3 rotor: H = (n-alpha)^2/(2I) + mu(1-cos 3phi).
//!
//! A flux breaks time reversal and can split the exact E1=E2 doublet without
//! adding an explicit cos(phi) field. This module asks whether a distinguished
//! flux value supplies the Koide ratio without fitting to lepton masses.

use crate::Check;
use crate::linalg::symmetric_eigenvalues;

pub fn flux_levels(alpha: f64, mu: f64, cutoff: usize) -> Vec<f64> {
    let size = 2 * cutoff + 1;
    let mut matrix = vec![vec![0.0; size]; size];
    for row in 0..size {
        let n = row as isize - cutoff as isize;
        let p = n as f64 - alpha;
        matrix[row][row] = p * p / 2.0 + mu;
        for shift in [3isize, -3] {
            let target = n + shift;
            let bound = cutoff as isize;
            if (-bound..=bound).contains(&target) {
                let col = (target + bound) as usize;
                matrix[row][col] -= 0.5 * mu;
            }
        }
    }
    symmetric_eigenvalues(&matrix).into_iter().take(3).collect()
}

pub fn q_levels(levels: &[f64; 3]) -> f64 {
    let m = [levels[0].max(1e-12), levels[1].max(1e-12), levels[2].max(1e-12)];
    let sum: f64 = m.iter().sum();
    let roots: f64 = m.iter().map(|v| v.sqrt()).sum();
    sum / (roots * roots)
}

pub fn checks() -> Vec<Check> {
    let zero = flux_levels(0.0, 1.0, 40);
    let half = flux_levels(0.5, 1.0, 40);
    vec![
        Check::new("flux.zero_ground", zero[0], "Z3 rotor ground level at zero flux"),
        Check::new("flux.zero_split", (zero[1] - zero[2]).abs(), "degeneracy at zero flux"),
        Check::new("flux.half_split", half[2] - half[1], "doublet splitting at alpha=1/2"),
        Check::new("flux.half_q", q_levels(&[half[0], half[1], half[2]]),
                   "Koide Q of the flux-split levels"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn zero_flux_recovers_known_rotor() {
        let v = flux_levels(0.0, 1.0, 40);
        assert!(approx(v[0], 0.89118615974, 1e-8), "{:?}", v);
        assert!((v[1] - v[2]).abs() < 1e-10);
    }

    #[test]
    fn flux_splits_the_doublet() {
        let v = flux_levels(0.5, 1.0, 40);
        assert!(v[2] - v[1] > 0.05, "{:?}", v);
    }

    #[test]
    fn flux_q_stays_bounded() {
        for alpha in [0.0, 0.1, 0.25, 0.5, 0.75, 1.0] {
            let v = flux_levels(alpha, 1.0, 40);
            let q = q_levels(&[v[0], v[1], v[2]]);
            assert!((1.0 / 3.0 - 1e-9..=1.0 + 1e-9).contains(&q), "alpha={alpha} q={q}");
        }
    }
}
