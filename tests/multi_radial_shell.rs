use fluid_universe::linalg::approx;
use fluid_universe::multi_radial_shell::{
    GRIDS, LENGTH, LOCAL_LENGTH, checks, fd_q_at_node, nodes, phase, phase_slip,
    phase_slip_margin, q_at_node, raw_norm_closed_form, raw_norm_numeric, run,
    singular_gradient_max, velocity, velocity_max,
};
use std::f64::consts::PI;

#[test]
fn nodes_are_predicted_from_the_3s_polynomial() {
    let expected = [
        (9.0 - 3.0 * 3.0_f64.sqrt()) / 2.0,
        (9.0 + 3.0 * 3.0_f64.sqrt()) / 2.0,
    ];
    assert_eq!(nodes(), expected);
    assert!(approx(nodes()[0], 1.901923788646684, 1e-15));
    assert!(approx(nodes()[1], 7.098076211353316, 1e-15));
}

#[test]
fn product_phase_is_monotone_with_the_frozen_deficit_bound() {
    let eps = 0.1;
    let mut previous = phase(0.0, eps);
    for i in 1..=4000 {
        let r = LENGTH * i as f64 / 4000.0;
        let current = phase(r, eps);
        assert!(current <= previous + 1e-12);
        previous = current;
    }
    let slip = phase_slip(eps, LENGTH);
    assert!(slip > 0.0 && slip < 2.0 * PI);
    assert!(approx(slip, 6.2032919230819665, 1e-12));
    assert!(2.0 * PI - slip <= phase_slip_margin(eps, LENGTH));
}

#[test]
fn both_singular_controls_retain_inverse_grid_spacing() {
    for node in nodes() {
        let dx: Vec<f64> = GRIDS.iter().map(|&n| LOCAL_LENGTH / n as f64).collect();
        let gmax: Vec<f64> = GRIDS
            .iter()
            .map(|&n| singular_gradient_max(node, n))
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
    }
}

#[test]
fn actual_velocity_is_finite_at_both_nodes() {
    let eps = 0.1;
    assert!(approx(velocity_max(eps).abs(), 10.003702332469455, 1e-12));
    for node in nodes() {
        assert!(approx(velocity(node, eps).abs(), 10.003702332469455, 1e-12));
    }
}

#[test]
fn quantum_pressure_is_finite_and_converged_at_both_nodes() {
    let eps = 0.1;
    let expected = [-49.843277943472785, -49.971578003292700];
    for (index, node) in nodes().into_iter().enumerate() {
        let analytic = q_at_node(eps, node);
        assert!(approx(analytic, expected[index], 1e-9));
        let fine = fd_q_at_node(eps, node, GRIDS[3]);
        let previous = fd_q_at_node(eps, node, GRIDS[2]);
        assert!((fine - analytic).abs() / analytic.abs() <= 0.05);
        assert!((fine - previous).abs() / previous.abs() <= 0.05);
    }
}

#[test]
fn raw_norm_matches_the_closed_form() {
    let eps = 0.1;
    let expected = raw_norm_closed_form(eps);
    let numeric = raw_norm_numeric(eps);
    assert!(approx(expected, 1232.010675, 1e-9));
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
        assert!(row.id.starts_with("multi_radial_shell."));
        assert!(row.value.is_finite());
        assert!(ids.insert(row.id));
    }
    assert_eq!(rows, checks());
}
