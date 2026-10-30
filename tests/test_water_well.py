"""Finite spherical well for the hydrated electron (claim 33)."""

from __future__ import annotations

import pytest

from theory.water_well import (
    born_depth_ev,
    infinite_well_transition_ev,
    minimum_required_depth,
    required_depth_ev,
    threshold_depth_ev,
    well_roots,
    well_scales,
)

R_CAPILLARY = 2.8580710555848763e-10
R_MINIMUM = 3.181452582965287e-10
R_THREE_TWO = 3.2e-10
R_INFINITE = 4.809506419720198e-10


def test_born_depth_at_the_capillary_radius():
    assert born_depth_ev(R_CAPILLARY) == pytest.approx(2.486987864, rel=1e-8)


def test_born_well_has_only_a_1s_state():
    roots = well_roots(R_CAPILLARY, born_depth_ev(R_CAPILLARY))

    assert roots["kappa"] == pytest.approx(2.309130015, rel=1e-8)
    assert roots["u_1s"] == pytest.approx(2.049532545, rel=1e-8)
    assert roots["energy_1s_ev"] == pytest.approx(-0.527752862, rel=1e-8)
    assert roots["u_1p"] is None
    assert roots["energy_1p_ev"] is None
    assert roots["transition_ev"] is None


def test_deeper_well_binds_the_1p_state():
    roots = well_roots(R_CAPILLARY, 5.0)

    assert roots["u_1s"] == pytest.approx(2.343763368, rel=1e-8)
    assert roots["u_1p"] == pytest.approx(3.214247618, rel=1e-8)
    assert roots["transition_ev"] == pytest.approx(2.256613654, rel=1e-8)


def test_threshold_depth_and_born_maximum_gap():
    scales = well_scales()

    assert threshold_depth_ev(R_CAPILLARY) == pytest.approx(
        4.603378007, rel=1e-8
    )
    assert scales["threshold"]["depth_at_capillary_ev"] == pytest.approx(
        4.603378007, rel=1e-8
    )
    assert scales["threshold"]["gap_at_capillary_ev"] == pytest.approx(
        2.106462769, rel=1e-8
    )
    assert scales["threshold"]["radius_m"] == pytest.approx(5.290247544e-10, rel=1e-8)
    assert scales["threshold"]["max_born_gap_ev"] == pytest.approx(
        0.614819796, rel=1e-8
    )


def test_infinite_well_transition_scale():
    assert infinite_well_transition_ev(4.703128884e-10) == pytest.approx(
        1.777772477, rel=1e-8
    )
    assert well_scales()["infinite_well"]["radius_for_1p7_m"] == pytest.approx(
        R_INFINITE, rel=1e-8
    )


def test_born_depth_cannot_reach_1p7_ev():
    assert required_depth_ev(R_CAPILLARY, 1.7) is None
    assert required_depth_ev(3.0e-10, 1.7) is None
    assert required_depth_ev(4.9e-10, 1.7) is None
    assert well_scales()["threshold"]["max_born_gap_ev"] < 1.7


def test_required_depth_at_three_point_two_angstrom():
    required = required_depth_ev(R_THREE_TWO, 1.7)

    assert required == pytest.approx(3.715900201, rel=1e-7)
    assert required - born_depth_ev(R_THREE_TWO) == pytest.approx(
        1.494653942, rel=1e-7
    )


def test_minimum_required_depth():
    minimum = minimum_required_depth(1.7)

    assert minimum["radius_m"] == pytest.approx(R_MINIMUM, rel=1e-7)
    assert minimum["required_depth_ev"] == pytest.approx(3.715110814, rel=1e-7)
    assert minimum["born_depth_ev"] == pytest.approx(2.234195810, rel=1e-7)
    assert minimum["excess_ev"] == pytest.approx(1.480915004, rel=1e-7)


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="radius"):
        born_depth_ev(0.0)
    with pytest.raises(ValueError, match="depth"):
        well_roots(R_CAPILLARY, 0.0)
    with pytest.raises(ValueError, match="radius"):
        threshold_depth_ev(0.0)
    with pytest.raises(ValueError, match="radius"):
        infinite_well_transition_ev(0.0)
    with pytest.raises(ValueError, match="target"):
        required_depth_ev(R_CAPILLARY, 0.0)
    with pytest.raises(ValueError, match="target"):
        minimum_required_depth(0.0)
