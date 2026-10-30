"""Water reorganization and the VDE-optical difference (claim 38)."""

from __future__ import annotations

import pytest

from theory.water_reorganization import (
    radius_for_difference,
    reorganization_energy_ev,
    reorganization_scales,
)

R_CAPILLARY = 2.8580710555848763e-10
R_REQUIRED = 2.5178405289556243e-10
R_PRESSURE = 3.973423200472016e-10


def test_reorganization_at_capillary_radius():
    assert reorganization_energy_ev(R_CAPILLARY) == pytest.approx(
        1.383104042, rel=1e-8
    )


def test_required_radius_for_the_observed_difference():
    assert radius_for_difference(1.57) == pytest.approx(R_REQUIRED, rel=1e-8)


def test_reorganization_scales():
    scales = reorganization_scales()
    capillary = scales["capillary"]
    required = scales["required"]
    pressure = scales["pressure"]

    assert scales["observed_difference_ev"] == pytest.approx(1.57)
    assert capillary["lambda_ev"] == pytest.approx(1.383104042, rel=1e-8)
    assert capillary["predicted_vde_ev"] == pytest.approx(3.083104042, rel=1e-8)
    assert capillary["residual_ev"] == pytest.approx(-0.186895958, rel=1e-7)
    assert capillary["relative_residual"] == pytest.approx(0.119042011, rel=1e-7)
    assert capillary["sigma"] == pytest.approx(1.868959576, rel=1e-7)
    assert required["radius_m"] == pytest.approx(R_REQUIRED, rel=1e-7)
    assert required["radius_ratio"] == pytest.approx(0.880957989, rel=1e-7)
    assert pressure["lambda_ev"] == pytest.approx(0.994862473, rel=1e-8)
    assert pressure["lambda_ratio"] == pytest.approx(0.633670365, rel=1e-7)
    assert pressure["residual_ev"] == pytest.approx(0.575137527, rel=1e-7)


def test_pressure_balanced_radius_is_inconsistent():
    pressure = reorganization_scales()["pressure"]

    assert pressure["lambda_ev"] < 1.1
    assert pressure["lambda_ratio"] < 0.7


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="radius"):
        reorganization_energy_ev(0.0)
    with pytest.raises(ValueError, match="difference"):
        radius_for_difference(0.0)
    with pytest.raises(ValueError, match="epsilon"):
        reorganization_energy_ev(R_CAPILLARY, epsilon_s=0.0)
