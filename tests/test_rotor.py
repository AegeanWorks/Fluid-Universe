"""Rotor, sector-degeneracy, and domain-wall tests (claim 5)."""

from __future__ import annotations

import math

import numpy as np
import pytest

from theory.rotor import (
    convergence,
    domain_wall_tension,
    domain_wall_tension_quad,
    free_rotor_levels,
    grid_spectrum,
    implied_roots,
    mode_map,
    parity_obstruction,
    sector_matrix,
    spectrum,
)


def test_hamiltonian_is_symmetric_and_banded():
    H = sector_matrix(I=1.0, mu=0.7, sector=1, n_cutoff=20)
    assert np.allclose(H, H.T)
    # The retained n values differ by 3, so in the compressed matrix the only
    # off-diagonal entries are the first sub/super-diagonals.
    for i in range(H.shape[0]):
        for j in range(H.shape[1]):
            if i != j and abs(i - j) != 1:
                assert H[i, j] == 0.0


def test_sector_degeneracy_is_exact():
    for mu in (0.1, 1.0, 5.0):
        spec = spectrum(I=1.0, mu=mu, n_cutoff=80)
        for level in range(5):
            e1 = spec.lowest(1, 5)[level]
            e2 = spec.lowest(2, 5)[level]
            assert e1 == pytest.approx(e2, abs=1e-11)


def test_free_rotor_limit():
    spec = spectrum(I=1.0, mu=0.0, n_cutoff=40)
    assert spec.lowest(0, 3)[0] == pytest.approx(0.0, abs=1e-12)
    assert spec.lowest(1, 3)[0] == pytest.approx(0.5, abs=1e-12)
    assert spec.lowest(2, 3)[0] == pytest.approx(0.5, abs=1e-12)
    assert spec.lowest(0, 5)[1] == pytest.approx(9.0 / 2.0, abs=1e-12)
    for sector in (0, 1, 2):
        assert np.allclose(spec.lowest(sector, 3), free_rotor_levels(sector, 3), atol=1e-12)


def test_truncation_convergence():
    rows = convergence(I=1.0, mu=1.0, cutoffs=(10, 20, 40, 80, 160))
    # Monotone approach of the ground state from above within truncation changes.
    e0 = [row["E0"] for row in rows]
    assert e0[-1] < e0[0]
    assert abs(e0[-1] - e0[-2]) < 1e-6
    assert all(row["E1_minus_E2"] == pytest.approx(0.0, abs=1e-11) for row in rows)


def test_fourier_and_grid_agree_in_continuum():
    exact = spectrum(I=1.0, mu=1.0, n_cutoff=120)
    grid = grid_spectrum(I=1.0, mu=1.0, n_grid=2048, count=5)
    # Finite differences have O(dphi^2) error; the observed relative error at
    # N=2048 is about 1e-6, so 1e-5 is a meaningful (not vacuous) tolerance.
    assert exact.lowest(0, 1)[0] == pytest.approx(grid[0], rel=1e-5)
    assert exact.lowest(1, 1)[0] == pytest.approx(grid[1], rel=1e-5)
    assert exact.lowest(1, 1)[0] == pytest.approx(grid[2], rel=1e-5)


def test_both_map_signs_give_two_distinct_roots():
    spec = spectrum(I=1.0, mu=1.0, n_cutoff=80)
    levels = (spec.lowest(0, 1)[0], spec.lowest(1, 1)[0], spec.lowest(2, 1)[0])
    a, c = mode_map(levels)
    for signed_c in (c, -c):
        roots = np.array(
            [a + 2.0 * signed_c * math.cos(2.0 * math.pi * k / 3.0) for k in range(3)]
        )
        assert len(np.unique(np.round(roots, 8))) == 2
    # The opposite sign is the one that reproduces (l0, l1, l1).
    corrected = np.array(
        [a + 2.0 * (-c) * math.cos(2.0 * math.pi * k / 3.0) for k in range(3)]
    )
    assert corrected[0] == pytest.approx(levels[0], abs=1e-11)
    assert corrected[1] == pytest.approx(levels[1], abs=1e-11)


def test_naive_map_cannot_give_three_distinct_masses():
    spec = spectrum(I=1.0, mu=1.0, n_cutoff=80)
    levels = (spec.lowest(0, 1)[0], spec.lowest(1, 1)[0], spec.lowest(2, 1)[0])
    roots = implied_roots(levels)
    assert roots[1] == pytest.approx(roots[2], abs=1e-11)
    assert len(np.unique(np.round(roots, 8))) == 2
    a, c = mode_map(levels)
    assert c == pytest.approx((levels[1] - levels[0]) / 3.0)
    info = parity_obstruction()
    assert info["distinct_roots_from_naive_map"] == 2
    assert info["needed_for_three_masses"] == 3


def test_domain_wall_tension_analytic_and_quadrature():
    for mu in (0.0, 0.5, 1.0, 7.3):
        analytic = domain_wall_tension(mu)
        numeric = domain_wall_tension_quad(mu)
        assert numeric == pytest.approx(analytic, rel=1e-10, abs=1e-12)
    assert domain_wall_tension(1.0) == pytest.approx(8.0 / 3.0)
    assert domain_wall_tension(4.0) == pytest.approx(16.0 / 3.0)
    assert domain_wall_tension(0.0) == 0.0


def test_energy_scales_with_mu():
    # For fixed I, mu sets the barrier; E(mu=0) is the free rotor.
    low = spectrum(I=1.0, mu=0.01, n_cutoff=80).lowest(0, 1)[0]
    high = spectrum(I=1.0, mu=10.0, n_cutoff=80).lowest(0, 1)[0]
    assert 0.0 < low < high
