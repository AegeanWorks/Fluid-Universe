"""Center+width co-emergence tests (claim 53)."""

from __future__ import annotations

import math

import numpy as np
import pytest

from theory.variational_center import (
    BASE,
    GRID_N,
    HORIZON_FRACTION,
    RELEASES,
    STARTS,
    discrete_el_system,
    dynamics_scales,
    horizon,
    newton_pair,
    release_pair,
)


def test_joint_extremum_is_constant_path():
    scales = dynamics_scales()

    for entry in scales["extrema"]:
        assert entry["converged"] is True
        assert entry["max_dev_mu"] <= 1e-9
        assert entry["max_dev_sg"] <= 1e-9
        assert entry["max_residual"] <= 1e-12


def test_mixed_start_converges_to_rest():
    entry = newton_pair(0.5, 1.0)

    assert entry["converged"] is True
    assert entry["max_dev_mu"] <= 1e-9
    assert entry["max_dev_sg"] <= 1e-9


def test_center_period_is_two_pi():
    out = release_pair(0.1, 1.0)

    assert math.isfinite(out["mu_period"])
    assert abs(out["mu_period"] - 2.0 * math.pi) <= 1e-3


def test_width_period_is_pi():
    out = release_pair(0.0, 1.01)

    assert math.isfinite(out["sg_period"])
    assert abs(out["sg_period"] - math.pi) <= 1e-3


def test_term_match_is_exact():
    from theory.gaussian_fluid import ermakov_rhs
    from theory.variational_center import sigma_acceleration

    for s in (0.5, 0.7071067811865476, 1.0, 1.5):
        own = sigma_acceleration(s, 1.0, 1.0, 1.0)
        _, ermakov = ermakov_rhs(s, 0.0, 1.0, 1.0)

        assert own == pytest.approx(float(ermakov), rel=1e-9)


def test_scaling_gives_pi_and_half_pi():
    mu_out = release_pair(0.05, 1.0, omega=2.0)
    sg_out = release_pair(0.0, 1.01, omega=2.0)

    assert abs(mu_out["mu_period"] - math.pi) <= 1e-3
    assert abs(sg_out["sg_period"] - math.pi / 2.0) <= 1e-3


def test_coemergence_verdict_holds():
    from phenomenology.variational_center_report import run_variational_center_check

    payload = run_variational_center_check(write=False)
    gates = payload["result"]["gates"]
    verdict = payload["result"]["verdict"]

    assert not any(k for _, k in gates["kills"])
    assert verdict["claim_53_status"].startswith("NUMERIC")


def test_inputs_are_frozen():
    assert BASE == {"omega": 1.0, "hbar": 1.0, "mass": 1.0}
    assert GRID_N == 401
    assert HORIZON_FRACTION == 0.25
    assert STARTS == ((0.0, 1.0), (0.0, 1.05), (0.0, 0.95), (0.5, 1.0), (-0.5, 1.0))
    assert RELEASES == ((0.1, 1.0), (0.0, 1.01), (0.3, 1.2))
    assert horizon(1.0) == pytest.approx(math.pi / 4.0)
    with pytest.raises(ValueError, match="positive"):
        horizon(-1.0)
