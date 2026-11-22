//! Linearised director dispersion and the emergent-metric question (claims
//! 11-12), ported from `theory/lorentz.py` with no external crates.
//!
//! For a background director `n0 = z` the two transverse modes have the 2x2
//! elastic matrix `M(k)` built from the Frank constants `K1, K2, K3`. Its
//! eigenvalues are the squared mode speeds (density `rho = 1`). Residual
//! anisotropy is the peak-to-peak speed-squared split over sampled directions,
//! normalised by the mean:
//!
//! `xi = (c2_max - c2_min) / mean(c2)`.
//!
//! The Python source draws directions from numpy's PCG64; here the sampling is
//! driven by a deterministic linear congruential generator (seed 20261025), so
//! the value is reproducible in a dependency-free Rust build.

use crate::linalg::symmetric_eigenvalues;
use crate::Check;

/// Default random seed used by the Python reference scan.
pub const SEED: u64 = 20261025;

/// Ascending squared mode speeds of the 2x2 elastic matrix
/// `[[K1 kx^2 + K2 ky^2 + K3 kz^2, (K1-K2) kx ky],
///   [(K1-K2) kx ky, K1 ky^2 + K2 kx^2 + K3 kz^2]]`.
///
/// The input vector need not be a unit vector; the caller normalises it. The
/// result is `[c2_min, c2_max]` for unit density.
pub fn elastic_eigenvalues(
    kx: f64,
    ky: f64,
    kz: f64,
    k1: f64,
    k2: f64,
    k3: f64,
) -> [f64; 2] {
    let matrix = vec![
        vec![
            k1 * kx * kx + k2 * ky * ky + k3 * kz * kz,
            (k1 - k2) * kx * ky,
        ],
        vec![
            (k1 - k2) * kx * ky,
            k1 * ky * ky + k2 * kx * kx + k3 * kz * kz,
        ],
    ];
    let values = symmetric_eigenvalues(&matrix);
    [values[0], values[1]]
}

/// Minimal deterministic random source: a 64-bit LCG with a Box-Muller
/// transform, mirroring the `rng.normal(size=(n, 3))` draw in the Python
/// reference without pulling in numpy or any other crate.
struct Lcg {
    state: u64,
    spare: Option<f64>,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed, spare: None }
    }

    /// Uniform deviate in `[0, 1)`.
    fn uniform(&mut self) -> f64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.state >> 11) as f64) / ((1u64 << 53) as f64)
    }

    /// Standard normal deviate (the spare from the Box-Muller pair is cached).
    fn normal(&mut self) -> f64 {
        if let Some(spare) = self.spare.take() {
            return spare;
        }
        let radius = (-2.0 * self.uniform().max(f64::MIN_POSITIVE).ln()).sqrt();
        let angle = 2.0 * std::f64::consts::PI * self.uniform();
        self.spare = Some(radius * angle.sin());
        radius * angle.cos()
    }
}

/// Fractional peak-to-peak anisotropy of the squared mode speeds over
/// `n_samples` deterministic unit directions. It is exactly zero when
/// `K1 == K2 == K3` (the isotropic Frank point).
pub fn residual_anisotropy(k1: f64, k2: f64, k3: f64, n_samples: usize) -> f64 {
    assert!(k1 > 0.0 && k2 > 0.0 && k3 > 0.0, "K1, K2, K3 must be positive");
    assert!(n_samples > 0, "n_samples must be positive");
    if k1 == k2 && k2 == k3 {
        return 0.0;
    }

    let mut rng = Lcg::new(SEED);
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut sum = 0.0;
    let mut count = 0usize;
    for _ in 0..n_samples {
        let (mut x, mut y, mut z) = (rng.normal(), rng.normal(), rng.normal());
        let norm = (x * x + y * y + z * z).sqrt();
        x /= norm;
        y /= norm;
        z /= norm;
        let [low, high] = elastic_eigenvalues(x, y, z, k1, k2, k3);
        for c2 in [low, high] {
            min = min.min(c2);
            max = max.max(c2);
            sum += c2;
            count += 1;
        }
    }
    let mean = sum / count as f64;
    (max - min) / mean
}

