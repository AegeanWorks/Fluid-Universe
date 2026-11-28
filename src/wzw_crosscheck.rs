//! Independent cross-check of the SU(3)_3 WZW framework against other sectors.
//!
//! In the WZW Brannen formula sqrt(m_k) = mu(1 + A cos(delta + 2 pi k/3)),
//! the Koide quotient is Q = 1/3 + A^2/6, and the SU(3)_k framework identifies
//! A^2 with the fundamental quantum dimension d_□(k) = sin(3pi/K)/sin(pi/K),
//! K = k+3. For k>=1 this is bounded: d_□ in (1,3), hence Q in (1/2, 5/6).
//!
//! We test this against the recorded neutrino and quark Koide triplets:
//!   * neutrinos: Q_nu(max) ~ 0.585612  -> A^2 ~ 1.51 (in range, k~4.3)
//!   * down quarks: Q_D ~ 0.748065     -> A^2 ~ 2.49 (in range, k~5.8)
//!   * up quarks: Q_U ~ 0.887569       -> A^2 ~ 3.33 (OUT of range, needs k<0)
//! The up-quark triplet therefore falsifies a naive extension of the same
//! quantum-dimension mechanism. This is an independent, recorded test.

use crate::Check;
use std::f64::consts::PI;

/// Fundamental quantum dimension of SU(3)_k: sin(3pi/K)/sin(pi/K), K=k+3.
pub fn quantum_dimension_fundamental(k: i32) -> f64 {
    let big_k = (k + 3) as f64;
    (3.0 * PI / big_k).sin() / (PI / big_k).sin()
}

/// Koide quotient implied by the quantum dimension A^2 = d_□.
pub fn q_from_quantum_dimension(k: i32) -> f64 {
    1.0 / 3.0 + quantum_dimension_fundamental(k) / 6.0
}

/// Recover the required amplitude squared A^2 from a measured Q.
pub fn amplitude_squared_from_q(q: f64) -> f64 { 6.0 * (q - 1.0 / 3.0) }

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("wzw_crosscheck.d_k3", quantum_dimension_fundamental(3),
                   "d_□(SU(3)_3) = 2"),
        Check::new("wzw_crosscheck.q_k3", q_from_quantum_dimension(3),
                   "Q from k=3 (leptons, 2/3)"),
        Check::new("wzw_crosscheck.nu_a2", amplitude_squared_from_q(0.5856120093687199),
                   "A^2 required by the neutrino Q"),
        Check::new("wzw_crosscheck.down_a2", amplitude_squared_from_q(0.7480650548031402),
                   "A^2 required by the down-quark Q"),
        Check::new("wzw_crosscheck.up_a2", amplitude_squared_from_q(0.8875692313187327),
                   "A^2 required by the up-quark Q (>3, out of range)"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantum_dimension_is_bounded() {
        assert!((quantum_dimension_fundamental(3) - 2.0).abs() < 1e-12);
        for k in [1, 2, 3, 5, 10, 100] {
            let d = quantum_dimension_fundamental(k);
            assert!((1.0..=3.0).contains(&d), "k={k} d={d}");
        }
    }

    #[test]
    fn lepton_and_down_and_neutrino_are_in_range() {
        let nu = amplitude_squared_from_q(0.5856120093687199);
        let down = amplitude_squared_from_q(0.7480650548031402);
        assert!(nu > 1.0 && nu < 3.0, "nu A^2 = {nu}");
        assert!(down > 1.0 && down < 3.0, "down A^2 = {down}");
    }

    #[test]
    fn up_quark_falsifies_the_simple_extension() {
        let up = amplitude_squared_from_q(0.8875692313187327);
        assert!(up > 3.0, "up A^2 = {up} exceeds max quantum dimension 3");
        assert!((q_from_quantum_dimension(3) - 2.0 / 3.0).abs() < 1e-12);
    }
}
