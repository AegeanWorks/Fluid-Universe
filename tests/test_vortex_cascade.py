"""One-parameter vortex hierarchy tests (claim 18)."""

from __future__ import annotations

import numpy as np
import pytest

from theory.vortex_cascade import (
    geometric_spectrum,
    power_law_spectrum,
    solve_power_for_koide,
    run_hypotheses,
)
from theory.vortex_particles import koide_q

LEPTONS = (0.51099895, 105.6583755, 1776.93)


def test_power_law_koide_root():
    exponent = solve_power_for_koide()

    assert exponent == pytest.approx(7.0643, abs=1e-3)
    assert koide_q(power_law_spectrum(exponent)) == pytest.approx(2.0 / 3.0)
    assert 3.0**exponent == pytest.approx(2347.08, rel=1e-3)


def test_geometric_cascade_is_falsified():
    result = run_hypotheses(LEPTONS)
    geometric = result["models"]["geometric"]

    assert geometric["predicted_tau_ratio"] == pytest.approx(
        (LEPTONS[1] / LEPTONS[0]) ** 2
    )
    assert geometric["tau_relative_error"] > 1.0
    assert geometric["koide_q_predicted"] > 0.8
    assert geometric["falsified"] is True


def test_power_law_muon_fit_is_falsified():
    result = run_hypotheses(LEPTONS)
    power = result["models"]["power_law_muon_fit"]

    assert power["tau_relative_error"] > 0.1
    assert power["koide_q_predicted"] == pytest.approx(0.6965, abs=1e-3)
    assert power["falsified"] is True


def test_all_one_parameter_hierarchies_fail():
    result = run_hypotheses(LEPTONS)

    assert result["falsified"] is True
    assert result["status"].startswith("FALSIFIED")
    assert all(model["falsified"] for model in result["models"].values())


def test_invalid_inputs_are_rejected():
    with pytest.raises(ValueError, match="positive"):
        geometric_spectrum(0.0)
    with pytest.raises(ValueError, match="target"):
        solve_power_for_koide(0.1)
    with pytest.raises(ValueError, match="positive"):
        run_hypotheses((1.0, -1.0, 2.0))
