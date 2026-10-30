"""Marcus band width from the water reorganization energy (claim 40)."""

from __future__ import annotations

import pytest

from theory.water_marcus import (
    gaussian_line_shape,
    marcus_scales,
    marcus_width_ev,
)

LAMBDA = 1.6171325440878093


def test_width_at_300_k():
    assert marcus_width_ev(LAMBDA, 300.0) == pytest.approx(
        0.680914531, rel=1e-8
    )


def test_width_scaling():
    width_200 = marcus_width_ev(LAMBDA, 200.0)
    width_350 = marcus_width_ev(LAMBDA, 350.0)

    assert width_200 == pytest.approx(0.555964386, rel=1e-8)
    assert width_350 == pytest.approx(0.735471752, rel=1e-8)
    assert width_350 / marcus_width_ev(LAMBDA, 300.0) == pytest.approx(
        1.080123450, rel=1e-8
    )


def test_gaussian_line_shape():
    assert gaussian_line_shape(1.7, 1.7, 0.68) == pytest.approx(1.0)
    assert gaussian_line_shape(1.7 + 0.34, 1.7, 0.68) == pytest.approx(0.5)
    assert gaussian_line_shape(1.7 - 0.34, 1.7, 0.68) == pytest.approx(0.5)


def test_marcus_scales():
    scales = marcus_scales()

    assert scales["lambda_ev"] == pytest.approx(LAMBDA, rel=1e-8)
    assert scales["widths_ev"]["300"] == pytest.approx(0.680914531, rel=1e-8)
    assert scales["widths_ev"]["200"] == pytest.approx(0.555964386, rel=1e-8)
    assert scales["widths_ev"]["350"] == pytest.approx(0.735471752, rel=1e-8)
    assert scales["reference_classical_ev"] == pytest.approx(0.706)
    assert scales["reference_quantized_ev"] == pytest.approx(0.773)
    assert scales["ratio_classical"] == pytest.approx(0.964468, rel=1e-6)
    assert scales["ratio_quantized"] == pytest.approx(0.880873, rel=1e-6)


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="temperature"):
        marcus_width_ev(LAMBDA, 0.0)
    with pytest.raises(ValueError, match="lambda"):
        marcus_width_ev(0.0, 300.0)
    with pytest.raises(ValueError, match="fwhm"):
        gaussian_line_shape(1.7, 1.7, 0.0)
