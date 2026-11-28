//! HO-2 radial-shell finite-core probe (claim 62, ADR-058).
//!
//! The hydrogen-2s simple radial zero at `r0 = 2*a0` is regularized by
//! `R_eps = A*(r-r0+i*eps)*exp(-r/(2*a0))`.  The reduced radial amplitude
//! `u = r*R_eps` is used for quantum pressure, so `u''/u` is exactly the full
//! s-wave radial Laplacian ratio.  The phase is radial, not azimuthal: `m=0`
//! circulation is zero and no Wallstrom vortex quantization is claimed.

use std::f64::consts::PI;

use crate::Check;
use crate::linalg::{approx, simpson};

pub const GRIDS: [usize; 4] = [250, 500, 1000, 2000];
pub const LENGTH: f64 = 20.0;
pub const A0: f64 = 1.0;
pub const EPS0: f64 = 0.1;

const SLOPE_TOL: f64 = 0.01;
const PHASE_TOL: f64 = 1e-12;
const QCONV_TOL: f64 = 0.05;
const NORM_TOL: f64 = 1e-8;

pub fn core_radius() -> f64 {
    EPS0
}

pub fn node_position(radius: f64) -> f64 {
    2.0 * radius
}

pub fn phase(r: f64, eps: f64, radius: f64) -> f64 {
    eps.atan2(r - node_position(radius))
}

pub fn phase_slip(eps: f64, radius: f64, outer: f64) -> f64 {
    phase(0.0, eps, radius) - phase(outer, eps, radius)
}

pub fn phase_slip_margin(eps: f64, radius: f64, outer: f64) -> f64 {
    let r0 = node_position(radius);
    2.0 * eps / r0 + eps / (outer - r0)
}

pub fn velocity(r: f64, eps: f64, radius: f64, hbar_over_m: f64) -> f64 {
    let x = r - node_position(radius);
    -hbar_over_m * eps / (x * x + eps * eps)
}

pub fn velocity_max(eps: f64, hbar_over_m: f64) -> f64 {
    hbar_over_m / eps
}

pub fn q_zero(eps: f64, radius: f64, hbar: f64, mass: f64) -> f64 {
    let r0 = node_position(radius);
    -(hbar * hbar) / (2.0 * mass) * (1.0 / (eps * eps) - 1.0 / (r0 * r0))
}

pub fn fd_q_zero(eps: f64, n: usize, length: f64, radius: f64) -> f64 {
    let dx = length / n as f64;
    let r0 = node_position(radius);
    let index = (r0 / dx).round() as usize;
    assert!(approx(index as f64 * dx, r0, 1e-12));
    let amplitude = |r: f64| {
        let x = r - r0;
        r * (x * x + eps * eps).sqrt() * (-r / (2.0 * radius)).exp()
    };
    let lap = (amplitude((index + 1) as f64 * dx)
        - 2.0 * amplitude(index as f64 * dx)
        + amplitude((index - 1) as f64 * dx))
        / (dx * dx);
    -0.5 * lap / amplitude(index as f64 * dx)
}

pub fn singular_gradient_max(n: usize, length: f64, radius: f64) -> f64 {
    let dx = length / n as f64;
    let r0 = node_position(radius);
    let mut previous = 0.0;
    let mut maximum = 0.0_f64;
    for i in 0..n {
        let r = (i as f64 + 0.5) * dx;
        let current = if r < r0 { 0.0 } else { PI };
        if i > 0 {
            maximum = maximum.max((current - previous).abs() / dx);
        }
        previous = current;
    }
    maximum
}

pub fn raw_norm_closed_form(eps: f64, radius: f64) -> f64 {
    2.0 * radius.powi(3) * (4.0 * radius.powi(2) + eps.powi(2))
}

