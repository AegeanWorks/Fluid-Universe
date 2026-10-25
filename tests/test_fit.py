"""Regression tests for the PDG pole-mass fit."""

from __future__ import annotations

import numpy as np
import pytest

from phenomenology.fit import R_KOIDE, fit_point, load_pole_masses, run_fit
from theory.circulant import q_masses


def test_pdg_input_values():
    masses = load_pole_masses()
    assert [m.name for m in masses] == ["m_e", "m_mu", "m_tau"]
    assert [m.definition for m in masses] == ["pole"] * 3
    assert pytest.approx(0.51099895000) == masses[0].value_MeV
    assert pytest.approx(105.6583755) == masses[1].value_MeV
    assert pytest.approx(1776.93) == masses[2].value_MeV


def test_fit_regression_numbers():
    masses = np.array([m.value_MeV for m in load_pole_masses()])
    fit = fit_point(masses)
    assert fit["Q"] == pytest.approx(0.666664463415, abs=5e-13)
    assert fit["r"] == pytest.approx(0.707104444281, abs=5e-13)
    assert fit["delta_mod_2pi_over_3"] == pytest.approx(0.222224761904, abs=5e-12)
    assert fit["tau_prediction_MeV"] == pytest.approx(1776.969027, abs=1e-6)
    assert fit["max_abs_residual"] < 1e-12
    assert fit["all_roots_positive"]


def test_fit_is_exact_by_construction():
    rng = np.random.default_rng(7)
    for _ in range(20):
        masses = rng.uniform(0.1, 2000.0, size=3)
        fit = fit_point(masses)
        assert fit["max_abs_residual"] < 1e-9
        assert q_masses(masses) == pytest.approx(fit["Q_from_ac"], rel=1e-11)


def test_monte_carlo_is_reproducible():
    a = run_fit(n_mc=2000, seed=11, write=False)["monte_carlo"]
    b = run_fit(n_mc=2000, seed=11, write=False)["monte_carlo"]
    assert a == b
    # The 1-sigma band must contain the central value and be small.
    assert a["r"]["std"] < 1e-4
    assert abs(a["Q"]["mean"] - 2.0 / 3.0) < 1e-4
    assert abs(a["r"]["mean"] - R_KOIDE) < 1e-4


def test_two_thirds_is_inside_one_sigma_of_pole_fit():
    payload = run_fit(n_mc=2000, seed=11, write=False)
    assert abs(payload["tests"]["Q_deviation_sigma"]) < 1.0
    assert abs(payload["tests"]["r_deviation_sigma"]) < 1.0


def test_tau_prediction_consistency():
    payload = run_fit(n_mc=2000, seed=11, write=False)
    assert abs(payload["tests"]["tau_measured_minus_predicted_sigma"]) < 1.0
