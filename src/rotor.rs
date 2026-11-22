//! Quantum rotor H = L^2/(2I) + mu (1 - cos 3 phi) and its spectra.
//!
//! In the Fourier basis |n> = exp(i n phi)/sqrt(2 pi) the Hamiltonian couples
//! only n <-> n +/- 3 inside each congruence class n = sector + 3j:
//!
//!  - `<n|H|n>   = n^2/(2I) + mu`,
//!  - `<n|H|n+3> = <n|H|n-3> = -mu/2`.
//!
//! Since H(phi) = H(-phi), complex conjugation maps n -> -n and therefore
//! maps sector 1 <-> sector 2: those two spectra are exactly degenerate.

use crate::Check;
use crate::linalg::{simpson, symmetric_eigenvalues};

/// Defensive cyclic-Jacobi pre-pass, finished off by the shared kernel.
///
/// `linalg::symmetric_eigenvalues` now performs cyclic sweeps itself, so this
/// extra pass is redundant; it is kept because the 107-row sector (cutoff 160)
/// is the tightest convergence case and a second independent sweep makes the
/// E1/E2 degeneracy robust at machine precision. Once off-diagonals are below
/// 1e-14 the shared kernel returns the values in ascending order.
fn converged_eigenvalues(matrix: &[Vec<f64>]) -> Vec<f64> {
    let n = matrix.len();
    let mut a = matrix.to_vec();
    for _ in 0..60 {
        let mut largest = 0.0f64;
        for p in 0..n {
            for q in (p + 1)..n {
                let apq = a[p][q];
                largest = largest.max(apq.abs());
                if apq == 0.0 {
                    continue;
                }
                let theta = (a[q][q] - a[p][p]) / (2.0 * apq);
                let t = if theta >= 0.0 {
                    1.0 / (theta + (1.0 + theta * theta).sqrt())
                } else {
                    -1.0 / (-theta + (1.0 + theta * theta).sqrt())
                };
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = t * c;
                let app = a[p][p];
                let aqq = a[q][q];
                a[p][p] = app - t * apq;
                a[q][q] = aqq + t * apq;
                a[p][q] = 0.0;
                a[q][p] = 0.0;
                for k in 0..n {
                    if k == p || k == q {
                        continue;
                    }
                    let akp = a[k][p];
                    let akq = a[k][q];
                    a[k][p] = c * akp - s * akq;
                    a[p][k] = a[k][p];
                    a[k][q] = s * akp + c * akq;
                    a[q][k] = a[k][q];
                }
            }
        }
        if largest < 1e-14 {
            break;
        }
    }
    symmetric_eigenvalues(&a)
}

/// Dense Hamiltonian in the truncated sector n = sector + 3j, |n| <= n_cutoff.
///
/// Rows/columns are ordered by increasing n. The three sectors are the
/// residue classes n mod 3, so every coupling stays inside the returned block.
pub fn sector_matrix(i: f64, mu: f64, sector: i32, n_cutoff: i32) -> Vec<Vec<f64>> {
    assert!((0..=2).contains(&sector), "sector must be 0, 1 or 2");
    assert!(i > 0.0, "moment of inertia must be positive");
    assert!(n_cutoff >= 0, "n_cutoff must be non-negative");

    // rem_euclid reproduces numpy's non-negative `n % 3` for negative n.
    let values: Vec<i32> = (-n_cutoff..=n_cutoff)
        .filter(|&n| n.rem_euclid(3) == sector.rem_euclid(3))
        .collect();
    let size = values.len();
    let mut h = vec![vec![0.0; size]; size];

    for (a, &n) in values.iter().enumerate() {
        let nf = n as f64;
        h[a][a] = nf * nf / (2.0 * i) + mu;
        for shift in [3_i32, -3] {
            if let Ok(b) = values.binary_search(&(n + shift)) {
                h[a][b] -= 0.5 * mu;
            }
        }
    }
    h
}

/// The `count` lowest eigenvalues of one Fourier sector, ascending.
pub fn lowest_levels(i: f64, mu: f64, sector: i32, n_cutoff: i32, count: usize) -> Vec<f64> {
    let h = sector_matrix(i, mu, sector, n_cutoff);
    let mut levels = converged_eigenvalues(&h);
    levels.truncate(count);
    levels
}

/// Free-rotor (mu = 0) levels n^2/(2I) in a sector, sorted and truncated.
///
/// The range -1000..=1000 mirrors the Python reference and is ample for the
/// handful of lowest levels used here.
pub fn free_levels(sector: i32, count: usize, i: f64) -> Vec<f64> {
    assert!((0..=2).contains(&sector), "sector must be 0, 1 or 2");
    assert!(i > 0.0, "moment of inertia must be positive");

    let mut levels: Vec<f64> = (-1000_i32..=1000_i32)
        .filter(|&n| n.rem_euclid(3) == sector.rem_euclid(3))
        .map(|n| {
            let nf = n as f64;
            nf * nf / (2.0 * i)
        })
        .collect();
    levels.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    levels.truncate(count);
    levels
}

