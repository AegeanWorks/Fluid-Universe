"""Yasue-bridge analytic family tests (claim 56)."""

from __future__ import annotations

import math

import numpy as np
import pytest

from theory.yasue_bridge import (
    BASE,
    GRID_N,
    attribution_diff,
    diffusivity,
    fd_ode_residual,
    horizon,
    jacobi_first_zero_yasue,
    newton_width,
    nu_limit_bridge,
    pinney_coeffs_yasue,
    pinney_path,
)


def test_branch_coefficients_match_endpoint_algebra():
    sq = math.sqrt(0.5)
    branches, disc = pinney_coeffs_yasue(sq, sq, math.pi / 4.0, 1.0, 1.0, 1.0)

    assert disc == pytest.approx(3.0)
    assert len(branches) == 2
    assert branches[0]["B"] == pytest.approx(0.3660254037844386)
    assert branches[0]["C"] == pytest.approx(-0.2320508075688772)
    assert branches[1]["B"] == pytest.approx(-1.3660254037844386)
    assert branches[0]["A"] * branches[0]["C"] - branches[0]["B"] ** 2 == pytest.approx(-0.25)


def test_b1_admissible_b2_killed_sibling():
    sq = math.sqrt(0.5)
    branches, _ = pinney_coeffs_yasue(sq, sq, math.pi / 4.0, 1.0, 1.0, 1.0)

    assert branches[0]["admissible"] is True
    assert branches[0]["min_s2"] == pytest.approx(0.5)
    assert branches[1]["admissible"] is False
    assert branches[1]["min_s2"] == pytest.approx(-0.0658262488)


def test_ode_identity_holds_term_by_term():
    sq = math.sqrt(0.5)
    branches, _ = pinney_coeffs_yasue(sq, sq, math.pi / 4.0, 1.0, 1.0, 1.0)
    fd = fd_ode_residual(branches[0], 1.0, 1.0, 1.0, math.pi / 4.0)

    assert fd["max_abs"] <= 1e-6


def test_fresh_newton_attains_and_matches_family():
    sq = math.sqrt(0.5)
    branches, _ = pinney_coeffs_yasue(sq, sq, math.pi / 4.0, 1.0, 1.0, 1.0)
    solves = newton_width(sq, sq, math.pi / 4.0)
    best = min(solves, key=lambda s: s["max_residual"])

    assert best["converged"] is True
    grid = np.linspace(0.0, math.pi / 4.0, GRID_N)
    analytic = pinney_path(branches[0], grid, 1.0)[1:-1].tolist()
    assert attribution_diff(analytic, best["interior"]) <= 1e-4


def test_jacobi_zero_free_with_yasue_sign():
    sq = math.sqrt(0.5)
    branches, _ = pinney_coeffs_yasue(sq, sq, math.pi / 4.0, 1.0, 1.0, 1.0)
    fine_t = np.linspace(0.0, math.pi / 4.0, 20001)
    fine_s = pinney_path(branches[0], fine_t, 1.0)

    assert jacobi_first_zero_yasue(fine_t, fine_s, 1.0, 0.5) is None

    branches2, _ = pinney_coeffs_yasue(0.5, 0.5, math.pi / 8.0, 2.0, 1.0, 1.0)
    fine_t2 = np.linspace(0.0, math.pi / 8.0, 20001)
    fine_s2 = pinney_path(branches2[0], fine_t2, 2.0)

    assert jacobi_first_zero_yasue(fine_t2, fine_s2, 2.0, 0.5) is None


def test_scaling_preserves_branch_structure():
    branches2, disc2 = pinney_coeffs_yasue(0.5, 0.5, math.pi / 8.0, 2.0, 1.0, 1.0)

    assert disc2 == pytest.approx(3.0)
    assert branches2[0]["B"] == pytest.approx(0.3660254037844386)
    assert branches2[1]["B"] == pytest.approx(-1.3660254037844386)
    assert branches2[0]["admissible"] is True
    assert branches2[1]["admissible"] is False


def test_nu_zero_limit_is_oscillator_bridge():
    limit = nu_limit_bridge()

    assert limit["alpha"] == pytest.approx(math.sqrt(2.0) - 1.0)
    assert limit["peak_ratio"] == pytest.approx(1.0823922003)

    solves = newton_width(math.sqrt(0.5), math.sqrt(0.5), math.pi / 4.0, nu=0.0)
    best = min(solves, key=lambda s: s["max_residual"])

    assert best["converged"] is True
    assert best["max_sigma"] / math.sqrt(0.5) == pytest.approx(1.0823922003, abs=1e-4)


def test_bridge_verdict_holds():
    from phenomenology.yasue_bridge_report import run_yasue_bridge_check

    payload = run_yasue_bridge_check(write=False)
    gates = payload["result"]["gates"]
    verdict = payload["result"]["verdict"]

    assert not any(k for _, k in gates["kills_hard"])
    assert not any(k for _, k in gates["kills_soft"])
    assert verdict["claim_56_status"].startswith("NUMERIC")


def test_inputs_are_frozen():
    assert BASE == {"omega": 1.0, "hbar": 1.0, "mass": 1.0}
    assert GRID_N == 401
    assert diffusivity(1.0, 1.0) == 0.5
    assert horizon(1.0) == pytest.approx(math.pi / 4.0)
    with pytest.raises(ValueError, match="positive"):
        diffusivity(-1.0, 1.0)
    with pytest.raises(ValueError, match="positive"):
        horizon(0.0)
