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

#[derive(Clone, Debug)]
pub struct MonitoredRelaxation {
    pub field: Field3,
    pub completed_steps: usize,
    pub inconclusive_steps: usize,
    pub first_inconclusive_step: Option<usize>,
    pub minimum_margin: Option<f64>,
    pub mean_minimum_margin: Option<f64>,
    pub mean_inconclusive_steps: usize,
    pub mean_first_inconclusive_step: Option<usize>,
    pub start_energy: f64,
    pub end_energy: f64,
}

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

/// Use the normalized sum of each cell's sixteen endpoint samples as a
/// separating direction. A zero sum yields an inconclusive zero margin.
pub fn homotopy_mean_margin(initial: &Field3, final_field: &Field3) -> Result<f64, String> {
    // Reuse the established grid and sample validation.
    homotopy_hemisphere_margin(initial, final_field)?;
    let n = initial.n;
    let count = n * n * n;
    let a = normalized_samples(initial, count)?;
    let b = normalized_samples(final_field, count)?;
    let mut margin = 1.0_f64;
    for x in 0..n { for y in 0..n { for z in 0..n {
        let mut samples = [[0.0; 3]; 16];
        let mut next = 0;
        for dx in 0..=1 { for dy in 0..=1 { for dz in 0..=1 {
            let i = (((x + dx) % n) * n + (y + dy) % n) * n + (z + dz) % n;
            samples[next] = a[i]; samples[next + 1] = b[i]; next += 2;
        }}}
        let mut sum = [0.0; 3];
        for v in samples { for k in 0..3 { sum[k] += v[k]; } }
        let length = sum[0].hypot(sum[1]).hypot(sum[2]);
        if length <= MARGIN_TOLERANCE { return Ok(0.0); }
        for v in samples {
            margin = margin.min((sum[0]*v[0] + sum[1]*v[1] + sum[2]*v[2]) / length);
        }
    }}}
    Ok(margin)
}

/// Run the existing relaxation one step at a time and test each completed
/// sampled-map transition. A nonpositive margin is inconclusive; even when
/// all transitions pass, the certificate concerns the piecewise interpolated
/// sampled maps, not an unsampled continuum trajectory.
pub fn monitored_relaxation(
    initial: &Field3, mu: f64, dt: f64, steps: usize, k: f64, kappa: f64,
) -> Result<MonitoredRelaxation, String> {
    if ![mu, dt, k, kappa].iter().all(|value| value.is_finite()) || dt <= 0.0 {
        return Err("Why: relaxation parameters are non-finite or dt is not positive. Fix: use finite parameters and dt>0.".into());
    }
    homotopy_hemisphere_margin(initial, initial)?;
    let start_energy = crate::simulation::energy_terms(initial, mu, k, kappa).total;
    if !start_energy.is_finite() {
        return Err("Why: the initial energy is non-finite. Fix: rescale the field or parameters.".into());
    }
    let mut result = MonitoredRelaxation {
        field: initial.clone(), completed_steps: 0, inconclusive_steps: 0, first_inconclusive_step: None,
        minimum_margin: None, mean_minimum_margin: None, mean_inconclusive_steps: 0, mean_first_inconclusive_step: None, start_energy, end_energy: start_energy,
    };
    for _ in 0..steps {
        let (next, history, _) = crate::simulation::relax(&result.field, mu, dt, 1, k, kappa);
        if history.iter().any(|value| !value.is_finite()) {
            return Err("Why: relaxation produced non-finite energy. Fix: reduce the step or rescale parameters.".into());
        }
        if history.len() < 2 { break; }
        let margin = homotopy_hemisphere_margin(&result.field, &next)?;
        let mean = homotopy_mean_margin(&result.field, &next)?;
        result.mean_minimum_margin = Some(result.mean_minimum_margin.map_or(mean, |old| old.min(mean)));
        if mean <= MARGIN_TOLERANCE {
            result.mean_inconclusive_steps += 1;
            if result.mean_first_inconclusive_step.is_none() { result.mean_first_inconclusive_step = Some(result.completed_steps + 1); }
        }
        result.minimum_margin = Some(result.minimum_margin.map_or(margin, |old| old.min(margin)));
        if margin <= MARGIN_TOLERANCE {
            result.inconclusive_steps += 1;
            if result.first_inconclusive_step.is_none() {
                result.first_inconclusive_step = Some(result.completed_steps + 1);
            }
        }
        result.completed_steps += 1;
        result.end_energy = *history.last().unwrap();
        result.field = next;
    }
    Ok(result)
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
