//! Derivation of the number of sectors from d=3 (no measured input).
//!
//! In the d=3 chain the Koide quotient of sector n is
//!   Q(n) = (n+1)^2 / (6n),   6 = d^2 - d.
//! A Koide quotient obeys Q <= 1 (it equals 1 only for exactly degenerate
//! masses). Requiring Q(n) <= 1 gives
//!   (n+1)^2 <= 6n  <=>  n^2 - 4n + 1 <= 0  <=>  n in [2-sqrt3, 2+sqrt3]
//!   = [0.2679, 3.7321],
//! whose only positive integers are n = 1, 2, 3. Thus the framework DERIVES
//! that there are exactly three fermion sectors (charged leptons, down, up)
//! from d=3 -- rather than assuming it. n=4 gives Q=25/24>1, impossible.
//!
//! The complements nu(n) = 1 - Q(n) = {1/3, 1/4, 1/9} coincide with known
//! fractional-quantum-Hall filling factors, extending the d=3 FQHE link.
//!
//! HONEST CAVEAT: this derives the COUNT of sectors, not their identification
//! with e/d/u (that assignment still uses electric charge). The bound Q<=1 is
//! a mathematical property of the quotient, not new physics.

use crate::Check;

pub fn q(n: i64) -> f64 { ((n+1)*(n+1)) as f64 / (6.0*n as f64) }
pub fn admissible(n: i64) -> bool { q(n) <= 1.0 }
pub fn lower_bound() -> f64 { 2.0 - 3.0_f64.sqrt() }
pub fn upper_bound() -> f64 { 2.0 + 3.0_f64.sqrt() }
pub fn nu(n: i64) -> f64 { 1.0 - q(n) }

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("sectors.lower", lower_bound(), "2 - sqrt(3) = 0.2679"),
        Check::new("sectors.upper", upper_bound(), "2 + sqrt(3) = 3.7321"),
        Check::new("sectors.q4", q(4), "Q(4) = 25/24 > 1 (excluded)"),
        Check::new("sectors.nu1", nu(1), "nu_1 = 1/3 (FQHE)"),
        Check::new("sectors.nu2", nu(2), "nu_2 = 1/4 (FQHE)"),
        Check::new("sectors.nu3", nu(3), "nu_3 = 1/9 (FQHE)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exactly_three_admissible_sectors() {
        let ok: Vec<i64> = (1..=10).filter(|&n| admissible(n)).collect();
        assert_eq!(ok, vec![1,2,3]);
    }

    #[test]
    fn bound_is_exact() {
        assert!((lower_bound() - (2.0-3.0_f64.sqrt())).abs() < 1e-12);
        assert!((upper_bound() - (2.0+3.0_f64.sqrt())).abs() < 1e-12);
        assert!(admissible(3) && !admissible(4));
    }

    #[test]
    fn fqhe_complements() {
        assert!((nu(1) - 1.0/3.0).abs() < 1e-12);
        assert!((nu(2) - 1.0/4.0).abs() < 1e-12);
        assert!((nu(3) - 1.0/9.0).abs() < 1e-12);
    }
}
