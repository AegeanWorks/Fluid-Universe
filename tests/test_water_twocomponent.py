"""Two-component band model vs observed side widths (claim 45)."""

from __future__ import annotations

import pytest

from theory.water_observed_shape import jou_freeman_side_widths
from theory.water_twocomponent import (
    EMAX_EV,
    FWHM_CLASSICAL_TOL,
    FWHM_QUANTIZED_TOL,
    T_REF_K,
    WIDTH_RATIO_TOL,
    twocomponent_scales,
)


def test_side_widths_match_jou_freeman_fits():
    observed = jou_freeman_side_widths(299.15)

    assert observed["red_gaussian_hwhm_ev"] == pytest.approx(
        0.1517 + 0.00051 * 299.15, rel=1e-12
    )
    assert observed["blue_lorentzian_hwhm_ev"] == pytest.approx(
        0.5124 - 0.00010 * 299.15, rel=1e-12
    )
    assert observed["side_ratio_blue_over_red"] == pytest.approx(
        1.585732, rel=1e-6
    )
    assert observed["emax_ev"] == pytest.approx(1.73264, rel=1e-6)


def test_twocomponent_prediction_uses_fixed_inputs_only():
    scales = twocomponent_scales()

    assert scales["temperature_k"] == T_REF_K
    assert scales["emax_ev"] == EMAX_EV == pytest.approx(1.72, rel=1e-12)
    assert scales["n_tail"] == pytest.approx(4.4, rel=1e-12)
    assert scales["red_hwhm_ev"] == pytest.approx(
        scales["marcus_fwhm_ev"] / 2.0, rel=1e-12
    )
    assert scales["blue_hwhm_ev"] == pytest.approx(0.293466, rel=1e-6)
    assert scales["r_pred"] == pytest.approx(0.863201, rel=1e-6)
    assert scales["fwhm_model_ev"] == pytest.approx(0.633441, rel=1e-6)


def test_comparison_kills_both_legs():
    from phenomenology.water_twocomponent_report import (
        run_water_twocomponent_check,
    )

    payload = run_water_twocomponent_check(write=False)
    scales = payload["result"]["scales"]
    verdict = payload["result"]["verdict"]

    assert scales["ratio_diff"] == pytest.approx(0.722531, rel=1e-6)
    assert scales["ratio_diff"] > WIDTH_RATIO_TOL
    assert scales["classical_rel_diff"] == pytest.approx(0.102775, rel=1e-6)
    assert scales["classical_rel_diff"] > FWHM_CLASSICAL_TOL
    assert scales["quantized_rel_diff"] == pytest.approx(0.1805422, rel=1e-6)
    assert scales["quantized_rel_diff"] > FWHM_QUANTIZED_TOL
    assert verdict["ratio_kill"] is True
    assert verdict["width_kill"] is True
    assert verdict["claim_45_status"].startswith("FALSIFIED")


def test_thresholds_are_frozen():
    scales = twocomponent_scales()

    assert scales["thresholds"]["width_ratio_tol"] == WIDTH_RATIO_TOL == 0.15
    assert scales["thresholds"]["fwhm_classical_tol"] == FWHM_CLASSICAL_TOL
    assert FWHM_CLASSICAL_TOL == 0.05
    assert scales["thresholds"]["fwhm_quantized_tol"] == FWHM_QUANTIZED_TOL
    assert FWHM_QUANTIZED_TOL == 0.15
