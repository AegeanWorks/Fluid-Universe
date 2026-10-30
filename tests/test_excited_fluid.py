"""First-excited-state node tests (claim 24)."""

from __future__ import annotations

import numpy as np
import pytest

from theory.excited_fluid import (
    excited_density,
    excited_quantum_pressure_regular,
    excited_wavefunction,
    phase_gradient_max,
    phase_slip,
    scaling_slope,
)


def _grid(n: int = 256, length: float = 20.0):
    dx = length / n
    x = (np.arange(n) - n // 2) * dx
    return x, dx


def test_density_is_normalized():
    x, dx = _grid(2048)
    density = excited_density(x, 1.0)

    assert np.sum(density) * dx == pytest.approx(1.0, rel=1e-6)


def test_regular_quantum_pressure():
    sigma = 1.0
    at_zero = excited_quantum_pressure_regular(np.array([0.0]), 1.0)[0]
    at_sigma = excited_quantum_pressure_regular(np.array([sigma]), 1.0)[0]

    assert at_zero == pytest.approx(1.5)
    assert at_sigma == pytest.approx(1.0)


def test_phase_slip_and_gradient():
    x, dx = _grid(256)
    psi = excited_wavefunction(x, 1.0)

    assert phase_slip(psi) == pytest.approx(np.pi)
    assert phase_gradient_max(psi, dx) == pytest.approx(
        np.pi / (2.0 * dx), rel=1e-10
    )


def test_phase_gradient_scales_as_inverse_dx():
    dx_values = []
    maxima = []
    for n in (256, 512, 1024, 2048):
        x, dx = _grid(n)
        dx_values.append(dx)
        maxima.append(phase_gradient_max(excited_wavefunction(x, 1.0), dx))

    assert scaling_slope(np.array(dx_values), np.array(maxima)) == pytest.approx(
        -1.0, abs=1e-10
    )


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="positive"):
        excited_density(np.zeros(4), 0.0)
    with pytest.raises(ValueError, match="dx"):
        phase_gradient_max(np.ones(4), 0.0)
    with pytest.raises(ValueError, match="same length"):
        scaling_slope(np.array([0.1]), np.array([1.0]))
