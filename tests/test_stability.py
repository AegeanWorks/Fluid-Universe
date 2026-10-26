"""Coarse stability-check smoke tests."""

from __future__ import annotations

import numpy as np

from sims.stability import build_candidate, hessian_spectrum


def test_candidate_and_spectrum_shapes():
    candidate, info = build_candidate(n=8, length=4.0, iterations=1, steps=5, dt=5e-4)
    assert candidate.shape == (8, 8, 8, 3)
    assert np.allclose(np.linalg.norm(candidate, axis=-1), 1.0)
    spectrum = hessian_spectrum(candidate, info["dx"], k=2)
    assert spectrum["interior_dimension"] == 6**3 * 3
    assert len(spectrum["smallest_eigenvalues"]) == 2
    assert spectrum["min_eigenvalue"] <= spectrum["smallest_eigenvalues"][-1]
