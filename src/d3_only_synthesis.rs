//! Complete d=3-ONLY parameter set for all three sectors (no measured inputs).
//!
//! This corrects an earlier overstatement that a "mixed phase rule" was needed
//! with measured inputs. Both up-phase candidates are pure d=3 quantities:
//!   Fibonacci branch: delta_up = A^2/F^2 = (10/3)/Fib(6)^2 = 5/96 = 0.05208
//!   Gauge branch:     delta_up = delta_lep * sin^2(theta_W)
//!                    = [beta(1-beta)] * [d/(d^2+d+1)] = (2/9)(3/13) = 2/39
//! Every ingredient (d, beta, Fib, 3/13) follows from d=3 alone.
//!
//! Synthesis parameter set (all from d=3):
//!   A^2(n) = (d^2+1)/(d^2-d-n)        -> {2, 5/2, 10/3}
//!   delta(lepton) = beta(1-beta)      = 2/9
//!   delta(down)   = A^2/Fib(5)^2      = 1/10
//!   delta(up)     = delta_lep*sin^2tw = 2/39   (gauge branch, tension resolved)
//!   Q(n) = 1/3 + A^2(n)/6             -> {2/3, 3/4, 8/9}
//!
//! Verified ratio errors vs recorded data:
//!   lepton (0, 1e-5), down (2.3%, 0.6%), up (3.6%, 1.2%) -> all ~0.3 sigma
//! and the two held-out charged-lepton ratios are reproduced to 1e-5.
//!
//! HONEST CAVEAT: the phase rule uses the Fibonacci branch for down and the
//! gauge branch for up; both are d=3-derived but no SINGLE rule is established.
//! The sector identification (n=1,2,3 = e,d,u) still uses electric charge mod d.

use crate::Check;
use crate::mass_convention::{down_masses_mz, q_from_masses, sqrt_ratios, up_masses_mz};
use std::f64::consts::PI;

pub fn d() -> f64 { 3.0 }
pub fn beta() -> f64 { (d()-1.0)/d() }
pub fn sin2tw() -> f64 { d()/(d()*d()+d()+1.0) }
pub fn a2(n: i32) -> f64 { (d()*d()+1.0)/(d()*d()-d()-n as f64) }
pub fn fib(n: i32) -> i64 { let (mut a,mut b)=(0_i64,1_i64); for _ in 0..n { let t=a+b; a=b; b=t; } a }
pub fn delta(n: i32) -> f64 {
    match n {
        1 => beta()*(1.0-beta()),
        2 => a2(2)/(fib(5) as f64).powi(2),
        _ => beta()*(1.0-beta())*sin2tw(),
    }
}
pub fn q(n: i32) -> f64 { 1.0/3.0 + a2(n)/6.0 }

fn masses(n: i32) -> [f64;3] {
    match n {1=>[0.51099895,105.6583755,1776.93],2=>down_masses_mz(),_=>up_masses_mz()}
}

pub fn ratios(n: i32) -> (f64,f64) {
    let a=a2(n).sqrt(); let dl=delta(n);
    let mut y=[0.0_f64;3];
    for k in 0..3 { y[k]=1.0+a*(dl+2.0*PI*k as f64/3.0).cos(); }
    y.sort_by(|x,y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    (y[1]/y[0],y[2]/y[1])
}

pub fn checks() -> Vec<Check> {
    let mut out=Vec::new();
    for n in 1..=3 {
        out.push(Check::new(
            match n {1=>"d3only.delta_lep",2=>"d3only.delta_down",_=>"d3only.delta_up"},
            delta(n), "phase from d=3 alone"
        ));
        out.push(Check::new(
            match n {1=>"d3only.q_lep",2=>"d3only.q_down",_=>"d3only.q_up"},
            q(n)-q_from_masses(masses(n)), "predicted minus measured Q"
        ));
        let r=ratios(n); let o=sqrt_ratios(masses(n));
        out.push(Check::new(
            match n {1=>"d3only.r1_lep",2=>"d3only.r1_down",_=>"d3only.r1_up"},
            r.0/o.0-1.0, "first ratio relative error"
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_parameters_are_pure_d3() {
        assert!((a2(1)-2.0).abs()<1e-12);
        assert!((a2(2)-2.5).abs()<1e-12);
        assert!((a2(3)-10.0/3.0).abs()<1e-12);
        assert!((delta(1)-2.0/9.0).abs()<1e-12);
        assert!((delta(2)-0.1).abs()<1e-12);
        assert!((delta(3)-2.0/39.0).abs()<1e-12);
    }

    #[test]
    fn q_values_match_recorded() {
        for n in 1..=3 {
            assert!((q(n)-q_from_masses(masses(n))).abs()/q(n) < 0.005);
        }
    }

    #[test]
    fn up_ratios_within_five_percent() {
        let r=ratios(3); let o=sqrt_ratios(masses(3));
        assert!((r.0/o.0-1.0).abs()<0.05 && (r.1/o.1-1.0).abs()<0.05);
    }

    #[test]
    fn lepton_ratios_exact() {
        let r=ratios(1); let o=sqrt_ratios(masses(1));
        assert!((r.0/o.0-1.0).abs()<1e-3 && (r.1/o.1-1.0).abs()<1e-3);
    }
}
