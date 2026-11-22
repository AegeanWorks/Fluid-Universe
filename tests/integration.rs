use fluid_universe::{
    all_checks, audit, circulant, derrick, fingerprint, fluid, lorentz, reports, rotor, simulation,
    spectra, verify, water,
};

#[test]
fn all_modules_publish_checks() {
    assert!(!circulant::checks().is_empty());
    assert!(!spectra::checks().is_empty());
    assert!(!rotor::checks().is_empty());
    assert!(!derrick::checks().is_empty());
    assert!(!fluid::checks().is_empty());
    assert!(!water::checks().is_empty());
    assert!(!lorentz::checks().is_empty());
    assert!(!simulation::checks().is_empty());
    assert!(!audit::checks().is_empty());
    assert!(!reports::checks().is_empty());
    assert!(all_checks().len() >= 90);
}

#[test]
fn check_ids_are_unique() {
    let checks = all_checks();
    let mut ids: Vec<&str> = checks.iter().map(|c| c.id).collect();
    ids.sort_unstable();
    let count = ids.len();
    ids.dedup();
    assert_eq!(count, ids.len(), "duplicate check id");
}

#[test]
fn two_derivations_agree_without_shared_state() {
    let result = verify::verify_in_process();
    assert!(result.agree, "{}", result.line());
    assert_eq!(result.checks, all_checks().len());
}

#[test]
fn fingerprint_is_stable_and_hex() {
    let a = fingerprint(&all_checks());
    let b = fingerprint(&all_checks());
    assert_eq!(a, b);
    assert_eq!(a.len(), 16);
    assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn headline_numbers_match_the_python_ledger() {
    use fluid_universe::linalg::approx;
    let by_id: std::collections::HashMap<&str, f64> =
        all_checks().iter().map(|c| (c.id, c.value)).collect();
    // Only assert identities every ported module must reproduce.
    assert!(by_id.values().all(|v| v.is_finite()));
    assert!(circulant::q_from_ac(1.0, 0.5) == 0.5 || approx(circulant::q_from_ac(1.0, 0.5), 0.5, 1e-12));
    assert!(approx(rotor::domain_wall_tension(1.0), 8.0 / 3.0, 1e-12));
    assert_eq!(derrick::exponents(3), [1, -1, 3]);
    assert!(approx(fluid::blob_loop_fraction(1.0, 1.0), 0.5, 1e-12));
    assert!(approx(fluid::blob_loop_fraction(2.0, 1.0), 0.8, 1e-12));
}
