"""Quark Koide ratios from full-SM MS-bar masses (claim 8)."""

from __future__ import annotations

import pytest

from phenomenology.quarks import load_rows, q_down, q_up, run_quark_check


def test_data_has_six_scales():
    rows = load_rows()
    assert len(rows) == 6
    assert rows[0].scale_label == "MZ"
    assert rows[-1].scale_label == "1e12"


def test_mz_regression_values():
    row = load_rows()[0]
    assert q_up(row) == pytest.approx(0.8875692313187327, abs=1e-12)
    assert q_down(row) == pytest.approx(0.7480650548031402, abs=1e-12)


def test_direct_extension_falsified():
    payload = run_quark_check(write=False)
    assert payload["verdict"]["direct_extension_falsified"]
    assert payload["verdict"]["min_abs_Q_minus_2_3"] > 0.05
    for point in payload["points"]:
        assert point["Q_up"] > 2.0 / 3.0
        assert point["Q_down"] > 2.0 / 3.0


def test_ranges_are_scale_dependent():
    payload = run_quark_check(write=False)
    ranges = payload["ranges"]
    assert ranges["Q_up_max"] > ranges["Q_up_min"]
    assert ranges["Q_down_max"] > ranges["Q_down_min"]
    assert ranges["Q_up_min"] > 0.88
    assert ranges["Q_down_max"] < 0.75
