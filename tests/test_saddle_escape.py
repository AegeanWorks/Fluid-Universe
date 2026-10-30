"""Saddle-escape mode tests (claim 14b/ADR-006)."""

from __future__ import annotations

import numpy as np
import pytest

from sims.stability import (
    build_candidate,
    classify_escape,
    hessian_modes,
    normalize_mode,
    perturb_along_mode,
)
from theory.hopf import compactified_field


def test_hessian_modes_returns_a_tangent_field():
    candidate, info = build_candidate(n=8, length=4.0, iterations=1, steps=5)
    result = hessian_modes(candidate, info["dx"], k=1)

    assert result["dimension"] == 2 * 6**3
    assert len(result["modes"]) == 1
    mode = result["modes"][0]
    assert mode.shape == candidate.shape
    assert np.all(mode[0] == 0.0)
    assert np.all(mode[-1] == 0.0)
    normal = candidate[1:-1, 1:-1, 1:-1]
    assert np.allclose(
        np.sum(mode[1:-1, 1:-1, 1:-1] * normal, axis=-1), 0.0, atol=1e-10
    )
    normalized = normalize_mode(mode)
    rms = np.sqrt(np.mean(normalized[1:-1, 1:-1, 1:-1] ** 2))
    assert rms == pytest.approx(1.0)


def test_perturb_along_mode_preserves_norm_and_boundary():
    field, _dx = compactified_field(8, 4.0)
    rng = np.random.default_rng(20261030)
    mode = np.zeros_like(field)
    mode[1:-1, 1:-1, 1:-1] = rng.normal(size=(6, 6, 6, 3))

    perturbed = perturb_along_mode(field, mode, 0.1)

    assert np.allclose(np.linalg.norm(perturbed, axis=-1), 1.0)
    assert np.allclose(perturbed[0], field[0])
    assert np.allclose(perturbed[-1], field[-1])


def test_perturb_along_mode_rejects_invalid_inputs():
    field, _dx = compactified_field(8, 4.0)
    with pytest.raises(ValueError, match="zero interior norm"):
        normalize_mode(np.zeros_like(field))
    with pytest.raises(ValueError, match="amplitude"):
        perturb_along_mode(field, np.zeros_like(field), 0.0)


def test_classify_escape_finds_a_local_minimum():
    verdict = classify_escape([{"local_minimum": False}, {"local_minimum": True}])

    assert verdict["local_minimum_found"] is True
    assert verdict["claim_14b_status"].startswith("NUMERIC")


def test_classify_escape_keeps_the_claim_unverified():
    verdict = classify_escape([{"local_minimum": False}, {"local_minimum": False}])

    assert verdict["local_minimum_found"] is False
    assert verdict["claim_14b_status"] == "UNVERIFIED"
