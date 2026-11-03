"""Yasui dynamical discriminator tests (claim 52)."""

from __future__ import annotations

import math

import numpy as np
import pytest

from theory.variational_dynamics import diffusivity
from theory.yasue_dynamics import (
    BASE,
    DT,
    FLOORS,
    RELEASE_FACTORS,
    T_END,
    yasue_acceleration,
    yasue_release,
    yasue_scales,
)


def test_yasui_collapses_with_no_return():
    scales = yasue_scales()

    for entry in scales["releases"]:
        assert entry["monotone"] is True
        assert entry["floor_hit_time"] is not None
        assert entry["floor_hit_time"] < math.pi
        assert entry["upward_crossings"] == 0


def test_gm_twin_controls_the_apparatus():
    scales = yasue_scales()

    for entry in scales["twins"]:
        assert math.isfinite(entry["period"])
        assert abs(entry["period"] - math.pi) <= 1e-3


def test_yasui_accel_negative_everywhere():
    grid = np.sqrt(np.linspace(0.125, 2.0, 2001))

    assert all(yasue_acceleration(float(s), 1.0, 1.0, 1.0) < 0.0 for s in grid)
    assert yasue_acceleration(0.5, 1.0, 1.0, 1.0) == pytest.approx(-2.5)


def test_quadrature_matches_verlet_at_mild_point():
    scales = yasue_scales()

    for entry, quad in zip(scales["releases"], scales["mild_quadrature"]):
        mild = entry["mild_crossing_time"]
        assert mild is not None
        assert abs(mild - quad) / quad <= 1e-3


def test_scaling_bounds_collapse():
    scales = yasue_scales()
    scaling = scales["scaling"]

    assert scaling["release"]["floor_hit_time"] is not None
    assert scaling["release"]["floor_hit_time"] < math.pi / 4.0
    assert abs(scaling["twin"]["period"] - math.pi / 2.0) <= 1e-3


def test_discriminator_verdict_holds():
    from phenomenology.yasue_dynamics_report import run_yasue_dynamics_check

    payload = run_yasue_dynamics_check(write=False)
    gates = payload["result"]["gates"]
    verdict = payload["result"]["verdict"]

    assert not any(k for _, k in gates["kills"])
    assert verdict["claim_52_status"].startswith("NUMERIC")


def test_inputs_are_frozen():
    assert BASE == {"omega": 1.0, "hbar": 1.0, "mass": 1.0}
    assert RELEASE_FACTORS == (1.01, 0.99, 1.5)
    assert FLOORS == (0.05, 0.02)
    assert DT == 0.001
    assert T_END == 2.0 * math.pi
    assert yasue_release(1.01)["mild_crossing_time"] is not None
    with pytest.raises(ValueError, match="positive"):
        yasue_acceleration(-1.0, 1.0, 1.0, 1.0)
