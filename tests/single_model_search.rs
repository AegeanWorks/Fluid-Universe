use fluid_universe::single_model_test::{
    WITNESSES, all_witnesses_exact, braiding_fraction, canonical_conformal_weights,
    canonical_quantum_dimensions, central_charge, inverse_search, lower_rank_inverse_search,
    minimal_level_inverse_search, minimal_witness_search, relaxed_inverse_search, search,
    single_model_exists, sm_charge_compatible, sm_gauge_central_charge_upper_bound,
    su7_block_coset_central_charge, su7_block_sm_central_charge, su7_minimal_coset_central_charge,
    witness_exact, witness_fractions,
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
    assert_eq!(
        result.phase_masks.get(&(2, 13)).copied().unwrap_or(0) & 2,
        2
    );
    assert_eq!(
        result.phase_masks.get(&(3, 49)).copied().unwrap_or(0) & 4,
        4
    );
}

#[test]
fn relaxed_label_inverse_search_finds_the_rank_six_host() {
    let result = relaxed_inverse_search();
    assert_eq!(result.labels_tested, 125_888);
    assert_eq!(result.models_with_any_phase, 1_259);
    assert_eq!(result.hit_models, vec![(7, 1163)]);
}

#[test]
fn lower_rank_search_reproduces_the_bounded_negative() {
    let result = lower_rank_inverse_search();
    assert_eq!(result.labels_tested, 4_288_305);
    assert_eq!(result.models_with_any_phase, 5_499);
    assert!(result.hit_models.is_empty());
}

#[test]
fn su7_witnesses_have_minimal_label_sums() {
    let result = minimal_witness_search();
    assert_eq!(result.labels_tested, 85_766_121);
    assert_eq!(result.best_sums, [16, 10, 6]);
}

#[test]
fn minimal_witness_families_have_canonical_current_vectors() {
    let result = minimal_witness_search();
    assert_eq!(result.representative_counts, [4, 2, 2]);
    for phase in 0..3 {
        assert_eq!(result.canonical_dynkin[phase], WITNESSES[2].dynkin[phase]);
    }
}

#[test]
fn canonical_su7_conformal_data_matches_pre_registration() {
    let weights = canonical_conformal_weights();
    assert!((weights[0] - 934.0 / 4095.0).abs() <= 1e-15);
    assert!((weights[1] - 289.0 / 2730.0).abs() <= 1e-15);
    assert!((weights[2] - 2.0 / 35.0).abs() <= 1e-15);
    let dimensions = canonical_quantum_dimensions();
    let expected = [
        1.136464998628996e11,
        4.072382134390699e8,
        4.568658409657278e5,
    ];
    for (actual, target) in dimensions.iter().zip(expected) {
        assert!((actual - target).abs() / target <= 1e-12);
    }
    assert!(dimensions[0] > dimensions[1] && dimensions[1] > dimensions[2]);
}

#[test]
fn su7_host_cannot_directly_embed_the_sm_gauge_algebra() {
    let host = central_charge(7, 1163);
    assert!((host - 9304.0 / 195.0).abs() <= 1e-14);
    assert_eq!(sm_gauge_central_charge_upper_bound(), 12.0);
    assert!(host > 12.0);
    assert!(!sm_charge_compatible());
    assert!((su7_minimal_coset_central_charge() - 6964.0 / 195.0).abs() <= 1e-14);
    assert!(central_charge(17, 1153) > 12.0);
    assert!(central_charge(23, 1147) > 12.0);
}

#[test]
fn su7_block_sm_subsector_has_exact_coset_central_charge() {
    // Two independent traceless, orthogonal diagonal generators commuting with
    // the 3+2 blocks; the full block centralizer has a third U(1) direction.
    let generators = [[2_i32, 2, 2, -3, -3, 0, 0], [0, 0, 0, 0, 0, 1, -1]];
    for generator in generators {
        assert_eq!(generator.iter().sum::<i32>(), 0);
        assert_eq!(generator[0], generator[1]);
        assert_eq!(generator[1], generator[2]);
        assert_eq!(generator[3], generator[4]);
        assert!(generator.iter().any(|entry| *entry != 0));
    }
    assert_eq!(
        generators[0]
            .iter()
            .zip(generators[1])
            .map(|(a, b)| a * b)
            .sum::<i32>(),
        0
    );
    let sub = su7_block_sm_central_charge();
    let coset = su7_block_coset_central_charge();
    assert!((sub - 8_812_057.0 / 679_195.0).abs() <= 1e-14);
    assert!((coset - 920_175_833.0 / 26_488_605.0).abs() <= 1e-14);
    assert!(sub < central_charge(7, 1163));
    assert!(coset > 34.0);
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

#[test]
fn su7_host_is_minimal_level_in_the_widened_domain() {
    let result = minimal_level_inverse_search();
    assert_eq!(result.labels_tested, 85_766_121);
    assert_eq!(result.models_with_any_phase, 244);
    assert!(result.hit_models.is_empty());
}
