//! HO-3 double-radial-shell finite-core probe (claim 63, ADR-059).
//!
//! Hydrogen 3s has two simple radial zeros from `L_2^1(x)=x^2/2-3*x+3`.
//! Both are regularized together by
//! `R_eps ∝ ((r-r1)+i*eps)*((r-r2)+i*eps)*exp(-r/(3*a0))`.
//! The reduced radial amplitude `u=r*R_eps` supplies the full s-wave radial
//! Laplacian ratio.  The product phase is radial, not azimuthal: `m=0`
//! circulation is zero and no vortex quantization is claimed.

use std::f64::consts::PI;

use crate::Check;
use crate::linalg::{approx, simpson};

pub const GRIDS: [usize; 4] = [250, 500, 1000, 2000];
pub const LENGTH: f64 = 20.0;
pub const LOCAL_LENGTH: f64 = 2.0;
pub const A0: f64 = 1.0;
pub const EPS0: f64 = 0.1;

const SLOPE_TOL: f64 = 0.01;
const PHASE_TOL: f64 = 1e-12;
const QCONV_TOL: f64 = 0.05;
const NORM_TOL: f64 = 1e-8;

pub fn nodes() -> [f64; 2] {
    let offset = 3.0 * 3.0_f64.sqrt() / 2.0;
    [9.0 / 2.0 - offset, 9.0 / 2.0 + offset]
}

pub fn phase(r: f64, eps: f64) -> f64 {
    let [r1, r2] = nodes();
    eps.atan2(r - r1) + eps.atan2(r - r2)
}

pub fn phase_slip(eps: f64, outer: f64) -> f64 {
    phase(0.0, eps) - phase(outer, eps)
}

pub fn phase_slip_margin(eps: f64, outer: f64) -> f64 {
    let [r1, r2] = nodes();
    2.0 * eps / r1
        + eps / (outer - r1)
        + 2.0 * eps / r2
        + eps / (outer - r2)
}

pub fn velocity(r: f64, eps: f64) -> f64 {
    let [r1, r2] = nodes();
    let x1 = r - r1;
    let x2 = r - r2;
    -eps * (1.0 / (x1 * x1 + eps * eps) + 1.0 / (x2 * x2 + eps * eps))
}

pub fn velocity_max(eps: f64) -> f64 {
    let [r1, r2] = nodes();
    let separation = r2 - r1;
    1.0 / eps + eps / (separation * separation + eps * eps)
}

fn log_derivatives(r: f64, eps: f64) -> (f64, f64) {
    let [r1, r2] = nodes();
    let x1 = r - r1;
    let x2 = r - r2;
    let d1 = x1 * x1 + eps * eps;
    let d2 = x2 * x2 + eps * eps;
    let first = 1.0 / r - 1.0 / 3.0 + x1 / d1 + x2 / d2;
    let second = -1.0 / (r * r)
        + (eps * eps - x1 * x1) / (d1 * d1)
        + (eps * eps - x2 * x2) / (d2 * d2);
    (first, second)
}

pub fn q_at_node(eps: f64, node: f64) -> f64 {
    let (first, second) = log_derivatives(node, eps);
    -0.5 * (second + first * first)
}

pub fn fd_q_at_node(eps: f64, node: f64, n: usize) -> f64 {
    let dx = LOCAL_LENGTH / n as f64;
    let [r1, r2] = nodes();
    let amplitude = |r: f64| {
        let x1 = r - r1;
        let x2 = r - r2;
        r * (x1 * x1 + eps * eps).sqrt()
            * (x2 * x2 + eps * eps).sqrt()
            * (-r / 3.0).exp()
    };
    let center = amplitude(node);
    let left = amplitude(node - dx);
    let right = amplitude(node + dx);
    -0.5 * (right - 2.0 * center + left) / (dx * dx) / center
}

