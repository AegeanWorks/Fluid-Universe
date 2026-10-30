"""Shallow-water acoustic horizon tests (claim 27)."""

from __future__ import annotations

import math

import pytest

from theory.water_analogue import (
    acoustic_metric,
    flow_velocity,
    hawking_temperature,
    horizon_radius,
    lab_scales,
    sound_speed,
    surface_gravity,
)


def test_sound_speed():
    assert sound_speed(0.1) == pytest.approx(math.sqrt(0.981))


def test_horizon_satisfies_v_equals_c():
    radius = horizon_radius(1e-3, 0.1)
    speed = sound_speed(0.1)
    velocity = flow_velocity(radius, 1e-3, 0.1)

    assert velocity == pytest.approx(speed)
    assert acoustic_metric(radius, 1e-3, 0.1)["g_tt"] == pytest.approx(0.0, abs=1e-15)


def test_metric_sign_changes_across_the_horizon():
    radius = horizon_radius(1e-3, 0.1)
    outside = acoustic_metric(2.0 * radius, 1e-3, 0.1)
    inside = acoustic_metric(0.5 * radius, 1e-3, 0.1)

    assert outside["g_tt"] < 0.0
    assert inside["g_tt"] > 0.0


def test_surface_gravity_and_temperature():
    radius = horizon_radius(1e-3, 0.1)
    speed = sound_speed(0.1)
    kappa = surface_gravity(1e-3, 0.1)

    assert kappa == pytest.approx(speed * speed / radius)
    assert hawking_temperature(kappa) > 0.0


def test_lab_scales():
    scales = lab_scales()

    assert scales["sound_speed_m_s"] == pytest.approx(0.990454, rel=1e-5)
    assert scales["horizon_radius_m"] == pytest.approx(1.6068e-3, rel=1e-3)
    assert scales["surface_gravity_m_s2"] == pytest.approx(610.5, rel=1e-2)
    assert scales["hawking_temperature_k"] == pytest.approx(7.43e-10, rel=1e-2)


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="positive"):
        sound_speed(0.0)
    with pytest.raises(ValueError, match="positive"):
        horizon_radius(-1e-3, 0.1)
    with pytest.raises(ValueError, match="kappa"):
        hawking_temperature(0.0)
