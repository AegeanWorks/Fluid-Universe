"""Dimensional-reduction and moment-of-inertia tests (claim 6b)."""

from __future__ import annotations

import math

import pytest

from theory.reduction import (
    moment_finite,
    moment_log_divergent,
    moment_numeric,
    moment_power,
    wall_lower_bound_2d,
    wall_tension,
)
from phenomenology.uv_negatives import run_reduction_check


def test_wall_tension_and_bound():
    assert wall_tension(1.0) == pytest.approx(8.0 / 3.0)
    assert wall_lower_bound_2d(1.0, 2.0) == pytest.approx(16.0 / 3.0)
    assert wall_lower_bound_2d(0.0, 5.0) == 0.0


def test_moment_convergence_criterion():
    assert moment_finite(5.0)
    assert not moment_finite(4.0)
    assert not moment_finite(2.0)
    assert moment_log_divergent(4.0)
    assert moment_power(4.0) == pytest.approx(-1.0)


def test_numeric_moment_log_and_power():
    # r^-4: I ~ 2*pi log(R/r_min)
    assert moment_numeric(4.0, 1.0, 100.0) == pytest.approx(2.0 * math.pi * math.log(100.0))
    # r^-2: I ~ 2*pi (R^2 - 1)/2
    assert moment_numeric(2.0, 1.0, 10.0) == pytest.approx(2.0 * math.pi * (100.0 - 1.0) / 2.0)
    # r^-5: converges to 2*pi as R grows.
    a = moment_numeric(5.0, 1.0, 10.0)
    b = moment_numeric(5.0, 1.0, 100.0)
    assert b > a
    assert b == pytest.approx(2.0 * math.pi, rel=1e-2)


def test_verdict_downgrades_exact_reduction():
    payload = run_reduction_check(write=False)
    v = payload["verdict"]
    assert v["6b_i_trivial_lower_bound"] is True
    assert v["6b_ii_exact_dimensional_reduction"] is False
    assert v["6b_iii_moment_finite_for_stated_profile"] is False
    assert v["stated_tail_status"] == "log-divergent"