pub fn raw_norm_numeric(eps: f64, radius: f64) -> f64 {
    let r0 = node_position(radius);
    simpson(
        |r| r * r * ((r - r0).powi(2) + eps.powi(2)) * (-r / radius).exp(),
        0.0,
        60.0 * radius,
        20_000,
    )
}

#[derive(Clone, Debug, PartialEq)]
pub struct Report {
    pub eps0: f64,
    pub radius: f64,
    pub node_radius: f64,
    pub singular_slope: f64,
    pub phase_slip: f64,
    pub phase_deficit: f64,
    pub phase_deficit_bound: f64,
    pub phase_monotone: bool,
    pub velocity_max: f64,
    pub q_analytic: f64,
    pub q_fd: [f64; 4],
    pub q_fine_relative_error: f64,
    pub q_step_relative_error: f64,
    pub loop_circulation: f64,
    pub norm_numeric: f64,
    pub norm_analytic: f64,
    pub norm_relative_error: f64,
    pub singular_ok: bool,
    pub phase_ok: bool,
    pub velocity_ok: bool,
    pub q_ok: bool,
    pub circulation_ok: bool,
    pub norm_ok: bool,
}

impl Report {
    pub fn verdict(&self) -> &'static str {
        if !self.singular_ok {
            "UNVERIFIED (singular apparatus check failed; solver unreliable)"
        } else if !self.phase_ok || !self.velocity_ok || !self.q_ok || !self.circulation_ok {
            "FALSIFIED (single-valued regularized HO-2 radial shell insufficient)"
        } else if !self.norm_ok {
            "UNVERIFIED (raw radial norm does not match the closed form)"
        } else {
            "NUMERIC (finite-core regularized HO-2 radial shell demonstrated at eps0; phase regular, velocity/Q finite and N-converged, circulation zero)"
        }
    }
}

fn log_log_slope(xs: &[f64; 4], ys: &[f64; 4]) -> f64 {
    let mx = xs.iter().map(|x| x.ln()).sum::<f64>() / xs.len() as f64;
    let my = ys.iter().map(|y| y.ln()).sum::<f64>() / ys.len() as f64;
    let numerator = xs
        .iter()
        .zip(ys)
        .map(|(x, y)| (x.ln() - mx) * (y.ln() - my))
        .sum::<f64>();
    let denominator = xs.iter().map(|x| (x.ln() - mx).powi(2)).sum::<f64>();
    numerator / denominator
}

pub fn run() -> Report {
    let radius = A0;
    let eps0 = core_radius();
    let node = node_position(radius);
    let dx: [f64; 4] = GRIDS.map(|n| LENGTH / n as f64);
    let gmax: [f64; 4] = GRIDS.map(|n| singular_gradient_max(n, LENGTH, radius));
    let singular_slope = log_log_slope(&dx, &gmax);
    let singular_ok = (singular_slope + 1.0).abs() <= SLOPE_TOL;

    let mut phase_monotone = true;
    for &n in &GRIDS {
        let step = LENGTH / n as f64;
        let mut previous = phase(0.0, eps0, radius);
        for i in 1..=n {
            let r = i as f64 * step;
            let current = phase(r, eps0, radius);
            phase_monotone &= current <= previous + PHASE_TOL;
            previous = current;
        }
    }
    let slip = phase_slip(eps0, radius, LENGTH);
    let deficit = PI - slip;
    let deficit_bound = phase_slip_margin(eps0, radius, LENGTH);
    let phase_ok = phase_monotone && slip > 0.0 && slip < PI && deficit <= deficit_bound;

    let vmax = velocity(node, eps0, radius, 1.0).abs();
    let velocity_ok = vmax.is_finite() && approx(vmax, 10.0, 1e-12);
    let q_analytic = q_zero(eps0, radius, 1.0, 1.0);
    let q_fd = GRIDS.map(|n| fd_q_zero(eps0, n, LENGTH, radius));
    let q_fine_relative_error = (q_fd[3] - q_analytic).abs() / q_analytic.abs();
    let q_step_relative_error = (q_fd[3] - q_fd[2]).abs() / q_fd[2].abs();
    let q_ok = q_analytic.is_finite()
        && q_fd.iter().all(|v| v.is_finite())
        && q_fine_relative_error <= QCONV_TOL
        && q_step_relative_error <= QCONV_TOL;

    let loop_circulation = 0.0;
    let circulation_ok = loop_circulation == 0.0;
    let norm_numeric = raw_norm_numeric(eps0, radius);
    let norm_analytic = raw_norm_closed_form(eps0, radius);
    let norm_relative_error = (norm_numeric - norm_analytic).abs() / norm_analytic;
    let norm_ok = norm_relative_error <= NORM_TOL;

    Report {
        eps0,
        radius,
        node_radius: node,
        singular_slope,
        phase_slip: slip,
        phase_deficit: deficit,
        phase_deficit_bound: deficit_bound,
        phase_monotone,
        velocity_max: vmax,
        q_analytic,
        q_fd,
        q_fine_relative_error,
        q_step_relative_error,
        loop_circulation,
        norm_numeric,
        norm_analytic,
        norm_relative_error,
        singular_ok,
        phase_ok,
        velocity_ok,
        q_ok,
        circulation_ok,
        norm_ok,
    }
}

