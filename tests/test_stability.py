"""Tangent-space Hessian tests (claim 14d)."""

from __future__ import annotations

import numpy as np

from sims.stability import (
    build_candidate,
    hessian_spectrum,
    tangent_basis,
    validate_operator,
)
from theory.hopf import compactified_field


def test_tangent_basis_is_orthonormal():
    field, _dx = compactified_field(8, 4.0)
    normal = field[1:-1, 1:-1, 1:-1]
    normal = normal / np.linalg.norm(normal, axis=-1, keepdims=True)
    e1, e2 = tangent_basis(field)
    assert np.allclose(np.linalg.norm(e1, axis=-1), 1.0)
    assert np.allclose(np.linalg.norm(e2, axis=-1), 1.0)
    assert np.allclose(np.sum(e1 * e2, axis=-1), 0.0, atol=1e-12)
    assert np.allclose(np.sum(e1 * normal, axis=-1), 0.0, atol=1e-12)
    assert np.allclose(np.sum(e2 * normal, axis=-1), 0.0, atol=1e-12)


def test_dense_validation_passes():
    validation = validate_operator(n=8, length=4.0, steps=5)
    assert validation["passed"] is True
    assert validation["max_abs_error"] < 1e-8
    assert validation["symmetry_error"] < 1e-8
    assert np.allclose(
        validation["dense_smallest"], validation["operator_smallest"], atol=1e-8
    )


def test_spectrum_uses_tangent_dimension():
    candidate, info = build_candidate(
        n=8, length=4.0, iterations=1, steps=5
    )
    spectrum = hessian_spectrum(candidate, info["dx"], k=2)
    assert spectrum["dimension"] == 2 * 6**3
    assert len(spectrum["smallest_eigenvalues"]) == 2
    assert spectrum["min_eigenvalue"] <= spectrum["smallest_eigenvalues"][-1]
    assert spectrum["negative_count"] in (0, 1, 2)
