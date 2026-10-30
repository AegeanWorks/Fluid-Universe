"""Soft-wall cavity and the hard VDE depth bound (claim 36)."""

from __future__ import annotations

import pytest

from theory.water_softwall import (
    cavity_for_observables,
    depth_for_binding,
    woods_saxon_level,
)

R_CAPILLARY = 2.8580710555848763e-10
BORN_DEPTH = 2.4869878638766574
R_SQUARE = 3.7321228442651773e-10
V_SQUARE = 4.9975229011145625


def test_square_limit_matches_claim_35():
    e_1s = woods_saxon_level(R_SQUARE, V_SQUARE, 1.0e-13, 0)
    e_1p = woods_saxon_level(R_SQUARE, V_SQUARE, 1.0e-13, 1)

    assert e_1s == pytest.approx(-3.27, abs=0.01)
    assert e_1p == pytest.approx(-1.57, abs=0.01)
    assert e_1p - e_1s == pytest.approx(1.7, abs=0.01)


def test_born_depth_cannot_bind_the_vde():
    e_1s = woods_saxon_level(R_CAPILLARY, BORN_DEPTH, 1.0e-10, 0)

    assert e_1s > -BORN_DEPTH
    assert -e_1s < 3.27


def test_soft_wall_solution_at_one_angstrom():
    cavity = cavity_for_observables(1.0e-10, 3.27, 1.7)

    assert cavity["radius_m"] == pytest.approx(3.68655e-10, rel=1e-3)
    assert cavity["depth_ev"] == pytest.approx(5.91971, rel=1e-3)
    assert cavity["energy_1s_ev"] == pytest.approx(-3.27, abs=0.01)
    assert cavity["energy_1p_ev"] == pytest.approx(-1.57, abs=0.01)
    assert cavity["gap_ev"] == pytest.approx(1.7, abs=0.01)


def test_softness_trades_radius_for_depth():
    stiff = cavity_for_observables(1.0e-10, 3.27, 1.7)
    soft = cavity_for_observables(1.5e-10, 3.27, 1.7)

    assert soft["radius_m"] < stiff["radius_m"]
    assert soft["depth_ev"] > stiff["depth_ev"]
    assert soft["depth_ev"] > V_SQUARE
    assert stiff["depth_ev"] > V_SQUARE


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="radius"):
        woods_saxon_level(0.0, BORN_DEPTH, 1.0e-10, 0)
    with pytest.raises(ValueError, match="depth"):
        woods_saxon_level(R_CAPILLARY, 0.0, 1.0e-10, 0)
    with pytest.raises(ValueError, match="diffuseness"):
        woods_saxon_level(R_CAPILLARY, BORN_DEPTH, 0.0, 0)
    with pytest.raises(ValueError, match="angular"):
        woods_saxon_level(R_CAPILLARY, BORN_DEPTH, 1.0e-10, 2)
    with pytest.raises(ValueError, match="binding"):
        depth_for_binding(R_CAPILLARY, 1.0e-10, 0.0)
    with pytest.raises(ValueError, match="gap"):
        cavity_for_observables(1.0e-10, 3.27, 0.0)
