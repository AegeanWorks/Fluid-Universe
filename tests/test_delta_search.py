"""Tests for the delta closed-form search (claim 9)."""

from __future__ import annotations

import math

import pytest

from phenomenology.delta_search import all_candidates, rational_candidates, run_delta_search


def test_two_ninths_is_in_the_prior():
    values = rational_candidates()
    assert any(abs(value - 2.0 / 9.0) < 1e-15 for value in values)


def test_all_candidates_are_in_range_and_unique():
    values = all_candidates()
    assert len(values) > 100
    assert all(0.0 < value <= 2.0 * math.pi / 3.0 for value in values)


def test_search_finds_two_ninths_and_does_not_promote():
    payload = run_delta_search(write=False)
    best = payload["best"]
    assert best["label"] == "2/9"
    assert best["abs_diff"] == pytest.approx(2.539681804081262e-06, abs=1e-15)
    assert best["sigma_diff"] == pytest.approx(0.4071, abs=1e-3)
    assert payload["look_elsewhere"]["p_look_elsewhere"] < 0.01
    assert payload["promoted"] is False
    assert "not evidence" in payload["verdict"]
