"""Osmotic-closure no-go tests (claim 48)."""

from __future__ import annotations

import math

import numpy as np
import pytest

from theory.nelson_closure import (
    DIFFUSION,
    GRID_LENGTH,
    GRID_NS,
    TRIAL_SIGMA_SQ,
    closure_scales,
    fp_residual,
    interior_mask,
    trial_bimodal,
    trial_gaussian,
)


def test_residual_scales_as_dx_squared():
    scales = closure_scales()
    rows = scales["stationary_map"]["rows"]

    for name, row in rows.items():
        assert row["slope_interior"] == pytest.approx(-2.0, abs=0.1), name


def test_residual_decreases_monotonically():
    scales = closure_scales()
    rows = scales["stationary_map"]["rows"]

    for name, row in rows.items():
        interior = row["interior"]
        assert all(b < a for a, b in zip(interior, interior[1:])), name


def test_sigma_map_is_identity():
    scales = closure_scales()

    for entry in scales["sigma_map"]["entries"]:
        sigma0 = math.sqrt(entry["sigma0_sq"])
        assert abs(entry["sigma_star"][-1] - sigma0) <= 1e-9


def test_sigma_map_error_is_flat_not_contracting():
    scales = closure_scales()

    for entry in scales["sigma_map"]["entries"]:
        assert abs(entry["error_slope"]) < 0.5, entry["sigma0_sq"]


def test_closure_verdict_holds():
    from phenomenology.nelson_closure_report import run_nelson_closure_check

    payload = run_nelson_closure_check(write=False)
    gates = payload["result"]["gates"]

    assert not any(k for _, k in gates["slope_kills"])
    assert not any(k for _, k in gates["monotone_kills"])
    assert not any(k for _, k, _ in gates["sigma_kills"])


def test_inputs_are_frozen():
    assert GRID_NS == (256, 512, 1024, 2048)
    assert GRID_LENGTH == 20.0
    assert DIFFUSION == 0.5
    assert TRIAL_SIGMA_SQ == (0.125, 0.25, 0.5, 1.0, 2.0)
    x = np.array([-1.0, 0.0, 1.0])
    assert fp_residual(np.array([1.0, 1.0, 1.0]), 1.0).shape == (3,)
    assert interior_mask(x).shape == (3,)
    assert trial_gaussian(x, 0.5)[0] > 0.0
    assert trial_bimodal(np.array([-2.0, 0.0, 2.0])).shape == (3,)
