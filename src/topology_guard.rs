//! Conservative homotopy certificate for periodic sampled unit-vector fields.
//!
//! At each spatial cell, the initial corner is used as a separating vector.
//! If it has positive projection on all eight corners at both endpoints,
//! every space-time multilinear interpolation has positive projection and is
//! nonzero. Normalizing that continuous interpolation gives a homotopy between
//! the sampled maps. A nonpositive margin is inconclusive, not a failure of
//! topology; it does not certify any unsampled production trajectory.

use crate::{Check, simulation::{Field3, analytic_field}};

/// Minimum projection required to report a numerical certificate.
pub const MARGIN_TOLERANCE: f64 = 1e-12;

fn normalized_samples(field: &Field3, count: usize) -> Result<Vec<[f64; 3]>, String> {
    if field.data.len() != count {
        return Err("Why: the grid shape and sample count differ. Fix: provide exactly n^3 samples.".into());
    }
    field.data.iter().map(|v| {
        let norm = v[0].hypot(v[1]).hypot(v[2]);
        if !norm.is_finite() || norm == 0.0 {
            return Err("Why: a field sample is zero or non-finite. Fix: provide finite nonzero vectors.".into());
        }
        Ok([v[0] / norm, v[1] / norm, v[2] / norm])
    }).collect()
}

/// Minimum anchor projection across all sixteen space-time corners of every
/// periodic cell. A margin greater than `MARGIN_TOLERANCE` certifies a
/// nonvanishing interpolation of the normalized samples, not their Hopf value.
pub fn homotopy_hemisphere_margin(initial: &Field3, final_field: &Field3) -> Result<f64, String> {
    let n = initial.n;
    if n < 2 || n != final_field.n || !initial.dx.is_finite() || initial.dx <= 0.0
        || initial.dx != final_field.dx {
        return Err("Why: the periodic grids do not match or are invalid. Fix: use matching n>=2 and positive finite spacing.".into());
    }
    let count = n.checked_mul(n).and_then(|v| v.checked_mul(n))
        .ok_or_else(|| "Why: grid size overflows. Fix: use a smaller grid.".to_string())?;
    let a = normalized_samples(initial, count)?;
    let b = normalized_samples(final_field, count)?;
    let index = |x: usize, y: usize, z: usize| (x * n + y) * n + z;
    let mut margin = 1.0_f64;
    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                let anchor = a[index(x, y, z)];
                for dx in 0..=1 {
                    for dy in 0..=1 {
                        for dz in 0..=1 {
                            let site = index((x + dx) % n, (y + dy) % n, (z + dz) % n);
                            for v in [a[site], b[site]] {
                                margin = margin.min(anchor[0] * v[0] + anchor[1] * v[1] + anchor[2] * v[2]);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(margin)
}

pub fn checks() -> Vec<Check> {
    let initial = analytic_field(32, 8.0, false);
    let mut rotated = initial.clone();
    let (sin, cos) = 0.01_f64.sin_cos();
    for v in &mut rotated.data {
        *v = [cos * v[0] - sin * v[1], sin * v[0] + cos * v[1], v[2]];
    }
    let margin = homotopy_hemisphere_margin(&initial, &rotated).unwrap();
    vec![
        Check::new("topology_guard.analytic_rotation_margin", margin,
                   "N=32 analytic field to a 0.01-radian rotation; positive margin certifies sampled-map homotopy"),
        Check::new("topology_guard.analytic_rotation_certified", if margin > MARGIN_TOLERANCE { 1.0 } else { 0.0 },
                   "conservative sampled-grid homotopy certificate; production snapshots not tested"),
    ]
}
