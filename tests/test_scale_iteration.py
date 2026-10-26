"""Scale-iteration candidate tests."""

from __future__ import annotations

import numpy as np
import pytest

from sims.scale_iteration import rescale_field, run_scale_iteration


def test_rescale_preserves_unit_norm_and_boundary():
    rng = np.random.default_rng(3)
    field = rng.normal(size=(8, 8, 8, 3))
    field /= np.linalg.norm(field, axis=-1, keepdims=True)
    out = rescale_field(field, dx=6.0 / 8.0, length=6.0, scale=0.9)
    assert out.shape == field.shape
    assert np.allclose(np.linalg.norm(out, axis=-1), 1.0)
    vacuum = np.array([0.0, 0.0, 1.0])
    assert np.allclose(out[0], vacuum)
    assert np.allclose(out[:, 0], vacuum)


def test_smoke_scale_iteration():
    payload = run_scale_iteration(n=12, length=6.0, iterations=1, steps=5, dt=5e-4)
    assert len(payload["rows"]) == 1
    row = payload["rows"][0]
    for key in ("E", "E2", "E4", "virial", "H", "Z3", "force_rms", "lambda"):
        assert key in row
    assert payload["verdict"]["full_stability_proved"] is False
    assert payload["verdict"]["claim_14b_status"] == "UNVERIFIED"
    assert payload["verdict"]["claim_14c_status"].startswith("NUMERIC")
