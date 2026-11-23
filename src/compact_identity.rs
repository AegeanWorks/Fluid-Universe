//! Compact algebraic identity and a new SU(Fib) structural test.
//!
//! From the index framework A^2(n) = n + 1/n, the Koide quotient simplifies:
//!   Q(n) = 1/3 + A^2/6 = 1/3 + (n + 1/n)/6 = (n^2 + 2n + 1)/(6n)
//!        = (n+1)^2 / (6n).
//! This gives Q(1)=2/3, Q(2)=3/4, Q(3)=8/9 exactly.
//!
//! New structural hypothesis tested here: the Fibonacci labels F=Fib(n+3)
//! = {3,5,8} are the ranks of SU(N) groups, with the phase supplied by the
//! fundamental conformal dimension h = C2(box)/(k+N) = delta(n). Solving for
//! the required level k = C2/delta - N gives:
//!   n=1: N=3, k=3 EXACT (the known SU(3)_3 lepton case; d_box=2 matches A^2)
//!   n=2: N=5, k=19 EXACT integer, BUT d_box(SU(5)_19) ~ 4.66, not A^2=2.5
//!   n=3: N=8, k=67.6 NOT an integer
//! The hypothesis therefore FAILS for down and up: matching the phase does not
//! match the amplitude, and the up level is not integral. Recorded honestly.

use crate::Check;

pub fn fib(k: i32) -> i64 {
    let (mut a, mut b) = (0_i64, 1_i64);
    for _ in 0..k { let t = a + b; a = b; b = t; }
    a
}

/// Compact Koide quotient: Q(n) = (n+1)^2/(6n).
pub fn q_compact(n: i32) -> f64 {
    ((n + 1) * (n + 1)) as f64 / (6.0 * n as f64)
}

/// Casimir of the SU(N) fundamental (long-root normalization).
pub fn c2_box(n: i64) -> f64 { (n as f64 * n as f64 - 1.0) / (2.0 * n as f64) }

/// Level k required by h = C2/(k+N) = delta(n) for SU(N=Fib(n+3)).
pub fn required_level(n: i32) -> f64 {
    let nn = fib(n + 3) as f64;
    let a2 = n as f64 + 1.0 / n as f64;
    let delta = a2 / (nn * nn);
    c2_box(nn as i64) / delta - nn
}

/// Quantum dimension of the SU(N) fundamental at level k.
pub fn d_box(nn: i64, k: f64) -> f64 {
    let kbig = k + nn as f64;
    (std::f64::consts::PI * nn as f64 / kbig).sin()
        / (std::f64::consts::PI / kbig).sin()
}

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("compact.q1", q_compact(1), "Q(1) = 2/3"),
        Check::new("compact.q2", q_compact(2), "Q(2) = 3/4"),
        Check::new("compact.q3", q_compact(3), "Q(3) = 8/9"),
        Check::new("compact.level_n1", required_level(1), "SU(Fib(4)=3) required k = 3"),
        Check::new("compact.level_n2", required_level(2), "SU(Fib(5)=5) required k = 19"),
        Check::new("compact.level_n3", required_level(3),
                   "SU(Fib(6)=8) required k = 67.6 (NOT integer -> fails)"),
        Check::new("compact.dbox_n2", d_box(fib(5), required_level(2)),
                   "SU(5)_19 fundamental dimension ~4.66, NOT A^2=2.5"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_identity_is_exact() {
        assert!((q_compact(1) - 2.0/3.0).abs() < 1e-12);
        assert!((q_compact(2) - 3.0/4.0).abs() < 1e-12);
        assert!((q_compact(3) - 8.0/9.0).abs() < 1e-12);
        for n in 1..=3 {
            assert!((q_compact(n) - (1.0/3.0 + (n as f64 + 1.0/n as f64)/6.0)).abs() < 1e-12);
        }
    }

    #[test]
    fn su_fib_hypothesis_fails() {
        assert!((required_level(1) - 3.0).abs() < 1e-9);
        assert!((required_level(2) - 19.0).abs() < 1e-9);
        let k3 = required_level(3);
        assert!((k3 - k3.round()).abs() > 0.1, "n=3 level must be non-integral (k={k3})");
        // n=2: the phase matches at k=19, but the quantum dimension does not.
        assert!((d_box(fib(5), required_level(2)) - 2.5).abs() > 1.0);
    }
}
