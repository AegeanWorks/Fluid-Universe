"""3D E2+E4+Z3 pilot tests (claim 15)."""

from __future__ import annotations

import numpy as np
import pytest

from sims.core3d import (
    energy_terms,
    gradient_check,
    normalize,
    pilot_scan,
    z3_order,
)
from theory.hopf import compactified_field


def test_gradient_matches_finite_difference():
    assert gradient_check(n=8, length=6.0, mu=1.0) < 1e-6


def test_z3_order_limits():
    aligned = np.zeros((4, 4, 4, 3))
    aligned[..., 0] = 1.0
    assert z3_order(aligned) == pytest.approx(1.0)
    vacuum = np.zeros((4, 4, 4, 3))
    vacuum[..., 2] = 1.0
    assert z3_order(vacuum) == pytest.approx(0.0)


def test_energy_terms_are_positive():
    field, dx = compactified_field(16, 10.0)
    terms = energy_terms(field, dx, mu=1.0)
    assert terms["E2"] > 0.0
    assert terms["E4"] > 0.0
    assert terms["V"] >= 0.0
    assert terms["E"] == pytest.approx(terms["E2"] + terms["E4"] + terms["V"])


def test_small_pilot_no_coexistence():
    payload = pilot_scan(n=16, length=10.0, mu_values=(0.0, 20.0), steps=20, dt=5e-4)
    rows = payload["rows"]
    assert all(row["monotone"] for row in rows)
    assert all(row["H"] < 1.0 for row in rows)
    assert payload["verdict"]["coexistence_H_and_Z3"] is False
    assert payload["verdict"]["single_seed_pilot"] is True
    assert payload["verdict"]["production_ready"] is False
    assert payload["verdict"]["claim_15_status"].startswith("UNVERIFIED")
