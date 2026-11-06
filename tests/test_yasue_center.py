"""Yasue center-width discriminator tests (claim 55)."""

from __future__ import annotations

import math

import numpy as np
import pytest

from theory.yasue_center import (
    BASE,
    DT,
    RELEASES,
    STARTS,
    T_END,
    YASUE_FLOORS,
    newton_attempt_yasue,
    release_pair_yasue,
    static_residuals,
    term_match,
    yasue_acceleration,
    yasue_scales,
)


def test_yasue_static_residual_is_order_one():
    statics = static_residuals()

    assert len(statics) == len(STARTS)
    for entry in statics:
        assert entry["max_residual"] > 1e-3
        assert entry["max_res_mu"] <= 1e-9


def test_yasue_arc_attained_single_attractor():
    residuals = []
    mins = []
    maxs = []
    for mu0, s0 in STARTS:
        attempt = newton_attempt_yasue(mu0, s0)

        assert attempt["attained"] is True
        assert attempt["diverged"] is False
        residuals.append(attempt["max_residual"])
        mins.append(attempt["min_sigma"])
        maxs.append(attempt["max_sigma"])

    assert max(residuals) <= 1e-9
    assert max(mins) - min(mins) <= 1e-9
    assert max(maxs) - min(maxs) <= 1e-9
    assert min(mins) > 0.7071067811865476
    assert max(maxs) > min(mins)


def test_yasue_center_tracks_width_collapses():
    scales = yasue_scales()

    for entry in scales["releases"]:
        assert entry["mu_track_max_dev"] <= 1e-4
        assert entry["monotone"] is True
        assert entry["floor_hit_time"] is not None
        assert entry["floor_hit_time"] < math.pi
        assert entry["upward_crossings"] == 0


def test_gm_twin_controls_the_apparatus():
    scales = yasue_scales()

    for entry in scales["twins"]:
        if entry["mu0"] != 0.0:
            assert math.isfinite(entry["mu_period"])
            assert abs(entry["mu_period"] - 2.0 * math.pi) <= 1e-3
        if entry["s0_factor"] != 1.0:
            assert math.isfinite(entry["sg_period"])
            assert abs(entry["sg_period"] - math.pi) <= 1e-3


def test_yasue_accel_negative_everywhere():
    grid = np.sqrt(np.linspace(0.125, 2.0, 2001))

    assert all(yasue_acceleration(float(s), 1.0, 1.0, 1.0) < 0.0 for s in grid)
    assert yasue_acceleration(0.5, 1.0, 1.0, 1.0) == pytest.approx(-2.5)


def test_quadrature_matches_verlet_at_mild_point():
    scales = yasue_scales()

    for quad, entry in zip(scales["mild_quadrature"], scales["releases"]):
        mild = entry["mild_crossing_time"]
        if quad is None and mild is None:
            continue
        assert quad is not None and mild is not None
        assert abs(mild - quad) / quad <= 1e-3


def test_scaling_bounds_collapse():
    scales = yasue_scales()
    scaling = scales["scaling"]

    assert scaling["yasue"]["floor_hit_time"] is not None
    assert scaling["yasue"]["floor_hit_time"] < math.pi / 4.0
    assert abs(scaling["twin_mu"]["mu_period"] - math.pi) <= 1e-3
    assert abs(scaling["twin_sg"]["sg_period"] - math.pi / 2.0) <= 1e-3


def test_discriminator_verdict_holds():
    from phenomenology.yasue_center_report import run_yasue_center_check

    payload = run_yasue_center_check(write=False)
    gates = payload["result"]["gates"]
    verdict = payload["result"]["verdict"]

    assert not any(k for _, k in gates["kills"])
    assert verdict["claim_55_status"].startswith("NUMERIC")


def test_inputs_are_frozen():
    assert BASE == {"omega": 1.0, "hbar": 1.0, "mass": 1.0}
    assert STARTS == ((0.0, 1.0), (0.0, 1.05), (0.0, 0.95), (0.5, 1.0), (-0.5, 1.0))
    assert RELEASES == ((0.1, 1.0), (0.0, 1.01), (0.3, 1.2))
    assert YASUE_FLOORS == (0.05, 0.02)
    assert DT == 0.001
    assert T_END == 4.0 * math.pi
    assert term_match()["max_accel"] < 0.0
    with pytest.raises(ValueError, match="positive"):
        yasue_acceleration(-1.0, 1.0, 1.0, 1.0)
    probe = release_pair_yasue(0.3, 1.2, floor=0.02)
    assert probe["monotone"] is True
    assert probe["upward_crossings"] == 0
