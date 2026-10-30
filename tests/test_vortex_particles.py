"""Vortex-knot mass hypothesis tests (claim 17)."""

from __future__ import annotations

import numpy as np
import pytest

from theory.vortex_particles import (
    biot_savart_energy,
    knot_length,
    koide_q,
    mass_hypothesis,
    torus_knot,
)

LEPTONS = (0.51099895, 105.6583755, 1776.93)


def test_torus_knot_shapes_and_closure():
    points, tangents = torus_knot(3, samples=64)

    assert points.shape == (64, 3)
    assert tangents.shape == (64, 3)
    assert np.allclose(np.sum(tangents, axis=0), 0.0, atol=1e-12)
    assert knot_length(points) > 0.0
    assert biot_savart_energy(points, core=0.1) > 0.0


def test_koide_q_known_values():
    assert koide_q((1.0, 1.0, 1.0)) == pytest.approx(1.0 / 3.0)
    assert koide_q((1.0, 1.0, 4.0)) == pytest.approx(6.0 / 16.0)


def test_simple_vortex_spectrum_is_falsified():
    result = mass_hypothesis(LEPTONS)

    assert result["falsified"] is True
    assert result["status"].startswith("FALSIFIED")
    assert result["koide_q_energy"] < 0.5
    assert result["required_power"] > 2.0
    assert result["energy_ratios"][-1] < 3.0


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="at least 1"):
        torus_knot(0)
    with pytest.raises(ValueError, match="core"):
        biot_savart_energy(np.zeros((8, 3)), core=0.0)
    with pytest.raises(ValueError, match="positive"):
        mass_hypothesis((1.0, -1.0, 1.0))
