"""New-prediction audit tests (claim 16)."""

from __future__ import annotations

from phenomenology.predictions import CANDIDATES, run_prediction_audit


def test_candidate_table_is_complete():
    classes = {row["class"] for row in CANDIDATES}
    assert "mass relation" in classes
    assert "mixing relation" in classes
    assert "dispersion deviation" in classes
    assert "defect-spectrum count" in classes


def test_no_new_prediction_is_found():
    payload = run_prediction_audit(write=False)
    assert payload["verdict"]["new_prediction_found"] is False
    assert payload["verdict"]["count"] == 0
    assert all(row["new"] is False for row in payload["candidates"])
