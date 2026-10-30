"""Gaussian Madelung reduction tests (claim 22)."""

from __future__ import annotations

import numpy as np
import pytest

from theory.gaussian_fluid import (
    effective_potential,
    energy,
    ermakov_rhs,
    quantum_width,
    simulate,
)


def test_fixed_point_is_the_quantum_width():
    width = quantum_width(1.0)
    sigma_dot, acceleration = ermakov_rhs(width, 0.0, 1.0)

    assert width == pytest.approx(1.0 / np.sqrt(2.0))
    assert sigma_dot == 0.0
    assert acceleration == pytest.approx(0.0, abs=1e-14)


def test_effective_potential_has_a_minimum_at_the_quantum_width():
    width = quantum_width(1.0)
    center = effective_potential(width, 1.0)

    assert effective_potential(width * 0.9, 1.0) > center
    assert effective_potential(width * 1.1, 1.0) > center


def test_ermakov_oscillates_around_the_fixed_point():
    width = quantum_width(1.0)
    result = simulate(1.02 * width, 0.0, 1.0, duration=2.0 * np.pi, dt=1e-4)
    sigma = result["sigma"]
    times = result["times"]

    assert sigma.min() < width < sigma.max()
    crossings = [
        times[index]
        for index in range(1, len(sigma))
        if sigma[index - 1] < width <= sigma[index]
    ]
    assert len(crossings) >= 2
    period = (crossings[-1] - crossings[0]) / (len(crossings) - 1)
    assert period == pytest.approx(np.pi, rel=0.02)


def test_energy_is_conserved():
    width = quantum_width(1.0)
    result = simulate(2.0 * width, 0.0, 1.0, duration=3.0 * np.pi, dt=1e-3)
    values = [
        energy(s, v, 1.0)
        for s, v in zip(result["sigma"], result["sigma_dot"])
    ]
    drift = max(values) - min(values)

    assert drift < 1e-6


def test_width_scaling():
    assert quantum_width(4.0) == pytest.approx(0.5 * quantum_width(1.0))


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="positive"):
        quantum_width(0.0)
    with pytest.raises(ValueError, match="sigma"):
        ermakov_rhs(0.0, 0.0, 1.0)
    with pytest.raises(ValueError, match="sigma0"):
        simulate(0.0, 0.0, 1.0, 1.0)
