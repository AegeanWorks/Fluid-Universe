"""Dispersive shallow-water horizon tests (claim 29)."""

from __future__ import annotations

import math

import pytest

from theory.water_dispersive import (
    capillary_frequency,
    dispersive_horizon,
    dispersion,
    group_velocity,
    horizon_wavenumber,
)


def test_low_k_dispersion_and_group_velocity():
    depth = 0.1
    c = math.sqrt(9.81 * depth)
    k = 1e-6

    assert dispersion(k, depth) == pytest.approx(c * k, rel=1e-6)
    assert group_velocity(k, depth) == pytest.approx(c, rel=1e-6)


def test_low_frequency_horizon_approaches_the_long_wavelength_value():
    result = dispersive_horizon(0.1, 1e-3, 0.1)

    assert result["ratio"] == pytest.approx(1.000013, rel=1e-4)


def test_high_frequency_horizon_is_shifted_outward():
    result = dispersive_horizon(100.0, 1e-3, 0.1)

    assert result["ratio"] == pytest.approx(5.1924, rel=1e-3)


def test_capillary_frequency():
    assert capillary_frequency(0.1) == pytest.approx(84.8, rel=1e-3)


def test_horizon_wavenumber_solves_the_dispersion_condition():
    omega = 10.0
    k = horizon_wavenumber(omega, 0.1)
    residual = k * group_velocity(k, 0.1) + dispersion(k, 0.1) - omega

    assert abs(residual) < 1e-10


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="non-negative"):
        dispersion(-1.0, 0.1)
    with pytest.raises(ValueError, match="positive"):
        horizon_wavenumber(1.0, 0.0)
    with pytest.raises(ValueError, match="flow_rate"):
        dispersive_horizon(1.0, 0.0, 0.1)
