"""Static finite-energy H=1 configuration tests (claim 14b)."""

from __future__ import annotations

import math

import pytest

from theory.hopfion import energies, scaling_study


def test_energy_is_positive_and_h1():
    row = energies(n=48, length=10.0)
    assert row["E2"] > 0.0
    assert row["E4"] > 0.0
    assert row["H"] == pytest.approx(1.0, abs=1e-3)
    assert math.isfinite(row["E_min"])


def test_scaling_virial_and_minimum():
    row = energies(n=48, length=10.0)
    lam = row["lambda_star"]
    assert lam > 0.0
    # dE/dlambda at lambda* is zero for E(lambda)=lambda E2 + E4/lambda.
    h = 1e-6
    derivative = (lam * (1 + h) * row["E2"] + row["E4"] / (lam * (1 + h))) - (
        lam * (1 - h) * row["E2"] + row["E4"] / (lam * (1 - h))
    )
    assert derivative / (2 * h * lam) == pytest.approx(0.0, abs=1e-6)
    assert row["E_min"] == pytest.approx(2.0 * math.sqrt(row["E2"] * row["E4"]))


def test_convergence_and_no_stability_claim():
    payload = scaling_study(sizes=(32, 48, 64), length=10.0)
    rows = payload["rows"]
    assert rows[-1]["E2"] == pytest.approx(rows[0]["E2"], rel=0.01)
    assert rows[-1]["E4"] == pytest.approx(rows[0]["E4"], rel=0.01)
    assert payload["verdict"]["finite_energy_H1_configuration"] is True
    assert payload["verdict"]["full_stability_proved"] is False
    assert payload["verdict"]["z3_order_measured"] is False
    assert payload["verdict"]["claim_14b_status"] == "UNVERIFIED"
