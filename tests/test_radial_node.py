"""Hydrogen-2s radial-node representation tests (claim 47)."""

from __future__ import annotations

import math

import numpy as np
import pytest

from theory.hydrogen_fluid import bohr_radius
from theory.hydrogen_radial_nodes import (
    BALANCE_RADII,
    LOOP_R,
    LOOP_THETA,
    WINDING_M,
    branch_split_20,
    node_position,
    overlap_10_20,
    peak_radii_20,
    phase_slip_20,
    representation_scales,
)


def test_branch_identities_are_exact():
    r = np.linspace(0.0, 60.0, 60001)
    split = branch_split_20(r)

    assert split["identity_residual"] == 0.0
    assert node_position() == pytest.approx(2.0, abs=1e-9)
    assert phase_slip_20(r) == pytest.approx(math.pi)


def test_norm_overlap_and_peaks():
    scales = representation_scales()
    radial = scales["radial"]

    assert radial["norm"] == pytest.approx(1.0, rel=1e-6)
    assert abs(radial["overlap_10_20"]) < 1e-6
    assert radial["outer_peak"] == pytest.approx(3.0 + math.sqrt(5.0), rel=1e-6)
    assert radial["inner_peak"] == pytest.approx(3.0 - math.sqrt(5.0), rel=1e-6)


def test_radius_comes_from_derived_bohr_radius():
    assert node_position() == pytest.approx(2.0 * bohr_radius(), abs=1e-12)
    assert peak_radii_20()[1] == pytest.approx(
        (3.0 + math.sqrt(5.0)) * bohr_radius(), rel=1e-12
    )


def test_preserved_1s_2p_numbers_hold():
    scales = representation_scales()
    preserved = scales["preserved"]

    assert np.allclose(preserved["balance_residuals"], 0.0, atol=1e-12)
    assert preserved["loop_circulation"] == pytest.approx(
        2.0 * math.pi, rel=1e-6
    )


def test_representation_verdict_holds():
    from phenomenology.radial_node_report import run_radial_node_check

    payload = run_radial_node_check(write=False)
    verdict = payload["result"]["verdict"]
    kills = payload["result"]["gates"]["kills"]

    assert not any(kills.values())
    assert verdict["claim_47_status"].startswith("NUMERIC")


def test_inputs_are_frozen():
    assert BALANCE_RADII == (0.5, 1.0, 2.0, 5.0)
    assert LOOP_R == 2.0
    assert LOOP_THETA == math.pi / 2.0
    assert WINDING_M == 1
    assert overlap_10_20() == pytest.approx(0.0, abs=1e-6)
