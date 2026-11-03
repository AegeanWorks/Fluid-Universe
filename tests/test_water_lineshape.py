"""Fixed-lambda cavity-Marcus line-shape prediction (claim 43)."""

from __future__ import annotations

import math

import pytest

from theory.water_lineshape import (
    LAMBDA_EV,
    SKEW_TOL,
    SHAPE_RATIO_TOL,
    WIDTH_RATIO_TOL,
    gaussian_moments,
    kill_thresholds,
    lineshape_scales,
    predicted_widths,
)
from theory.water_observed_shape import (
    CHRISTENSEN_DWIDTH_DT_EV_PER_K,
    JOU_FREEMAN_G1_WIDTH_EV,
    JOU_FREEMAN_G2_WIDTH_EV,
    JOU_FREEMAN_TAIL_POWER_WATER_N,
    observed_scales,
    observed_width_change_ev,
    shape_evidence,
)


def test_gaussian_symmetry_is_exact():
    moments = gaussian_moments()

    assert moments["skewness"] == 0.0
    assert moments["half_width_ratio"] == 1.0
    assert moments["third_central_moment"] == 0.0
    assert moments["symmetric"] is True


def test_predicted_widths_match_temperature_scan():
    prediction = predicted_widths()

    assert prediction["lambda_0c_ev"] == pytest.approx(1.637567127, rel=1e-7)
    assert prediction["lambda_45c_ev"] == pytest.approx(1.589505512, rel=1e-7)
    assert prediction["width_0c_ev"] == pytest.approx(0.653821691, rel=1e-7)
    assert prediction["width_45c_ev"] == pytest.approx(0.695194215, rel=1e-7)
    assert prediction["width_change_ev"] == pytest.approx(0.041372524, rel=1e-6)
    assert prediction["width_ratio_45_over_0"] == pytest.approx(
        1.063278, rel=1e-6
    )


def test_ratio_follows_sqrt_lambda_t():
    prediction = predicted_widths()
    expected = math.sqrt(
        (prediction["lambda_45c_ev"] * 318.15)
        / (prediction["lambda_0c_ev"] * 273.15)
    )

    assert prediction["width_ratio_45_over_0"] == pytest.approx(
        expected, rel=1e-12
    )


def test_kill_thresholds_are_data():
    thresholds = kill_thresholds()

    assert thresholds["width_ratio_tol"] == pytest.approx(WIDTH_RATIO_TOL)
    assert thresholds["shape_ratio_tol"] == pytest.approx(SHAPE_RATIO_TOL)
    assert thresholds["skew_tol"] == pytest.approx(SKEW_TOL)


def test_lineshape_scales_use_fixed_lambda():
    scales = lineshape_scales()

    assert scales["lambda_ev"] == pytest.approx(LAMBDA_EV, rel=1e-12)
    assert scales["moments"]["symmetric"] is True
    assert scales["prediction"]["width_ratio_45_over_0"] == pytest.approx(
        1.063278, rel=1e-6
    )


def test_observed_width_slope_is_citable_value():
    change = observed_width_change_ev(45.0)

    assert change["slope_ev_per_k"] == pytest.approx(2.0e-4, rel=1e-12)
    assert change["change_as_stated_ev"] == pytest.approx(0.009, rel=1e-12)
    assert change["change_if_hwhm_ev"] == pytest.approx(0.018, rel=1e-12)


def test_observed_shape_needs_two_gaussians_plus_tail():
    shape = shape_evidence()

    assert shape["g1_width_ev"] == pytest.approx(0.71, rel=1e-12)
    assert shape["g2_width_ev"] == pytest.approx(0.37, rel=1e-12)
    assert shape["component_width_ratio"] == pytest.approx(
        0.71 / 0.37, rel=1e-12
    )
    assert shape["tail_power_n_water"] == pytest.approx(4.4, rel=1e-12)
    assert shape["tail_is_gaussian"] is False
    assert shape["single_gaussian_compatible"] is False


def test_comparison_kills_gaussian_shape_not_ratio():
    from phenomenology.water_lineshape_report import (
        run_water_lineshape_comparison,
    )

    comparison = run_water_lineshape_comparison()

    # Shape: component-width ratio 1.9189 differs from 1 by 0.9189 > 0.15.
    assert comparison["component_width_ratio"] == pytest.approx(
        JOU_FREEMAN_G1_WIDTH_EV / JOU_FREEMAN_G2_WIDTH_EV, rel=1e-12
    )
    assert comparison["shape_kill_gaussian_only"] is True
    # Ratio: observed change 0.009 eV vs predicted 0.04137 eV; on the
    # predicted scale the ratio diff is 0.0495 < 0.05 (HWHM reading
    # 0.0358 < 0.05). The scaling survives this comparator.
    assert comparison["observed_change_ev"] == pytest.approx(
        CHRISTENSEN_DWIDTH_DT_EV_PER_K * 45.0, rel=1e-12
    )
    assert comparison["ratio_diff"] == pytest.approx(0.0495, abs=1.0e-3)
    assert comparison["ratio_kill"] is False
    assert comparison["ratio_kill_if_hwhm"] is False
    assert comparison["width_ratio_threshold"] == WIDTH_RATIO_TOL
    assert comparison["shape_kill_threshold"] == SHAPE_RATIO_TOL
    assert JOU_FREEMAN_TAIL_POWER_WATER_N == pytest.approx(4.4, rel=1e-12)


def test_observed_scales_payload_shape():
    scales = observed_scales()

    assert scales["emax_ev"] == pytest.approx(1.72, rel=1e-12)
    assert scales["shape"]["two_gaussians_plus_tail"] is True
