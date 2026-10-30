"""Resolution-comparison tests for claim 15 (ADR-002)."""

from __future__ import annotations

import pytest

from sims.convergence import compare_scans, compare_temporal, summarize_trend


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


def _trend_row(steps: int, t: float, z3: float, force: float) -> dict:
    return {
        "chunk": steps // 3000,
        "steps": steps,
        "t": t,
        "H": 0.9999,
        "Z3": z3,
        "force_rms": force,
        "claim_15_status": "UNVERIFIED (not converged)",
    }


def test_summarize_trend_projects_the_thresholds():
    rows = [
        _trend_row(3000, 1.5, 0.172071, 0.01495),
        _trend_row(6000, 3.0, 0.322071, 0.01395),
        _trend_row(9000, 4.5, 0.472000, 0.01295),
    ]

    summary = summarize_trend(rows)

    assert summary["z3_monotone"] is True
    assert summary["force_monotone"] is True
    assert summary["z3_slope_per_step"] == pytest.approx((0.472 - 0.172071) / 6000)
    assert summary["projected_steps_to_z3_target"] == pytest.approx(
        (0.5 - 0.472) / ((0.472 - 0.172071) / 6000)
    )
    assert summary["projected_steps_to_force_tol"] == pytest.approx(
        (0.01295 - 0.01) / ((0.01495 - 0.01295) / 6000)
    )
    assert summary["claim_15_status"].startswith("UNVERIFIED")


def test_summarize_trend_handles_saturation_and_no_projection():
    rows = [
        _trend_row(3000, 1.5, 0.500000, 0.0120),
        _trend_row(6000, 3.0, 0.490000, 0.0121),
        _trend_row(9000, 4.5, 0.480000, 0.0122),
    ]

    summary = summarize_trend(rows)

    assert summary["z3_monotone"] is False
    assert summary["force_monotone"] is False
    assert summary["projected_steps_to_z3_target"] is None
    assert summary["projected_steps_to_force_tol"] is None


def test_summarize_trend_requires_two_rows():
    with pytest.raises(ValueError, match="at least two"):
        summarize_trend([_trend_row(3000, 1.5, 0.17, 0.015)])
