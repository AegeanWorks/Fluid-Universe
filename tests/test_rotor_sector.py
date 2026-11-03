"""Sector validation: only 0, 1, 2 select distinct Fourier sectors."""

import pytest

from theory.rotor import sector_matrix as rotor_sector_matrix
from theory.rotor_potential import sector_matrix as potential_sector_matrix


def test_rotor_sector_out_of_range_raises():
    with pytest.raises(ValueError, match="sector must be 0, 1 or 2"):
        rotor_sector_matrix(1.0, 1.0, 3, 10)
    with pytest.raises(ValueError, match="sector must be 0, 1 or 2"):
        rotor_sector_matrix(1.0, 1.0, -1, 10)


def test_rotor_potential_sector_out_of_range_raises():
    with pytest.raises(ValueError, match="sector must be 0, 1 or 2"):
        potential_sector_matrix(1.0, 0.0, 0.0, 3)
    with pytest.raises(ValueError, match="sector must be 0, 1 or 2"):
        potential_sector_matrix(1.0, 0.0, 0.0, 99)