pub fn singular_gradient_max(node: f64, n: usize) -> f64 {
    let dx = LOCAL_LENGTH / n as f64;
    let [r1, r2] = nodes();
    let mut previous = 0.0;
    let mut maximum = 0.0_f64;
    for i in 0..n {
        let x = (i as f64 - n as f64 / 2.0 + 0.5) * dx;
        let r = node + x;
        let a1 = if r - r1 >= 0.0 { 0.0 } else { PI };
        let a2 = if r - r2 >= 0.0 { 0.0 } else { PI };
        let current = a1 + a2;
        if i > 0 {
            maximum = maximum.max((current - previous).abs() / dx);
        }
        previous = current;
    }
    maximum
}

pub fn raw_norm_closed_form(eps: f64) -> f64 {
    27.0 * eps.powi(4) / 4.0 + 729.0 * eps.powi(2) / 4.0 + 19683.0 / 16.0
}

pub fn raw_norm_numeric(eps: f64) -> f64 {
    let [r1, r2] = nodes();
    simpson(
        |r| {
            r * r
                * ((r - r1).powi(2) + eps.powi(2))
                * ((r - r2).powi(2) + eps.powi(2))
                * (-2.0 * r / 3.0).exp()
        },
        0.0,
        60.0,
        20_000,
    )
}

