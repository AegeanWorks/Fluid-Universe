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
