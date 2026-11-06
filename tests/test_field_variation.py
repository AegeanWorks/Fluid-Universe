"""Unstructured field variation tests (claim 58)."""

from __future__ import annotations

import math

import numpy as np
import pytest

from theory.field_variation import (
    ACTION_TOL,
    ATTRIB_TOL,
    BASE,
    GRAD_TOL,
    GRID_COARSE,
    FieldProblem,
    bridge_widths,
    diffusivity,
    horizon,
)
from theory.gaussian_fluid import quantum_width


def test_gradient_matches_finite_differences():
    problem = FieldProblem(21, 11, 0.5)
    rng = np.random.default_rng(20261158)
    phi = rng.standard_normal((problem.n_interior, problem.nx - 1)) * 0.1
    _, grad = problem.action_and_gradient(phi)
    grad = grad.reshape(problem.n_interior, problem.nx - 1)
    h = 1e-7
    worst = 0.0
    for i, j in [(0, 0), (3, 10), (5, 18), (7, 5), (8, 17)]:
        up = phi.copy()
        dn = phi.copy()
        up[i, j] += h
        dn[i, j] -= h
        ju, _ = problem.action_and_gradient(up)
        jd, _ = problem.action_and_gradient(dn)
        worst = max(worst, abs((ju - jd) / (2.0 * h) - grad[i, j]))

    assert worst <= 1e-6


def test_quantum_and_classical_banks_are_frozen():
    assert BASE == {"omega": 1.0, "hbar": 1.0, "mass": 1.0}
    assert GRID_COARSE == (101, 101)
    assert diffusivity(1.0, 1.0) == 0.5
    assert horizon(1.0) == pytest.approx(math.pi / 4.0)
    assert quantum_width(1.0, 1.0, 1.0) == pytest.approx(math.sqrt(0.5))


def test_classical_closed_form_matches_quadrature():
    from phenomenology.field_variation_report import _classical_transport_action

    sq = math.sqrt(0.5)
    T = math.pi / 4.0
    closed = _classical_transport_action(1.2 * sq, sq, T, 1.0, 1.0)

    assert closed["A"] == pytest.approx(0.848528137423857)
    assert closed["B"] == pytest.approx(0.1514718625761429)
    assert closed["value"] == pytest.approx(-0.23852813742385703)
    t = np.linspace(0.0, T, 200001)
    s = closed["A"] * np.cos(t) + closed["B"] * np.sin(t)
    v = np.gradient(s, t[1] - t[0])
    quad = float(np.trapezoid(0.5 * v * v - 0.5 * s * s, t))

    assert abs(quad - closed["value"]) <= 1e-6


def test_field_action_on_bridge_matches_bank():
    from phenomenology.field_variation_report import _field_action_on_rho

    problem = FieldProblem(101, 101, 0.5)
    t = np.linspace(0.0, problem.T, problem.nt)
    s = bridge_widths(t, problem.s0, problem.omega)
    rho = np.empty((problem.nt, problem.nx))
    for k in range(problem.nt):
        rho[k] = np.exp(-problem.xc**2 / (2.0 * s[k] ** 2)) / math.sqrt(
            2.0 * math.pi * s[k] ** 2
        )
    rho = rho / (rho.sum(axis=1, keepdims=True) * problem.dx)
    j_field = _field_action_on_rho(problem, rho)

    assert abs(j_field - (-0.3906340559)) <= 1e-4


def test_mass_conserved_and_positive():
    problem = FieldProblem(31, 11, 0.5)
    run = problem.optimize("bimodal", maxiter=50)

    assert run["mass_err"] <= 1e-12
    assert run["min_rho"] > 0.0


def test_field_verdict_holds():
    from phenomenology.field_variation_report import run_field_variation_check

    payload = run_field_variation_check(write=False)
    verdict = payload["result"]["verdict"]

    assert verdict["claim_58_status"].startswith("FALSIFIED")
