use fluid_universe::linalg::approx;
use fluid_universe::radial_shell::{
    GRIDS, LENGTH, checks, core_radius, fd_q_zero, node_position, phase, phase_slip,
    phase_slip_margin, q_zero, raw_norm_closed_form, raw_norm_numeric, run,
    singular_gradient_max, velocity, velocity_max,
};
use std::f64::consts::PI;

#[test]
fn scale_and_node_are_predicted() {
    assert_eq!(core_radius(), 0.1);
    assert_eq!(node_position(1.0), 2.0);
}

#[test]
fn phase_is_monotone_with_the_pre_registered_deficit_bound() {
    let eps = core_radius();
    let mut previous = PI;
    for i in 0..=4000 {
        let r = LENGTH * i as f64 / 4000.0;
        let current = phase(r, eps, 1.0);
        assert!(current <= previous + 1e-12);
        assert!(approx(current, (eps).atan2(r - 2.0), 1e-12));
        previous = current;
    }
    let slip = phase_slip(eps, 1.0, LENGTH);
    assert!(slip > 0.0 && slip < PI);
    assert!(PI - slip <= phase_slip_margin(eps, 1.0, LENGTH));
}

#[test]
fn singular_control_retains_inverse_grid_spacing() {
    let dx: Vec<f64> = GRIDS.iter().map(|&n| LENGTH / n as f64).collect();
    let gmax: Vec<f64> = GRIDS
        .iter()
        .map(|&n| singular_gradient_max(n, LENGTH, 1.0))
        .collect();
    let mx = dx.iter().map(|v| v.ln()).sum::<f64>() / dx.len() as f64;
    let my = gmax.iter().map(|v| v.ln()).sum::<f64>() / gmax.len() as f64;
    let slope = dx
        .iter()
        .zip(&gmax)
        .map(|(x, y)| (x.ln() - mx) * (y.ln() - my))
        .sum::<f64>()
        / dx.iter().map(|x| (x.ln() - mx).powi(2)).sum::<f64>();
    assert!(approx(slope, -1.0, 0.01));
    assert!(gmax[3] > gmax[0]);
}

#[test]
fn velocity_and_quantum_pressure_are_finite_and_converged() {
    let eps = core_radius();
    assert!(approx(velocity_max(eps, 1.0).abs(), 10.0, 1e-12));
    assert!(approx(velocity(2.0, eps, 1.0, 1.0).abs(), 10.0, 1e-12));
    let analytic = q_zero(eps, 1.0, 1.0, 1.0);
    assert!(approx(analytic, -49.875, 1e-12));
    let fine = fd_q_zero(eps, GRIDS[3], LENGTH, 1.0);
    let previous = fd_q_zero(eps, GRIDS[2], LENGTH, 1.0);
    assert!((fine - analytic).abs() / analytic.abs() <= 0.05);
    assert!((fine - previous).abs() / previous.abs() <= 0.05);
}

#[test]
fn raw_norm_matches_the_closed_form() {
    let eps = core_radius();
    let expected = raw_norm_closed_form(eps, 1.0);
    let numeric = raw_norm_numeric(eps, 1.0);
    assert!((numeric - expected).abs() / expected <= 1e-8);
}

#[test]
fn report_and_checks_follow_the_frozen_gates() {
    let report = run();
    assert_eq!(report.loop_circulation, 0.0);
    assert!(report.verdict().starts_with("NUMERIC"));
    let rows = checks();
    assert!(!rows.is_empty());
    let mut ids = std::collections::HashSet::new();
    for row in &rows {
        assert!(row.id.starts_with("radial_shell."));
        assert!(row.value.is_finite());
        assert!(ids.insert(row.id));
    }
    assert_eq!(rows, checks());
}
