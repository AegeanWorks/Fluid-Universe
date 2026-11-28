//! Conditional SU(N)_N conformal weights and phase comparisons.
//!
//! For SU(N) at level k=N (three generations fix N=k=3):
//!   C2(□)      = (N^2 - 1)/(2N)
//!   h^v        = N,  k + h^v = 2N
//!   h_□        = C2/(k+h^v) = (N^2-1)/(4N^2)
//!   C2(Λ^2□)   = (N-2)(N+1)/N
//!   braiding   = |h(Λ^2□) - 2 h(□)| / h(□) = 2/(N-1)
//!
//! The following algebraic comparisons hold for the assumed family SU(N)_N:
//!   1. h_□ = 2/9  =>  (N^2-1)/(4N^2) = 2/9  =>  N^2 = 9  =>  N=3 (unique >0)
//!   2. braiding ratio = 1  =>  2/(N-1) = 1  =>  N=3 (unique)
//!   3. a phase extracted from charged-lepton masses is close to 2/9; the
//!      closed-form identification remains unverified and post-hoc (claim 9).
//! These algebraic conditions do not independently measure the phase or select
//! the WZW family. The identification delta = h_□ remains a hypothesis.

use crate::Check;

/// Quadratic Casimir of the fundamental of SU(N).
pub fn casimir_box(n: i64) -> f64 { (n * n - 1) as f64 / (2.0 * n as f64) }

/// Quadratic Casimir of the antisymmetric 2-index representation Λ²□.
pub fn casimir_lambda2(n: i64) -> f64 {
    ((n - 2) * (n + 1)) as f64 / n as f64
}

/// Conformal dimension h_□ at level k=N: C2/(k + h^v) = (N^2-1)/(4N^2).
pub fn h_box(n: i64) -> f64 { (n * n - 1) as f64 / (4.0 * n as f64 * n as f64) }

/// Braiding ratio |h(Λ²□) - 2 h(□)| / h(□) = 2/(N-1).
pub fn braiding_ratio(n: i64) -> f64 {
    let denom = 2.0 * n as f64;
    let h2 = casimir_lambda2(n) / denom;
    let h1 = h_box(n);
    (h2 - 2.0 * h1).abs() / h1
}

/// Solve (N^2-1)/(4N^2) = target for positive integer N.
pub fn solve_n_for_h(target: f64) -> Option<i64> {
    // (N^2-1) = 4 target N^2  =>  N^2 (1 - 4 target) = 1  =>  N = sqrt(1/(1-4t)).
    if 1.0 - 4.0 * target <= 0.0 { return None; }
    let n = (1.0 / (1.0 - 4.0 * target)).sqrt();
    let rounded = n.round() as i64;
    if rounded > 0 && (h_box(rounded) - target).abs() < 1e-9 { Some(rounded) } else { None }
}

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("conformal_block.casimir_box_3", casimir_box(3), "C2(□) of SU(3) = 4/3"),
        Check::new("conformal_block.h_box_3", h_box(3), "h_□(SU(3)_3) = 2/9"),
        Check::new("conformal_block.braiding_3", braiding_ratio(3),
                   "braiding ratio at N=3 (unity)"),
        Check::new("conformal_block.solved_n", solve_n_for_h(2.0 / 9.0).unwrap_or(0) as f64,
                   "N solving h_□=2/9 (unique: 3)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn casimir_and_h_are_exact_at_three() {
        assert!((casimir_box(3) - 4.0 / 3.0).abs() < 1e-12);
        assert!((h_box(3) - 2.0 / 9.0).abs() < 1e-12);
    }

    #[test]
    fn braiding_ratio_is_two_over_n_minus_one() {
        for n in [2, 3, 4, 5, 6] {
            assert!((braiding_ratio(n) - 2.0 / (n - 1) as f64).abs() < 1e-10);
        }
    }

    #[test]
    fn n_equals_three_is_the_unique_solution() {
        assert_eq!(solve_n_for_h(2.0 / 9.0), Some(3));
        assert_eq!(solve_n_for_h(0.25), None); // trivial/no finite solution
        // braiding = 1 only at N=3
        assert!((braiding_ratio(3) - 1.0).abs() < 1e-12);
        for n in [2, 4, 5] {
            assert!((braiding_ratio(n) - 1.0).abs() > 0.1);
        }
    }

    #[test]
    fn derived_h_matches_recorded_phase() {
        let delta_from_pdg = 0.22222140379215438;
        assert!((h_box(3) - delta_from_pdg).abs() / delta_from_pdg < 1e-4);
    }
}
