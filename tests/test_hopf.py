"""Hopf-charge estimator tests (claim 15, baseline 4)."""

from __future__ import annotations

import numpy as np
import pytest

from theory.hopf import (
    grid_charge,
    hopf_map,
    preimage_charge,
    reflection_charge,
    rotation_charge,
    shear_charge,
    target_pair_charge,
    validate,
)


def test_hopf_map_is_unit_and_boundary_condition():
    x = [0.0, 1.0, 5.0, -5.0]
    y = [0.0, 0.0, 0.0, 0.0]
    z = [0.0, 0.0, 0.0, 0.0]
    values = hopf_map(np.array(x), np.array(y), np.array(z))
    norms = np.linalg.norm(values, axis=-1)
    assert np.allclose(norms, 1.0)
    assert np.allclose(values[-2], [0.0, 0.0, 1.0], atol=1e-6)
    assert np.allclose(values[-1], [0.0, 0.0, 1.0], atol=1e-6)


def test_preimage_linking_converges_to_one():
    values = [preimage_charge(m) for m in (100, 200, 400)]
    assert values[0] < values[1] < values[2]
    assert values[-1] == pytest.approx(1.0, abs=1e-3)
    assert target_pair_charge(200) == pytest.approx(1.0, abs=1e-3)


def test_reflection_flips_sign_and_rotation_shear_preserve():
    base = preimage_charge(200)
    assert reflection_charge(200) == pytest.approx(-base, abs=1e-6)
    assert rotation_charge(200) == pytest.approx(base, abs=1e-6)
    assert shear_charge(200) == pytest.approx(base, abs=1e-5)


def test_grid_integral_matches_known_answer():
    values = [grid_charge(n) for n in (32, 48, 64)]
    assert all(abs(value - 1.0) < 0.01 for value in values)
    assert values[1] == pytest.approx(1.0, abs=1e-4)
    assert values[2] == pytest.approx(1.0, abs=1e-4)
    assert grid_charge(48, reflect=True) == pytest.approx(-1.0, abs=0.01)
    payload = validate()
    assert payload["verdict"]["grid_integral_matches_one"] is True
    assert payload["verdict"]["grid_reflection_flips_sign"] is True
    assert payload["verdict"]["two_method_agreement"] is True
    assert payload["verdict"]["baseline_4_pass"] is True
    assert payload["verdict"]["claim_15_status"] == "NOT ACHIEVED"
