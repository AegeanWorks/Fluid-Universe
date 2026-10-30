"""Gaussian reduction of the Madelung fluid (claim 22, ADR-013).

For a Gaussian density with v = A(t) x, continuity gives sigma_dot = A sigma
and Euler gives the Ermakov equation

    sigma_ddot = -omega^2 sigma + hbar^2 / (4 m^2 sigma^3).

This is the equation of motion in the effective potential

    V(sigma) = omega^2 sigma^2 / 2 + hbar^2 / (8 m^2 sigma^2),

whose minimum is the quantum ground-state width sigma^2 = hbar/(2 m omega).
The reduction derives the width from the fluid equations, not from the
Schrodinger wavefunction.
"""

from __future__ import annotations

import math

import numpy as np


def quantum_width(omega: float, hbar: float = 1.0, mass: float = 1.0) -> float:
    if omega <= 0.0 or hbar <= 0.0 or mass <= 0.0:
        raise ValueError("omega, hbar and mass must be positive")
    return math.sqrt(hbar / (2.0 * mass * omega))


def effective_potential(
    sigma: float, omega: float, hbar: float = 1.0, mass: float = 1.0
) -> float:
    if sigma <= 0.0:
        raise ValueError("sigma must be positive")
    return 0.5 * omega * omega * sigma * sigma + (
        hbar * hbar / (8.0 * mass * mass * sigma * sigma)
    )


def energy(
    sigma: float,
    sigma_dot: float,
    omega: float,
    hbar: float = 1.0,
    mass: float = 1.0,
) -> float:
    return 0.5 * sigma_dot * sigma_dot + effective_potential(
        sigma, omega, hbar, mass
    )


def ermakov_rhs(
    sigma: float,
    sigma_dot: float,
    omega: float,
    hbar: float = 1.0,
    mass: float = 1.0,
) -> tuple[float, float]:
    if sigma <= 0.0:
        raise ValueError("sigma must be positive")
    acceleration = -omega * omega * sigma + (
        hbar * hbar / (4.0 * mass * mass * sigma**3)
    )
    return sigma_dot, acceleration


def simulate(
    sigma0: float,
    sigma_dot0: float,
    omega: float,
    duration: float,
    hbar: float = 1.0,
    mass: float = 1.0,
    dt: float = 1e-3,
) -> dict:
    if sigma0 <= 0.0 or duration <= 0.0 or dt <= 0.0:
        raise ValueError("sigma0, duration and dt must be positive")
    steps = int(round(duration / dt))
    times = np.empty(steps + 1)
    sigmas = np.empty(steps + 1)
    velocities = np.empty(steps + 1)
    sigmas[0] = sigma0
    velocities[0] = sigma_dot0
    times[0] = 0.0
    sigma, sigma_dot = sigma0, sigma_dot0
    for index in range(steps):
        k1 = ermakov_rhs(sigma, sigma_dot, omega, hbar, mass)
        k2 = ermakov_rhs(
            sigma + 0.5 * dt * k1[0],
            sigma_dot + 0.5 * dt * k1[1],
            omega,
            hbar,
            mass,
        )
        k3 = ermakov_rhs(
            sigma + 0.5 * dt * k2[0],
            sigma_dot + 0.5 * dt * k2[1],
            omega,
            hbar,
            mass,
        )
        k4 = ermakov_rhs(
            sigma + dt * k3[0],
            sigma_dot + dt * k3[1],
            omega,
            hbar,
            mass,
        )
        sigma += dt * (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0]) / 6.0
        sigma_dot += dt * (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1]) / 6.0
        times[index + 1] = (index + 1) * dt
        sigmas[index + 1] = sigma
        velocities[index + 1] = sigma_dot
    return {"times": times, "sigma": sigmas, "sigma_dot": velocities}
