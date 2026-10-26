"""Coarse linear stability check for the scale-stationary H=1 candidate.

This computes the Hessian spectrum of the pure Faddeev-Niemi energy at a
scale-iteration candidate on a small grid (N=12 by default), with fixed vacuum
boundaries and the radial normalization mode projected out.

A positive spectrum at N=12 is evidence, not a proof, of stability at N=64.
A negative eigenvalue would falsify stability of this candidate.
"""

from __future__ import annotations

import math

import jax
import jax.numpy as jnp
import numpy as np
from scipy.sparse.linalg import LinearOperator, eigsh

from sims.core3d import energy, normalize, relax, z3_order
from sims.scale_iteration import rescale_field
from theory.hopf import chern_simons_charge, compactified_field


def build_candidate(
    n: int = 12,
    length: float = 6.0,
    iterations: int = 6,
    steps: int = 300,
    dt: float = 5.0e-4,
) -> tuple[np.ndarray, dict]:
    initial, dx = compactified_field(n, length)
    field = initial
    best = np.asarray(initial)
    best_virial = float("inf")
    best_terms = None
    for _ in range(iterations):
        relaxed, _history, terms = relax(field, dx, 0.0, steps, dt, fix_boundary=True)
        snapshot = np.asarray(relaxed)
        virial = terms["E2"] - terms["E4"]
        if abs(virial) < best_virial:
            best_virial = abs(virial)
            best = snapshot
            best_terms = terms
        lam = math.sqrt(terms["E4"] / terms["E2"])
        field = rescale_field(snapshot, dx, length, math.sqrt(lam))
    if best_terms is None:
        raise RuntimeError("no candidate recorded")
    return best, {
        "n": n,
        "length": length,
        "dx": dx,
        "E2": float(best_terms["E2"]),
        "E4": float(best_terms["E4"]),
        "virial": float(best_virial),
        "H": chern_simons_charge(best, dx),
        "Z3": z3_order(best),
        "force_rms": float(best_terms["force_rms"]),
    }


def hessian_spectrum(candidate: np.ndarray, dx: float, k: int = 10) -> dict:
    n = candidate.shape[0]
    v = jnp.asarray(candidate)
    interior = (n - 2) ** 3 * 3

    normal_full = normalize(v)
    grad_fn = jax.jit(lambda x: jax.grad(energy, argnums=0)(x, dx, 1.0, 1.0, 0.0))

    def matvec(vector: np.ndarray) -> np.ndarray:
        full = jnp.zeros_like(v)
        full = full.at[1:-1, 1:-1, 1:-1].set(
            jnp.asarray(vector).reshape(n - 2, n - 2, n - 2, 3)
        )
        full = full - jnp.sum(full * normal_full, axis=-1, keepdims=True) * normal_full
        _, product = jax.jvp(grad_fn, (v,), (full,))
        product = product[1:-1, 1:-1, 1:-1].reshape(-1, 3)
        normal = normal_full[1:-1, 1:-1, 1:-1].reshape(-1, 3)
        product = product - jnp.sum(product * normal, axis=1, keepdims=True) * normal
        return np.asarray(product).reshape(-1)

    operator = LinearOperator((interior, interior), matvec=matvec, dtype=np.float64)
    values = eigsh(operator, k=min(k, interior - 1), which="SA", return_eigenvectors=False)
    values = np.sort(values)
    return {
        "n": n,
        "interior_dimension": interior,
        "smallest_eigenvalues": [float(x) for x in values],
        "min_eigenvalue": float(values[0]),
        "negative_count": int(np.sum(values < -1e-6)),
        "near_zero_count": int(np.sum(np.abs(values) < 1e-4)),
    }


def run_stability_check(n: int = 12, length: float = 6.0, k: int = 10) -> dict:
    candidate, info = build_candidate(n=n, length=length)
    spectrum = hessian_spectrum(candidate, info["dx"], k=k)
    return {
        "candidate": info,
        "spectrum": spectrum,
        "verdict": {
            "local_minimum_at_this_N": bool(spectrum["min_eigenvalue"] > -1e-6),
            "stable_at_N64_proved": False,
            "claim_14b_status": "UNVERIFIED",
            "note": (
                "Hessian spectrum at N=12 only. A positive spectrum is evidence, "
                "not a proof, of stability at N=64. A negative eigenvalue would "
                "falsify stability of this candidate."
            ),
        },
    }
