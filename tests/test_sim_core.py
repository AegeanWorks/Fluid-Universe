"""1D simulation-core baselines (claim 15, validation only)."""

from __future__ import annotations

import numpy as np
import pytest

from sims.core import (
    analytic_kink,
    energy,
    force,
    gradient_error,
    gradient_flow,
    grid,
    hessian,
    kink_convergence,
    kink_tension,
    relax_kink,
    rotor_crosscheck,
)


def test_analytic_kink_limits_and_tension():
    x = np.array([-20.0, 0.0, 20.0])
    phi = analytic_kink(x, 1.0)
    assert phi[0] == pytest.approx(0.0, abs=1e-12)
    assert phi[2] == pytest.approx(2.0 * np.pi / 3.0, abs=1e-12)
    assert phi[1] == pytest.approx(np.pi / 3.0, abs=1e-12)
    assert kink_tension(1.0) == pytest.approx(8.0 / 3.0)
    assert kink_tension(4.0) == pytest.approx(16.0 / 3.0)


def test_kink_energy_converges_quadratically():
    result = kink_convergence(mu=1.0, length=30.0, sizes=(201, 401, 801))
    errors = [abs(row["error_vs_tension"]) for row in result["rows"]]
    assert errors[0] > errors[1] > errors[2]
    for order in result["observed_orders"][:2]:
        assert 1.8 < order < 2.2


def test_gradient_matches_finite_difference():
    x, dx = grid(30.0, 301)
    phi = analytic_kink(x, 1.0)
    phi += 0.05 * np.sin(2.0 * np.pi * (x + 15.0) / 30.0)
    phi[0] = 0.0
    phi[-1] = 2.0 * np.pi / 3.0
    assert gradient_error(phi, dx, 1.0) < 1e-6


def test_gradient_flow_decreases_energy():
    x, dx = grid(30.0, 301)
    phi = analytic_kink(x, 1.0)
    phi += 0.05 * np.sin(2.0 * np.pi * (x + 15.0) / 30.0)
    phi[0] = 0.0
    phi[-1] = 2.0 * np.pi / 3.0
    _phi, energies = gradient_flow(phi, dx, 1.0, dt=0.05 * dx**2, steps=200)
    assert np.all(np.diff(energies) <= 1e-10)
    assert energies[-1] < energies[0]


def test_relaxed_hessian_is_symmetric_and_stable():
    x, dx = grid(30.0, 301)
    phi = analytic_kink(x, 1.0)
    relaxed, info = relax_kink(phi, dx, 1.0)
    assert info["success"]
    matrix = hessian(relaxed, dx, 1.0)
    assert np.allclose(matrix, matrix.T)
    eigenvalues = np.linalg.eigvalsh(matrix)
    assert eigenvalues[0] > -1e-4
    vacuum = np.linalg.eigvalsh(hessian(np.zeros_like(phi), dx, 1.0))
    assert vacuum[0] > 0.0
    assert int(np.sum(np.abs(eigenvalues) < 1e-4)) >= 1


def test_rotor_fourier_and_grid_agree():
    result = rotor_crosscheck()
    assert result["abs_delta_E0"] < 1e-3
    assert result["abs_delta_E1"] < 1e-3


def test_report_makes_no_production_claim():
    from phenomenology.sim_report import run_1d_baseline

    payload = run_1d_baseline(write=False)
    assert payload["verdict"]["production_ready"] is False
    assert payload["verdict"]["claim_15_status"] == "UNVERIFIED / NOT ACHIEVED"
    assert "No 3D lattice" in payload["verdict"]["note"]
