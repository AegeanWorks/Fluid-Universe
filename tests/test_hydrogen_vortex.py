"""Hydrogen 2p vortex tests (claim 26)."""

from __future__ import annotations

import math

import numpy as np
import pytest

from theory.hydrogen_vortex import (
    angular_momentum,
    azimuthal_velocity,
    circulation,
    loop_circulation,
    most_probable_radius,
    radial_probability,
)


def test_radial_probability_is_normalized():
    r = np.linspace(0.0, 80.0, 160001)
    integral = np.trapezoid(radial_probability(r, 1.0), r)

    assert integral == pytest.approx(1.0, rel=1e-6)


def test_most_probable_radius_is_four_bohr_radii():
    assert most_probable_radius(1.0) == pytest.approx(4.0)


def test_circulation_is_quantized():
    assert circulation(1) == pytest.approx(2.0 * math.pi)
    assert circulation(-1) == pytest.approx(-2.0 * math.pi)
    assert loop_circulation(2.0, math.pi / 2.0, 1) == pytest.approx(2.0 * math.pi)


def test_azimuthal_velocity_and_angular_momentum():
    assert azimuthal_velocity(2.0, math.pi / 2.0, 1) == pytest.approx(0.5)
    assert angular_momentum(1) == pytest.approx(1.0)


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="poles"):
        azimuthal_velocity(1.0, 0.0, 1)
    with pytest.raises(ValueError, match="positive"):
        most_probable_radius(0.0)
    with pytest.raises(ValueError, match="samples"):
        loop_circulation(1.0, math.pi / 2.0, 1, samples=4)
