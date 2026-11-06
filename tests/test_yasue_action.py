"""Yasue-bridge action stationarity tests (claim 57)."""

from __future__ import annotations

import math

import numpy as np
import pytest

from theory.yasue_action import (
    ACTION_TOL,
    BASE,
    GRID_N,
    PROBE_TOL,
    classical_bridge_action,
    probe_slope,
    variation_probe,
    yasue_analytic_action,
    yasue_discrete_action,
)
from theory.yasue_bridge import (
    diffusivity,
    horizon,
    newton_width,
    pinney_coeffs_yasue,
    pinney_path,
)
from theory.gaussian_fluid import quantum_width


def test_action_densities_match_on_recorded_arc():
    sq = math.sqrt(0.5)
    T = math.pi / 4.0
    branches, _ = pinney_coeffs_yasue(sq, sq, T, 1.0, 1.0, 1.0)
    b1 = next(b for b in branches if b["admissible"])
    grid = np.linspace(0.0, T, GRID_N)
    dt = T / (GRID_N - 1)

    solves = newton_width(sq, sq, T)
    best = min(solves, key=lambda s: s["max_residual"])

    assert best["converged"] is True
    full = np.concatenate([[sq], np.array(best["interior"]), [sq]])
    j_discrete = yasue_discrete_action(full, 1.0, 1.0, 1.0, 0.5, dt)
    j_analytic = yasue_analytic_action(b1, 1.0, 1.0, 1.0, 0.5, T)

    assert abs(j_discrete - j_analytic) <= 1e-4


def test_classical_closed_form_value():
    closed = classical_bridge_action(math.sqrt(0.5), 1.0, 1.0)

    assert closed["alpha"] == pytest.approx(math.sqrt(2.0) - 1.0)
    assert closed["value"] == pytest.approx(-0.20710678118654752)


def test_nu0_action_matches_closed_form():
    a = math.sqrt(0.5)
    T = math.pi / 4.0
    dt = T / (GRID_N - 1)
    solves = newton_width(a, a, T, nu=0.0)
    best = min(solves, key=lambda s: s["max_residual"])

    assert best["converged"] is True
    full = np.concatenate([[a], np.array(best["interior"]), [a]])
    j_discrete = yasue_discrete_action(full, 1.0, 1.0, 1.0, 0.0, dt)
    closed = classical_bridge_action(a, 1.0, 1.0)

    assert abs(j_discrete - closed["value"]) <= 1e-4


def test_probe_slopes_flat_two_shapes():
    sq = math.sqrt(0.5)
    T = math.pi / 4.0
    dt = T / (GRID_N - 1)
    branches, _ = pinney_coeffs_yasue(sq, sq, T, 1.0, 1.0, 1.0)
    b1 = next(b for b in branches if b["admissible"])
    grid = np.linspace(0.0, T, GRID_N)
    base = pinney_path(b1, grid, 1.0)

    for kind in ("fundamental", "harmonic"):
        fam = variation_probe(base, T, kind)
        for lam in (0.01, 0.02):
            row = probe_slope(base, fam["bump"], 1.0, 1.0, 1.0, 0.5, dt, lam=lam)

            assert abs(row["slope"]) <= 1e-6


def test_scaling_action_pair_matches():
    branches, _ = pinney_coeffs_yasue(0.5, 0.5, math.pi / 8.0, 2.0, 1.0, 1.0)
    b1 = next(b for b in branches if b["admissible"])
    solves = newton_width(0.5, 0.5, math.pi / 8.0, omega=2.0)
    best = min(solves, key=lambda s: s["max_residual"])

    assert best["converged"] is True
    full = np.concatenate([[0.5], np.array(best["interior"]), [0.5]])
    dt2 = (math.pi / 8.0) / (GRID_N - 1)
    j_discrete = yasue_discrete_action(full, 2.0, 1.0, 1.0, 0.5, dt2)
    j_analytic = yasue_analytic_action(b1, 2.0, 1.0, 1.0, 0.5, math.pi / 8.0)

    assert abs(j_discrete - j_analytic) <= 1e-4


def test_b2_form_admits_no_action():
    branches, _ = pinney_coeffs_yasue(
        math.sqrt(0.5), math.sqrt(0.5), math.pi / 4.0, 1.0, 1.0, 1.0
    )
    b2 = next(b for b in branches if not b["admissible"])

    assert b2["min_s2"] < 0.0


def test_action_verdict_holds():
    from phenomenology.yasue_action_report import run_yasue_action_check

    payload = run_yasue_action_check(write=False)
    gates = payload["result"]["gates"]
    verdict = payload["result"]["verdict"]

    assert not any(k for _, k in gates["kills_hard"])
    assert not any(k for _, k in gates["kills_soft"])
    assert verdict["claim_57_status"].startswith("NUMERIC")


def test_inputs_are_frozen():
    assert BASE == {"omega": 1.0, "hbar": 1.0, "mass": 1.0}
    assert GRID_N == 401
    assert ACTION_TOL == 1e-4
    assert PROBE_TOL == 1e-6
    assert diffusivity(1.0, 1.0) == 0.5
    assert horizon(1.0) == pytest.approx(math.pi / 4.0)
    assert quantum_width(1.0, 1.0, 1.0) == pytest.approx(math.sqrt(0.5))
    with pytest.raises(ValueError, match="unknown bump"):
        variation_probe(np.ones(11), 1.0, "sawtooth")
