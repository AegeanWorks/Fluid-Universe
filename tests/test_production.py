"""Production-scan smoke tests (claim 15)."""

from __future__ import annotations

import numpy as np
import pytest

from sims.core3d import relax
from sims.production import MAX_WALL_SECONDS, run_production, run_stage, spectral_upsample
from theory.hopf import chern_simons_charge, compactified_field


def test_spectral_upsample_preserves_analytic_charge():
    field, dx = compactified_field(24, 10.0)
    upsampled = spectral_upsample(field, 2)
    assert upsampled.shape == (48, 48, 48, 3)
    assert np.allclose(np.linalg.norm(upsampled, axis=-1), 1.0)
    assert abs(chern_simons_charge(upsampled, dx / 2) - 1.0) < 1e-3


def test_run_stage_matches_relax():
    field, dx = compactified_field(8, 6.0)
    relaxed, _history, terms = relax(
        field, dx, 2.0, 5, 5e-4, fix_boundary=True
    )
    staged, stage = run_stage(field, dx, 2.0, 5, 5e-4)
    assert np.allclose(staged, np.asarray(relaxed), atol=1e-12)
    assert stage["E"] == terms["E"]
    assert stage["force_rms"] == terms["force_rms"]
    assert stage["min_pre_norm"] > 0.1


def test_production_verdict_shape():
    payload = run_production(
        n=8, length=6.0, mu=2.0, steps=3, dt=5e-4, upsample_factor=1
    )
    assert payload["verdict"]["claim_15_status"].startswith("UNVERIFIED")
    assert payload["verdict"]["production_ready"] is False
    assert payload["final"]["H"] == payload["rows"][-1]["H"]
    assert len(payload["history"]) == 4
    assert payload["min_pre_norm"] > 0.1
    assert payload["max_wall_seconds"] == MAX_WALL_SECONDS == 900.0


def test_production_enforces_wall_clock_limit():
    with pytest.raises(TimeoutError, match="G-003"):
        run_production(
            n=8,
            length=6.0,
            mu=2.0,
            steps=3,
            dt=5e-4,
            upsample_factor=1,
            max_wall_seconds=1e-9,
        )


def test_run_production_resumes_from_a_field_exactly():
    first = run_production(
        n=8,
        length=6.0,
        mu=2.0,
        steps=3,
        dt=5e-4,
        upsample_factor=1,
        return_field=True,
    )
    direct = run_production(
        n=8,
        length=6.0,
        mu=2.0,
        steps=6,
        dt=5e-4,
        upsample_factor=1,
        return_field=True,
    )
    second = run_production(
        n=8,
        length=6.0,
        mu=2.0,
        steps=3,
        dt=5e-4,
        upsample_factor=1,
        initial_field=first["field"],
        return_field=True,
    )
    assert np.allclose(second["field"], direct["field"], atol=1e-12)
    assert second["final"]["H"] == pytest.approx(direct["final"]["H"], abs=1e-12)
    assert second["rows"][-1]["step"] == 3


def test_run_production_validates_initial_field_and_hides_it_by_default():
    field, _dx = compactified_field(8, 6.0)
    with pytest.raises(ValueError, match="initial_field"):
        run_production(
            n=8,
            length=6.0,
            mu=2.0,
            steps=1,
            dt=5e-4,
            upsample_factor=1,
            initial_field=field[:-1],
        )
    payload = run_production(
        n=8, length=6.0, mu=2.0, steps=1, dt=5e-4, upsample_factor=1
    )
    assert "field" not in payload
