"""Temperature-dependent Born cavity (claim 41)."""

from __future__ import annotations

import pytest

from theory.water_temperature import (
    born_cavity_at_temperature,
    temperature_scales,
    water_dielectric_constant,
    water_surface_tension,
)

R0 = 2.4199224723012955e-10
R45 = 2.481377230067627e-10


def test_water_properties():
    assert water_dielectric_constant(0.0) == pytest.approx(87.740)
    assert water_dielectric_constant(25.0) == pytest.approx(78.30334375)
    assert water_surface_tension(0.0) == pytest.approx(0.07564)
    assert water_surface_tension(25.0) == pytest.approx(0.0719525)


def test_born_cavity_at_temperature():
    cold = born_cavity_at_temperature(0.0)
    warm = born_cavity_at_temperature(45.0)

    assert cold["radius_m"] == pytest.approx(R0, rel=1e-7)
    assert cold["depth_ev"] == pytest.approx(2.941318976, rel=1e-7)
    assert cold["energy_1s_ev"] == pytest.approx(-0.431765433, rel=1e-7)
    assert cold["u_1p"] is None
    assert cold["gap_ev"] is None
    assert cold["lambda_ev"] == pytest.approx(1.637567127, rel=1e-7)
    assert cold["predicted_vde_ev"] == pytest.approx(3.337567127, rel=1e-7)
    assert cold["fwhm_ev"] == pytest.approx(0.653821691, rel=1e-7)

    assert warm["radius_m"] == pytest.approx(R45, rel=1e-7)
    assert warm["lambda_ev"] == pytest.approx(1.589505512, rel=1e-7)
    assert warm["predicted_vde_ev"] == pytest.approx(3.289505512, rel=1e-7)
    assert warm["fwhm_ev"] == pytest.approx(0.695194215, rel=1e-7)


def test_temperature_scales():
    scales = temperature_scales()

    assert scales["lambda_slope_mev_per_k"] == pytest.approx(
        -1.068035887, rel=1e-7
    )
    assert scales["observed_shift_mev_per_k"] == pytest.approx(2.4)
    assert scales["observed_shift_upper_mev_per_k"] == pytest.approx(2.8)
    assert scales["fraction_of_lower"] == pytest.approx(0.445015, rel=1e-6)
    assert scales["fraction_of_upper"] == pytest.approx(0.3814414, rel=1e-6)
    assert scales["width_0c_ev"] == pytest.approx(0.653821691, rel=1e-7)
    assert scales["width_45c_ev"] == pytest.approx(0.695194215, rel=1e-7)


def test_no_bound_1p_at_any_temperature():
    for row in temperature_scales()["scan"]:
        assert row["u_1p"] is None
        assert row["gap_ev"] is None


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="temperature"):
        water_dielectric_constant(-1.0)
    with pytest.raises(ValueError, match="temperature"):
        water_surface_tension(101.0)
    with pytest.raises(ValueError, match="temperature"):
        born_cavity_at_temperature(101.0)
