use fluid_universe::single_model_test::{
    WITNESSES, all_witnesses_exact, braiding_fraction, inverse_search,
    relaxed_inverse_search, search, single_model_exists, witness_exact, witness_fractions,
};

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
}

#[test]
fn all_positive_levels_reproduce_the_inverse_negative() {
    let result = inverse_search();
    assert_eq!(result.labels_tested, 19_530);
    assert_eq!(result.models_with_any_phase, 694);
    assert!(result.hit_models.is_empty());
    assert_eq!(result.phase_masks.get(&(3, 9)).copied().unwrap_or(0) & 1, 1);
    assert_eq!(result.phase_masks.get(&(2, 13)).copied().unwrap_or(0) & 2, 2);
    assert_eq!(result.phase_masks.get(&(3, 49)).copied().unwrap_or(0) & 4, 4);
}

#[test]
fn relaxed_label_inverse_search_finds_the_rank_seven_host() {
    let result = relaxed_inverse_search();
    assert_eq!(result.labels_tested, 125_888);
    assert_eq!(result.models_with_any_phase, 1_259);
    assert_eq!(result.hit_models, vec![(7, 1163)]);
}

#[test]
fn explicit_single_model_witnesses_are_exact_and_integrable() {
    assert_eq!(WITNESSES.len(), 3);
    assert_eq!((WITNESSES[0].n, WITNESSES[0].k), (17, 1153));
    assert_eq!((WITNESSES[1].n, WITNESSES[1].k), (23, 1147));
    assert_eq!((WITNESSES[2].n, WITNESSES[2].k), (7, 1163));
    assert!(all_witnesses_exact());
    assert!(single_model_exists());
    let targets = [(2, 9), (1, 10), (2, 39)];
    for index in 0..WITNESSES.len() {
        assert!(witness_exact(index));
        for (fraction, &(p, q)) in witness_fractions(index).iter().zip(&targets) {
            assert_eq!(fraction.0 * q, p * fraction.1);
        }
    }
}
