use fluid_universe::single_model_test::{braiding_fraction, search, single_model_exists};

#[test]
fn known_singlet_labels_are_exact_rationals() {
    let lepton = braiding_fraction(3, 9, &[0, 0]);
    let down = braiding_fraction(2, 13, &[0]);
    let up = braiding_fraction(3, 49, &[0, 0]);
    assert_eq!(lepton.0 * 9, 2 * lepton.1);
    assert_eq!(down.0 * 10, down.1);
    assert_eq!(up.0 * 39, 2 * up.1);
}

#[test]
fn bounded_search_reproduces_the_negative_and_coverage() {
    let result = search();
    assert_eq!(result.models_tested, 474);
    assert_eq!(result.labels_tested, 1_337_784);
    assert!(result.hit_models.is_empty());
    assert!(!single_model_exists());
}
