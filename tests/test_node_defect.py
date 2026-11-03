"""Two-branch defect representation tests (claim 46)."""

from __future__ import annotations

import math

import numpy as np
import pytest

from theory.node_defect import (
    GRID_LENGTH,
    GRID_NS,
    LOOP_R,
    LOOP_THETA,
    WINDING_M,
    branch_split,
    representation_scales,
)


def test_branch_identities_are_exact():
    x = (np.arange(256) - 128) * (GRID_LENGTH / 256)
    split = branch_split(x, 1.0)

    assert split["identity_residual"] == 0.0
    assert split["node_density"] == 0.0
    assert split["phase_slip"] == pytest.approx(math.pi)


def test_node_scaling_meets_kill_gate():
    scales = representation_scales()
    slope = scales["scaling"]["slope"]

    assert slope == pytest.approx(-1.0, abs=0.01)
    assert scales["scaling"]["grid_ns"] == [256, 512, 1024, 2048]


def test_vortex_gate_is_quantized():
    scales = representation_scales()
    vortex = scales["vortex"]

    assert vortex["circulation"] == pytest.approx(2.0 * math.pi, rel=1e-6)
    assert vortex["loop_circulation"] == pytest.approx(2.0 * math.pi, rel=1e-6)
    assert vortex["azimuthal_velocity"] == pytest.approx(0.5)
    assert vortex["angular_momentum"] == pytest.approx(1.0)
    assert vortex["most_probable_radius"] == pytest.approx(4.0)
    assert vortex["radial_norm"] == pytest.approx(1.0, rel=1e-6)


def test_representation_verdict_holds():
    from phenomenology.node_defect_report import run_node_defect_check

    payload = run_node_defect_check(write=False)
    verdict = payload["result"]["verdict"]
    gates = payload["result"]["gates"]

    assert gates["slope_kill"] is False
    assert gates["circulation_kill"] is False
    assert gates["loop_kill"] is False
    assert verdict["claim_46_status"].startswith("NUMERIC")


def test_inputs_are_frozen():
    assert GRID_NS == (256, 512, 1024, 2048)
    assert GRID_LENGTH == 20.0
    assert LOOP_R == 2.0
    assert LOOP_THETA == math.pi / 2.0
    assert WINDING_M == 1
