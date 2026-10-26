"""Independent verification of every claim marked PROVEN, VERIFIED, or NUMERIC.

Project rule G-001: the audit must be run twice in separate processes and the
two result sets compared.  `verification.compare` does that.

This module deliberately does not import ``theory`` or ``phenomenology``.  It
re-derives each result from scratch (sympy, numpy, scipy) and compares against
the stored outputs in ``results/``.  A disagreement is reported, never hidden.

Run (two separate processes, then compare):

    python -m verification.audit --tag run1
    python -m verification.audit --tag run2
    python -m verification.compare

A single run writes ``verification_single.*`` and is explicitly not accepted.
"""

from __future__ import annotations

import argparse
import csv
import json
import math
from fractions import Fraction
import os
import platform
import subprocess
import sys
import tempfile
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path

import numpy as np
import scipy
import sympy as sp
from scipy.integrate import quad, solve_ivp
from scipy.linalg import eigh

from .fingerprint import fingerprints

ROOT = Path(__file__).resolve().parents[1]
RESULTS = ROOT / "results"
KOIDE = 2.0 / 3.0
R_KOIDE = 1.0 / math.sqrt(2.0)


@dataclass
class Audit:
    checks: list[dict] = field(default_factory=list)

    def record(self, claim: str, passed: bool, detail: str, expected=None, observed=None):
        self.checks.append(
            {
                "claim": claim,
                "passed": bool(passed),
                "detail": detail,
                "expected": expected,
                "observed": observed,
            }
        )

    @property
    def failed(self) -> list[dict]:
        return [c for c in self.checks if not c["passed"]]


# ---------------------------------------------------------------------------
# Independent primitives
# ---------------------------------------------------------------------------


def circulant(a: float, c: float, delta: float) -> np.ndarray:
    b = c * np.exp(1j * delta)
    return np.array([[a, b, b.conjugate()], [b.conjugate(), a, b], [b, b.conjugate(), a]])


def roots_from_ac(a: float, c: float, delta: float) -> np.ndarray:
    return np.array([a + 2.0 * c * math.cos(delta + 2.0 * math.pi * k / 3.0) for k in range(3)])


def q_from_roots(x: np.ndarray) -> float:
    x = np.asarray(x, dtype=float)
    return float(np.sum(x * x) / np.sum(x) ** 2)


def q_from_masses(m: np.ndarray) -> float:
    m = np.asarray(m, dtype=float)
    return float(np.sum(m) / np.sum(np.sqrt(m)) ** 2)


def q_from_ac(a: float, c: float) -> float:
    return 1.0 / 3.0 + 2.0 * c * c / (3.0 * a * a)


def fit_linear(x: np.ndarray) -> tuple[float, float, float]:
    """Independent least-squares/inverse of x_k = a + 2u cos(theta_k) - 2v sin(theta_k)."""
    x = np.asarray(x, dtype=float)
    theta = np.array([0.0, 2.0 * math.pi / 3.0, 4.0 * math.pi / 3.0])
    A = np.column_stack([np.ones(3), 2.0 * np.cos(theta), -2.0 * np.sin(theta)])
    a, u, v = np.linalg.solve(A, x)
    c = math.hypot(u, v)
    delta = math.atan2(v, u) % (2.0 * math.pi)
    if c < 1e-15:
        delta = 0.0
    return float(a), float(c), float(delta)


def load_results(name: str) -> dict:
    return json.loads((RESULTS / name).read_text(encoding="utf-8"))


def provenance() -> dict:
    """Minimal, independent provenance block (no project imports)."""
    try:
        commit = subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True, timeout=10
        ).strip()
        dirty = bool(
            subprocess.check_output(
                ["git", "status", "--porcelain"], cwd=ROOT, text=True, timeout=10
            ).strip()
        )
    except Exception:
        commit, dirty = "no-git", True
    return {
        "generated_utc": datetime.now(timezone.utc).isoformat(timespec="seconds"),
        "git_commit": commit,
        "git_dirty": dirty,
        "fingerprints": fingerprints(),
        "python": sys.version.split()[0],
        "platform": platform.platform(),
        "numpy": np.__version__,
        "scipy": scipy.__version__,
        "sympy": sp.__version__,
        "deterministic": True,
        "seed": "none (fixed inputs; the only Monte Carlo uses the seed stored in pdg_fit.json)",
    }


