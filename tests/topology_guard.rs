use fluid_universe::simulation::{Field3, analytic_field};
use fluid_universe::topology_guard::homotopy_hemisphere_margin;

fn uniform(v: [f64; 3]) -> Field3 {
    Field3 { n: 2, dx: 1.0, data: vec![v; 8] }
}

#[test]
fn mean_direction_guard_certifies_wide_rotation_and_rejects_degenerate_input() {
    use fluid_universe::topology_guard::homotopy_mean_margin;
    let a = uniform([1.0, 0.0, 0.0]);
    let b = uniform([-0.5, 3.0_f64.sqrt() / 2.0, 0.0]);
    assert!((homotopy_mean_margin(&a, &b).unwrap() - 0.5).abs() < 1e-12);
    assert!(homotopy_mean_margin(&a, &uniform([-1.0, 0.0, 0.0])).unwrap() <= 0.0);
    assert!(homotopy_mean_margin(&a, &uniform([0.0; 3])).is_err());
}

#[test]
fn hemisphere_guard_certifies_and_remains_conservative() {
    let a = uniform([1.0, 0.0, 0.0]);
    let close = uniform([0.5, 3.0_f64.sqrt() / 2.0, 0.0]);
    assert!((homotopy_hemisphere_margin(&a, &close).unwrap() - 0.5).abs() < 1e-12);
    let wide = uniform([-0.5, 3.0_f64.sqrt() / 2.0, 0.0]);
    assert!(homotopy_hemisphere_margin(&a, &wide).unwrap() < 0.0);
    // The 120-degree linear path is nonzero: failure is only inconclusive.
    assert!(((a.data[0][0] + wide.data[0][0]).hypot(wide.data[0][1])) > 0.0);
}

#[test]
fn hemisphere_guard_rejects_invalid_samples_and_incompatible_grids() {
    let a = uniform([1.0, 0.0, 0.0]);
    assert!(homotopy_hemisphere_margin(&a, &uniform([0.0; 3])).is_err());
    assert!(homotopy_hemisphere_margin(&a, &uniform([f64::NAN, 0.0, 0.0])).is_err());
    let mut b = a.clone();
    b.dx = 2.0;
    assert!(homotopy_hemisphere_margin(&a, &b).is_err());
    b = a.clone();
    b.data.pop();
    assert!(homotopy_hemisphere_margin(&a, &b).is_err());
}

#[test]
fn analytic_grid_and_small_rotation_have_positive_margin_at_32() {
    let coarse = analytic_field(16, 8.0, false);
    assert!(homotopy_hemisphere_margin(&coarse, &coarse).unwrap() < 0.0);
    let field = analytic_field(32, 8.0, false);
    let mut rotated = field.clone();
    let (sin, cos) = 0.01_f64.sin_cos();
    for v in &mut rotated.data {
        *v = [cos * v[0] - sin * v[1], sin * v[0] + cos * v[1], v[2]];
    }
    let margin = homotopy_hemisphere_margin(&field, &rotated).unwrap();
    assert!((margin - 0.00892819001238837).abs() < 1e-12);
}

#[test]
fn monitored_steps_match_existing_relaxation_and_report_inconclusive_cells() {
    use fluid_universe::topology_guard::monitored_relaxation;
    let initial = analytic_field(8, 8.0, false);
    let monitored = monitored_relaxation(&initial, 0.0, 0.05, 3, 1.0, 1.0).unwrap();
    let (direct, history, _) = fluid_universe::simulation::relax(&initial, 0.0, 0.05, 3, 1.0, 1.0);
    assert_eq!(monitored.completed_steps, history.len() - 1);
    for (a, b) in monitored.field.data.iter().zip(&direct.data) {
        for i in 0..3 { assert!((a[i] - b[i]).abs() < 1e-14); }
    }
    assert!(monitored.inconclusive_steps > 0);
    assert!(monitored.minimum_margin.unwrap() <= 0.0);
    assert_eq!(monitored.first_inconclusive_step, Some(1));
    if monitored.mean_inconclusive_steps > 0 { assert_eq!(monitored.mean_first_inconclusive_step, Some(1)); }
    assert!(monitored_relaxation(&initial, 0.0, f64::NAN, 1, 1.0, 1.0).is_err());
}

#[test]
fn monitored_stationary_field_does_not_claim_a_certified_step() {
    use fluid_universe::topology_guard::monitored_relaxation;
    let result = monitored_relaxation(&uniform([0.0, 0.0, 1.0]), 0.0, 0.05, 3, 1.0, 1.0).unwrap();
    assert_eq!(result.completed_steps, 0);
    assert_eq!(result.minimum_margin, None);
    assert_eq!(result.first_inconclusive_step, None);
    assert_eq!(result.mean_first_inconclusive_step, None);
}
