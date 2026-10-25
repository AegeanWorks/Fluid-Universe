"""Residual anisotropy and LV-bound tests (claims 11-12)."""

from __future__ import annotations

import numpy as np
import pytest

from theory.lorentz import (
    BOUNDS,
    axis_speeds,
    conditional_bound,
    residual_anisotropy,
    speeds,
    verdict,
)


def test_equal_K_is_isotropic():
    for direction in ((1, 0, 0), (0, 1, 0), (0, 0, 1), (1, 2, 3)):
        values = speeds(direction, 2.0, 2.0, 2.0)
        assert np.allclose(values, values[0])
    aniso = residual_anisotropy(2.0, 2.0, 2.0)
    assert aniso["xi"] == pytest.approx(0.0, abs=1e-15)
    assert verdict(2.0, 2.0, 2.0)["12_equal_K_is_isotropic"]


def test_unequal_K_has_order_one_anisotropy():
    aniso = residual_anisotropy(1.0, 2.0, 3.0)
    assert aniso["xi"] > 0.1
    assert aniso["c2_min"] < aniso["c2_max"]
    axis = axis_speeds(1.0, 2.0, 3.0)
    assert axis["along_x"] == pytest.approx([1.0, np.sqrt(2.0)])
    assert axis["along_z"] == pytest.approx([np.sqrt(3.0), np.sqrt(3.0)])


def test_bounds_are_marked_not_directly_applicable():
    assert len(BOUNDS) == 4
    assert all(bound["applies_to_director_modes"] is False for bound in BOUNDS)
    assert all("photon" in bound["why"] or "not modelled" in bound["why"] for bound in BOUNDS)


def test_conditional_bound_excludes_order_one_anisotropy():
    check = conditional_bound(1.0, 2.0, 3.0)
    assert check["xi"] > 1e-18
    assert all(item["passes_if_photon"] is False for item in check["checks"])
    # Equal K passes every Delta c/c bound exactly.
    equal = conditional_bound(2.0, 2.0, 2.0)
    assert equal["xi"] == pytest.approx(0.0, abs=1e-15)
    assert all(item["passes_if_photon"] for item in equal["checks"])


def test_verdict_downgrades_claims_11_and_12():
    v = verdict(1.0, 2.0, 3.0)
    assert v["11a_emergent_metric_at_tree_level"] is False
    assert v["11b_one_loop_RG_to_Lorentz_invariant_IR"] == "UNVERIFIED"
    assert v["12_lv_bounds_directly_applicable"] is False
    assert v["12_conditional_xi_above_grb_bound"] is True
