"""Resolution comparison for the claim-15 production scan (ADR-002).

The N=64 production scan has a native Hopf estimate below 1 while the 2x
upsampled estimate is close to 1, so one grid cannot separate a grid-scale
estimator artifact from a real failure of the configuration.  This module
compares two production payloads at the same physical time and reports whether
the native and upsampled estimates, the Z3 order, the residual force and the
field smoothness improve with resolution.

The comparison is deliberately pure: it never imports JAX or runs a scan, and it
never promotes claim 15.  The pre-registered tolerances are fixed in ADR-002.
"""

from __future__ import annotations

H_RESOLUTION_TOL = 1.0e-2
Z3_RESOLUTION_TOL = 2.0e-2
_SAME_PROBLEM_KEYS = ("length", "mu", "steps", "dt")


def _require_same_problem(low: dict, high: dict) -> None:
    if low["n"] >= high["n"]:
        raise ValueError("low resolution must have the smaller n")
    for key in _SAME_PROBLEM_KEYS:
        if low[key] != high[key]:
            raise ValueError(
                "scans must solve the same physical problem: "
                f"{key} differs ({low[key]!r} != {high[key]!r})"
            )


def compare_scans(low: dict, high: dict) -> dict:
    """Compare a coarse and a fine production payload at the same physical time."""
    _require_same_problem(low, high)
    coarse = low["final"]
    fine = high["final"]

    delta_h = fine["H"] - coarse["H"]
    delta_h_up = fine["H_upsampled"] - coarse["H_upsampled"]
    delta_z3 = fine["Z3"] - coarse["Z3"]
    delta_force = fine["force_rms"] - coarse["force_rms"]
    delta_bond = fine["max_bond"] - coarse["max_bond"]

    criteria = {
        "H_agrees": abs(delta_h) <= H_RESOLUTION_TOL,
        "H_upsampled_agrees": abs(delta_h_up) <= H_RESOLUTION_TOL,
        "Z3_agrees": abs(delta_z3) <= Z3_RESOLUTION_TOL,
        "force_not_worse": delta_force <= 0.0,
        "max_bond_not_worse": delta_bond <= 0.0,
        "H_toward_one": abs(fine["H"] - 1.0) <= abs(coarse["H"] - 1.0),
        "H_upsampled_toward_one": abs(fine["H_upsampled"] - 1.0)
        <= abs(coarse["H_upsampled"] - 1.0),
    }
    resolution_converged = all(
        criteria[key]
        for key in (
            "H_agrees",
            "H_upsampled_agrees",
            "Z3_agrees",
            "force_not_worse",
            "max_bond_not_worse",
        )
    )
    return {
        "n_low": low["n"],
        "n_high": high["n"],
        "steps": high["steps"],
        "dt": high["dt"],
        "deltas": {
            "H": delta_h,
            "H_upsampled": delta_h_up,
            "Z3": delta_z3,
            "force_rms": delta_force,
            "max_bond": delta_bond,
        },
        "tolerances": {
            "H": H_RESOLUTION_TOL,
            "Z3": Z3_RESOLUTION_TOL,
        },
        "criteria": criteria,
        "resolution_converged": bool(resolution_converged),
        "claim_15_status": high["verdict"]["claim_15_status"],
        "note": (
            "Resolution convergence is a diagnostic, not claim verification: "
            "the fine run must still satisfy the unchanged production criteria."
        ),
    }
