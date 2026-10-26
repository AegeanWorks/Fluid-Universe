"""Hopf grid-resolution tests."""

from __future__ import annotations

import pytest

from theory.hopf import resolution_study


def test_resolution_converges_and_flags_coarse_grids():
    payload = resolution_study(sizes=(16, 32, 64))
    rows = {row["n"]: row for row in payload["rows"]}
    assert rows[16]["H_grid"] == pytest.approx(0.801799780520344, abs=1e-12)
    assert rows[32]["H_grid"] == pytest.approx(1.000141490106325, abs=1e-12)
    assert rows[64]["H_grid"] == pytest.approx(1.0000000385194467, abs=1e-12)
    assert rows[16]["abs_error_vs_preimage"] > 1e-3
    assert rows[32]["abs_error_vs_preimage"] < 1e-3
    assert rows[64]["abs_error_vs_preimage"] < 1e-4
    assert rows[64]["H_grid"] == pytest.approx(1.0, abs=1e-6)
    assert payload["verdict"]["coarse_under_resolved"] is True
    assert payload["verdict"]["fine_within_1e_3"] is True
