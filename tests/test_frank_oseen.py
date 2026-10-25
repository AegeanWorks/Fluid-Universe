"""Frank-Oseen mode and dispersion checks (claim 10)."""

from __future__ import annotations

import numpy as np
import pytest

from theory.frank_oseen import (
    anisotropy,
    dispersion,
    elastic_matrix,
    equal_constant_dispersion,
    modes,
    symbolic_matches_numeric,
)


def test_symbolic_hessian_matches_numeric_matrix():
    assert symbolic_matches_numeric()


def test_one_constant_is_isotropic():
    for direction in ((1, 0, 0), (0, 1, 0), (0, 0, 1), (1, 2, 3), (-2, 1, 4)):
        unit = np.array(direction, dtype=float)
        unit /= np.linalg.norm(unit)
        k = 1.7
        matrix = elastic_matrix(*(unit * k), 2.0, 2.0, 2.0)
        assert np.allclose(matrix, 2.0 * k * k * np.eye(2))
        values = dispersion(*(unit * k), 2.0, 2.0, 2.0, rho=1.3)
        assert np.allclose(values, 2.0 * k * k / 1.3)


def test_axis_dispersions_for_unequal_constants():
    k = 2.0
    along_x = dispersion(k, 0.0, 0.0, 1.0, 2.0, 3.0)
    along_z = dispersion(0.0, 0.0, k, 1.0, 2.0, 3.0)
    assert np.allclose(along_x, [1.0 * k * k, 2.0 * k * k])
    assert np.allclose(along_z, [3.0 * k * k, 3.0 * k * k])


def test_anisotropy_zero_for_equal_constants_positive_for_unequal():
    equal = anisotropy((1, 1, 1), 2.0, 2.0, 2.0)
    assert equal["fractional_split"] == pytest.approx(0.0, abs=1e-15)
    unequal = anisotropy((1, 0, 0), 1.0, 2.0, 3.0)
    assert unequal["fractional_split"] == pytest.approx(2.0 / 3.0)


def test_mode_count_and_no_photon():
    info = modes()
    assert info["propagating_modes"] == 2
    assert info["gauge_field"] is False
    assert info["can_be_photon"] is False
    assert "preferred frame" in info["lorentz_invariance"]


def test_equal_constant_dispersion_formula():
    assert equal_constant_dispersion(3.0, 2.0, 1.5) == pytest.approx(
        3.0 * np.sqrt(2.0 / 1.5)
    )
    assert equal_constant_dispersion(-3.0, 2.0, 1.5) == pytest.approx(
        3.0 * np.sqrt(2.0 / 1.5)
    )
