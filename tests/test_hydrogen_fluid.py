"""Hydrogen 1s Madelung reduction tests (claim 25)."""

from __future__ import annotations

import numpy as np
import pytest

from theory.hydrogen_fluid import (
    bohr_radius,
    force_balance,
    ground_energy,
    hydrogen_density,
    si_values,
)


def test_bohr_radius_atomic_units():
    assert bohr_radius() == pytest.approx(1.0)
    assert ground_energy(1.0) == pytest.approx(-0.5)


def test_density_is_normalized():
    r = np.linspace(0.0, 60.0, 60001)
    density = hydrogen_density(r, 1.0)
    integral = np.trapezoid(4.0 * np.pi * r * r * density, r)

    assert integral == pytest.approx(1.0, rel=1e-6)


def test_force_balance_vanishes_at_the_bohr_radius():
    r = np.array([0.5, 1.0, 2.0, 5.0])
    residual = force_balance(r, 1.0)

    assert np.allclose(residual, 0.0, atol=1e-12)


def test_force_balance_is_nonzero_at_the_wrong_radius():
    residual = force_balance(np.array([1.0]), 2.0)

    assert abs(residual[0]) > 0.1


def test_si_values():
    values = si_values()

    assert values["bohr_radius_m"] == pytest.approx(5.29177e-11, rel=1e-4)
    assert values["ground_energy_ev"] == pytest.approx(-13.6057, rel=1e-4)


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="positive"):
        bohr_radius(hbar=0.0)
    with pytest.raises(ValueError, match="r"):
        force_balance(np.array([0.0]), 1.0)
    with pytest.raises(ValueError, match="radius"):
        hydrogen_density(np.array([1.0]), 0.0)
