"""Derrick scaling tests for claim 14."""

from __future__ import annotations

import math

import pytest

from theory.derrick import (
    exponents,
    scaling_energy,
    scaling_minimum,
    second_derivative,
    stable_region,
    stationary_scale_squared,
    symbolic_virial,
    virial,
)


def test_exponents_in_three_dimensions():
    assert exponents(3) == {"gradient": 1, "skyrme": -1, "potential": 3}


def test_virial_relation():
    assert virial(2.0, 1.0, 0.5) == pytest.approx(2.0 - 1.0 + 3.0 * 0.5)
    assert virial(2.0, 1.0, 0.0) == pytest.approx(1.0)
    assert second_derivative(2.0, 1.0, 0.5) == pytest.approx(2.0 * 1.0 + 6.0 * 0.5)


def test_scaling_energy_derivative_matches_virial():
    e2, e4, v = 2.0, 1.0, 0.5
    h = 1e-7
    numeric = (scaling_energy(1.0 + h, e2, e4, v) - scaling_energy(1.0 - h, e2, e4, v)) / (2 * h)
    assert numeric == pytest.approx(virial(e2, e4, v), rel=1e-7)


def test_zero_potential_minimizer():
    e2, e4 = 2.0, 8.0
    y = stationary_scale_squared(e2, e4, 0.0)
    assert y == pytest.approx(4.0)
    minimum = scaling_minimum(e2, e4, 0.0)
    assert minimum["lambda"] == pytest.approx(2.0)
    assert minimum["is_minimum"]
    # At the minimum lambda, the virial relation is satisfied in the rescaled
    # energy (equivalently dE/dlambda = 0).
    h = 1e-7
    numeric = (
        scaling_energy(minimum["lambda"] + h, e2, e4, 0.0)
        - scaling_energy(minimum["lambda"] - h, e2, e4, 0.0)
    ) / (2 * h)
    assert numeric == pytest.approx(0.0, abs=1e-6)


def test_positive_potential_minimizer_is_stable():
    e2, e4, v = 2.0, 8.0, 0.5
    y = stationary_scale_squared(e2, e4, v)
    assert y is not None and y > 0.0
    minimum = scaling_minimum(e2, e4, v)
    assert minimum["is_minimum"]
    assert minimum["curvature"] > 0.0


def test_no_skyrme_means_no_scaling_minimum():
    assert stationary_scale_squared(2.0, 0.0, 0.5) is None
    region = stable_region(1.0, 0.0, 0.5)
    assert not region["scaling_minimum_exists"]
    assert not region["hopfion_existence_proved"]


def test_negative_skyrme_is_unbounded():
    region = stable_region(1.0, -1.0, 0.5)
    assert not region["energy_bounded_below"]
    assert not region["scaling_minimum_exists"]
    assert stationary_scale_squared(2.0, -1.0, 0.0) is None


def test_positive_skyrme_region_and_no_existence_claim():
    region = stable_region(1.0, 1.0, 0.5)
    assert region["scaling_minimum_exists"]
    assert not region["hopfion_existence_proved"]
    assert "not proved" in region["note"]


def test_symbolic_virial_identity():
    assert symbolic_virial()
