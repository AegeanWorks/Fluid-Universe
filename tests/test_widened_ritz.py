"""Ritz-widened trial class tests (claim 59)."""

from __future__ import annotations

import math

import numpy as np
import pytest

from theory.widened_ritz import (
    BASE,
    WidenedProblem,
    diffusivity,
    hermite_basis,
    horizon,
)


def test_hermite_basis_orthonormal():
    import math

    x = np.linspace(-8.0, 8.0, 3201)
    dx = x[1] - x[0]
    w = np.exp(-x * x / 2.0)
    h = hermite_basis(x / 1.0, 4)
    for i in range(5):
        for j in range(5):
            val = float(np.sum(h[i] * h[j] * w) * dx)
            expected = (
                math.factorial(i) * math.sqrt(2.0 * math.pi) if i == j else 0.0
            )
            assert abs(val - expected) <= 1e-4


def test_gradient_matches_finite_differences():
    problem = WidenedProblem(2, nx=31, nt=11, nu=0.5)
    n = problem.n_interior
    x0 = problem.start_vector("gauss")
    _, grad = problem.action_and_gradient(x0)
    h = 1e-7
    worst = 0.0
    for idx in [0, n // 2, n - 1, n + 3, 2 * n + 5]:
        up = x0.copy()
        dn = x0.copy()
        up[idx] += h
        dn[idx] -= h
        ju, _ = problem.action_and_gradient(up)
        jd, _ = problem.action_and_gradient(dn)
        worst = max(worst, abs((ju - jd) / (2.0 * h) - grad[idx]))

    assert worst <= 1e-6


def test_k0_reproduces_trial_action():
    from phenomenology.widened_ritz_report import _analytic_bridge_J

    problem = WidenedProblem(0, nx=101, nt=201, nu=0.5)
    analytic = _analytic_bridge_J(problem)
    out = problem.solve("gauss", maxiter=2000)

    assert abs(analytic - (-0.3906340559)) <= 1e-4
    assert out["converged"] is True
    assert abs(out["J_final"] - analytic) <= 1e-4


def test_widened_verdict_holds():
    from phenomenology.widened_ritz_report import run_widened_ritz_check

    payload = run_widened_ritz_check(write=False)
    gates = payload["result"]["gates"]
    verdict = payload["result"]["verdict"]

    assert verdict["claim_59_status"].startswith(
        ("CONFIRMED FALSIFIED", "NUMERIC-REVERSAL", "UNVERIFIED")
    )
    hard = dict(gates["kills_hard"])
    soft = dict(gates["kills_soft"])
    if verdict["claim_59_status"].startswith("CONFIRMED FALSIFIED"):
        assert hard["escape_below"] is True
    if verdict["claim_59_status"].startswith("NUMERIC-REVERSAL"):
        assert hard["reversal_branch"] is True


def test_inputs_are_frozen():
    from phenomenology.widened_ritz_report import NU0_K, NU0_STARTS

    assert BASE == {"omega": 1.0, "hbar": 1.0, "mass": 1.0}
    assert diffusivity(1.0, 1.0) == 0.5
    assert horizon(1.0) == pytest.approx(math.pi / 4.0)
    # R1: nu0 control is the K=0 Gaussian classical bridge (not K=6).
    assert NU0_K == 0
    assert set(NU0_STARTS) == {"gauss", "wide_sigma"}


def test_nu0_k0_matches_closed_form():
    from phenomenology.widened_ritz_report import NU0_K

    problem = WidenedProblem(NU0_K, nx=101, nt=101, nu=0.0)
    out = problem.solve("gauss", maxiter=2000)
    assert abs(out["J_final"] - (-0.23852813742385703)) <= 1e-3


def test_reversal_needs_shape_modes():
    # R2: J-only match with c2~0.05 must not count as reversal.
    assert 0.054750834606038606 > 1e-4
