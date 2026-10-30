"""Radial ray tests at the water horizon (claim 30)."""

from __future__ import annotations

import math

import pytest

from theory.water_analogue import horizon_radius, sound_speed, surface_gravity
from theory.water_rays import (
    analytic_outgoing_time,
    ingoing_speed,
    integrate_outgoing,
    outgoing_speed,
    peeling_rate,
)

DEPTH = 0.1
FLOW = 0.05


def test_outgoing_speed_changes_sign_at_the_horizon():
    radius = horizon_radius(FLOW, DEPTH)

    assert outgoing_speed(radius, FLOW, DEPTH) == pytest.approx(0.0, abs=1e-12)
    assert outgoing_speed(1.1 * radius, FLOW, DEPTH) > 0.0
    assert outgoing_speed(0.9 * radius, FLOW, DEPTH) < 0.0
    assert ingoing_speed(radius, FLOW, DEPTH) < 0.0


def test_analytic_ray_time():
    radius = horizon_radius(FLOW, DEPTH)
    speed = sound_speed(DEPTH)
    time = analytic_outgoing_time(2.0 * radius, 1.01 * radius, radius, speed)
    expected = (0.99 + math.log(100.0)) * radius / speed

    assert time == pytest.approx(expected)


def test_numerical_ray_time_matches_analytic():
    radius = horizon_radius(FLOW, DEPTH)
    speed = sound_speed(DEPTH)
    numeric = integrate_outgoing(
        1.01 * radius, 2.0 * radius, FLOW, DEPTH, dt=1e-6
    )
    analytic = analytic_outgoing_time(
        2.0 * radius, 1.01 * radius, radius, speed
    )

    assert numeric["time"] == pytest.approx(analytic, rel=1e-3)


def test_peeling_rate_is_the_surface_gravity():
    radius = horizon_radius(FLOW, DEPTH)
    speed = sound_speed(DEPTH)

    assert peeling_rate(FLOW, DEPTH) == pytest.approx(speed * speed / radius)
    assert peeling_rate(FLOW, DEPTH) == pytest.approx(
        surface_gravity(FLOW, DEPTH)
    )


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="positive"):
        outgoing_speed(0.0, FLOW, DEPTH)
    with pytest.raises(ValueError, match="horizon"):
        analytic_outgoing_time(0.5, 2.0, 1.0, 1.0)
    with pytest.raises(ValueError, match="r_end"):
        integrate_outgoing(2.0, 1.0, FLOW, DEPTH)
