"""Hydrated-electron cavity scale tests (claim 32)."""

from __future__ import annotations

import pytest

from theory.water_electron import (
    capillary_cavity_energy_ev,
    capillary_cavity_radius,
    hydrated_scales,
    infinite_well_gap_ev,
    screened_bohr_radius,
    screened_energy_ev,
    well_radius_for_gap,
)


def test_screened_hydrogen_scales():
    assert screened_bohr_radius() == pytest.approx(4.148749e-9, rel=1e-5)
    assert screened_energy_ev() == pytest.approx(-0.0022135, rel=1e-4)


def test_capillary_cavity_minimum():
    radius = capillary_cavity_radius()

    assert radius == pytest.approx(2.858071e-10, rel=1e-5)
    assert capillary_cavity_energy_ev() == pytest.approx(0.932839, rel=1e-4)


def test_infinite_well_scales():
    assert infinite_well_gap_ev(2.5e-10) == pytest.approx(6.01648, rel=1e-4)
    assert well_radius_for_gap(1.7) == pytest.approx(4.70313e-10, rel=1e-4)


def test_hydrated_scale_summary():
    scales = hydrated_scales()

    assert scales["cavity_radius_m"] == pytest.approx(2.858071e-10, rel=1e-5)
    assert 1.0 < scales["well_gap_at_cavity_ev"] < 5.0


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="epsilon"):
        screened_bohr_radius(0.0)
    with pytest.raises(ValueError, match="gamma"):
        capillary_cavity_radius(gamma=0.0)
    with pytest.raises(ValueError, match="radius"):
        infinite_well_gap_ev(0.0)
