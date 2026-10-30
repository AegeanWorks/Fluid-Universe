"""Real Z3 rotor potential tests (claim 19)."""

from __future__ import annotations

import numpy as np
import pytest

from theory.rotor_potential import scan, sector_energies, sector_matrix


def test_zero_flux_real_potential_is_degenerate():
    energies = sector_energies(1.0, 0.0, 0.0)

    assert energies[1] == pytest.approx(energies[2], abs=1e-12)


def test_fractional_flux_splits_the_sectors():
    energies = sector_energies(1.0, 0.0, 0.2)

    assert abs(energies[1] - energies[2]) > 1e-6


def test_scan_is_falsified():
    result = scan(nus=(0.0, 1.0), fluxes=(0.0, 0.5, 1.0), n_cutoff=80)

    assert result["zero_flux_degenerate"] is True
    assert result["best"]["koide_q"] < 0.5
    assert result["falsified"] is True
    assert result["status"].startswith("FALSIFIED")
    assert result["best_convergence_delta"] < 1e-6


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="non-negative"):
        sector_matrix(-1.0, 0.0, 0.0, 0)
    with pytest.raises(ValueError, match="inertia"):
        sector_matrix(1.0, 0.0, 0.0, 0, inertia=0.0)
    with pytest.raises(ValueError, match="n_cutoff"):
        sector_matrix(1.0, 0.0, 0.0, 0, n_cutoff=4)


def test_second_harmonic_does_not_force_a_large_spread():
    energies = sector_energies(1.0, 10.0, 0.5)
    spread = (max(energies) - min(energies)) / np.mean(energies)

    assert spread < 0.1
