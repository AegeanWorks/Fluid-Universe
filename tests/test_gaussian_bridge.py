"""Fixed-endpoint Gaussian bridge tests (claim 54)."""

from __future__ import annotations

import math

import numpy as np
import pytest

from theory.gaussian_bridge import (
    GRID_N,
    STARTS,
    bridge_scales,
    horizon,
    pinney_coeffs,
)


def test_pinney_coefficients_match_quadratic():
    coeffs = pinney_coeffs(1.2 * (1.0 / math.sqrt(2.0)), 1.0 / math.sqrt(2.0), math.pi / 4.0, 1.0, 1.0, 1.0)

    assert len(coeffs) == 2
    assert coeffs[0]["A"] == pytest.approx(0.72)
    assert coeffs[0]["B"] == pytest.approx(-0.0344345399, abs=1e-9)
    assert coeffs[1]["B"] == pytest.approx(-1.4055654601, abs=1e-9)


def test_newton_finds_only_pinney_branches():
    scales = bridge_scales()
    matches = scales["matches"]

    for entry, match in zip(scales["extrema"], matches):
        assert entry["converged"] is True
        assert entry["max_residual"] <= 1e-9
        assert min(match) <= 1e-4


def test_both_branches_are_attained():
    from phenomenology.gaussian_bridge_report import run_gaussian_bridge_check

    payload = run_gaussian_bridge_check(write=False)
    attributed = payload["result"]["gates"]["attributed"]

    assert sorted(set(attributed)) == [0, 1]


def test_jacobi_primary_has_no_conjugate_point():
    scales = bridge_scales()

    assert scales["jacobi_primary_first_zero"] is None


def test_action_matches_analytic_quadrature():
    from theory.gaussian_bridge import analytic_action, discrete_action

    scales = bridge_scales()
    dt = scales["grids"]["T"] / (scales["grids"]["n"] - 1)
    analytic = scales["analytic_action"]
    for entry, match in zip(scales["extrema"], scales["matches"]):
        if int(np.argmin(match)) != 0:
            continue
        full = np.concatenate(
            [[scales["s0"]], np.array(entry["interior"]), [scales["sq"]]]
        )
        disc = discrete_action(full, 1.0, 1.0, 1.0, dt)
        assert abs(disc - analytic) <= 1e-4


def test_scaling_preserves_two_branches():
    from theory.gaussian_bridge import scaling_bundle

    bundle = scaling_bundle()

    assert len(bundle["coeffs"]) == 2
    assert bundle["sq"] == pytest.approx(0.5)
    for entry, match in zip(bundle["extrema"], bundle["matches"]):
        assert entry["converged"] is True
        assert min(match) <= 1e-4


def test_bridge_verdict_holds():
    from phenomenology.gaussian_bridge_report import run_gaussian_bridge_check

    payload = run_gaussian_bridge_check(write=False)
    gates = payload["result"]["gates"]
    verdict = payload["result"]["verdict"]

    assert not any(k for _, k in gates["kills"])
    assert verdict["claim_54_status"].startswith("NUMERIC")


def test_inputs_are_frozen():
    assert GRID_N == 401
    assert STARTS == ("const_s0", "const_sq", "linear", "bump", "vdip", "hbump")
    assert horizon(1.0) == pytest.approx(math.pi / 4.0)
    with pytest.raises(ValueError, match="degenerate"):
        pinney_coeffs(0.72, 0.5, math.pi, 1.0, 1.0, 1.0)
