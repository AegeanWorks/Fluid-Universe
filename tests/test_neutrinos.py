"""Neutrino Q_nu scan tests (claim 7a)."""

from __future__ import annotations

import numpy as np
import pytest

from phenomenology.neutrinos import (
    load_parameters,
    masses_inverted,
    masses_normal,
    q_nu,
    scan_ordering,
)
from theory.circulant import q_masses


def test_normal_ordering_mass_formula():
    p = load_parameters()
    m = masses_normal(0.0, p.dm21_sq, p.dm31_sq_no)
    assert m[0] == 0.0
    assert m[1] == pytest.approx(np.sqrt(p.dm21_sq))
    assert m[2] == pytest.approx(np.sqrt(p.dm31_sq_no))


def test_inverted_ordering_mass_formula():
    p = load_parameters()
    m = masses_inverted(0.0, p.dm21_sq, p.dm31_sq_io)
    assert m[2] == 0.0
    assert m[0] == pytest.approx(np.sqrt(abs(p.dm31_sq_io)))
    assert m[1] == pytest.approx(np.sqrt(abs(p.dm31_sq_io) + p.dm21_sq))


def test_q_nu_regression_values():
    p = load_parameters()
    no = q_nu(masses_normal(0.0, p.dm21_sq, p.dm31_sq_no))
    io = q_nu(masses_inverted(0.0, p.dm21_sq, p.dm31_sq_io))
    assert no == pytest.approx(0.5856120093687199, abs=1e-12)
    assert io == pytest.approx(0.5000067395867646, abs=1e-12)
    assert q_nu(np.array([1.0, 1.0, 1.0])) == pytest.approx(1.0 / 3.0)


def test_scan_never_reaches_two_thirds():
    p = load_parameters()
    for ordering in ("NO", "IO"):
        result = scan_ordering(ordering, p)
        assert result["Q_max"] < 2.0 / 3.0
        assert result["Q_max"] == pytest.approx(result["Q_at_m_light_zero"])
        assert result["m_light_at_Q_max_eV"] == 0.0
        assert result["monotonic_decreasing"]


def test_three_sigma_corners_stay_below_two_thirds():
    from phenomenology.neutrinos import corner_max

    p = load_parameters()
    for ordering in ("NO", "IO"):
        result = corner_max(ordering, p)
        assert result["Q_max_corner"] < 2.0 / 3.0
    assert corner_max("NO", p)["Q_max_corner"] == pytest.approx(0.5906060371336499, abs=1e-12)


def test_q_nu_matches_generic_mass_ratio():
    m = np.array([0.001, 0.009, 0.05])
    assert q_nu(m) == pytest.approx(q_masses(m))
