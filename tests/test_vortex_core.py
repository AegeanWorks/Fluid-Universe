"""Finite-core node probe tests (claim 60)."""

from __future__ import annotations

import math

import numpy as np

from theory.vortex_core import (
    GRID_NS,
    core_phase,
    core_q_zero,
    core_radius,
    core_velocity,
    fd_q_zero,
)


def test_eps0_is_predicted():
    from theory.hydrogen_fluid import bohr_radius

    assert core_radius() == 0.1 * bohr_radius(1.0, 1.0, 1.0) == 0.1


def test_core_velocity_capped():
    x = np.array([0.0])
    v = core_velocity(x, 0.1)[0]
    assert abs(abs(v) - 10.0) <= 1e-12


def test_core_q_zero_closed_form():
    assert core_q_zero(0.1, 1.0) == -(0.5) * (1.0 / (0.1 * 0.1) - 1.0)
    assert abs(core_q_zero(0.1, 1.0) + 49.5) <= 1e-9


def test_core_winding_and_q_converge():
    from phenomenology.vortex_core_report import run_vortex_core_check

    payload = run_vortex_core_check(write=False)
    s = payload["result"]["scales"]
    assert abs(s["slips"][2048] - math.pi) <= 0.05
    assert abs(s["q_fd"][2048] - s["q_analytic"]) / abs(s["q_analytic"]) <= 0.05
    v = payload["result"]["verdict"]
    assert v["claim_60_status"].startswith(("NUMERIC", "FALSIFIED", "UNVERIFIED"))
