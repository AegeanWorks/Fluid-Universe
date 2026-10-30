"""Self-consistent Born cavity synthesis (claim 39)."""

from __future__ import annotations

import pytest

from theory.water_born_cavity import (
    born_cavity_energy,
    born_cavity_equilibrium,
    born_cavity_scales,
)

R_EQ = 2.444456173312708e-10
R_REQ = 2.5178405289556243e-10


def test_equilibrium_values():
    cavity = born_cavity_equilibrium()

    assert cavity["radius_m"] == pytest.approx(R_EQ, rel=1e-7)
    assert cavity["depth_ev"] == pytest.approx(2.907799333, rel=1e-7)
    assert cavity["energy_1s_ev"] == pytest.approx(-0.436571724, rel=1e-7)
    assert cavity["binding_ev"] == pytest.approx(0.436571724, rel=1e-7)
    assert cavity["u_1p"] is None
    assert cavity["gap_ev"] is None
    assert cavity["surface_energy_ev"] == pytest.approx(0.341189266, rel=1e-7)
    assert cavity["total_energy_ev"] == pytest.approx(-0.095382458, rel=1e-7)


def test_equilibrium_is_a_minimum():
    middle = born_cavity_energy(R_EQ)
    left = born_cavity_energy(R_EQ - 1.0e-11)
    right = born_cavity_energy(R_EQ + 1.0e-11)

    assert middle < left
    assert middle < right


def test_reorganization_synthesis():
    scales = born_cavity_scales()
    equilibrium = scales["equilibrium"]
    synthesis = scales["synthesis"]

    assert equilibrium["radius_m"] == pytest.approx(R_EQ, rel=1e-7)
    assert synthesis["lambda_ev"] == pytest.approx(1.617132544, rel=1e-7)
    assert synthesis["predicted_vde_ev"] == pytest.approx(3.317132544, rel=1e-7)
    assert synthesis["residual_ev"] == pytest.approx(0.047132544, rel=1e-7)
    assert synthesis["sigma"] == pytest.approx(0.471325441, rel=1e-7)
    assert synthesis["required_radius_m"] == pytest.approx(R_REQ, rel=1e-7)
    assert synthesis["required_ratio"] == pytest.approx(1.030020729, rel=1e-7)
    assert synthesis["capillary_ratio"] == pytest.approx(1.169205276, rel=1e-7)
    assert synthesis["pressure_ratio"] == pytest.approx(1.625483510, rel=1e-7)


def test_no_bound_1p():
    cavity = born_cavity_equilibrium()

    assert cavity["u_1p"] is None
    assert cavity["gap_ev"] is None


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="radius"):
        born_cavity_energy(0.0)
    with pytest.raises(ValueError, match="bounds"):
        born_cavity_equilibrium(lower_m=3.0e-10, upper_m=2.0e-10)
