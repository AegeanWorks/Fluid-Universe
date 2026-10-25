"""Minimal circulant PMNS branches (claim 7b)."""

from __future__ import annotations

import numpy as np
import pytest

from phenomenology.pmns import angles_from_pmns, dft_matrix, run_pmns_check


def test_identity_has_no_mixing():
    angles = angles_from_pmns(np.eye(3))
    assert angles["theta12_deg"] == pytest.approx(0.0)
    assert angles["theta13_deg"] == pytest.approx(0.0)
    assert angles["theta23_deg"] == pytest.approx(0.0)


def test_dft_is_trimaximal():
    U = dft_matrix()
    assert np.allclose(np.abs(U), 1.0 / np.sqrt(3.0))
    angles = angles_from_pmns(U)
    assert angles["theta13_deg"] == pytest.approx(35.264389682754654, abs=1e-10)
    assert angles["theta12_deg"] == pytest.approx(45.0, abs=1e-10)
    assert angles["theta23_deg"] == pytest.approx(45.0, abs=1e-10)


def test_both_branches_excluded_and_claim_ill_posed():
    payload = run_pmns_check(write=False)
    assert payload["verdict"]["same_basis_branch_excluded"]
    assert payload["verdict"]["trimaximal_branch_excluded_by_theta13"]
    assert not payload["verdict"]["original_claim_well_posed"]
    assert payload["measured"]["theta13_deg"] == pytest.approx(8.58, abs=1e-12)