/// Analytic wall tension T = (8/3) sqrt(mu) of the 1 - cos(3 phi) wall.
pub fn domain_wall_tension(mu: f64) -> f64 {
    assert!(mu >= 0.0, "mu must be non-negative");
    (8.0 / 3.0) * mu.sqrt()
}

/// Numerical tension: integral_0^{2pi/3} sqrt(2 mu (1 - cos 3 phi)) dphi.
///
/// The integrand equals 2 sqrt(mu) sin(3 phi/2) on this interval, so it is
/// smooth and composite Simpson converges to machine precision with 1024
/// panels.
pub fn domain_wall_tension_quadrature(mu: f64) -> f64 {
    assert!(mu >= 0.0, "mu must be non-negative");
    simpson(
        |phi| (2.0 * mu * (1.0 - (3.0 * phi).cos())).max(0.0).sqrt(),
        0.0,
        2.0 * std::f64::consts::PI / 3.0,
        1024,
    )
}

/// Reproducible result lines for this module (ids are part of the audit fingerprint).
pub fn checks() -> Vec<crate::Check> {
    let e0 = lowest_levels(1.0, 1.0, 0, 160, 1)[0];
    let e1 = lowest_levels(1.0, 1.0, 1, 160, 1)[0];
    let e2 = lowest_levels(1.0, 1.0, 2, 160, 1)[0];
    vec![
        Check::new("rotor.sector0_ground", e0, "I=1, mu=1, n_cutoff=160"),
        Check::new("rotor.sector1_ground", e1, "I=1, mu=1, n_cutoff=160"),
        Check::new("rotor.sector2_ground", e2, "I=1, mu=1, n_cutoff=160"),
        Check::new("rotor.degeneracy_gap", (e1 - e2).abs(), "|E1 - E2|, exact by parity"),
        Check::new("rotor.free_level_0", free_levels(0, 1, 1.0)[0], "mu=0, I=1, n=0"),
        Check::new("rotor.domain_wall_tension", domain_wall_tension(1.0), "(8/3) sqrt(mu), mu=1"),
        Check::new("rotor.domain_wall_tension_quadrature", domain_wall_tension_quadrature(1.0), "Simpson integral, mu=1"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn sector_matrix_diagonal_and_couplings() {
        let h = sector_matrix(1.0, 2.0, 1, 4);
        assert_eq!(h.len(), 3); // n = -2, 1, 4
        assert!(approx(h[0][0], 4.0, 1e-12)); // (-2)^2/2 + 2
        assert!(approx(h[1][1], 2.5, 1e-12)); // 1^2/2 + 2
        assert!(approx(h[2][2], 10.0, 1e-12)); // 4^2/2 + 2
        assert!(approx(h[0][1], -1.0, 1e-12)); // -mu/2
        assert!(approx(h[1][2], -1.0, 1e-12));
        assert_eq!(h[0][2], 0.0); // no direct n <-> n+6 coupling
        for a in 0..3 {
            for b in 0..3 {
                assert_eq!(h[a][b], h[b][a], "matrix must be symmetric");
            }
        }
    }

    #[test]
    fn lowest_levels_reproduce_reference() {
        let e0 = lowest_levels(1.0, 1.0, 0, 160, 3);
        assert!(approx(e0[0], 0.8911861597, 1e-9), "E0 = {}", e0[0]);
        assert!(approx(e0[1], 5.4814973267, 1e-9), "E0[1] = {}", e0[1]);
        let e1 = lowest_levels(1.0, 1.0, 1, 160, 1)[0];
        assert!(approx(e1, 1.3170345140, 1e-9), "E1 = {}", e1);
    }

    #[test]
    fn sectors_one_and_two_are_degenerate() {
        let e1 = lowest_levels(1.0, 1.0, 1, 160, 3);
        let e2 = lowest_levels(1.0, 1.0, 2, 160, 3);
        for (a, b) in e1.iter().zip(&e2) {
            assert!(approx(*a, *b, 1e-9), "sector 1 {} vs sector 2 {}", a, b);
        }
    }

    #[test]
    fn free_levels_are_exact() {
        assert_eq!(free_levels(0, 3, 1.0), vec![0.0, 4.5, 4.5]);
        assert_eq!(free_levels(1, 3, 1.0), vec![0.5, 2.0, 8.0]);
        assert!(approx(free_levels(2, 1, 2.0)[0], 0.25, 1e-12));
    }

    #[test]
    fn wall_tension_closed_form_matches_quadrature() {
        for mu in [0.25, 1.0, 4.0] {
            assert!(
                approx(domain_wall_tension(mu), domain_wall_tension_quadrature(mu), 1e-9),
                "mu = {}",
                mu
            );
        }
        assert!(approx(domain_wall_tension(4.0), 16.0 / 3.0, 1e-12));
    }

    #[test]
    fn checks_have_stable_unique_ids() {
        let rows = checks();
        assert!(rows.len() >= 4);
        let mut ids: Vec<&str> = rows.iter().map(|c| c.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), rows.len());
        assert!(rows.iter().all(|c| c.value.is_finite()));
    }
}
