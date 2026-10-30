"""Bound-free absorption in the finite spherical well (claim 34)."""

from __future__ import annotations

import pytest

from theory.water_absorption import (
    bound_state,
    oscillator_strength,
    oscillator_sum_rule,
    peak_absorption,
    required_extra_depth,
)

R_CAPILLARY = 2.8580710555848763e-10
BORN_DEPTH = 2.4869878638766574


def test_bound_state_normalization():
    state = bound_state(R_CAPILLARY, BORN_DEPTH)

    assert state["u_1s"] == pytest.approx(2.049532545, rel=1e-8)
    assert state["energy_ev"] == pytest.approx(-0.527752862, rel=1e-8)
    assert state["norm"] == pytest.approx(1.0, abs=1e-6)


def test_oscillator_strength_sum_rule():
    value = oscillator_sum_rule(
        R_CAPILLARY, BORN_DEPTH, grid=6000, points=600, max_ev=136.0
    )

    assert value == pytest.approx(1.0, abs=0.02)


def test_born_depth_continuum_peak():
    peak = peak_absorption(R_CAPILLARY, BORN_DEPTH, grid=6000, points=31)

    assert peak["threshold_ev"] == pytest.approx(0.527752862, rel=1e-6)
    assert peak["peak_ev"] == pytest.approx(1.02488, abs=0.03)
    assert peak["peak_strength"] > 0.0


def test_peak_moves_up_with_depth():
    shallow = peak_absorption(R_CAPILLARY, 3.0, grid=6000, points=31)
    deeper = peak_absorption(R_CAPILLARY, 3.6, grid=6000, points=31)

    assert shallow["peak_ev"] == pytest.approx(1.37258, abs=0.04)
    assert deeper["peak_ev"] == pytest.approx(1.66399, abs=0.04)
    assert deeper["peak_ev"] > shallow["peak_ev"]


def test_required_extra_depth_for_1p7():
    result = required_extra_depth(R_CAPILLARY, 1.7, grid=4000, points=21)

    assert result["depth_ev"] == pytest.approx(3.68483, abs=0.03)
    assert result["extra_ev"] == pytest.approx(1.19784, abs=0.03)
    assert result["peak_ev"] == pytest.approx(1.70092, abs=0.02)
    assert result["depth_ev"] > BORN_DEPTH + 1.0


def test_oscillator_strength_below_threshold_is_zero():
    assert oscillator_strength(R_CAPILLARY, BORN_DEPTH, 0.5) == 0.0


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="radius"):
        bound_state(0.0, BORN_DEPTH)
    with pytest.raises(ValueError, match="depth"):
        peak_absorption(R_CAPILLARY, 0.0)
    with pytest.raises(ValueError, match="photon"):
        oscillator_strength(R_CAPILLARY, BORN_DEPTH, -0.1)
    with pytest.raises(ValueError, match="target"):
        required_extra_depth(R_CAPILLARY, 0.0)
