"""Capillary validity tests for the shallow-water horizon (claim 28)."""

from __future__ import annotations

import pytest

from theory.water_analogue import (
    capillary_length,
    capillary_ratio,
    long_wavelength_min_flow,
    validity,
)


def test_capillary_length_of_water():
    assert capillary_length() == pytest.approx(2.726879e-3, rel=1e-5)


def test_lab_example_is_capillary_dominated():
    result = validity(flow_rate=1e-3)

    assert result["long_wavelength"] is False
    assert result["capillary_ratio"] == pytest.approx(2.8798, rel=1e-3)


def test_larger_flow_reaches_the_long_wavelength_regime():
    result = validity(flow_rate=0.05)

    assert result["long_wavelength"] is True
    assert result["capillary_ratio"] < 1e-2


def test_minimum_flow_for_ten_capillary_lengths():
    minimum = long_wavelength_min_flow(0.1)

    assert minimum == pytest.approx(0.0169699, rel=1e-4)
    assert validity(flow_rate=minimum)["long_wavelength"] is True


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="positive"):
        capillary_length(gamma=0.0)
    with pytest.raises(ValueError, match="positive"):
        capillary_ratio(0.0, 0.1)
    with pytest.raises(ValueError, match="ratio"):
        long_wavelength_min_flow(0.1, ratio=0.0)
