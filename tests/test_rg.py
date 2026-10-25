"""Regression tests and cross-checks for the one-loop SM lepton running."""

from __future__ import annotations

import math

import numpy as np
import pytest

from phenomenology.rg import (
    LITERATURE,
    M_Z,
    MSBAR_MZ,
    V_EW,
    convert_pole_to_msbar_explicit,
    initial_state,
    observables_from_yukawas,
    run_rg,
    yukawa_from_mass,
)


@pytest.fixture(scope="module")
def payload():
    return run_rg(write=False)


def test_rhs_returns_log_derivatives():
    # Regression for the gauge-derivative bug: the state is logarithmic, so
    # d(ln g)/dt = c g^2, not c g^3.
    from phenomenology.rg import LOOP, M_Z, initial_state, rhs

    state = initial_state()
    derivative = rhs(math.log(M_Z), state.vector())
    g1, g2, g3 = state.g1, state.g2, state.g3
    assert derivative[5] == pytest.approx((41.0 / 10.0) * g1**2 / LOOP)
    assert derivative[6] == pytest.approx((-19.0 / 6.0) * g2**2 / LOOP)
    assert derivative[7] == pytest.approx(-7.0 * g3**2 / LOOP)
    # Analytic one-loop solution for g1 at 1e8 GeV.
    t = math.log(1.0e8 / M_Z)
    g1_at = 1.0 / math.sqrt(1.0 / g1**2 - (2.0 * 41.0 / 10.0) * t / LOOP)
    from phenomenology.rg import run_system

    _t, sol = run_system(initial=state, scale_max=1.0e8)
    assert math.exp(sol(math.log(1.0e8))[5]) == pytest.approx(g1_at, rel=1e-3)


def test_initial_state_sanity():
    state = initial_state()
    assert 0.0 < state.g3 < 2.0
    assert state.y_t > state.y_tau > state.y_mu > state.y_e
    assert state.y_e == pytest.approx(yukawa_from_mass(MSBAR_MZ["e"]))
    assert math.sqrt(2.0) * MSBAR_MZ["tau"] / V_EW == pytest.approx(state.y_tau)


def test_published_conversion_reproduced():
    # Xing & Zhang (2006) Eq. (8), alpha(MZ) = 1/128.89, one-pass QED.
    alpha = 1.0 / 128.89
    pole = {"e": 0.51099895000e-3, "mu": 105.6583755e-3, "tau": 1776.93e-3}
    published = {"e": 0.486755106e-3, "mu": 102.740394e-3, "tau": 1746.56e-3}
    for key in pole:
        ours = convert_pole_to_msbar_explicit(pole[key], M_Z, alpha)
        assert abs(ours - published[key]) / published[key] < 5e-4


def test_q_of_converted_pole_masses_matches_literature(payload):
    # Xing & Zhang report Q_l(M_Z) = 0.66792 from converted pole masses.
    ours = payload["published_conversion_check"]["Q_from_converted_pole_masses"]
    assert ours == pytest.approx(0.66792, abs=2e-5)
    assert payload["published_conversion_check"]["xing_zhang_Q_MZ"] == 0.66792


def test_msbar_q_at_mz_is_not_two_thirds(payload):
    d = payload["drift"]
    assert d["Q_MZ"] == pytest.approx(0.667823994186, abs=5e-13)
    assert d["Q_MZ"] - 2.0 / 3.0 > 1e-3
    assert d["Q_MZ"] - 2.0 / 3.0 < 2e-3


def test_exact_invariance_is_false(payload):
    d = payload["drift"]
    assert d["r_change"] != 0.0
    assert not payload["verdict"]["exact_rg_invariance_of_r"]


def test_drift_is_small(payload):
    d = payload["drift"]
    assert abs(d["Q_change"]) < 1e-5
    assert abs(d["r_change"]) < 1e-5
    assert payload["verdict"]["numerical_drift_is_negligible"]


def test_shape_only_approximation_is_close_but_not_exact(payload):
    # Dropping the common gauge/T factor is a diagnostic, not an equivalence.
    cross = payload["shape_only_approximation"]
    assert 0.0 < cross["max_abs_delta_Q"] < 1e-6
    assert 0.0 < cross["max_abs_delta_r"] < 1e-6


def test_high_scale_literature_agreement(payload):
    # Absolute masses are truncation-limited: one loop versus three loops differs
    # by about 3% at 1e8 GeV and 5% at 1e12 GeV.
    for scale, item in payload["literature_high_scale_check"].items():
        for key, lit in LITERATURE[float(scale)].items():
            assert abs(item["ours"][key] - lit) / lit < 0.06
    # Q from the literature masses is the meaningful shape comparison.
    for scale, item in payload["literature_high_scale_check"].items():
        lit = LITERATURE[float(scale)]
        m = np.array([lit["e"], lit["mu"], lit["tau"]])
        q_lit = np.sum(m) / np.sum(np.sqrt(m)) ** 2
        ours = item["ours"]
        m_ours = np.array([ours["e"], ours["mu"], ours["tau"]])
        q_ours = np.sum(m_ours) / np.sum(np.sqrt(m_ours)) ** 2
        assert abs(q_ours - q_lit) / q_lit < 1e-4


def test_observables_use_sqrt_masses():
    # Yukawa eigenvalues are proportional to the masses, so the circulant acts
    # on sqrt(y), and Q = sum(y) / (sum sqrt(y))^2.
    y = np.array([1.0, 2.0, 3.0])
    obs = observables_from_yukawas(y)
    expected_Q = np.sum(y) / np.sum(np.sqrt(y)) ** 2
    assert obs["Q"] == pytest.approx(expected_Q)
    assert obs["m_tau_GeV"] == pytest.approx(3.0 * V_EW / math.sqrt(2.0))
    masses = y * V_EW / math.sqrt(2.0)
    expected_a = np.mean(np.sqrt(masses))
    assert obs["a_sqrt_GeV"] == pytest.approx(expected_a)
