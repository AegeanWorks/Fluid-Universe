"""Variational dynamics tests (claim 51)."""

from __future__ import annotations

import math

import numpy as np
import pytest

from theory.gaussian_fluid import ermakov_rhs, quantum_width
from theory.variational_dynamics import (
    BASE,
    GRID_N,
    HORIZON_FRACTION,
    START_FACTORS,
    TEST_SIGMAS,
    acceleration,
    confining_term,
    dynamics_scales,
    newton_extremum,
    release_period,
    repulsion_term,
)


def test_discrete_extremum_is_constant_path():
    for factor in START_FACTORS:
        out = newton_extremum(factor)

        assert out["converged"] is True
        assert out["max_deviation"] <= 1e-9
        assert out["max_residual"] <= 1e-12


def test_el_residual_at_constant_path():
    from theory.variational_dynamics import discrete_el_residual, grid

    width = quantum_width(1.0)
    t = grid()
    dt = float(t[1] - t[0])
    const = np.full(GRID_N - 2, width)

    assert float(np.max(np.abs(
        discrete_el_residual(const, 1.0, 1.0, 1.0, dt, endpoint=width)
    ))) <= 1e-12


def test_release_period_is_pi():
    for factor, expected in ((1.01, math.pi), (0.99, math.pi)):
        out = release_period(factor)

        assert math.isfinite(out["period"])
        assert abs(out["period"] - expected) <= 1e-3
        assert len(out["crossings"]) >= 3


def test_term_match_is_exact():
    for s in TEST_SIGMAS:
        own = acceleration(s, 1.0, 1.0, 1.0)
        _, ermakov = ermakov_rhs(s, 0.0, 1.0, 1.0, 1.0)
        rep_own = repulsion_term(s, 1.0, 1.0, 1.0)
        rep_ermakov = 1.0 / (4.0 * s * s * s)

        assert own == pytest.approx(float(ermakov), rel=1e-9)
        assert rep_own == pytest.approx(rep_ermakov, rel=1e-9)
        assert confining_term(s, 1.0, 1.0, 1.0) == pytest.approx(-s, rel=0.0)


def test_scaling_period_is_pi_over_two():
    out = release_period(1.01, omega=2.0)

    assert math.isfinite(out["period"])
    assert abs(out["period"] - math.pi / 2.0) <= 1e-3


def test_dynamics_verdict_holds():
    from phenomenology.variational_dynamics_report import (
        run_variational_dynamics_check,
    )

    payload = run_variational_dynamics_check(write=False)
    gates = payload["result"]["gates"]
    verdict = payload["result"]["verdict"]

    assert not any(k for _, k in gates["kills"])
    assert verdict["claim_51_status"].startswith("NUMERIC")


def test_inputs_are_frozen():
    assert BASE == {"omega": 1.0, "hbar": 1.0, "mass": 1.0}
    assert GRID_N == 401
    assert HORIZON_FRACTION == 0.25
    assert START_FACTORS == (1.0, 1.05, 0.95, 1.5)
    assert TEST_SIGMAS == (0.5, 0.7071067811865476, 1.0, 1.5)
    with pytest.raises(ValueError, match="positive"):
        acceleration(-1.0, 1.0, 1.0, 1.0)
