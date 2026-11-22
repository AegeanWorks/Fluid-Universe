//! Joint Froggatt-Nielsen charge scan: both hierarchy ratios at once.
//!
//! Extends froggatt.rs from the single (0,1,3) triple to all integer charge
//! triples with q2 <= 20, scoring each by the joint relative error against the
//! observed sqrt-mass ratios. Koide fixes eps per triple; the hierarchy is a
//! prediction. The scan asks whether any discrete assignment reproduces both.

use crate::Check;
use crate::linalg::bisect;

/// Koide Q of three masses.
pub fn q_of(m: &[f64; 3]) -> f64 {
    let sum: f64 = m.iter().sum();
    let roots: f64 = m.iter().map(|v| v.sqrt()).sum();
    sum / (roots * roots)
}

/// eps in (0,1) making Q=2/3 for charges (0,q1,q2), or None.
pub fn eps_for(q1: usize, q2: usize) -> Option<f64> {
    let qat = |e: f64| q_of(&[1.0, e.powf(2.0 * q1 as f64), e.powf(2.0 * q2 as f64)]);
    if qat(1e-4) < 2.0 / 3.0 || qat(0.9999) > 2.0 / 3.0 {
        return None;
    }
    bisect(|e| qat(e) - 2.0 / 3.0, 1e-4, 0.9999, 1e-12)
}

pub struct Assignment {
    pub charges: [usize; 3],
    pub eps: f64,
    pub heavy: f64,
    pub light: f64,
    pub error: f64,
}

/// Scan integer triples and rank by joint relative error against the observed ratios.
pub fn scan(q2_max: usize) -> Vec<Assignment> {
    let (target_heavy, target_light) = (4.098_f64, 14.377_f64);
    let mut out = Vec::new();
    for q1 in 1..q2_max {
        for q2 in (q1 + 1)..=q2_max {
            if let Some(eps) = eps_for(q1, q2) {
                let heavy = eps.powf(-(q1 as f64));
                let light = eps.powf(-((q2 - q1) as f64));
                let error = ((heavy / target_heavy - 1.0).powi(2)
                             + (light / target_light - 1.0).powi(2)).sqrt();
                out.push(Assignment {
                    charges: [0, q1, q2], eps, heavy, light, error,
                });
            }
        }
    }
    out.sort_by(|a, b| a.error.partial_cmp(&b.error).unwrap_or(std::cmp::Ordering::Equal));
    out
}

pub fn checks() -> Vec<Check> {
    let ranked = scan(14);
    let best = &ranked[0];
    vec![
        Check::new("fn_scan.best_charges", best.charges[2] as f64,
                   "largest charge of the best jointly-scored triple"),
        Check::new("fn_scan.best_error", best.error, "joint relative error of the best triple"),
        Check::new("fn_scan.best_heavy", best.heavy, "predicted heavy sqrt-mass ratio"),
        Check::new("fn_scan.best_light", best.light, "predicted light sqrt-mass ratio"),
        Check::new("fn_scan.triple_count", ranked.len() as f64, "charge triples with a Koide solution"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eps_for_013_is_recovered() {
        let e = eps_for(1, 3).unwrap();
        assert!((e - 0.2465686).abs() < 1e-6, "eps = {e}");
    }

    #[test]
    fn best_joint_fit_is_not_exact() {
        let ranked = scan(14);
        assert!(!ranked.is_empty());
        assert!(ranked[0].error > 0.05, "best error = {}", ranked[0].error);
        assert_eq!(ranked[0].charges, [0, 5, 14]);
    }

    #[test]
    fn koide_holds_for_every_scored_triple() {
        for a in scan(6).iter().take(5) {
            let m = [1.0, a.eps.powf(2.0 * a.charges[1] as f64),
                     a.eps.powf(2.0 * a.charges[2] as f64)];
            assert!((q_of(&m) - 2.0 / 3.0).abs() < 1e-8);
        }
    }
}
