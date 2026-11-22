//! Froggatt-Nielsen charge sector: masses m_i = eps^(2 q_i) for integer charges.
//!
//! Requiring Q=2/3 fixes eps for each integer charge triple; the resulting
//! hierarchy is then a prediction, not a fit. The best triple (0,1,3) predicts
//! sqrt-mass ratios ~4.06 and ~16.4 against observed ~4.10 and ~14.4.

use crate::Check;
pub fn q_of_masses(m: &[f64; 3]) -> f64 {
    let sum: f64 = m.iter().sum();
    let roots: f64 = m.iter().map(|v| v.sqrt()).sum();
    sum / (roots * roots)
}

/// eps in (0,1) making Q(1, eps^(2q1), eps^(2q2)) = 2/3, bisection.
pub fn eps_for_koide(charges: &[usize; 3]) -> Option<f64> {
    if charges[0] != 0 {
        return None; // scale invariance: normalise the smallest charge to zero
    }
    let q_at = |eps: f64| {
        let m = [1.0, eps.powf(2.0 * charges[1] as f64),
                 eps.powf(2.0 * charges[2] as f64)];
        q_of_masses(&m)
    };
    let (mut lo, mut hi) = (1e-4_f64, 0.999_f64);
    // Q rises from ~1/3 to ~1 as eps grows; find the crossing.
    // Q decreases monotonically with eps for these charges (from ~1 to 1/3).
    if q_at(lo) < 2.0 / 3.0 || q_at(hi) > 2.0 / 3.0 {
        return None;
    }
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if q_at(mid) > 2.0 / 3.0 { lo = mid; } else { hi = mid; }
    }
    Some(0.5 * (lo + hi))
}

/// sqrt-mass ratios (large/small, medium/small) implied by a charge triple.
pub fn ratios(charges: &[usize; 3], eps: f64) -> (f64, f64) {
    let m = [1.0, eps.powf(2.0 * charges[1] as f64),
             eps.powf(2.0 * charges[2] as f64)];
    // Return the ratios in decreasing-mass order (heaviest/middle, middle/lightest).
    ((m[0] / m[1]).sqrt(), (m[1] / m[2]).sqrt())
}

pub fn checks() -> Vec<Check> {
    let eps = eps_for_koide(&[0, 1, 3]).unwrap_or(f64::NAN);
    let (big, small) = ratios(&[0, 1, 3], eps);
    vec![
        Check::new("froggatt.eps_for_013", eps, "eps fixed by Q=2/3 for charges (0,1,3)"),
        Check::new("froggatt.root_ratio_large", big, "sqrt(m3/m2) prediction"),
        Check::new("froggatt.root_ratio_small", small, "sqrt(m2/m1) prediction"),
        Check::new("froggatt.observed_large", 4.098, "observed sqrt(m_tau/m_mu)"),
        Check::new("froggatt.observed_small", 14.377, "observed sqrt(m_mu/m_e)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn charge_013_is_solved() {
        let eps = eps_for_koide(&[0, 1, 3]).unwrap();
        assert!(eps > 0.2 && eps < 0.3, "eps = {eps}");
        let m = [1.0, eps.powf(2.0), eps.powf(6.0)];
        assert!((q_of_masses(&m) - 2.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn predicted_ratios_are_near_but_not_exact() {
        let eps = eps_for_koide(&[0, 1, 3]).unwrap();
        let (big, small) = ratios(&[0, 1, 3], eps);
        assert!((big - 4.098).abs() < 0.3, "big = {big}");
        assert!((small - 14.377).abs() > 1.0, "small = {small}");
    }


}
