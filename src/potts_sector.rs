//! Z3 center sector: the 3-state clock/Potts transfer matrix.
//!
//! T_ij = exp(K cos(2pi(i-j)/3 + delta)) is a Hermitian circulant matrix, so its
//! eigenvalues are a + 2c cos(delta + 2pi k/3) with a = exp(K) and c = exp(-K/2).
//! This gives a group-theoretic route to the two Koide parameters: the ratio
//! r = c/a = exp(-3K/2) and the twist delta. The 2D self-dual critical point is
//! K_c = ln(1+sqrt(3)); we test whether either parameter is fixed there.

use crate::Check;
use std::f64::consts::PI;

pub fn transfer_ratio(k: f64) -> f64 {
    (-1.5 * k).exp()
}

/// Self-dual critical coupling of the 2D q=3 Potts model.
pub fn critical_coupling() -> f64 {
    (1.0 + 3.0_f64.sqrt()).ln()
}

pub fn eigenvalues(k: f64, delta: f64) -> [f64; 3] {
    let a = k.exp();
    let c = (-0.5 * k).exp();
    [0, 1, 2].map(|j| a + 2.0 * c * (delta + 2.0 * PI * j as f64 / 3.0).cos())
}

/// Koide Q of the signed transfer-matrix eigenvalues (needs positive roots).
pub fn q_signed(values: &[f64; 3]) -> Option<f64> {
    let sum: f64 = values.iter().sum();
    let squares: f64 = values.iter().map(|v| v * v).sum();
    if values.iter().all(|v| *v > 0.0) { Some(squares / (sum * sum)) } else { None }
}

/// Coupling at which the circulant Q hits 2/3, i.e. r = 1/sqrt(2).
pub fn koide_coupling() -> f64 {
    2.0_f64.ln() / 3.0
}

pub fn checks() -> Vec<Check> {
    let kc = critical_coupling();
    let r = transfer_ratio(kc);
    let roots = eigenvalues(kc, 0.0);
    vec![
        Check::new("potts.critical_coupling", kc, "2D q=3 Potts self-dual coupling"),
        Check::new("potts.ratio_at_critical", r, "exp(-3K_c/2) at the critical point"),
        Check::new("potts.q_at_critical", q_signed(&roots).unwrap_or(f64::NAN),
                   "Koide Q of the critical transfer eigenvalues"),
        Check::new("potts.ratio_vs_two_ninths", (r - 2.0 / 9.0).abs(),
                   "distance of the critical ratio from the empirical phase 2/9"),
        Check::new("potts.koide_coupling", koide_coupling(),
                   "fitted K where Q=2/3, not a Potts critical point"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn critical_coupling_is_self_dual() {
        assert!(approx(critical_coupling(), 1.00505253874, 1e-9));
        let r = transfer_ratio(critical_coupling());
        assert!(r > 0.22 && r < 0.223, "r = {r}");
    }

    #[test]
    fn critical_ratio_is_a_near_miss_to_two_ninths() {
        let gap = (transfer_ratio(critical_coupling()) - 2.0 / 9.0).abs();
        assert!(gap < 0.002, "gap = {gap}");
        assert!(gap > 1e-6, "must not be an identity, gap = {gap}");
    }

    #[test]
    fn eigenvalues_are_circulant_at_zero_twist() {
        let v = eigenvalues(1.0, 0.0);
        assert!(approx(v[1], v[2], 1e-12));
        assert!(v[0] > v[1]);
    }

    #[test]
    fn koide_coupling_is_not_the_critical_point() {
        let gap = (critical_coupling() - koide_coupling()).abs();
        assert!(gap > 0.5, "gap = {gap}");
        let roots = eigenvalues(koide_coupling(), 0.0);
        assert!(approx(q_signed(&roots).unwrap(), 2.0 / 3.0, 1e-12));
    }

    #[test]
    fn q_stays_below_two_thirds_near_criticality() {
        for k in [0.5_f64, 1.0, critical_coupling(), 1.5, 2.0] {
            if let Some(q) = q_signed(&eigenvalues(k, 0.0)) {
                assert!(q < 2.0 / 3.0, "K={k} q={q}");
            }
        }
    }
}
