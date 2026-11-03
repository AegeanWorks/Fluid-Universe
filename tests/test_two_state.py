"""Shared-lambda two-state Marcus assignment (claim 44)."""

from __future__ import annotations

import pytest

from theory.two_state_marcus import (
    LAMBDA_EV,
    ROOM_TEMPERATURE_K,
    SHAPE_RATIO_TOL,
    component_ratio,
    kill_thresholds,
    shared_lambda_widths,
    two_state_scales,
)
from theory.water_marcus import marcus_width_ev
from theory.water_observed_shape import (
    JOU_FREEMAN_G1_WIDTH_EV,
    JOU_FREEMAN_G2_WIDTH_EV,
)


def test_shared_lambda_gives_identical_widths():
    widths = shared_lambda_widths()

    assert widths["lambda_ev"] == pytest.approx(LAMBDA_EV, rel=1e-12)
    assert widths["width_g1_ev"] == widths["width_g2_ev"]
    assert widths["width_g1_ev"] == pytest.approx(0.680915, rel=1e-6)
    assert widths["predicted_ratio"] == 1.0


def test_ratio_prediction_is_t_independent():
    for temperature_k in (200.0, 298.0, 300.0, 350.0):
        widths = shared_lambda_widths(temperature_k)

        assert widths["width_g1_ev"] == widths["width_g2_ev"]
        assert widths["predicted_ratio"] == 1.0
        assert widths["width_g1_ev"] == pytest.approx(
            marcus_width_ev(LAMBDA_EV, temperature_k), rel=1e-12
        )


def test_observed_component_ratio():
    ratio = component_ratio()

    assert ratio["g1_width_ev"] == pytest.approx(
        JOU_FREEMAN_G1_WIDTH_EV, rel=1e-12
    )
    assert ratio["g2_width_ev"] == pytest.approx(
        JOU_FREEMAN_G2_WIDTH_EV, rel=1e-12
    )
    assert ratio["observed_ratio"] == pytest.approx(
        JOU_FREEMAN_G1_WIDTH_EV / JOU_FREEMAN_G2_WIDTH_EV, rel=1e-12
    )
    assert ratio["diff_vs_one"] == pytest.approx(0.9189, abs=1.0e-3)
    # Even the convention-confusion worst case (exactly 2) still kills.
    assert ratio["worst_case_diff_vs_one"] == 1.0


def test_kill_threshold_is_frozen():
    thresholds = kill_thresholds()

    assert thresholds["shape_ratio_tol"] == pytest.approx(SHAPE_RATIO_TOL)
    assert SHAPE_RATIO_TOL == pytest.approx(0.15)


def test_comparison_falsifies_shared_lambda_assignment():
    from phenomenology.two_state_report import run_two_state_comparison

    comparison = run_two_state_comparison()

    assert comparison["predicted_ratio"] == 1.0
    assert comparison["observed_ratio"] == pytest.approx(
        JOU_FREEMAN_G1_WIDTH_EV / JOU_FREEMAN_G2_WIDTH_EV, rel=1e-12
    )
    assert comparison["ratio_diff_vs_one"] == pytest.approx(
        0.9189, abs=1.0e-3
    )
    assert comparison["ratio_diff_vs_one"] > SHAPE_RATIO_TOL
    assert comparison["worst_case_diff_vs_one"] > SHAPE_RATIO_TOL
    assert comparison["shape_kill_shared_lambda"] is True
    assert comparison["temperature_k"] == ROOM_TEMPERATURE_K


def test_two_state_scales_use_frozen_lambda():
    scales = two_state_scales()

    assert scales["lambda_ev"] == pytest.approx(LAMBDA_EV, rel=1e-12)
    assert scales["prediction"]["predicted_ratio"] == 1.0
    assert scales["observed"]["observed_ratio"] == pytest.approx(
        1.9189, abs=1.0e-3
    )
