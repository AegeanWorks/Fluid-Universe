use fluid_universe::su2_monodromy::{fusion_allowed, reversed_monodromy_residue};

#[test]
fn higher_spin_witness_has_three_fusion_channels_and_common_orientation() {
    let (level, external) = (2338, 558);
    for (channel, target_num, target_den) in [(1070, 2_i128, 9_i128), (1044, 1, 10), (688, 2, 39)] {
        assert!(fusion_allowed(level, external, external, channel));
        let (num, den) = reversed_monodromy_residue(level, external, channel).unwrap();
        assert_eq!(num * target_den, den * target_num);
    }
    assert!(!fusion_allowed(level, external, external, 1118));
    assert!(!fusion_allowed(level, external, external, 1071));
    assert!(!fusion_allowed(level, level + 1, external, 0));
    assert!(reversed_monodromy_residue(level, external, 1071).is_none());
}

#[test]
fn every_lower_candidate_level_lacks_a_triple_match() {
    use fluid_universe::su2_monodromy::lower_level_search;
    let rows = lower_level_search();
    assert_eq!(rows[0], (583, 85_556, [390, 30, 80, 0, 68, 16, 0, 0]));
    assert_eq!(rows[1], (1168, 342_225, [930, 71, 0, 0, 150, 18, 0, 0]));
    assert_eq!(rows[2], (1753, 770_006, [1162, 23, 268, 0, 154, 147, 0, 0]));
}

#[test]
fn smaller_spin_witness_and_all_lower_spin_search() {
    use fluid_universe::su2_monodromy::lower_spin_search;
    for (channel, p, q) in [(478, 2_i128, 9_i128), (556, 1, 10), (0, 2, 39)] {
        let (reverse, den) = reversed_monodromy_residue(7018, 288, channel).unwrap();
        assert_eq!(reverse * q, den * (q - p));
    }
    assert_eq!(*lower_spin_search(), (30_656, 6_642_629, 0, 7_458, 1_013_210, 0));
}

#[test]
fn independent_orientation_witness_has_smaller_spin() {
    for (channel, p, q) in [(128, 2_i128, 9_i128), (234, 1, 10)] {
        let (reverse, den) = reversed_monodromy_residue(2338, 180, channel).unwrap();
        assert_eq!(reverse * q, den * p);
    }
    let (reverse, den) = reversed_monodromy_residue(2338, 180, 168).unwrap();
    assert_eq!(reverse * 39, den * 37);
}

#[test]
fn su2_witnesses_have_less_central_charge_than_the_sm_gauge_minimum() {
    use fluid_universe::single_model_test::central_charge;
    let checks = fluid_universe::su2_monodromy::checks();
    let gap = checks.iter().find(|check| check.id == "su2_monodromy.sm_minimum_gap").unwrap();
    let minimum = central_charge(3, 1) + central_charge(2, 1) + 1.0;
    assert_eq!(minimum, 4.0);
    assert!((central_charge(2, 2338) - 1169.0 / 390.0).abs() < 1e-14);
    assert!((central_charge(2, 7018) - 3509.0 / 1170.0).abs() < 1e-14);
    assert!((gap.value - (minimum - central_charge(2, 2338))).abs() < 1e-14);
    assert!(gap.value > 1.0);
}

#[test]
fn unextended_su2_has_too_few_weight_one_gauge_currents() {
    let checks = fluid_universe::su2_monodromy::checks();
    let deficit = checks.iter().find(|check| check.id == "su2_monodromy.sm_current_deficit").unwrap();
    assert_eq!(deficit.value, (8 + 3 + 1 - 3) as f64);
}
