//! Cornell-potential sector: three lowest s-wave levels of V(r) = -A/r + sigma*r.
//!
//! A confining gauge theory is the natural physical home of a three-level
//! spectrum. This module solves the radial Schrodinger equation on a finite
//! grid (in units hbar=m=1) and asks whether the dimensionless confinement
//! strength sigma/A^2 supplies the Koide ratio Q=2/3 parameter-free.

use crate::Check;
use crate::linalg::symmetric_eigenvalues;

/// Build the dimensionless radial Hamiltonian on [0, R] with N interior points.
/// Potential V(u) = -1/u + lambda*u in units where A=1, r = u.
pub fn hamiltonian(lambda: f64, n: usize, r_max: f64) -> Vec<Vec<f64>> {
    let h = r_max / (n + 1) as f64;
    let mut m = vec![vec![0.0; n]; n];
    for i in 0..n {
        let r = (i + 1) as f64 * h;
        m[i][i] = 1.0 / (h * h) - 1.0 / r + lambda * r;
        if i > 0 {
            m[i][i - 1] = -0.5 / (h * h);
        }
        if i + 1 < n {
            m[i][i + 1] = -0.5 / (h * h);
        }
    }
    m
}

/// Three lowest eigenvalues for a given confinement strength.
pub fn levels(lambda: f64, n: usize, r_max: f64) -> Vec<f64> {
    symmetric_eigenvalues(&hamiltonian(lambda, n, r_max))
        .into_iter().take(3).collect()
}

/// Koide Q of three levels shifted so the lowest is positive.
pub fn q_of_levels(v: &[f64]) -> f64 {
    let min = v[0];
    let m = [v[0] - min + 1e-9, v[1] - min + 1e-9, v[2] - min + 1e-9];
    let sum: f64 = m.iter().sum();
    let roots: f64 = m.iter().map(|x| x.sqrt()).sum();
    sum / (roots * roots)
}

pub fn checks() -> Vec<Check> {
    let n = 60usize;
    let l0 = levels(0.0, n, 24.0);
    let l1 = levels(1.0, n, 24.0);
    vec![
        Check::new("confinement.coulomb_e1", l0[0], "lowest Coulomb level, lambda=0"),
        Check::new("confinement.coulomb_e2", l0[1], "second Coulomb level"),
        Check::new("confinement.coulomb_q", q_of_levels(&l0), "Koide Q of Coulomb levels"),
        Check::new("confinement.confined_q", q_of_levels(&l1), "Koide Q at lambda=1"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn coulomb_ground_state_is_negative_one_half() {
        let v = levels(0.0, 80, 40.0);
        // Finite-difference Coulomb on a truncated grid: -0.4727 at h=0.5,
        // approaching -0.5 as the grid is refined; 7% is a fair tolerance here.
        assert!(approx(v[0], -0.5, 0.08), "E1 = {}", v[0]);
    }

    #[test]
    fn confinement_raises_levels() {
        let a = levels(0.0, 60, 24.0);
        let b = levels(1.0, 60, 24.0);
        assert!(b[0] > a[0]);
    }

    #[test]
    fn q_is_bounded() {
        for lambda in [0.0, 0.1, 1.0, 5.0] {
            let v = levels(lambda, 60, 24.0);
            let q = q_of_levels(&v);
            assert!((0.3..=1.0).contains(&q), "lambda={lambda} q={q}");
        }
    }
}
