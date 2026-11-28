//! Tilted Z3 potential spinodal sector.
//!
//! V(phi) = 1 - cos(3phi) + h cos(phi). The exact spinodal of the central
//! minimum is h = 9 (V'=V''=0 at phi=0). This module computes the curvatures of
//! the surviving minima and tests whether three distinct masses can emerge.

use crate::Check;

const TWO_PI: f64 = std::f64::consts::TAU;

fn first(phi: f64, h: f64) -> f64 {
    3.0 * (3.0 * phi).sin() - h * phi.sin()
}

fn second(phi: f64, h: f64) -> f64 {
    9.0 * (3.0 * phi).cos() - h * phi.cos()
}

/// Newton refinement of a stationary point; returns (phi, V'').
pub fn stationary(guess: f64, h: f64) -> Option<(f64, f64)> {
    let mut phi = guess;
    for _ in 0..100 {
        let d = second(phi, h);
        if d.abs() < 1e-14 {
            return None;
        }
        let step = first(phi, h) / d;
        phi -= step;
        if step.abs() < 1e-14 {
            break;
        }
    }
    if first(phi, h).abs() > 1e-8 {
        return None;
    }
    Some((phi.rem_euclid(TWO_PI), second(phi, h)))
}

/// Distinct minima of the tilted potential (curvature > 0), deduplicated by phase.
pub fn minima(h: f64) -> Vec<(f64, f64)> {
    let mut out: Vec<(f64, f64)> = Vec::new();
    for index in 0..96 {
        let guess = TWO_PI * index as f64 / 96.0;
        if let Some((phi, curvature)) = stationary(guess, h) {
            if curvature <= 1e-8 {
                continue;
            }
            if out.iter().all(|(other, _)| {
                let delta = (phi - other).abs().min(TWO_PI - (phi - other).abs());
                delta > 1e-5
            }) {
                out.push((phi, curvature));
            }
        }
    }
    out.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    out
}

/// Koide Q of the three largest positive curvatures (the three most stable minima).
pub fn q_from_curvatures(values: &[f64]) -> f64 {
    let mut positive: Vec<f64> = values.iter().copied().filter(|v| *v > 0.0).collect();
    if positive.len() < 3 {
        return 0.0;
    }
    positive.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    let m = &positive[..3];
    let sum: f64 = m.iter().sum();
    let roots: f64 = m.iter().map(|v| v.sqrt()).sum();
    sum / (roots * roots)
}

pub fn checks() -> Vec<Check> {
    let near = minima(8.9);
    let values: Vec<f64> = near.iter().map(|(_, c)| *c).collect();
    vec![
        Check::new("spinodal.critical_h", 9.0, "exact spinodal of the central minimum"),
        Check::new("spinodal.minima_at_8_9", values.len() as f64,
                   "number of distinct minima below the spinodal"),
        Check::new("spinodal.q_at_8_9", q_from_curvatures(&values),
                   "Koide Q of the curvature masses near the spinodal"),
        Check::new("spinodal.degenerate_pair_gap",
                   (values.get(1).copied().unwrap_or(0.0)
                    - values.get(2).copied().unwrap_or(0.0)).abs(),
                   "near-degeneracy of the two off-centre minima"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn critical_field_is_exact() {
        use crate::linalg::approx;
        assert!(approx(first(0.0, 9.0), 0.0, 1e-14));
        assert!(approx(second(0.0, 9.0), 0.0, 1e-14));
    }

    #[test]
    fn flat_potential_has_three_equal_minima() {
        use crate::linalg::approx;
        let values = minima(0.0);
        assert!(values.len() >= 3);
        let curvatures: Vec<f64> = values.iter().map(|(_, c)| *c).collect();
        assert!(approx(q_from_curvatures(&curvatures), 1.0 / 3.0, 1e-6));
    }

    #[test]
    fn spinodal_keeps_a_degenerate_pair() {
        let values = minima(8.9);
        assert_eq!(values.len(), 3);
        let gap = (values[1].1 - values[2].1).abs();
        assert!(gap < 1e-6, "gap = {gap}");
    }

    #[test]
    fn curvature_masses_never_reach_koide() {
        for h in [0.0, 2.0, 5.0, 8.0, 8.9] {
            let values = minima(h);
            let q = q_from_curvatures(&values.iter().map(|(_, c)| *c).collect::<Vec<_>>());
            assert!(q < 0.35, "h={h} q={q}");
        }
    }
}