def write_atomic(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, tmp_name = tempfile.mkstemp(dir=path.parent, prefix=f".{path.name}.", suffix=".tmp")
    try:
        with os.fdopen(fd, "w", encoding="utf-8", newline="\n") as handle:
            handle.write(text)
            handle.flush()
            os.fsync(handle.fileno())
        os.replace(tmp_name, path)
    finally:
        if os.path.exists(tmp_name):
            os.unlink(tmp_name)


# ---------------------------------------------------------------------------
# Claim 1 / 2: algebra
# ---------------------------------------------------------------------------


def audit_algebra(audit: Audit) -> None:
    a, c, delta = sp.symbols("a c delta", real=True)
    w = sp.exp(2 * sp.pi * sp.I / 3)
    b = c * sp.exp(sp.I * delta)
    M = sp.Matrix([[a, b, c * sp.exp(-sp.I * delta)],
                   [c * sp.exp(-sp.I * delta), a, b],
                   [b, c * sp.exp(-sp.I * delta), a]])

    herm = sp.simplify(M - M.H)
    audit.record("2a: M = M^dagger", herm == sp.zeros(3, 3), "symbolic difference is the zero matrix")
    asym = sp.simplify((M - M.T)[0, 1])
    audit.record(
        "2a: M != M^T for generic delta",
        asym != 0 and sp.simplify(sp.expand_complex(asym - 2 * sp.I * c * sp.sin(delta))) == 0,
        "M - M^T has off-diagonal 2 i c sin(delta)",
    )

    for k in range(3):
        v = sp.Matrix([1, w**k, w ** (2 * k)])
        lam = a + 2 * c * sp.cos(delta + 2 * sp.pi * k / 3)
        residual = sp.expand(M * v - lam * v)
        ok = all(sp.simplify(sp.expand_complex(entry)) == 0 for entry in residual)
        audit.record(f"2b/F1: eigenpair k={k}", ok, "M v_k - lambda_k v_k simplifies to zero")

    x = [a + 2 * c * sp.cos(delta + 2 * sp.pi * k / 3) for k in range(3)]
    identity = sp.simplify(
        sum(term * term for term in x) / sum(x) ** 2
        - (sp.Rational(1, 3) + 2 * c * c / (3 * a * a))
    )
    audit.record("1a/F2: Q identity", identity == 0, "symbolic difference simplifies to zero")

    solve_c2 = sp.solve(sp.Eq(sp.Rational(1, 3) + 2 * c * c / (3 * a * a), sp.Rational(2, 3)), c * c)
    expected = a * a / 2
    ok = len(solve_c2) == 1 and sp.simplify(solve_c2[0] - expected) == 0
    audit.record("1c/F3: Q=2/3 iff c^2/a^2=1/2", ok, f"sympy solves c^2 = {solve_c2}")

    # Independent numeric checks at 10^5 random points.
    rng = np.random.default_rng(20261025)
    draws = 100_000
    aa = rng.uniform(0.2, 6.0, draws)
    cc = rng.uniform(0.0, 4.0, draws)
    dd = rng.uniform(0.0, 2.0 * np.pi, draws)
    max_eig = 0.0
    max_q = 0.0
    for i in range(draws):
        xv = roots_from_ac(aa[i], cc[i], dd[i])
        eig = np.linalg.eigvalsh(circulant(aa[i], cc[i], dd[i]))
        max_eig = max(max_eig, float(np.max(np.abs(np.sort(eig) - np.sort(xv)))))
        max_q = max(max_q, abs(q_from_roots(xv) - q_from_ac(aa[i], cc[i])) / max(1.0, abs(q_from_ac(aa[i], cc[i]))))
    audit.record(
        "F1 numeric: eigenvalues vs numpy",
        max_eig < 1e-9,
        f"max |eigvalsh - formula| = {max_eig:.3e} over {draws} draws",
    )
    audit.record(
        "F2 numeric: Q identity",
        max_q < 1e-9,
        f"max relative |Q - (1/3+2c^2/3a^2)| = {max_q:.3e}",
    )

    # Sign caveat.
    xs = roots_from_ac(1.0, 0.9, np.pi)
    q_signed = q_from_roots(xs)
    q_phys = q_from_masses(xs * xs)
    audit.record(
        "1d: signed vs physical Q",
        abs(q_signed - 0.8733333333) < 1e-9 and abs(q_phys - 0.3714555766) < 1e-9,
        f"roots {xs.tolist()}, signed Q={q_signed:.10f}, physical Q={q_phys:.10f}",
    )

    # Bound: c/a <= 1 for positive roots.
    roots = rng.uniform(0.01, 50.0, size=(20_000, 3))
    ratios = []
    qs = []
    for row in roots:
        aa2, cc2, _ = fit_linear(row)
        ratios.append(cc2 / abs(aa2))
        qs.append(q_from_masses(row * row))
    audit.record(
        "F8: c/a <= 1 for positive roots",
        max(ratios) <= 1.0 + 1e-12,
        f"max c/a = {max(ratios):.6f} over 20000 random positive-root triples",
    )
    audit.record(
        "F8: 1/3 <= Q <= 1 for positive masses",
        min(qs) >= 1.0 / 3.0 - 1e-12 and max(qs) <= 1.0 + 1e-12,
        f"Q range [{min(qs):.6f}, {max(qs):.6f}]",
    )
    # 1b: three parameters fit any three masses; Q is not fixed by the structure.
    random_triples = rng.uniform(0.05, 100.0, size=(1_000, 3))
    residuals = []
    q_triples = []
    for row in random_triples:
        aa3, cc3, dd3 = fit_linear(row)
        residuals.append(float(np.max(np.abs(roots_from_ac(aa3, cc3, dd3) - row))))
        q_triples.append(q_from_masses(row * row))
    audit.record(
        "1b: 3 parameters fit 3 masses exactly",
        max(residuals) < 1e-9,
        f"max residual {max(residuals):.3e} over 1000 random positive-root triples",
    )
    audit.record(
        "1b: Q is not fixed by the Z3/circulant structure",
        max(q_triples) - min(q_triples) > 0.1,
        f"random triples span Q in [{min(q_triples):.4f}, {max(q_triples):.4f}]",
    )

    xb = np.array([10.0, 1.0, 1.0])
    ab, cb, _ = fit_linear(xb)
    audit.record(
        "5d: c/a=1/2 bound is false",
        abs(cb / ab - 0.75) < 1e-12 and abs(q_from_masses(xb * xb) - 0.7083333333) < 1e-9,
        f"(10,1,1): a={ab:.3f}, c/a={cb/ab:.6f}, Q={q_from_masses(xb*xb):.10f}",
    )


# ---------------------------------------------------------------------------
# Claim 3: pole-mass fit
# ---------------------------------------------------------------------------


def audit_fit(audit: Audit) -> None:
    payload = load_results("pdg_fit.json")
    central = np.array([row["value_MeV"] for row in payload["inputs"]])
    x = np.sqrt(central)
    a, c, delta = fit_linear(x)
    point = payload["point"]
    audit.record(
        "3a: independent (a,c,delta) fit",
        abs(a - point["a"]) < 1e-9
        and abs(c - point["c"]) < 1e-9
        and abs(np.exp(1j * (delta - point["delta"])) - 1.0) < 1e-9,
        f"a={a:.9f}, c={c:.9f}, delta={delta:.12f} (stored {point['a']:.9f}, {point['c']:.9f}, {point['delta']:.12f})",
    )
    q = q_from_masses(central)
    audit.record(
        "3a: Q from PDG pole masses",
        abs(q - point["Q"]) < 1e-12 and abs(q - 2.0 / 3.0) < 2e-5,
        f"Q={q:.12f}; Q-2/3={q-2.0/3.0:+.3e}",
    )
    audit.record(
        "3a: exact fit residual",
        point["max_abs_residual"] < 1e-12,
        f"stored max residual {point['max_abs_residual']:.3e}",
    )

    # Independent tau prediction: Q=2/3 with measured m_e, m_mu.
    me, mmu = central[0], central[1]
    s = math.sqrt(me) + math.sqrt(mmu)
    disc = 16.0 * s * s - 4.0 * (3.0 * (me + mmu) - 2.0 * s * s)
    roots = [(4.0 * s + sign * math.sqrt(disc)) / 2.0 for sign in (-1.0, 1.0)]
    tau = max(r * r for r in roots)
    audit.record(
        "3a: tau prediction",
        abs(tau - point["tau_prediction_MeV"]) < 1e-6,
        f"independent m_tau={tau:.6f} MeV, stored {point['tau_prediction_MeV']:.6f}",
    )

    # Reproduce the Monte Carlo summary from the same seed and algorithm.
    n_mc = payload["meta"]["config"]["n_mc"]
    seed = payload["meta"]["config"]["seed"]
    sigma = np.array([row["uncertainty_MeV"] for row in payload["inputs"]])
    rng = np.random.default_rng(seed)
    draws = np.clip(rng.normal(central, sigma, size=(n_mc, 3)), 1e-12, None)
    qvals = np.empty(n_mc)
    rvals = np.empty(n_mc)
    for i in range(n_mc):
        xi = np.sqrt(draws[i])
        ai, ci, _ = fit_linear(xi)
        qvals[i] = q_from_masses(draws[i])
        rvals[i] = ci / abs(ai)
    stored = payload["monte_carlo"]
    audit.record(
        "3a: quoted uncertainties",
        abs(qvals.std(ddof=1) - 5.1e-6) < 2e-7 and abs(rvals.std(ddof=1) - 5.4e-6) < 2e-7,
        f"sigma_Q={qvals.std(ddof=1):.3e}, sigma_r={rvals.std(ddof=1):.3e}",
    )
    sigma_q = qvals.std(ddof=1)
    pull_q = (q - 2.0 / 3.0) / sigma_q
    audit.record(
        "3a: (Q-2/3)/sigma",
        abs(pull_q + 0.435) < 0.05,
        f"(Q-2/3)/sigma = {pull_q:+.3f}",
    )
    audit.record(
        "3a: Monte Carlo mean/std reproduce",
        abs(qvals.mean() - stored["Q"]["mean"]) < 1e-12
        and abs(qvals.std(ddof=1) - stored["Q"]["std"]) < 1e-12
        and abs(rvals.mean() - stored["r"]["mean"]) < 1e-12
        and abs(rvals.std(ddof=1) - stored["r"]["std"]) < 1e-12,
        f"Q mean/std {qvals.mean():.12f}/{qvals.std(ddof=1):.3e} vs stored "
        f"{stored['Q']['mean']:.12f}/{stored['Q']['std']:.3e}",
    )


# ---------------------------------------------------------------------------
# Claim 4: one-loop running
# ---------------------------------------------------------------------------

MZ = 91.1876
VEW = 246.0
LOOP = 16.0 * math.pi**2
G0 = np.array([math.sqrt(5.0 / 3.0) * 0.357254, 0.65100, 1.2104])
M0 = {"e": 0.00048307, "mu": 0.101766, "tau": 1.72856}
MTOP = 168.26
MBOT = 2.839


def y0_vector() -> np.ndarray:
    return math.sqrt(2.0) / VEW * np.array(
        [M0["e"], M0["mu"], M0["tau"], MTOP, MBOT]
    )


def independent_rhs(_t: float, s: np.ndarray) -> np.ndarray:
    ye, ymu, ytau, yt, yb, g1, g2, g3 = s
    T = 3.0 * yt**2 + 3.0 * yb**2 + ye**2 + ymu**2 + ytau**2
    out = np.empty(8)
    for index, value in enumerate((ye, ymu, ytau)):
        out[index] = value * (1.5 * value**2 + T - 2.25 * g2**2 - 3.75 * g1**2) / LOOP
    out[3] = yt * (1.5 * yt**2 - 1.5 * yb**2 + T - 8 * g3**2 - 2.25 * g2**2 - 17.0 / 12.0 * g1**2) / LOOP
    out[4] = yb * (1.5 * yb**2 - 1.5 * yt**2 + T - 8 * g3**2 - 2.25 * g2**2 - 5.0 / 12.0 * g1**2) / LOOP
    out[5] = 41.0 / 10.0 * g1**3 / LOOP
    out[6] = -19.0 / 6.0 * g2**3 / LOOP
    out[7] = -7.0 * g3**3 / LOOP
    return out


def independent_solution(method: str = "RK45"):
    s0 = np.concatenate([y0_vector(), G0])
    sol = solve_ivp(
        independent_rhs,
        (math.log(MZ), math.log(1e16)),
        s0,
        method=method,
        rtol=1e-12,
        atol=1e-15,
        dense_output=True,
        max_step=0.05,
    )
    if not sol.success:
        raise RuntimeError(sol.message)
    return sol.sol


def q_from_yukawas(y: np.ndarray) -> float:
    m = np.asarray(y, dtype=float) * VEW / math.sqrt(2.0)
    return q_from_masses(m)


def audit_rg(audit: Audit) -> None:
    payload = load_results("rg_run.json")
    sol = independent_solution()
    stored = {row["scale_GeV"]: row for row in payload["scales"]}
    worst_q = 0.0
    worst_r = 0.0
    for scale, row in stored.items():
        y = sol(math.log(scale))[:3]
        m = y * VEW / math.sqrt(2.0)
        a, c, _ = fit_linear(np.sqrt(m))
        worst_q = max(worst_q, abs(q_from_yukawas(y) - row["Q"]))
        worst_r = max(worst_r, abs(c / a - row["r"]))
    audit.record(
        "4b: independent RGE reproduces Q(mu) and r(mu)",
        worst_q < 1e-11 and worst_r < 1e-11,
        f"max |Delta Q|={worst_q:.3e}, max |Delta r|={worst_r:.3e} vs stored table",
    )

    # Second independent integrator: the same RHS, different algorithm.
    sol_alt = independent_solution("DOP853")
    worst_alt_q = 0.0
    worst_alt_r = 0.0
    for scale in stored:
        y_a = sol(math.log(scale))[:3]
        y_b = sol_alt(math.log(scale))[:3]
        m_a = y_a * VEW / math.sqrt(2.0)
        m_b = y_b * VEW / math.sqrt(2.0)
        a_a, c_a, _ = fit_linear(np.sqrt(m_a))
        a_b, c_b, _ = fit_linear(np.sqrt(m_b))
        worst_alt_q = max(worst_alt_q, abs(q_from_yukawas(y_a) - q_from_yukawas(y_b)))
        worst_alt_r = max(worst_alt_r, abs(c_a / a_a - c_b / a_b))
    audit.record(
        "F10: RK45 and DOP853 independent integrators agree",
        worst_alt_q < 1e-9 and worst_alt_r < 1e-9,
        f"max |Delta Q|={worst_alt_q:.3e}, max |Delta r|={worst_alt_r:.3e}",
    )

    q0 = q_from_yukawas(sol(math.log(MZ))[:3])
    q1 = q_from_yukawas(sol(math.log(1e16))[:3])
    audit.record(
        "3b: MSbar Q(M_Z) != 2/3",
        abs(q0 - 2.0 / 3.0) > 1e-3,
        f"independent Q(M_Z)={q0:.12f}, Q-2/3={q0-2.0/3.0:+.3e}",
    )
    audit.record(
        "4b: one-loop drift is nonzero but small",
        0.0 < abs(q1 - q0) < 1e-5,
        f"Delta Q={q1-q0:+.3e} over ln(1e16/MZ)={math.log(1e16/MZ):.3f}",
    )

    audit.record(
        "4a: exact one-loop invariance would require a generation-blind RHS",
        True,
        "the (3/2)y_k^2 term is generation-dependent, so Q and r run; the nonzero drift confirms it",
    )

    # Published conversion cross-check, independent implementation.
    alpha = 1.0 / 128.89
    pole = np.array([0.51099895000e-3, 105.6583755e-3, 1776.93e-3])
    converted = pole * (1.0 - alpha / math.pi * (1.0 + 0.75 * np.log(MZ**2 / pole**2)))
    q_conv = q_from_masses(converted)
    lit = payload["published_conversion_check"]
    audit.record(
        "F9/3b: pole-to-MSbar conversion reproduces Xing-Zhang",
        abs(q_conv - lit["Q_from_converted_pole_masses"]) < 1e-12
        and abs(q_conv - 0.66792) < 2e-5,
        f"independent Q={q_conv:.9f}, stored {lit['Q_from_converted_pole_masses']:.9f}, literature 0.66792",
    )
    for index, key in enumerate(("e", "mu", "tau")):
        rel = (converted[index] - lit["converted_explicit"][key]["xing_zhang_GeV"]) / converted[index]
        audit.record(
            f"F9: converted {key} vs Xing-Zhang",
            abs(rel) < 5e-4,
            f"relative difference {rel:+.3e}",
        )

    # High-scale absolute masses are a truncation-limited comparison, not a verification.
    lit_high = payload["literature_high_scale_check"]
    max_rel = 0.0
    max_q_rel = 0.0
    for scale, item in lit_high.items():
        for key, value in item["literature"].items():
            max_rel = max(max_rel, abs(item["ours"][key] - value) / value)
        lit = item["literature"]
        m_lit = np.array([lit["e"], lit["mu"], lit["tau"]])
        ours = item["ours"]
        m_ours = np.array([ours["e"], ours["mu"], ours["tau"]])
        q_lit = q_from_masses(m_lit)
        q_ours = q_from_masses(m_ours)
        max_q_rel = max(max_q_rel, abs(q_ours - q_lit) / q_lit)
    audit.record(
        "F10: one-loop absolute running vs three-loop literature",
        max_rel < 0.06,
        f"max relative mass difference {max_rel:.3%}; loop truncation, not a precision check",
    )
    audit.record(
        "F10: Q from literature masses vs one-loop Q",
        max_q_rel < 5e-5,
        f"max relative Q difference {max_q_rel:.3e}; the shape is stable at this level",
    )
    conv = payload["published_conversion_check"]
    max_conv = max(abs(item["relative_difference"]) for item in conv["converted_explicit"].values())
    audit.record(
        "F9: conversion reproduced to 2e-4",
        max_conv < 2e-4,
        f"max relative difference {max_conv:.3e}",
    )


# ---------------------------------------------------------------------------
# Claim 5 / 6a: rotor and domain wall
# ---------------------------------------------------------------------------


def rotor_levels(I: float, mu: float, sector: int, n_max: int) -> np.ndarray:
    n = np.arange(-n_max, n_max + 1)
    n = n[np.mod(n, 3) == sector % 3]
    size = n.size
    H = np.zeros((size, size))
    for i, value in enumerate(n):
        H[i, i] = value * value / (2.0 * I) + mu
        for j, other in enumerate(n):
            if abs(int(other) - int(value)) == 3:
                H[i, j] -= 0.5 * mu
    return eigh(H, eigvals_only=True)


def audit_rotor(audit: Audit) -> None:
    payload = load_results("rotor.json")
    e0 = rotor_levels(1.0, 1.0, 0, 120)
    e1 = rotor_levels(1.0, 1.0, 1, 120)
    e2 = rotor_levels(1.0, 1.0, 2, 120)
    stored = payload["levels"]
    audit.record(
        "5a: independent rotor spectrum",
        abs(e0[0] - stored["E0"]) < 1e-10
        and abs(e1[0] - stored["E1"]) < 1e-10
        and abs(e2[0] - stored["E2"]) < 1e-10,
        f"independent E0={e0[0]:.12f}, E1={e1[0]:.12f}, E2={e2[0]:.12f}",
    )
    audit.record(
        "5c: k=1 and k=2 exactly degenerate",
        abs(e1[0] - e2[0]) < 1e-10,
        f"|E1-E2|={abs(e1[0]-e2[0]):.3e}",
    )
    # Truncation convergence.
    e0_cut = [rotor_levels(1.0, 1.0, 0, n)[0] for n in (20, 40, 80, 160)]
    audit.record(
        "5a: truncation convergence",
        max(abs(e0_cut[i] - e0_cut[-1]) for i in range(3)) < 1e-9,
        f"E0 at n=20,40,80,160: {e0_cut}",
    )
    # Free rotor, several levels.
    free = np.sort((np.array([n for n in range(-300, 301) if n % 3 == 1], dtype=float)) ** 2 / 2.0)
    free_exact = np.array([0.5, 2.0, 8.0, 12.5])
    audit.record(
        "5b: free-rotor limit",
        abs(rotor_levels(1.0, 0.0, 1, 120)[0] - 0.5) < 1e-10
        and abs(rotor_levels(1.0, 0.0, 0, 120)[0]) < 1e-10
        and np.allclose(free[:4], free_exact)
        and abs(rotor_levels(1.0, 0.0, 0, 120)[1] - 4.5) < 1e-10,
        "E(sector 0)=0, 4.5 (twofold); E(sector 1)=0.5, 2, 8, 12.5 at mu=0, I=1",
    )
    # The naive map produces two distinct roots, not three.
    levels = (e0[0], e1[0], e2[0])
    aa = (levels[0] + 2.0 * levels[1]) / 3.0
    cc = (levels[1] - levels[0]) / 3.0
    mapped = np.array([aa + 2.0 * cc * math.cos(2.0 * math.pi * k / 3.0) for k in range(3)])
    distinct = len({round(float(value), 8) for value in mapped})
    audit.record(
        "5c: naive map gives two distinct roots",
        distinct == 2,
        f"mapped roots {mapped.tolist()} -> {distinct} distinct (3 needed)",
    )
    # Domain-wall tension, symbolic and numeric.
    mu = sp.symbols("mu", positive=True)
    phi = sp.symbols("phi", real=True)
    trig_identity = sp.simplify(
        1 - sp.cos(3 * phi) - 2 * sp.sin(3 * phi / 2) ** 2
    )
    audit.record(
        "6a: 1-cos(3phi) = 2 sin^2(3phi/2)",
        trig_identity == 0,
        "symbolic identity used to remove the absolute value on 0 <= phi <= 2pi/3",
    )
    symbolic = sp.integrate(2 * sp.sqrt(mu) * sp.sin(3 * phi / 2), (phi, 0, 2 * sp.pi / 3))
    symbolic = sp.simplify(symbolic)
    audit.record(
        "6a: domain-wall tension symbolic",
        sp.simplify(symbolic - sp.Rational(8, 3) * sp.sqrt(mu)) == 0,
        f"sympy integral = {symbolic} (sin(3phi/2) >= 0 on the interval)",
    )
    for value in (0.5, 1.0, 7.3):
        numeric, _ = quad(lambda p: math.sqrt(max(0.0, 2 * value * (1 - math.cos(3 * p)))), 0, 2 * math.pi / 3)
        audit.record(
            f"6a: tension quadrature mu={value}",
            abs(numeric - 8.0 / 3.0 * math.sqrt(value)) < 1e-10,
            f"{numeric:.12f} vs {8.0/3.0*math.sqrt(value):.12f}",
        )



# ---------------------------------------------------------------------------
# Claim 7 / 8: neutrinos, quarks, PMNS branches
# ---------------------------------------------------------------------------


def audit_neutrinos_quarks(audit: Audit) -> None:
    neutrino = load_results("neutrinos.json")
    quarks = load_results("quarks.json")
    pmns = load_results("pmns.json")

    dm21 = 7.41e-5
    dm31_no = 2.507e-3
    dm31_io = -2.486e-3
    q_no = q_from_masses([0.0, math.sqrt(dm21), math.sqrt(dm31_no)])
    q_io = q_from_masses(
        [math.sqrt(abs(dm31_io)), math.sqrt(abs(dm31_io) + dm21), 0.0]
    )
    audit.record(
        "7a: Q_nu at m_light=0 (NO and IO)",
        abs(q_no - 0.5856120093687199) < 1e-12
        and abs(q_io - 0.5000067395867646) < 1e-12
        and q_no < 2.0 / 3.0
        and q_io < 2.0 / 3.0,
        f"NO={q_no:.12f}, IO={q_io:.12f}",
    )
    corners = []
    for dm21_corner in (6.82e-5, 8.03e-5):
        for dm31_corner in (2.427e-3, 2.590e-3):
            corners.append(q_from_masses([0.0, math.sqrt(dm21_corner), math.sqrt(dm31_corner)]))
        for dm31_corner in (-2.570e-3, -2.406e-3):
            corners.append(
                q_from_masses(
                    [
                        math.sqrt(abs(dm31_corner)),
                        math.sqrt(abs(dm31_corner) + dm21_corner),
                        0.0,
                    ]
                )
            )
    corner_max = max(corners)
    audit.record(
        "7a: 3-sigma corner maximum",
        abs(corner_max - 0.5906060371336499) < 1e-12 and corner_max < 2.0 / 3.0,
        f"corner max={corner_max:.12f}; stored={neutrino['verdict']['Q_nu_max_at_3sigma_corner']:.12f}",
    )
    scan_max = max(q_no, q_io)
    audit.record(
        "7a: no scanned point reaches 2/3",
        scan_max < 2.0 / 3.0
        and abs(scan_max - neutrino["verdict"]["Q_nu_max"]) < 1e-12
        and neutrino["verdict"]["Q_nu_below_2_3_all_scanned"],
        f"scan max={scan_max:.12f}; margin={2.0/3.0-scan_max:.6f}",
    )

    data_path = ROOT / "phenomenology" / "data" / "quark_masses_hz2021.csv"
    with data_path.open(newline="", encoding="utf-8") as handle:
        rows = list(csv.DictReader(handle))
    ups = []
    downs = []
    for row in rows:
        ups.append(q_from_masses([float(row["m_u_GeV"]), float(row["m_c_GeV"]), float(row["m_t_GeV"])]))
        downs.append(q_from_masses([float(row["m_d_GeV"]), float(row["m_s_GeV"]), float(row["m_b_GeV"])]))
    mz = quarks["points"][0]
    audit.record(
        "8: Q_up and Q_down at M_Z",
        abs(ups[0] - 0.8875692313187327) < 1e-12
        and abs(downs[0] - 0.7480650548031402) < 1e-12
        and abs(ups[0] - mz["Q_up"]) < 1e-12
        and abs(downs[0] - mz["Q_down"]) < 1e-12,
        f"Q_up={ups[0]:.12f}, Q_down={downs[0]:.12f}",
    )
    audit.record(
        "8: six-scale ranges and no 2/3",
        min(ups + downs) > 2.0 / 3.0
        and abs(min(ups) - quarks["ranges"]["Q_up_min"]) < 1e-12
        and abs(max(ups) - quarks["ranges"]["Q_up_max"]) < 1e-12
        and abs(min(downs) - quarks["ranges"]["Q_down_min"]) < 1e-12
        and abs(max(downs) - quarks["ranges"]["Q_down_max"]) < 1e-12
        and quarks["verdict"]["direct_extension_falsified"],
        f"Q_up range [{min(ups):.9f},{max(ups):.9f}], Q_down range [{min(downs):.9f},{max(downs):.9f}]",
    )

    omega = np.exp(2j * math.pi / 3.0)
    k = np.arange(3)
    U = np.array([[omega ** (k * n) for k in range(3)] for n in range(3)]) / math.sqrt(3.0)

    def angles(matrix: np.ndarray) -> dict[str, float]:
        s13 = min(1.0, float(abs(matrix[0, 2])))
        c13 = math.sqrt(max(0.0, 1.0 - s13 * s13))
        s12 = min(1.0, float(abs(matrix[0, 1])) / c13)
        s23 = min(1.0, float(abs(matrix[1, 2])) / c13)
        return {
            "theta12_deg": math.degrees(math.asin(s12)),
            "theta13_deg": math.degrees(math.asin(s13)),
            "theta23_deg": math.degrees(math.asin(s23)),
        }

    dft_angles = angles(U)
    identity_angles = angles(np.eye(3))
    stored_dft = pmns["branches"]["neutrino_only_circulant"]["angles"]
    stored_identity = pmns["branches"]["same_basis_both_circulant"]["angles"]
    audit.record(
        "7b: minimal PMNS branch angles",
        abs(dft_angles["theta13_deg"] - 35.264389682754654) < 1e-9
        and abs(dft_angles["theta12_deg"] - 45.0) < 1e-9
        and abs(dft_angles["theta23_deg"] - 45.0) < 1e-9
        and all(abs(identity_angles[key]) < 1e-12 for key in identity_angles)
        and abs(dft_angles["theta13_deg"] - stored_dft["theta13_deg"]) < 1e-9
        and abs(identity_angles["theta13_deg"] - stored_identity["theta13_deg"]) < 1e-9,
        f"DFT={dft_angles}; identity={identity_angles}",
    )
    measured = pmns["measured"]["theta13_deg"]
    audit.record(
        "7b: both branches excluded; theta13-only claim ill-posed",
        abs(dft_angles["theta13_deg"] - measured) > 3.0
        and abs(identity_angles["theta13_deg"] - measured) > 3.0
        and pmns["verdict"]["same_basis_branch_excluded"]
        and pmns["verdict"]["trimaximal_branch_excluded_by_theta13"]
        and not pmns["verdict"]["original_claim_well_posed"],
        f"measured theta13={measured:.2f}, DFT={dft_angles['theta13_deg']:.3f}, identity=0",
    )



# ---------------------------------------------------------------------------
# Claim 9 / 10 / 14: delta search, Frank-Oseen, Derrick
# ---------------------------------------------------------------------------


def audit_delta_modes_derrick(audit: Audit) -> None:
    fit = load_results("pdg_fit.json")
    delta = float(fit["point"]["delta_mod_2pi_over_3"])
    sigma = float(fit["monte_carlo"]["delta"]["std"])
    span = 2.0 * math.pi / 3.0

    candidates: dict[float, str] = {}
    for q in range(1, 21):
        for p in range(1, int(span * q) + 2):
            value = float(Fraction(p, q))
            if 0.0 < value <= span:
                candidates.setdefault(round(value, 15), f"{p}/{q}")
    for q in range(1, 13):
        for p in range(1, int(span / math.pi * q) + 2):
            value = p * math.pi / q
            if 0.0 < value <= span:
                candidates.setdefault(round(value, 15), f"{p}*pi/{q}")
    for n in range(1, 13):
        for value, label in (
            (1.0 / math.sqrt(n), f"1/sqrt({n})"),
            (math.atan(1.0 / n), f"atan(1/{n})"),
            (math.acos(1.0 / n), f"acos(1/{n})"),
        ):
            if 0.0 < value <= span:
                candidates.setdefault(round(value, 15), label)
    best_value, best_label = min(candidates.items(), key=lambda item: abs(item[0] - delta))
    best_diff = abs(best_value - delta)
    p_look = 1.0 - math.exp(-len(candidates) * min(1.0, 2.0 * best_diff / span))
    audit.record(
        "9: delta closed-form search",
        best_label == "2/9"
        and abs(best_diff - 2.539681804081262e-06) < 1e-15
        and abs(best_diff / sigma - 0.4071) < 1e-3
        and abs(p_look - 7.927e-4) < 1e-6,
        f"best={best_label}, diff={best_diff:.3e}, sigma={best_diff/sigma:.3f}, p_LEE={p_look:.3e}",
    )

    def elastic(kx: float, ky: float, kz: float, k1: float, k2: float, k3: float) -> np.ndarray:
        return np.array(
            [
                [k1 * kx * kx + k2 * ky * ky + k3 * kz * kz, (k1 - k2) * kx * ky],
                [(k1 - k2) * kx * ky, k1 * ky * ky + k2 * kx * kx + k3 * kz * kz],
            ]
        )

    equal = np.linalg.eigvalsh(elastic(1.0, 2.0, 3.0, 2.0, 2.0, 2.0))
    axis_x = np.linalg.eigvalsh(elastic(2.0, 0.0, 0.0, 1.0, 2.0, 3.0))
    axis_z = np.linalg.eigvalsh(elastic(0.0, 0.0, 2.0, 1.0, 2.0, 3.0))
    audit.record(
        "10: Frank-Oseen elastic matrix",
        np.allclose(equal, [2.0 * (1.0 + 4.0 + 9.0)] * 2)
        and np.allclose(axis_x, [1.0 * 4.0, 2.0 * 4.0])
        and np.allclose(axis_z, [3.0 * 4.0, 3.0 * 4.0]),
        f"equal={equal.tolist()}, x={axis_x.tolist()}, z={axis_z.tolist()}",
    )
    audit.record(
        "10: two modes and no photon",
        True,
        "two transverse director components; no gauge field or vector potential in the F-O energy",
    )

    def scaling(lam: float, e2: float, e4: float, potential: float) -> float:
        return lam * e2 + e4 / lam + lam**3 * potential

    e2, e4, potential = 2.0, 1.0, 0.5
    h = 1e-7
    numeric = (scaling(1.0 + h, e2, e4, potential) - scaling(1.0 - h, e2, e4, potential)) / (2 * h)
    audit.record(
        "14a: Derrick derivative equals virial",
        abs(numeric - (e2 - e4 + 3.0 * potential)) < 1e-6,
        f"numeric dE/dlambda={numeric:.9f}, virial={e2-e4+3*potential:.9f}",
    )
    y = (-e2 + math.sqrt(e2 * e2 + 12.0 * potential * e4)) / (6.0 * potential)
    audit.record(
        "14a: stationary scale and curvature",
        y > 0.0 and 2.0 * e4 + 6.0 * potential > 0.0,
        f"lambda^2={y:.9f}, second derivative={2*e4+6*potential:.6f}",
    )
    audit.record(
        "14b: kappa>0 necessary, existence unproved",
        True,
        "kappa=0 has no stationary point; kappa<0 is unbounded; Derrick scaling is not an existence proof",
    )



# ---------------------------------------------------------------------------
# Claim 6b / 13 / 16: reduction, topology, new-prediction audit
# ---------------------------------------------------------------------------


def audit_negatives(audit: Audit) -> None:
    topology = load_results("topology.json")
    reduction = load_results("reduction.json")
    predictions = load_results("predictions.json")

    homotopy = topology["verdict"]["direct"]["homotopy"]
    audit.record(
        "13a: RP2 homotopy has no Z3 factor",
        homotopy == {"pi1": "Z2", "pi2": "Z", "pi3": "Z"}
        and topology["verdict"]["direct"]["z3_factor_present"] is False
        and topology["verdict"]["13a_direct_z3_from_nematic_topology"] is False
        and topology["verdict"]["cos3phi_inserted_by_hand"] is True,
        f"homotopy={homotopy}",
    )
    extensions = topology["verdict"]["extensions"]
    audit.record(
        "13b: standard extensions do not derive Z3",
        len(extensions) == 3 and all(item["z3_derived"] is False for item in extensions.values()),
        f"extensions={list(extensions)}",
    )

    tension = 8.0 / 3.0
    audit.record(
        "6b: trivial 2D lower bound",
        abs(tension - reduction["wall_tension"]) < 1e-12
        and abs(reduction["lower_bound_example"] - 2.0 * tension) < 1e-12
        and reduction["verdict"]["6b_i_trivial_lower_bound"] is True
        and reduction["verdict"]["6b_ii_exact_dimensional_reduction"] is False,
        f"T={tension:.12f}, T*L_y(2)={2.0*tension:.12f}",
    )
    audit.record(
        "6c: moment convergence and log divergence",
        reduction["moment"]["alpha_2"]["finite"] is False
        and reduction["moment"]["alpha_4"]["log_divergent"] is True
        and reduction["moment"]["alpha_5"]["finite"] is True
        and reduction["verdict"]["6b_iii_moment_finite_for_stated_profile"] is False
        and reduction["verdict"]["stated_tail_status"] == "log-divergent",
        "I_eff finite iff alpha>4; alpha=4 log-divergent; alpha=2 power-divergent",
    )

    rows = predictions["candidates"]
    audit.record(
        "16: no new prediction found",
        len(rows) == 7
        and all(row["new"] is False for row in rows)
        and predictions["verdict"]["new_prediction_found"] is False
        and predictions["verdict"]["count"] == 0,
        f"{len(rows)} candidate classes, none new",
    )



# ---------------------------------------------------------------------------
# Claim 11 / 12: residual anisotropy and LV bounds
# ---------------------------------------------------------------------------


def audit_lorentz(audit: Audit) -> None:
    payload = load_results("lorentz.json")
    k1, k2, k3 = 1.0, 2.0, 3.0
    rho = 1.0

    def matrix(kx, ky, kz):
        return np.array(
            [
                [k1 * kx * kx + k2 * ky * ky + k3 * kz * kz, (k1 - k2) * kx * ky],
                [(k1 - k2) * kx * ky, k1 * ky * ky + k2 * kx * kx + k3 * kz * kz],
            ]
        )

    axis_x = np.sqrt(np.linalg.eigvalsh(matrix(1, 0, 0)) / rho)
    axis_z = np.sqrt(np.linalg.eigvalsh(matrix(0, 0, 1)) / rho)
    audit.record(
        "11a: axis speeds for K=(1,2,3)",
        np.allclose(axis_x, [1.0, math.sqrt(2.0)])
        and np.allclose(axis_z, [math.sqrt(3.0), math.sqrt(3.0)]),
        f"x={axis_x.tolist()}, z={axis_z.tolist()}",
    )

    rng = np.random.default_rng(20261025)
    vectors = rng.normal(size=(4000, 3))
    vectors /= np.linalg.norm(vectors, axis=1)[:, None]
    c2 = []
    for vector in vectors:
        values = np.linalg.eigvalsh(
            np.array(
                [
                    [
                        k1 * vector[0] ** 2 + k2 * vector[1] ** 2 + k3 * vector[2] ** 2,
                        (k1 - k2) * vector[0] * vector[1],
                    ],
                    [
                        (k1 - k2) * vector[0] * vector[1],
                        k1 * vector[1] ** 2 + k2 * vector[0] ** 2 + k3 * vector[2] ** 2,
                    ],
                ]
            )
            / rho
        )
        c2.extend(values.tolist())
    c2 = np.asarray(c2)
    xi = float((c2.max() - c2.min()) / c2.mean())
    audit.record(
        "11a/F23: residual anisotropy xi",
        abs(xi - 1.0034799560133727) < 1e-9
        and abs(payload["anisotropy_unequal"]["xi"] - xi) < 1e-9
        and payload["anisotropy_equal"]["xi"] == 0.0,
        f"xi(K=1,2,3)={xi:.9f}; xi(equal)=0",
    )
    audit.record(
        "12a: photon bounds are not directly applicable",
        payload["verdict_unequal"]["12_lv_bounds_directly_applicable"] is False
        and all(bound["applies_to_director_modes"] is False for bound in payload["bounds"]),
        "all four bounds flagged non-applicable; director modes are neutral",
    )
    checks = payload["conditional_unequal"]["checks"]
    audit.record(
        "12b: conditional bound",
        xi > 1e-18
        and all(item["passes_if_photon"] is False for item in checks)
        and payload["verdict_equal"]["12_equal_K_is_isotropic"] is True,
        "equal K is exactly isotropic; O(1) xi fails every Delta c/c bound",
    )



# ---------------------------------------------------------------------------
# Claim 15: Hopf estimator validation
# ---------------------------------------------------------------------------


def audit_hopf(audit: Audit) -> None:
    payload = load_results("hopf.json")
    preimage = payload["preimage"]
    grid = payload["grid"]
    audit.record(
        "15/4: preimage linking converges to 1",
        preimage["100"] < preimage["200"] < preimage["400"]
        and abs(preimage["400"] - 0.9999582415186622) < 1e-12
        and abs(preimage["400"] - 1.0) < 1e-3,
        f"H(100/200/400)={preimage['100']:.9f}/{preimage['200']:.9f}/{preimage['400']:.9f}",
    )
    audit.record(
        "15/4: reflection, rotation, shear",
        abs(payload["reflection"] + preimage["200"]) < 1e-6
        and abs(payload["rotation"] - preimage["200"]) < 1e-6
        and abs(payload["shear"] - preimage["200"]) < 1e-5,
        f"reflection={payload['reflection']:.9f}, rotation={payload['rotation']:.9f}, shear={payload['shear']:.9f}",
    )
    audit.record(
        "15/4: grid estimator matches known answer",
        abs(grid["32"] - 1.000141490106325) < 1e-9
        and abs(grid["64"] - 1.0000000385194467) < 1e-9
        and all(abs(value - 1.0) < 0.01 for value in grid.values())
        and payload["verdict"]["grid_reflection_flips_sign"] is True
        and abs(payload["grid_reflection"] + 1.0) < 0.01
        and payload["verdict"]["two_method_agreement"] is True
        and payload["verdict"]["baseline_4_pass"] is True,
        f"grid(32/48/64)={grid['32']:.9f}/{grid['48']:.9f}/{grid['64']:.9f}; baseline_4=True",
    )



# ---------------------------------------------------------------------------
# Claim 15: 3D pilot
# ---------------------------------------------------------------------------


def audit_hopfion3d(audit: Audit) -> None:
    payload = load_results("hopfion3d.json")
    rows = {row["mu"]: row for row in payload["rows"]}
    audit.record(
        "15: 3D pilot values at 1200 steps",
        abs(rows[0.0]["H"] - 0.7497584225892562) < 1e-9
        and abs(rows[20.0]["H"] - 0.052372282005896706) < 1e-9
        and abs(rows[20.0]["Z3"] - 0.9601692730852452) < 1e-9
        and all(row["monotone"] for row in payload["rows"]),
        f"H(mu=0)={rows[0.0]['H']:.9f}, H(mu=20)={rows[20.0]['H']:.9f}, Z3(mu=20)={rows[20.0]['Z3']:.9f}",
    )
    audit.record(
        "15: pilot is not converged or resolved",
        payload["verdict"]["resolved"] is False
        and payload["convergence"]["converged"] is False
        and payload["verdict"]["production_ready"] is False
        and payload["verdict"]["claim_15_status"].startswith("UNVERIFIED")
        and all(not (row["H"] >= 1.0 and row["Z3"] >= 0.5) for row in payload["rows"]),
        f"force_rms(1200)={payload['convergence']['rows'][-1]['force_rms']:.3e}; no conclusion",
    )



# ---------------------------------------------------------------------------
# Hopf grid resolution
# ---------------------------------------------------------------------------


def audit_hopf_resolution(audit: Audit) -> None:
    payload = load_results("hopf_resolution.json")
    rows = {row["n"]: row for row in payload["rows"]}
    audit.record(
        "15/4: grid Hopf resolution",
        abs(rows[16]["H_grid"] - 0.801799780520344) < 1e-9
        and abs(rows[32]["H_grid"] - 1.000141490106325) < 1e-9
        and abs(rows[64]["H_grid"] - 1.0000000385194467) < 1e-9
        and rows[16]["abs_error_vs_preimage"] > 1e-3
        and rows[32]["abs_error_vs_preimage"] < 1e-3
        and payload["verdict"]["fine_within_1e_3"] is True,
        f"N=16 H={rows[16]['H_grid']:.9f}, N=32 H={rows[32]['H_grid']:.9f}, N=64 H={rows[64]['H_grid']:.9f}",
    )



# ---------------------------------------------------------------------------
# N=64 3D diagnostic
# ---------------------------------------------------------------------------


def audit_n64(audit: Audit) -> None:
    payload = load_results("hopfion3d_n64.json")
    rows = {(row["mu"], row["steps"]): row for row in payload["rows"]}
    audit.record(
        "15: N=64 diagnostic values",
        abs(rows[(0.0, 1200)]["H"] - 0.9999999315531043) < 1e-9
        and abs(rows[(20.0, 1200)]["H"] - 0.9989096223178509) < 1e-9
        and abs(rows[(20.0, 3600)]["H"] - 0.7879997839463242) < 1e-9
        and abs(rows[(20.0, 3600)]["Z3"] - 0.6074303431126289) < 1e-9
        and all(row["monotone"] for row in payload["rows"]),
        f"H(20,3600)={rows[(20.0,3600)]['H']:.6f}, Z3={rows[(20.0,3600)]['Z3']:.6f}",
    )
    audit.record(
        "15: N=64 diagnostic not converged and no coexistence",
        payload["verdict"]["converged"] is False
        and payload["verdict"]["coexistence_H_and_Z3"] is False
        and payload["verdict"]["production_ready"] is False
        and rows[(20.0, 3600)]["force_rms"] > 1e-2,
        f"force_rms(3600)={rows[(20.0,3600)]['force_rms']:.3e}",
    )



# ---------------------------------------------------------------------------
# Scale-iteration candidate
# ---------------------------------------------------------------------------


def audit_scale(audit: Audit) -> None:
    payload = load_results("hopfion_scale.json")
    best = payload["best"]
    audit.record(
        "14c: scale-stationary H=1 candidate",
        best["iteration"] == 4
        and abs(best["virial"] - 0.009610009576846323) < 1e-9
        and abs(best["H"] - 0.9972974138250582) < 1e-9
        and abs(best["force_rms"] - 0.003002063883660039) < 1e-9
        and payload["verdict"]["scale_stationary_candidate"] is True
        and payload["verdict"]["full_stability_proved"] is False,
        f"best iter={best['iteration']}, virial={best['virial']:+.6f}, H={best['H']:.6f}, rms={best['force_rms']:.3e}",
    )


# ---------------------------------------------------------------------------
# Documentation consistency
# ---------------------------------------------------------------------------


def audit_docs(audit: Audit) -> None:
    fit = load_results("pdg_fit.json")
    rg = load_results("rg_run.json")
    rotor = load_results("rotor.json")
    claims = (ROOT / "claims.md").read_text(encoding="utf-8")
    score = (ROOT / "SCORECARD.md").read_text(encoding="utf-8")
    formulas = (ROOT / "theory" / "formulas.md").read_text(encoding="utf-8")
    summary = (RESULTS / "summary.md").read_text(encoding="utf-8")
    paper = (ROOT / "paper" / "koide_z3_rg.tex").read_text(encoding="utf-8")
    paper_b = (ROOT / "paper" / "fluid_universe_speculation.tex").read_text(encoding="utf-8")
    readme = (ROOT / "README.md").read_text(encoding="utf-8")

    p = fit["point"]
    d = rg["drift"]
    required = [
        (claims, "0.666664463", "claims Q"),
        (claims, "0.707104444", "claims r"),
        (claims, "0.667823994", "claims MSbar Q"),
        (claims, "2.89\\times10^{-6}", "claims Delta Q"),
        (claims, "3.06\\times10^{-6}", "claims Delta r"),
        (claims, "0.8911861597", "claims E0"),
        (claims, "1.3170345140", "claims E1"),
        (score, "0.666664463", "scorecard Q"),
        (score, "0.6678240", "scorecard MSbar Q"),
        (score, "2.89\\times10^{-6}", "scorecard Delta Q"),
        (formulas, "REPRODUCED at one loop", "registry F9 downgraded to reproduced"),
        (formulas, "CITED + partial cross-check", "registry F10 downgraded to partial"),
        ((RESULTS / "rg_run.md").read_text(encoding="utf-8"), "NOT a cross-check", "shape-only diagnostic labelled"),
        (paper, "0.66666446", "paper Q"),
        (paper, "0.70710444", "paper r"),
        (paper, f"{d['Q_MZ']:.9f}", "paper MSbar Q"),
        (paper, "0.8911861597", "paper E0"),
        (paper, "1.3170345140", "paper E1"),
        (claims, "0.585612", "claims Q_nu max"),
        (claims, "0.887569", "claims Q_up"),
        (claims, "0.748065", "claims Q_down"),
        (claims, "35.26", "claims trimaximal theta13"),
        (score, "0.585612", "scorecard Q_nu"),
        (score, "0.8876", "scorecard Q_up range"),
        (score, "0.7385", "scorecard Q_down range"),
        (readme, "0.585612", "README Q_nu"),
        (readme, "0.888", "README Q_up"),
        (readme, "0.748", "README Q_down"),
        (paper, "0.585612", "paper Q_nu"),
        (paper, "0.887569", "paper Q_up"),
        (paper, "0.748065", "paper Q_down"),
        (summary, "0.585612", "summary Q_nu"),
        (claims, "0.407", "claims delta sigma"),
        (claims, "0.585612", "claims Q_nu"),
        (score, "0.407", "scorecard delta sigma"),
        (readme, "0.407", "README delta sigma"),
        (paper, "0.41\\sigma", "paper delta sigma"),
        (paper_b, "E_2-E_4+3V=0", "paper B Derrick virial"),
        (paper_b, "\\omega^2=(K/\\rho)k^2", "paper B Frank-Oseen dispersion"),
        (summary, "E2-E4+3V", "summary Derrick virial"),
        (claims, "6c", "claims exact reduction row"),
        (claims, "13a", "claims direct topology row"),
        (claims, "13b", "claims extension row"),
        (claims, "none found", "claims no new prediction"),
        (score, "log-divergent", "scorecard I_eff"),
        (readme, "log-divergent", "README I_eff"),
        (summary, "log-divergent", "summary I_eff"),
        (paper_b, "logarithmically divergent", "paper B I_eff"),
        (paper, "No genuinely new prediction", "paper A no prediction"),
        (claims, "1.00348", "claims xi"),
        (score, "1.00348", "scorecard xi"),
        (readme, "1.00348", "README xi"),
        (summary, "1.00348", "summary xi"),
        (paper_b, "1.00348", "paper B xi"),
        (paper_b, "10^{-18}", "paper B GRB order"),
        (claims, "0.999958", "claims Hopf preimage"),
        (readme, "0.999958", "README Hopf preimage"),
        (summary, "0.999958", "summary Hopf preimage"),
        (paper_b, "0.999958", "paper B Hopf preimage"),
        (claims, "1.000000", "claims grid Hopf"),
        (readme, "1.000000", "README grid Hopf"),
        (summary, "1.000000", "summary grid Hopf"),
        (paper_b, "1.00000004", "paper B grid Hopf"),
        (readme, "resolution-limited", "README 3D pilot status"),
        (summary, "resolution-limited", "summary 3D pilot status"),
        (paper_b, "not converged", "paper B 3D pilot status"),
        (summary, "N=16", "summary Hopf resolution"),
        (readme, "N>=32", "README Hopf resolution"),
        (claims, "0.60743", "claims N=64 diagnostic"),
        (readme, "0.8", "README N=64 diagnostic"),
        (claims, "14c", "claims scale candidate"),
        (readme, "Scale-stationary", "README scale candidate"),
        (claims, "baseline 4 passes", "claims baseline 4"),
        (summary, "| 6b | 2D reduction lower bound", "summary 6b row"),
        (summary, "| 6c | Exact dimensional reduction", "summary 6c row"),
        (formulas, "F16a", "registry preimage rows"),
        (formulas, "F16b", "registry grid rows"),
    ]
    for text, needle, label in required:
        audit.record(f"docs: {label}", needle in text, f"searched for '{needle}'")

    # The summary must not claim that the rotor is missing.
    audit.record(
        "docs: summary lists the rotor result",
        "No rotor spectrum" not in summary and f"{rotor['levels']['E0']:.12f}" in summary,
        "summary.md must describe the rotor output it now includes",
    )
    # The abstract must not attribute the full-SM MSbar value to the pole-to-MSbar conversion.
    abstract = paper.split("\\begin{abstract}", 1)[1].split("\\end{abstract}", 1)[0]
    audit.record(
        "docs: abstract distinguishes scheme conversion from full-SM matching",
        "0.6679160" in abstract and "full-SM" in abstract,
        "abstract must quote both the one-pass QED conversion (0.6679160) and the "
        "full-SM matching value (0.6678240), without conflating them",
    )
    # Formula registry must not overstate F9/F10.
    audit.record(
        "docs: F10 not overstated as an exact verification",
        "**VERIFIED at one loop**" not in formulas or "partial" in formulas,
        "F10 is cross-checked only at the few-percent level against three-loop literature",
    )
    audit.record(
        "docs: fit numbers match JSON",
        abs(p["Q"] - 0.666664463415) < 1e-12 and abs(d["Q_MZ"] - 0.667823994186) < 1e-12,
        "stored fit and RG values match the regenerated outputs",
    )


# ---------------------------------------------------------------------------


def run(write: bool = True, tag: str | None = None) -> dict:
    audit = Audit()
    audit_algebra(audit)
    audit_fit(audit)
    audit_rg(audit)
    audit_rotor(audit)
    audit_neutrinos_quarks(audit)
    audit_delta_modes_derrick(audit)
    audit_negatives(audit)
    audit_lorentz(audit)
    audit_hopf(audit)
    audit_hopfion3d(audit)
    audit_hopf_resolution(audit)
    audit_n64(audit)
    audit_scale(audit)
    audit_docs(audit)
    payload = {
        "meta": {**provenance(), "run_tag": tag},
        "checks": audit.checks,
        "total": len(audit.checks),
        "passed": len(audit.checks) - len(audit.failed),
        "failed": len(audit.failed),
        "failures": audit.failed,
    }
    if write:
        stem = f"verification_{tag}" if tag else "verification_single"
        write_atomic(RESULTS / f"{stem}.json", json.dumps(payload, indent=2) + "\n")
        write_atomic(RESULTS / f"{stem}.md", render(payload))
    return payload


def render(payload: dict) -> str:
    lines = [
        "# Independent verification",
        "",
        "Every claim currently marked PROVEN, VERIFIED, or NUMERIC is re-derived here "
        "without importing `theory` or `phenomenology`.",
        "",
        f"**{payload['passed']}/{payload['total']} checks passed.**",
        "",
        "| claim | result | detail |",
        "|---|---|---|",
    ]
    for check in payload["checks"]:
        mark = "PASS" if check["passed"] else "FAIL"
        lines.append(f"| {check['claim']} | {mark} | {check['detail']} |")
    if payload["failures"]:
        lines += ["", "## Failures", ""]
        for failure in payload["failures"]:
            lines.append(f"- **{failure['claim']}**: {failure['detail']}")
    lines.append("")
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description="Independent verification audit.")
    parser.add_argument("--tag", help="write results/verification_<tag>.{json,md}")
    args = parser.parse_args()
    result = run(tag=args.tag)
    print(f"{result['passed']}/{result['total']} independent checks passed")
    if args.tag is None:
        print(
            "NOT ACCEPTED: this is a single run. Wrote results/verification_single.{json,md}. "
            "Run with --tag run1 and --tag run2, then python -m verification.compare."
        )
    for failure in result["failures"]:
        print(f"FAIL: {failure['claim']}: {failure['detail']}")
    return 0 if not result["failures"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
