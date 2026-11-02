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
