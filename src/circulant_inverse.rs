//! Exact inversion of the circulant mass triplet from its two hierarchy ratios.
//!
//! For the Brannen circulant sqrt(m_k) proportional to 1 + A cos(delta+2pi k/3)
//! the three signed roots y_k sum to 3 (with the constant term 1). Therefore
//! the two ratios r1=y2/y1, r2=y3/y2 determine the roots EXACTLY:
//!   y1 = 3/(1+r1+r1*r2), y2=r1*y1, y3=r2*y2.
//! The Koide-like quotient of the masses y_k^2 is then Q = sum(y_k^2)/9, with
//! no fitting of A or delta. Applying this to the recorded quark hierarchies
//! gives Q_up=0.8465 and Q_down=0.7339, both BELOW the measured running-mass
//! ranges [0.88757,0.89255] and [0.73854,0.74807]. A circulant representation
//! of the observed quark hierarchies is therefore inconsistent with the
//! observed quark Koide quotients. This is a rigorous no-go, not a fit failure.

use crate::Check;

/// Roots implied by two hierarchy ratios, using the sum rule y1+y2+y3=3.
pub fn roots_from_ratios(r1: f64, r2: f64) -> [f64; 3] {
    let y1 = 3.0 / (1.0 + r1 + r1 * r2);
    [y1, r1 * y1, r2 * r1 * y1]
}

/// Koide quotient Q = sum(y^2)/9 of the implied roots (all-positive branch).
pub fn implied_q(r1: f64, r2: f64) -> f64 {
    let y = roots_from_ratios(r1, r2);
    (y[0] * y[0] + y[1] * y[1] + y[2] * y[2]) / 9.0
}

/// Recorded quark hierarchy ratios at the reference scale (GeV masses).
pub fn up_ratios() -> (f64, f64) {
    ((1.27_f64 / 0.00418).sqrt(), (171.7_f64 / 1.27).sqrt())
}

pub fn down_ratios() -> (f64, f64) {
    ((0.093_f64 / 0.00418).sqrt(), (4.18_f64 / 0.093).sqrt())
}

pub fn checks() -> Vec<Check> {
    let (u1, u2) = up_ratios();
    let (d1, d2) = down_ratios();
    let yu = roots_from_ratios(u1, u2);
    vec![
        Check::new("circulant_inverse.sum_rule", yu[0] + yu[1] + yu[2],
                   "root sum rule y1+y2+y3 = 3"),
        Check::new("circulant_inverse.q_up_implied", implied_q(u1, u2),
                   "Q implied by the up hierarchy (recorded range starts 0.88757)"),
        Check::new("circulant_inverse.q_down_implied", implied_q(d1, d2),
                   "Q implied by the down hierarchy (recorded range starts 0.73854)"),
        Check::new("circulant_inverse.up_gap", 0.8875692313187327 - implied_q(u1, u2),
                   "measured minus implied up Q (positive = no-go)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_rule_is_exact() {
        let y = roots_from_ratios(14.377, 4.098);
        assert!((y[0] + y[1] + y[2] - 3.0).abs() < 1e-12);
        assert!((y[1] / y[0] - 14.377).abs() < 1e-9);
        assert!((y[2] / y[1] - 4.098).abs() < 1e-9);
    }

    #[test]
    fn lepton_ratios_reproduce_two_thirds() {
        let q = implied_q(14.377, 4.098);
        assert!((q - 2.0 / 3.0).abs() < 0.01, "q = {q}");
    }

    #[test]
    fn quark_hierarchies_imply_q_below_measured() {
        let (u1, u2) = up_ratios();
        let (d1, d2) = down_ratios();
        assert!(implied_q(u1, u2) < 0.88757, "up implied = {}", implied_q(u1, u2));
        assert!(implied_q(d1, d2) < 0.73854, "down implied = {}", implied_q(d1, d2));
    }
}
