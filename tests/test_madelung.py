"""Madelung fluid dictionary tests (claim 20)."""

from __future__ import annotations

import numpy as np
import pytest

from theory.madelung import (
    coherent_state_density,
    continuity_residual,
    euler_residual_fields,
    fluid_fields,
    free_gaussian_density,
    free_gaussian_fields,
    split_step,
)


def _grid(n: int = 256, length: float = 20.0):
    dx = length / n
    x = (np.arange(n) - n // 2) * dx
    return x, dx


def test_free_gaussian_density_matches_exact():
    x, dx = _grid()
    sigma = 1.0
    psi = (2.0 * np.pi * sigma**2) ** -0.25 * np.exp(-x * x / (4.0 * sigma**2))
    final = split_step(psi, 0.0, 1e-3, 500, dx)
    rho = np.abs(final) ** 2
    exact = free_gaussian_density(x, 0.5, sigma)

    assert np.sqrt(np.sum((rho - exact) ** 2) * dx) < 1e-10


def test_coherent_state_density_matches_exact():
    x, dx = _grid()
    x0 = 2.0
    sigma = 1.0 / np.sqrt(2.0)
    psi = (2.0 * np.pi * sigma**2) ** -0.25 * np.exp(
        -((x - x0) ** 2) / (4.0 * sigma**2)
    )
    potential = 0.5 * x * x
    final = split_step(psi, potential, 1e-4, int(np.pi / 2 / 1e-4), dx)
    rho = np.abs(final) ** 2
    exact = coherent_state_density(x, np.pi / 2, x0)

    assert np.sqrt(np.sum((rho - exact) ** 2) * dx) < 1e-3


def test_quantum_potential_of_a_gaussian():
    x, dx = _grid(n=512, length=40.0)
    sigma = 1.0
    psi = (2.0 * np.pi * sigma**2) ** -0.25 * np.exp(-x * x / (4.0 * sigma**2))
    _rho, _v, quantum = fluid_fields(psi, dx)
    center = np.argmin(np.abs(x))
    at_sigma = np.argmin(np.abs(x - sigma))

    assert quantum[center] == pytest.approx(1.0 / (4.0 * sigma**2), rel=1e-3)
    expected = (1.0 / (4.0 * sigma**2)) * (
        1.0 - x[at_sigma] ** 2 / (2.0 * sigma**2)
    )
    assert quantum[at_sigma] == pytest.approx(expected, rel=1e-2)


def test_exact_gaussian_satisfies_the_euler_equation():
    x, dx = _grid(n=512, length=40.0)
    t = 0.1
    dt = 1e-5
    rho0, v0, q0 = free_gaussian_fields(x, t - dt, 1.0)
    rho1, v1, q1 = free_gaussian_fields(x, t + dt, 1.0)

    assert (
        euler_residual_fields(rho0, v0, q0, rho1, v1, q1, 2.0 * dt, dx)
        < 1e-2
    )


def test_fluid_residuals_are_small():
    x, dx = _grid()
    sigma = 1.0
    psi = (2.0 * np.pi * sigma**2) ** -0.25 * np.exp(-x * x / (4.0 * sigma**2))
    dt = 1e-3
    final = split_step(psi, 0.0, dt, 1, dx)

    assert continuity_residual(psi, final, dt, dx) < 1e-2


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="dt"):
        split_step(np.ones(8), 0.0, 0.0, 1, 0.1)
    with pytest.raises(ValueError, match="sigma"):
        free_gaussian_density(np.zeros(8), 0.0, 0.0)
    with pytest.raises(ValueError, match="omega"):
        coherent_state_density(np.zeros(8), 0.0, 0.0, omega=0.0)
