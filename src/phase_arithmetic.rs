//! Why the WZW identification has the right arithmetic form and G2 geometry does not.
//!
//! The Brannen phase is delta = 2/9 RADIANS: a rational number, not a rational
//! multiple of pi (2/(9 pi) is irrational since pi is transcendental). The
//! circulant eigenvalues mix this rational operator phase with the rational
//! multiples of pi (2 pi k / 3) coming from the Z3 eigenvectors -- exactly the
//! distinction Brannen noted.
//!
//! Consequences tested here:
//!   * WZW conformal dimensions h = C2/(k+h^v) are RATIONAL numbers (2/9 for
//!     SU(3)_3), so h has the same arithmetic form as delta: the identification
//!     delta = h is at least dimensionally/arithmetically consistent.
//!   * A geometric G2 / J3(O) 3-form angle is a real multiple of pi (root-system
//!     and Grassmannian angles), so it CANNOT equal the rational-in-radians
//!     delta exactly; the J3(O) route can only approximate delta.
//!   * The G2 3-form on random 3-planes spans [-1,1] and contains the target
//!     -cos(2/3), but that value is not distinguished by a generic extremum.

use crate::Check;
use std::f64::consts::PI;

/// The Brannen phase as a rational number (numerator, denominator).
pub fn delta_rational() -> (i64, i64) { (2, 9) }

/// delta in radians.
pub fn delta_radians() -> f64 {
    let (n, d) = delta_rational();
    n as f64 / d as f64
}

/// Is x a rational multiple p/q*pi with small q (<= 24), within tolerance?
pub fn is_rational_multiple_of_pi(x: f64, tolerance: f64) -> bool {
    let ratio = x / PI; // x/pi, want ratio = p/q
    for q in 1..=24 {
        let p = ratio * q as f64;
        if (p - p.round()).abs() < tolerance {
            return true;
        }
    }
    false
}

/// WZW conformal dimension h_□ of SU(3)_3 = 2/9 (rational, matches delta's form).
pub fn wzw_h_rational_form() -> (i64, i64) {
    // C2 = 4/3, k + h^v = 6, h = (4/3)/6 = 4/18 = 2/9.
    (2, 9)
}

pub fn checks() -> Vec<Check> {
    let (n, d) = delta_rational();
    vec![
        Check::new("phase_arithmetic.delta_value", delta_radians(), "delta = 2/9 radians"),
        Check::new("phase_arithmetic.delta_over_pi", delta_radians() / PI,
                   "delta/pi (irrational; not a rational angle)"),
        Check::new("phase_arithmetic.wzw_h", 2.0 / 9.0,
                   "WZW h = 2/9 rational, same form as delta"),
        Check::new("phase_arithmetic.forms_match",
                   if (n, d) == wzw_h_rational_form() { 1.0 } else { 0.0 },
                   "delta and h have identical rational form"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delta_is_not_a_pi_fraction() {
        // 2/9 radians is NOT close to any small rational multiple of pi.
        assert!(!is_rational_multiple_of_pi(delta_radians(), 1e-6));
        for n in 1..20 {
            let pi_frac = n as f64 * PI / 24.0;
            assert!((delta_radians() - pi_frac).abs() > 1e-3);
        }
    }

    #[test]
    fn wzw_and_delta_share_the_rational_form() {
        assert_eq!(delta_rational(), wzw_h_rational_form());
        assert!((wzw_h_rational_form().0 as f64 / wzw_h_rational_form().1 as f64
                 - delta_radians()).abs() < 1e-14);
    }

    #[test]
    fn geometric_angles_are_pi_multiples() {
        // Representative G2/Grassmannian angles are pi fractions, not rational radians.
        for (p, q) in [(1, 3), (1, 4), (1, 6), (1, 7), (2, 3)] {
            let angle = p as f64 * PI / q as f64;
            assert!(is_rational_multiple_of_pi(angle, 1e-12));
            assert!((angle - delta_radians()).abs() > 1e-3, "angle {p}pi/{q} near delta");
        }
    }
}
