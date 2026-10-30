"""1D water-wave scattering tests (claim 31)."""

from __future__ import annotations

import math

import numpy as np
import pytest

from sims.water_scattering import (
    horizon_flow,
    initial_packet,
    simulate,
    sponge_profile,
    uniform_flow,
)

DEPTH = 0.1
C = math.sqrt(9.81 * DEPTH)


def test_uniform_flow_transmits_the_packet():
    result = simulate(
        lambda x: uniform_flow(x, C, 0.5),
        n=300,
        t_max=12.0,
        dt=5e-4,
    )

    assert result["transmitted_fraction"] > 0.9
    assert result["energy_total"] > 0.0


def test_horizon_blocks_the_packet():
    result = simulate(
        lambda x: horizon_flow(x, C, 10.0),
        n=300,
        t_max=12.0,
        dt=5e-4,
    )

    assert result["transmitted_fraction"] < 1e-3
    assert result["final_center"] < 0.0
    assert np.isfinite(result["energy_total"])


def test_initial_packet_and_sponge_shapes():
    x = np.linspace(-10.0, 10.0, 101)
    eta, u = initial_packet(x, -3.0, 0.5, C, DEPTH)
    sponge = sponge_profile(x, 20.0)

    assert eta.max() == pytest.approx(1.0)
    assert np.allclose(u, (C / DEPTH) * eta)
    assert sponge[0] > 0.0 and sponge[-1] > 0.0
    assert sponge[50] == 0.0


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="fraction"):
        uniform_flow(np.zeros(4), C, 1.0)
    with pytest.raises(ValueError, match="width"):
        initial_packet(np.zeros(4), 0.0, 0.0, C, DEPTH)
    with pytest.raises(ValueError, match="fraction"):
        sponge_profile(np.zeros(4), 10.0, 0.0)