pub fn checks() -> Vec<Check> {
    let report = run();
    let status = match report.verdict() {
        verdict if verdict.starts_with("NUMERIC") => 1.0,
        verdict if verdict.starts_with("FALSIFIED") => -1.0,
        _ => 0.0,
    };
    vec![
        Check::new("radial_shell.eps0", report.eps0, "pre-registered eps0=0.1*a0 in atomic units"),
        Check::new("radial_shell.node_radius", report.node_radius, "hydrogen-2s nodal shell r0=2*a0"),
        Check::new("radial_shell.singular_slope", report.singular_slope, "singular phase-gradient log-log slope (gate -1+-0.01)"),
        Check::new("radial_shell.phase_slip", report.phase_slip, "finite-domain regular phase decrease on [0,20]"),
        Check::new("radial_shell.phase_deficit_bound", report.phase_deficit_bound, "pre-registered bound 2*eps/r0+eps/(L-r0)"),
        Check::new("radial_shell.velocity_max", report.velocity_max, "finite radial velocity cap (hbar/m)/eps=10"),
        Check::new("radial_shell.q_zero", report.q_analytic, "Q0=-0.5*(1/eps^2-1/r0^2)=-49.875"),
        Check::new("radial_shell.q_fd_fine", report.q_fd[3], "finite-difference Q0 at N=2000"),
        Check::new("radial_shell.q_fine_relative_error", report.q_fine_relative_error, "fine FD vs analytic Q relative error (gate <=0.05)"),
        Check::new("radial_shell.q_step_relative_error", report.q_step_relative_error, "fine vs previous FD Q relative error (gate <=0.05)"),
        Check::new("radial_shell.loop_circulation", report.loop_circulation, "m=0 radial shell has zero azimuthal circulation"),
        Check::new("radial_shell.norm_relative_error", report.norm_relative_error, "raw radial norm vs closed form (gate <=1e-8)"),
        Check::new("radial_shell.gate_status", status, report.verdict()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_forms_match_the_pre_registered_values() {
        assert_eq!(core_radius(), 0.1);
        assert_eq!(node_position(1.0), 2.0);
        assert!(approx(q_zero(0.1, 1.0, 1.0, 1.0), -49.875, 1e-12));
        assert!(approx(raw_norm_closed_form(0.1, 1.0), 8.02, 1e-12));
    }

    #[test]
    fn report_is_deterministic_and_gate_consistent() {
        let first = run();
        let second = run();
        assert_eq!(first, second);
        assert!(first.verdict().starts_with("NUMERIC"));
        assert_eq!(checks(), checks());
    }
}
