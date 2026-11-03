"""2D Ritz saddle tests (claim 50)."""

from __future__ import annotations

import numpy as np
import pytest

from theory.variational_saddle import (
    BASE,
    GRID_C,
    GRID_S,
    action_gm_2d,
    action_yasui_2d,
    conditional_extremum,
    diffusivity,
    extremum_sc,
    hess_gm_2d,
    saddle_scales,
)
from theory.variational_width import action_gm as action_gm_1d


def test_saddle_location_is_half_zero():
    s_star, c_star = extremum_sc(1.0, 1.0, 1.0)

    assert s_star == pytest.approx(0.5, abs=1e-9)
    assert c_star == pytest.approx(0.0, abs=1e-9)


def test_conditional_extremum_agrees_and_refines():
    peak = conditional_extremum(np.array(GRID_S), np.array(GRID_C), 1.0, 1.0, 1.0)

    assert abs(peak["grid_s"] - 0.5) <= peak["grid_step_s"]
    assert abs(peak["grid_c"] - 0.0) <= peak["grid_step_c"]
    assert abs(peak["refined_s"] - 0.5) <= 1e-9
    assert abs(peak["refined_c"] - 0.0) <= 1e-9


def test_box_maximum_sits_above_the_saddle():
    peak = conditional_extremum(np.array(GRID_S), np.array(GRID_C), 1.0, 1.0, 1.0)

    assert peak["box_argmax_s"] == pytest.approx(2.0)
    assert abs(peak["box_argmax_c"]) == pytest.approx(1.0)
    assert peak["box_max_value"] > peak["saddle_value"]


def test_curvature_splits_max_min():
    d2s, d2c, mixed = hess_gm_2d(0.5, 0.0, 1.0, 1.0, 1.0)

    assert d2s == pytest.approx(-4.0, abs=1e-9)
    assert d2c == pytest.approx(1.0, abs=1e-9)
    assert mixed == pytest.approx(0.0, abs=1e-12)
    assert d2s < 0.0 < d2c


def test_slice_is_twice_the_1d_action():
    for s in (0.125, 0.5, 1.0, 2.0):
        assert action_gm_2d(s, 0.0, 1.0, 1.0, 1.0) == pytest.approx(
            2.0 * action_gm_1d(s, 1.0, 1.0, 1.0), abs=1e-12
        )


def test_yasui_selects_rotation_not_width():
    from theory.variational_saddle import yasui_scan

    assert diffusivity(1.0, 1.0) == pytest.approx(0.5)
    assert action_yasui_2d(2.0, 0.0, 1.0, 1.0, 1.0) == pytest.approx(-1.875)
    mono = yasui_scan(np.array(GRID_S), 1.0, 1.0, 1.0)

    assert mono["all_negative"] is True
    assert mono["max_derivative"] == pytest.approx(-1.0625)
    assert mono["max_derivative"] < -1e-3
    assert mono["max_diff"] < 0.0


def test_scaling_point_holds():
    scales = saddle_scales()
    entry = scales["scalings"]

    assert entry["predicted_s"] == pytest.approx(0.25, abs=1e-12)
    assert abs(entry["refined_s"] - 0.25) <= 1e-9
    assert abs(entry["refined_c"] - 0.0) <= 1e-9
    assert entry["curvature_s"] == pytest.approx(-32.0, abs=1e-9)
    assert entry["curvature_c"] == pytest.approx(0.5, abs=1e-9)


def test_saddle_verdict_holds():
    from phenomenology.variational_saddle_report import (
        run_variational_saddle_check,
    )

    payload = run_variational_saddle_check(write=False)
    gates = payload["result"]["gates"]
    verdict = payload["result"]["verdict"]

    assert not any(k for _, k in gates["kills"])
    assert verdict["claim_50_status"].startswith("NUMERIC")


def test_inputs_are_frozen():
    assert BASE == {"omega": 1.0, "hbar": 1.0, "mass": 1.0}
    assert len(GRID_S) == 2001
    assert len(GRID_C) == 401
    assert GRID_C[200] == pytest.approx(0.0)
    assert GRID_S[400] == pytest.approx(0.5)
    with pytest.raises(ValueError, match="positive"):
        extremum_sc(0.0, 1.0, 1.0)