/// Reproducible result lines for this module, with stable ids (the order is
/// part of the audit fingerprint). Recorded point: `K = (1, 2, 3)`, `rho = 1`,
/// 4000 directions from the fixed seed.
pub fn checks() -> Vec<Check> {
    let (k1, k2, k3) = (1.0, 2.0, 3.0);
    let x_axis = elastic_eigenvalues(1.0, 0.0, 0.0, k1, k2, k3);
    let z_axis = elastic_eigenvalues(0.0, 0.0, 1.0, k1, k2, k3);
    vec![
        Check::new(
            "lorentz.c2_along_x_low",
            x_axis[0],
            "lower elastic eigenvalue along x for K=(1,2,3)",
        ),
        Check::new(
            "lorentz.c2_along_x_high",
            x_axis[1],
            "upper elastic eigenvalue along x for K=(1,2,3)",
        ),
        Check::new(
            "lorentz.c2_along_z",
            z_axis[1],
            "degenerate elastic eigenvalue along the background z axis",
        ),
        Check::new(
            "lorentz.residual_anisotropy_equal_k",
            residual_anisotropy(2.0, 2.0, 2.0, 4000),
            "xi vanishes at the isotropic Frank point K1=K2=K3",
        ),
        Check::new(
            "lorentz.residual_anisotropy_k123",
            residual_anisotropy(k1, k2, k3, 4000),
            "xi for K=(1,2,3) over 4000 deterministic unit directions",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn elastic_eigenvalues_match_axis_reference() {
        let x = elastic_eigenvalues(1.0, 0.0, 0.0, 1.0, 2.0, 3.0);
        assert!(approx(x[0], 1.0, 1e-14));
        assert!(approx(x[1], 2.0, 1e-14));
        let y = elastic_eigenvalues(0.0, 1.0, 0.0, 1.0, 2.0, 3.0);
        assert!(approx(y[0], 1.0, 1e-14));
        assert!(approx(y[1], 2.0, 1e-14));
        let z = elastic_eigenvalues(0.0, 0.0, 1.0, 1.0, 2.0, 3.0);
        assert!(approx(z[0], 3.0, 1e-14));
        assert!(approx(z[1], 3.0, 1e-14));
    }

    #[test]
    fn elastic_eigenvalues_are_sorted_and_sign_invariant() {
        let inv = 1.0 / 2.0_f64.sqrt();
        let d = elastic_eigenvalues(inv, inv, 0.0, 1.0, 2.0, 3.0);
        assert!(approx(d[0], 1.0, 1e-12));
        assert!(approx(d[1], 2.0, 1e-12));
        assert!(d[0] <= d[1]);
        // The off-diagonal is quadratic in k, so flipping either component
        // leaves the spectrum unchanged.
        let flipped = elastic_eigenvalues(-inv, inv, 0.0, 1.0, 2.0, 3.0);
        assert_eq!(d[0].to_bits(), flipped[0].to_bits());
        assert_eq!(d[1].to_bits(), flipped[1].to_bits());
        let iso = elastic_eigenvalues(inv, inv, 0.0, 2.0, 2.0, 2.0);
        assert!(approx(iso[0], 2.0, 1e-14));
        assert!(approx(iso[1], 2.0, 1e-14));
    }

    #[test]
    fn residual_anisotropy_vanishes_for_equal_frank_constants() {
        assert_eq!(residual_anisotropy(2.0, 2.0, 2.0, 4000), 0.0);
        assert_eq!(residual_anisotropy(0.5, 0.5, 0.5, 7), 0.0);
    }

    #[test]
    fn residual_anisotropy_k123_is_deterministic_and_order_one() {
        let xi = residual_anisotropy(1.0, 2.0, 3.0, 4000);
        // The LCG-driven scan lands near the numpy reference of 1.00348; the
        // exact last digits depend on the random source (see module docs).
        assert!(approx(xi, 1.0009693746847284, 1e-6));
        assert!((xi - 1.00348).abs() < 0.01);
        assert!(xi > 0.0);
        assert_eq!(xi.to_bits(), residual_anisotropy(1.0, 2.0, 3.0, 4000).to_bits());
    }

    #[test]
    fn checks_are_stable_unique_and_match_references() {
        let first = checks();
        let second = checks();
        assert_eq!(first.len(), 5);
        let mut ids = std::collections::HashSet::new();
        for (a, b) in first.iter().zip(second.iter()) {
            assert_eq!(a.id, b.id);
            assert_eq!(a.value.to_bits(), b.value.to_bits());
            assert_eq!(a.detail, b.detail);
            assert!(a.id.starts_with("lorentz."));
            assert!(ids.insert(a.id));
        }
        let by_id = |id: &str| first.iter().find(|c| c.id == id).unwrap().value;
        assert!(approx(by_id("lorentz.c2_along_x_low"), 1.0, 1e-14));
        assert!(approx(by_id("lorentz.c2_along_x_high"), 2.0, 1e-14));
        assert!(approx(by_id("lorentz.c2_along_z"), 3.0, 1e-14));
        assert_eq!(by_id("lorentz.residual_anisotropy_equal_k"), 0.0);
        assert!(approx(by_id("lorentz.residual_anisotropy_k123"), 1.0009693746847284, 1e-6));
    }
}
