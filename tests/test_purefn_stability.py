"""Pure-FN candidate Hessian classification tests (claims 14b/14d)."""

from __future__ import annotations

from sims.stability import classify_candidate


def test_classify_candidate_accepts_a_local_minimum():
    verdict = classify_candidate(
        {"passed": True}, {"n": 64, "min_eigenvalue": 0.25}
    )

    assert verdict["local_minimum_at_N64"] is True
    assert verdict["claim_14b_status"].startswith("NUMERIC")
    assert verdict["candidate_status"].startswith("NUMERIC")
    assert verdict["stable_at_N64_proved"] is False


def test_classify_candidate_rejects_a_saddle():
    verdict = classify_candidate(
        {"passed": True}, {"n": 64, "min_eigenvalue": -5.799863}
    )

    assert verdict["local_minimum_at_N64"] is False
    assert verdict["claim_14b_status"] == "UNVERIFIED"
    assert verdict["candidate_status"].startswith("FALSIFIED")


def test_classify_candidate_requires_a_validated_operator():
    verdict = classify_candidate(
        {"passed": False}, {"n": 64, "min_eigenvalue": 0.25}
    )

    assert verdict["local_minimum_at_N64"] is False
    assert verdict["claim_14b_status"] == "UNVERIFIED"
    assert verdict["candidate_status"].startswith("UNVERIFIED")
