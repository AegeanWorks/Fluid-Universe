"""Moving-center Gaussian reduction tests (claim 23)."""

from __future__ import annotations

import numpy as np
import pytest

from theory.gaussian_fluid import (
    coherent_energy,
    quantum_width,
    simulate_coherent,
)


def test_center_follows_the_classical_oscillator():
    width = quantum_width(1.0)
    result = simulate_coherent(
        1.0, 0.0, width, 0.0, 1.0, duration=2.0 * np.pi, dt=1e-3
    )
    centers = result["x0"]
    times = result["times"]

    assert centers[0] == pytest.approx(1.0)
    assert centers[np.argmin(np.abs(times - np.pi))] == pytest.approx(-1.0, abs=1e-3)
    assert centers[-1] == pytest.approx(1.0, abs=1e-3)


def test_width_stays_at_the_fixed_point():
    width = quantum_width(1.0)
    result = simulate_coherent(
        1.0, 0.0, width, 0.0, 1.0, duration=2.0 * np.pi, dt=1e-3
    )

    assert np.max(np.abs(result["sigma"] - width)) < 1e-9
    energies = [
        coherent_energy((x, v, s, sd), 1.0)
        for x, v, s, sd in zip(
            result["x0"], result["v0"], result["sigma"], result["sigma_dot"]
        )
    ]
    assert max(energies) - min(energies) < 1e-6


def test_displaced_width_oscillates_with_energy_conservation():
    width = quantum_width(1.0)
    result = simulate_coherent(
        1.0, 0.0, 2.0 * width, 0.0, 1.0, duration=2.0 * np.pi, dt=1e-3
    )
    energies = [
        coherent_energy((x, v, s, sd), 1.0)
        for x, v, s, sd in zip(
            result["x0"], result["v0"], result["sigma"], result["sigma_dot"]
        )
    ]

    assert result["sigma"].min() < width < result["sigma"].max()
    assert max(energies) - min(energies) < 1e-6


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="sigma"):
        simulate_coherent(0.0, 0.0, 0.0, 0.0, 1.0, 1.0)
    with pytest.raises(ValueError, match="sigma"):
        coherent_energy((0.0, 0.0, 0.0, 0.0), 1.0)