#[derive(Clone, Debug, PartialEq)]
pub struct Report {
    pub eps0: f64,
    pub nodes: [f64; 2],
    pub singular_slopes: [f64; 2],
    pub phase_slip: f64,
    pub phase_deficit: f64,
    pub phase_deficit_bound: f64,
    pub phase_monotone: bool,
    pub velocity_at_nodes: [f64; 2],
    pub velocity_cap: f64,
    pub q_analytic: [f64; 2],
    pub q_fd: [[f64; 4]; 2],
    pub q_fine_relative_error: [f64; 2],
    pub q_step_relative_error: [f64; 2],
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
            "UNVERIFIED (a singular apparatus check failed; solver unreliable)"
        } else if !self.phase_ok || !self.velocity_ok || !self.q_ok || !self.circulation_ok {
            "FALSIFIED (single-valued regularized HO-3 double radial shell insufficient)"
        } else if !self.norm_ok {
            "UNVERIFIED (raw radial norm does not match the closed form)"
        } else {
            "NUMERIC (finite-core regularized HO-3 double radial shell demonstrated at eps0; both phases regular, velocity/Q finite and N-converged, circulation zero)"
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
    let eps0 = EPS0;
    let shell_nodes = nodes();
    let dx: [f64; 4] = GRIDS.map(|n| LOCAL_LENGTH / n as f64);
    let mut singular_slopes = [0.0; 2];
    for (index, node) in shell_nodes.into_iter().enumerate() {
        let gmax: [f64; 4] = GRIDS.map(|n| singular_gradient_max(node, n));
        singular_slopes[index] = log_log_slope(&dx, &gmax);
    }
    let singular_ok = singular_slopes
        .iter()
        .all(|slope| (slope + 1.0).abs() <= SLOPE_TOL);

    let mut phase_monotone = true;
    for &n in &GRIDS {
        let step = LENGTH / n as f64;
        let mut previous = phase(0.0, eps0);
        for i in 1..=n {
            let current = phase(i as f64 * step, eps0);
            phase_monotone &= current <= previous + PHASE_TOL;
            previous = current;
        }
    }
    let slip = phase_slip(eps0, LENGTH);
    let deficit = 2.0 * PI - slip;
    let deficit_bound = phase_slip_margin(eps0, LENGTH);
    let phase_ok = phase_monotone && slip > 0.0 && slip < 2.0 * PI && deficit <= deficit_bound;

    let velocity_at_nodes = shell_nodes.map(|node| velocity(node, eps0).abs());
    let velocity_cap = velocity_max(eps0);
    let velocity_ok = velocity_at_nodes
        .iter()
        .all(|value| value.is_finite() && approx(*value, velocity_cap, 1e-12));

    let q_analytic = shell_nodes.map(|node| q_at_node(eps0, node));
    let mut q_fd = [[0.0; 4]; 2];
    let mut q_fine_relative_error = [0.0; 2];
    let mut q_step_relative_error = [0.0; 2];
    for (index, node) in shell_nodes.into_iter().enumerate() {
        for (grid_index, &n) in GRIDS.iter().enumerate() {
            q_fd[index][grid_index] = fd_q_at_node(eps0, node, n);
        }
        q_fine_relative_error[index] =
            (q_fd[index][3] - q_analytic[index]).abs() / q_analytic[index].abs();
        q_step_relative_error[index] =
            (q_fd[index][3] - q_fd[index][2]).abs() / q_fd[index][2].abs();
    }
    let q_ok = q_analytic.iter().all(|value| value.is_finite())
        && q_fd.iter().flatten().all(|value| value.is_finite())
        && q_fine_relative_error.iter().all(|value| *value <= QCONV_TOL)
        && q_step_relative_error.iter().all(|value| *value <= QCONV_TOL);

    let loop_circulation = 0.0;
    let circulation_ok = loop_circulation == 0.0;
    let norm_numeric = raw_norm_numeric(eps0);
    let norm_analytic = raw_norm_closed_form(eps0);
    let norm_relative_error = (norm_numeric - norm_analytic).abs() / norm_analytic;
    let norm_ok = norm_relative_error <= NORM_TOL;

    Report {
        eps0,
        nodes: shell_nodes,
        singular_slopes,
        phase_slip: slip,
        phase_deficit: deficit,
        phase_deficit_bound: deficit_bound,
        phase_monotone,
        velocity_at_nodes,
        velocity_cap,
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
        Check::new("multi_radial_shell.node_inner", report.nodes[0], "inner 3s node (9-3*sqrt(3))*a0/2"),
        Check::new("multi_radial_shell.node_outer", report.nodes[1], "outer 3s node (9+3*sqrt(3))*a0/2"),
        Check::new("multi_radial_shell.singular_slope_inner", report.singular_slopes[0], "inner-node singular phase-gradient slope"),
        Check::new("multi_radial_shell.singular_slope_outer", report.singular_slopes[1], "outer-node singular phase-gradient slope"),
        Check::new("multi_radial_shell.phase_slip", report.phase_slip, "full-domain product-phase decrease"),
        Check::new("multi_radial_shell.phase_deficit_bound", report.phase_deficit_bound, "sum of the two pre-registered single-node bounds"),
        Check::new("multi_radial_shell.velocity_cap", report.velocity_cap, "actual finite radial velocity magnitude at both nodes"),
        Check::new("multi_radial_shell.q_inner", report.q_analytic[0], "inner-node Q0 from u=r*R_eps"),
        Check::new("multi_radial_shell.q_outer", report.q_analytic[1], "outer-node Q0 from u=r*R_eps"),
        Check::new("multi_radial_shell.q_fine_error_inner", report.q_fine_relative_error[0], "inner-node fine FD vs analytic Q error"),
        Check::new("multi_radial_shell.q_fine_error_outer", report.q_fine_relative_error[1], "outer-node fine FD vs analytic Q error"),
        Check::new("multi_radial_shell.q_step_error_inner", report.q_step_relative_error[0], "inner-node fine vs previous FD Q error"),
        Check::new("multi_radial_shell.q_step_error_outer", report.q_step_relative_error[1], "outer-node fine vs previous FD Q error"),
        Check::new("multi_radial_shell.loop_circulation", report.loop_circulation, "m=0 radial shell has zero azimuthal circulation"),
        Check::new("multi_radial_shell.norm_relative_error", report.norm_relative_error, "raw radial norm vs closed form"),
        Check::new("multi_radial_shell.gate_status", status, report.verdict()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_forms_match_the_pre_registered_values() {
        assert!(approx(q_at_node(0.1, nodes()[0]), -49.843277943472785, 1e-9));
        assert!(approx(q_at_node(0.1, nodes()[1]), -49.971578003292700, 1e-9));
        assert!(approx(raw_norm_closed_form(0.1), 1232.010675, 1e-9));
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
