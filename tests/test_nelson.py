"""Nelson stochastic kinematics tests (claim 21)."""

from __future__ import annotations

import numpy as np
import pytest

from theory.madelung import free_gaussian_density
from theory.nelson import (
    free_packet_drift,
    ground_state_density,
    ground_state_width,
    histogram_density,
    l1_error,
    simulate_free_packet,
    simulate_ou,
)


def test_ground_state_width():
    assert ground_state_width(1.0) == pytest.approx(1.0 / np.sqrt(2.0))


def test_ou_stationary_variance():
    samples = simulate_ou(5000, 1.0, steps=2000, dt=1e-3, seed=1)

    assert np.var(samples) == pytest.approx(0.5, rel=0.1)


def test_ou_histogram_matches_ground_state():
    samples = simulate_ou(10000, 1.0, steps=2000, dt=1e-3, seed=1)
    density, centers, width = histogram_density(samples, bins=60, span=5.0)
    exact = ground_state_density(centers, 1.0)

    assert l1_error(density, exact, width) < 0.1


def test_free_packet_drift_at_zero():
    x = np.array([0.0, 1.0, -1.0])
    drift = free_packet_drift(x, 0.0, 1.0)

    assert drift[0] == pytest.approx(0.0)
    assert drift[1] == pytest.approx(-0.5)
    assert drift[2] == pytest.approx(0.5)


def test_free_packet_histogram_matches_exact():
    samples = simulate_free_packet(10000, 1.0, 1.0, steps=500, seed=1)
    density, centers, width = histogram_density(samples, bins=60, span=6.0)
    exact = free_gaussian_density(centers, 1.0, 1.0)

    assert l1_error(density, exact, width) < 0.15


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="positive"):
        ground_state_width(0.0)
    with pytest.raises(ValueError, match="particles"):
        simulate_ou(0, 1.0)
    with pytest.raises(ValueError, match="sigma"):
        free_packet_drift(np.zeros(4), 0.0, 0.0)
