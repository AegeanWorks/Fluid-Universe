"""Self-consistent finite-well pressure balance (claim 37)."""

from __future__ import annotations

import pytest

from theory.water_pressure import (
    equilibrium_radius,
    surface_energy_ev,
    total_energy_ev,
    vde_matched_depth,
)

BORN_DEPTH = 2.4869878638766574
R_PRESSURE = 3.973423200472016e-10
R_VDE = 4.3358778604407663e-10
V_VDE = 4.610696624852577


def test_surface_energy_scale():
    assert surface_energy_ev(R_PRESSURE) == pytest.approx(0.9015, abs=1e-3)


def test_equilibrium_at_born_depth():
    cavity = equilibrium_radius(BORN_DEPTH)

    assert cavity["radius_m"] == pytest.approx(R_PRESSURE, rel=1e-7)
    assert cavity["energy_1s_ev"] == pytest.approx(-1.177350961, rel=1e-7)
    assert cavity["binding_ev"] == pytest.approx(1.177350961, rel=1e-7)
    assert cavity["gap_ev"] == pytest.approx(1.132993301, rel=1e-7)
    assert cavity["u_1p"] == pytest.approx(3.181502116, rel=1e-7)
    assert cavity["total_energy_ev"] == pytest.approx(-0.275861827, rel=1e-7)


def test_pressure_minimum_is_a_minimum():
    middle = total_energy_ev(R_PRESSURE, BORN_DEPTH)
    left = total_energy_ev(R_PRESSURE - 5.0e-12, BORN_DEPTH)
    right = total_energy_ev(R_PRESSURE + 5.0e-12, BORN_DEPTH)

    assert middle < left
    assert middle < right


def test_vde_matched_depth():
    cavity = vde_matched_depth(3.27)

    assert cavity["depth_ev"] == pytest.approx(V_VDE, rel=1e-7)
    assert cavity["radius_m"] == pytest.approx(R_VDE, rel=1e-7)
    assert cavity["binding_ev"] == pytest.approx(3.27, rel=1e-7)
    assert cavity["gap_ev"] == pytest.approx(1.344125074, rel=1e-7)
    assert cavity["u_1p"] == pytest.approx(3.639764595, rel=1e-7)


def test_capillary_radius_is_not_the_pressure_minimum():
    fixed = total_energy_ev(2.8580710555848763e-10, BORN_DEPTH)
    balanced = total_energy_ev(R_PRESSURE, BORN_DEPTH)

    assert balanced < fixed


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="radius"):
        surface_energy_ev(0.0)
    with pytest.raises(ValueError, match="depth"):
        equilibrium_radius(0.0)
    with pytest.raises(ValueError, match="binding"):
        vde_matched_depth(0.0)
