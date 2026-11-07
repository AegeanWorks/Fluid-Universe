"""2p core-threading loop probe tests (claim 61)."""

from __future__ import annotations

import math

import numpy as np


def test_delta0_is_predicted():
    from theory.hydrogen_fluid import bohr_radius
    from theory.vortex_loop import core_scale

    assert core_scale() == 0.1 * bohr_radius(1.0, 1.0, 1.0) == 0.1


def test_blob_fractions_match_frozen_values():
    from theory.vortex_loop import blob_fraction

    assert blob_fraction(0.1, 0.1) == 0.5
    assert blob_fraction(0.2, 0.1) == 0.8
    assert abs(blob_fraction(2.0, 0.1) - 0.9975062344139651) <= 1e-12


def test_floor_vmax_diverges_by_rule():
    from theory.vortex_loop import floor_vmax

    assert floor_vmax(256) == 25.6
    assert floor_vmax(2048) == 204.8
    assert floor_vmax(2048) > floor_vmax(1024) > floor_vmax(512) > floor_vmax(256)


def test_loop_verdict_follows_gates():
    from phenomenology.vortex_loop_report import run_vortex_loop_check

    payload = run_vortex_loop_check(write=False)
    s = payload["result"]["scales"]
    assert abs(s["singular_loop_r2"] - 2.0 * math.pi) / (2.0 * math.pi) <= 1e-6
    v = payload["result"]["verdict"]
    assert v["claim_61_status"].startswith(("FALSIFIED", "NUMERIC", "UNVERIFIED"))
