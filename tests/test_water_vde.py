"""VDE and 1.7 eV absorption constrain the finite well (claim 35)."""

from __future__ import annotations

import pytest

from theory.water_vde import (
    cavity_for_observables,
    depth_for_binding,
    vde_scales,
)

R_REQUIRED = 3.7321228442651773e-10
V_REQUIRED = 4.9975229011145625


def test_depth_for_binding_at_required_radius():
    depth = depth_for_binding(R_REQUIRED, 3.27)

    assert depth == pytest.approx(V_REQUIRED, rel=1e-7)


def test_cavity_satisfies_both_observables():
    cavity = cavity_for_observables(3.27, 1.7)

    assert cavity["radius_m"] == pytest.approx(R_REQUIRED, rel=1e-7)
    assert cavity["depth_ev"] == pytest.approx(V_REQUIRED, rel=1e-7)
    assert cavity["energy_1s_ev"] == pytest.approx(-3.27, rel=1e-7)
    assert cavity["energy_1p_ev"] == pytest.approx(-1.57, rel=1e-7)
    assert cavity["gap_ev"] == pytest.approx(1.7, rel=1e-7)
    assert cavity["u_1s"] == pytest.approx(2.513080317, rel=1e-7)
    assert cavity["u_1p"] == pytest.approx(3.539848250, rel=1e-7)
    assert cavity["born_depth_ev"] == pytest.approx(1.904542890, rel=1e-7)
    assert cavity["excess_ev"] == pytest.approx(3.092980011, rel=1e-7)


def test_vde_uncertainty_band():
    lower = cavity_for_observables(3.17, 1.7)
    upper = cavity_for_observables(3.37, 1.7)

    assert lower["radius_m"] == pytest.approx(3.714320539e-10, rel=1e-7)
    assert lower["depth_ev"] == pytest.approx(4.902844700, rel=1e-7)
    assert upper["radius_m"] == pytest.approx(3.749021247e-10, rel=1e-7)
    assert upper["depth_ev"] == pytest.approx(5.092648465, rel=1e-7)


def test_minimal_cavity_is_outside_the_band():
    scales = vde_scales()
    central = scales["central"]
    capillary = scales["capillary"]

    assert capillary["radius_m"] < scales["lower"]["radius_m"]
    assert capillary["born_depth_ev"] < scales["lower"]["depth_ev"]
    assert central["radius_m"] > 1.25 * capillary["radius_m"]
    assert central["depth_ev"] > 1.9 * capillary["born_depth_ev"]
    assert capillary["binding_ev"] == pytest.approx(0.527752862, rel=1e-7)


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="radius"):
        depth_for_binding(0.0, 3.27)
    with pytest.raises(ValueError, match="binding"):
        depth_for_binding(R_REQUIRED, 0.0)
    with pytest.raises(ValueError, match="binding"):
        cavity_for_observables(0.0, 1.7)
    with pytest.raises(ValueError, match="gap"):
        cavity_for_observables(3.27, 0.0)
