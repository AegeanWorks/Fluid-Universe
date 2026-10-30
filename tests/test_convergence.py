"""Resolution-comparison tests for claim 15 (ADR-002)."""

from __future__ import annotations

import pytest

from sims.convergence import compare_scans, compare_temporal


def _scan(n: int, *, h: float, h_up: float, z3: float, force: float, bond: float) -> dict:
    return {
        "n": n,
        "length": 10.0,
        "mu": 20.0,
        "steps": 3600,
        "dt": 5e-4,
        "final": {
            "H": h,
            "H_upsampled": h_up,
            "Z3": z3,
            "force_rms": force,
            "max_bond": bond,
        },
        "verdict": {
            "claim_15_status": "UNVERIFIED (H estimator not converged; not converged; field under-resolved)"
        },
    }


def test_compare_scans_accepts_a_converging_pair():
    low = _scan(64, h=0.788, h_up=0.9892, z3=0.6074, force=0.03716, bond=1.4799)
    high = _scan(96, h=0.795, h_up=0.9900, z3=0.6100, force=0.0300, bond=1.0000)

    result = compare_scans(low, high)

    assert result["n_low"] == 64 and result["n_high"] == 96
    assert result["deltas"]["H"] == pytest.approx(0.007)
    assert result["criteria"]["H_agrees"] is True
    assert result["criteria"]["H_upsampled_agrees"] is True
    assert result["criteria"]["Z3_agrees"] is True
    assert result["criteria"]["force_not_worse"] is True
    assert result["criteria"]["max_bond_not_worse"] is True
    assert result["resolution_converged"] is True
    assert result["claim_15_status"].startswith("UNVERIFIED")


def test_compare_scans_flags_a_native_H_regression():
    low = _scan(64, h=0.788, h_up=0.9892, z3=0.6074, force=0.03716, bond=1.4799)
    high = _scan(96, h=0.700, h_up=0.9900, z3=0.6100, force=0.0300, bond=1.0000)

    result = compare_scans(low, high)

    assert result["criteria"]["H_agrees"] is False
    assert result["criteria"]["H_toward_one"] is False
    assert result["resolution_converged"] is False


def test_compare_scans_flags_force_or_bond_regression():
    low = _scan(64, h=0.788, h_up=0.9892, z3=0.6074, force=0.03716, bond=1.4799)
    high = _scan(96, h=0.795, h_up=0.9900, z3=0.6100, force=0.0500, bond=1.6000)

    result = compare_scans(low, high)

    assert result["criteria"]["force_not_worse"] is False
    assert result["criteria"]["max_bond_not_worse"] is False
    assert result["resolution_converged"] is False


def test_compare_scans_requires_the_same_physical_problem():
    low = _scan(64, h=0.788, h_up=0.9892, z3=0.6074, force=0.03716, bond=1.4799)
    high = _scan(96, h=0.795, h_up=0.9900, z3=0.6100, force=0.0300, bond=1.0000)
    high["steps"] = 1800

    with pytest.raises(ValueError, match="same physical problem"):
        compare_scans(low, high)


def test_compare_scans_never_promotes_the_claim():
    low = _scan(64, h=0.999, h_up=0.9995, z3=0.700, force=0.005, bond=0.300)
    high = _scan(96, h=0.9995, h_up=0.9997, z3=0.701, force=0.004, bond=0.290)
    high["verdict"]["claim_15_status"] = "UNVERIFIED (dt not converged)"

    result = compare_scans(low, high)

    assert result["resolution_converged"] is True
    assert result["claim_15_status"] == "UNVERIFIED (dt not converged)"


def _temporal(
    n: int,
    steps: int,
    dt: float,
    *,
    h: float = 0.936965,
    h_up: float = 0.999521,
    z3: float = 0.528226,
    force: float = 0.03988,
    bond: float = 1.176,
) -> dict:
    return {
        "n": n,
        "length": 10.0,
        "mu": 20.0,
        "steps": steps,
        "dt": dt,
        "final": {
            "H": h,
            "H_upsampled": h_up,
            "Z3": z3,
            "force_rms": force,
            "max_bond": bond,
        },
        "verdict": {"claim_15_status": "UNVERIFIED (Z3 below threshold; not converged)"},
    }


def test_compare_temporal_accepts_a_converged_pair():
    coarse = _temporal(64, 3000, 5e-4)
    fine = _temporal(64, 6000, 2.5e-4, z3=0.528300)

    result = compare_temporal(coarse, fine)

    assert result["n"] == 64
    assert result["physical_time"] == pytest.approx(1.5)
    assert result["deltas"]["Z3"] == pytest.approx(0.000074)
    assert result["temporal_converged"] is True


def test_compare_temporal_flags_a_Z3_difference():
    coarse = _temporal(64, 3000, 5e-4)
    fine = _temporal(64, 6000, 2.5e-4, z3=0.400000)

    result = compare_temporal(coarse, fine)

    assert result["criteria"]["Z3_agrees"] is False
    assert result["temporal_converged"] is False


def test_compare_temporal_requires_the_same_physical_time():
    coarse = _temporal(64, 3000, 5e-4)
    fine = _temporal(64, 3000, 2.5e-4)

    with pytest.raises(ValueError, match="same physical time"):
        compare_temporal(coarse, fine)


def test_compare_temporal_requires_the_same_resolution():
    coarse = _temporal(64, 3000, 5e-4)
    fine = _temporal(96, 6000, 2.5e-4)

    with pytest.raises(ValueError, match="same n"):
        compare_temporal(coarse, fine)


def test_compare_temporal_never_promotes_the_claim():
    coarse = _temporal(64, 3000, 5e-4)
    fine = _temporal(64, 6000, 2.5e-4)
    fine["verdict"]["claim_15_status"] = "UNVERIFIED (not converged)"

    result = compare_temporal(coarse, fine)

    assert result["temporal_converged"] is True
    assert result["claim_15_status"] == "UNVERIFIED (not converged)"
