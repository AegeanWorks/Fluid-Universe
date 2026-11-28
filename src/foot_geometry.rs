//! Peer-reviewed geometric validation of Koide's beta = 2/3 (Foot).
//!
//! R. Foot interpreted the Koide formula geometrically: for the mass vector
//! v = (sqrt(m1), sqrt(m2), sqrt(m3)), the angle theta between v and the
//! democratic vector (1,1,1) satisfies cos^2(theta) = 1/(3Q), where
//! Q = sum(m)/(sum sqrt(m))^2. Koide's Q = 2/3 is therefore equivalent to
//! theta = 45 degrees exactly -- the mass vector has equal democratic and
//! traceless components. This is a peer-reviewed reformulation of Koide's
//! relation (not a derivation of it from deeper axioms).
//!
//! We verify the identity numerically against recorded masses:
//!   lepton: theta = 44.9999 deg  (cos^2 = 0.5000017 ~ 1/2)
//!   down:   theta = 48.1234 deg
//!   up:     theta = 52.2058 deg
//! and the identity cos^2(theta) = 1/(3Q_observed) holds to machine precision
//! in all three sectors. With the framework Q values {2/3, 3/4, 8/9} the
//! angles are {45.0000, 48.1897, 52.2388} deg.
//!
//! HONEST CAVEAT: this validates a geometric equivalence, not a derivation of
//! why nature chooses theta = 45 degrees for charged leptons.

use crate::Check;
use crate::mass_convention::{down_masses_mz, q_from_masses, up_masses_mz};
use std::f64::consts::PI;

fn masses(n: i32) -> [f64;3] {
    match n { 1 => [0.51099895,105.6583755,1776.93], 2 => down_masses_mz(), _ => up_masses_mz() }
}

/// Angle (radians) between sqrt-mass vector and (1,1,1).
pub fn theta(n: i32) -> f64 {
    let m = masses(n);
    let v: Vec<f64> = m.iter().map(|x| x.sqrt()).collect();
    let mag: f64 = v.iter().map(|x| x*x).sum::<f64>().sqrt();
    let s: f64 = v.iter().sum();
    (s / (mag * 3.0_f64.sqrt())).clamp(-1.0,1.0).acos()
}

/// Foot's identity: cos^2(theta) = 1/(3Q).
pub fn foot_rhs(n: i32) -> f64 { 1.0 / (3.0 * q_from_masses(masses(n))) }

pub fn cos2(n: i32) -> f64 { theta(n).cos().powi(2) }

/// Framework angle from Q = {2/3, 3/4, 8/9}.
pub fn framework_theta_deg(n: i32) -> f64 {
    let q = match n { 1 => 2.0/3.0, 2 => 3.0/4.0, _ => 8.0/9.0 };
    (1.0_f64/(3.0*q)).sqrt().acos() * 180.0/PI
}

pub fn checks() -> Vec<Check> {
    let mut out = vec![
        Check::new("foot.lepton_angle_deg", theta(1)*180.0/PI, "lepton angle ~45 deg"),
    ];
    for n in 1..=3 {
        out.push(Check::new(
            match n {1=>"foot.identity_lep",2=>"foot.identity_down",_=>"foot.identity_up"},
            cos2(n) - foot_rhs(n), "cos^2(theta) - 1/(3Q) ~ 0"
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foot_identity_is_exact() {
        for n in 1..=3 {
            assert!((cos2(n) - foot_rhs(n)).abs() < 1e-12, "sector {n}");
        }
    }

    #[test]
    fn lepton_angle_is_45_degrees() {
        assert!((theta(1)*180.0/PI - 45.0).abs() < 0.01);
    }

    #[test]
    fn framework_angles() {
        assert!((framework_theta_deg(1) - 45.0).abs() < 1e-9);
        assert!((framework_theta_deg(2) - 48.1897).abs() < 1e-3);
        assert!((framework_theta_deg(3) - 52.2388).abs() < 1e-3);
    }
}
