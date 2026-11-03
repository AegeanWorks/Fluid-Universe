"""Gaussian Ritz width-selection tests (claim 49)."""

from __future__ import annotations

import numpy as np
import pytest

from theory.variational_width import (
    BASE,
    GRID_S2,
    SCALINGS,
    action_gm,
    action_yasui,
    d2_action_gm_s2,
    d2_action_gm_sigma,
    extremum_s2,
    grid_maximum,
    selection_scales,
    yasui_monotone,
)


def test_gm_extremum_is_half():
    assert extremum_s2(1.0, 1.0, 1.0) == pytest.approx(0.5, abs=1e-9)


def test_gm_grid_agrees_with_analytic():
    peak = grid_maximum(np.array(GRID_S2), 1.0, 1.0, 1.0)

    assert abs(peak["grid_argmax"] - 0.5) <= peak["grid_step"]
    assert abs(peak["refined"] - 0.5) <= 1e-9


def test_gm_curvature_is_maximum():
    curv_s2 = d2_action_gm_s2(0.5, 1.0, 1.0, 1.0)
    curv_sigma = d2_action_gm_sigma(0.5**0.5, 1.0, 1.0, 1.0)

    assert curv_s2 == pytest.approx(-2.0, abs=1e-9)
    assert curv_s2 < 0.0
    assert curv_sigma == pytest.approx(-4.0, abs=1e-9)


def test_yasui_has_no_finite_extremum():
    mono = yasui_monotone(np.array(GRID_S2), 1.0, 1.0, 1.0)

    assert mono["all_negative"] is True
    assert mono["max_derivative"] < -1e-3
    assert mono["max_diff"] < 0.0


def test_scaling_law_holds():
    cases = [
        ({"omega": 2.0, "hbar": 1.0, "mass": 1.0}, (0.25, -16.0)),
        ({"omega": 1.0, "hbar": 1.0, "mass": 2.0}, (0.25, -8.0)),
    ]
    for params, expected in cases:
        pred = extremum_s2(params["omega"], params["hbar"], params["mass"])
        peak = grid_maximum(
            np.array(GRID_S2), params["omega"], params["hbar"], params["mass"]
        )
        curv = d2_action_gm_s2(
            pred, params["omega"], params["hbar"], params["mass"]
        )

        assert pred == pytest.approx(expected[0], abs=1e-12)
        assert abs(peak["refined"] - expected[0]) <= 1e-9
        assert curv == pytest.approx(expected[1], abs=1e-9)


def test_ritz_verdict_holds():
    from phenomenology.variational_width_report import (
        run_variational_width_check,
    )

    payload = run_variational_width_check(write=False)
    gates = payload["result"]["gates"]
    verdict = payload["result"]["verdict"]

    assert not any(k for _, k in gates["kills"])
    assert verdict["claim_49_status"].startswith("NUMERIC")


def test_inputs_are_frozen():
    assert BASE == {"omega": 1.0, "hbar": 1.0, "mass": 1.0}
    assert len(GRID_S2) == 2001
    assert GRID_S2[0] == pytest.approx(0.125)
    assert GRID_S2[-1] == pytest.approx(2.0)
    assert action_gm(0.5, 1.0, 1.0, 1.0) == pytest.approx(-0.5)
    assert action_yasui(0.125, 1.0, 1.0, 1.0) == pytest.approx(0.9375)
    with pytest.raises(ValueError, match="positive"):
        extremum_s2(0.0, 1.0, 1.0)
