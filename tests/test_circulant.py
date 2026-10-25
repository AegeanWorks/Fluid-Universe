"""Tests for claims 1-2: circulant eigenvalues, Q identity, Hermiticity, DFT vectors."""

from __future__ import annotations

import numpy as np
import pytest
import sympy as sp
from hypothesis import given, settings
from hypothesis import strategies as st

from theory.circulant import (
    OMEGA,
    circulant_matrix,
    circulant_symbolic,
    dft_vectors,
    eigenvalues,
    eigenvalues_symbolic,
    extract_ac_delta,
    fit_masses,
    koide_tau_prediction,
    q_from_ac,
    q_identity_symbolic,
    q_masses,
    q_physical_from_roots,
    q_signed,
)

RNG = np.random.default_rng(20261025)


def test_symbolic_eigenvalues():
    M, a, c, delta = circulant_symbolic()
    w = sp.exp(2 * sp.pi * sp.I / 3)
    for k in range(3):
        v = sp.Matrix([1, w**k, w ** (2 * k)])
        lam = eigenvalues_symbolic()[k]
        residual = sp.expand(M * v - lam * v)
        assert all(sp.simplify(sp.expand_complex(z)) == 0 for z in residual)


def test_hermitian_not_symmetric():
    M, a, c, delta = circulant_symbolic()
    assert sp.simplify(M - M.H) == sp.zeros(3, 3)
    # M - M^T = 2 i c sin(delta) on the off-diagonal; not zero for generic delta.
    off = sp.simplify((M - M.T)[0, 1])
    assert off != 0
    assert sp.simplify(sp.expand_complex(off - 2 * sp.I * c * sp.sin(delta))) == 0


def test_dft_eigenvectors():
    V = dft_vectors()
    assert np.allclose(V.conj().T @ V, np.eye(3))
    for _ in range(200):
        a = RNG.uniform(-4.0, 4.0)
        c = RNG.uniform(0.0, 3.0)
        delta = RNG.uniform(0.0, 2.0 * np.pi)
        M = circulant_matrix(a, c, delta)
        lam = eigenvalues(a, c, delta)
        for k in range(3):
            assert np.allclose(M @ V[:, k], lam[k] * V[:, k], atol=1e-12)
        # Eigenvectors are delta-independent by construction: V has no delta.
        assert np.allclose(V, dft_vectors())


def test_q_identity_symbolic():
    assert q_identity_symbolic() == 0


def test_q_identity_numeric():
    errors = []
    for _ in range(10_000):
        a = RNG.uniform(0.2, 6.0)
        c = RNG.uniform(0.0, 4.0)
        delta = RNG.uniform(0.0, 2.0 * np.pi)
        x = eigenvalues(a, c, delta)
        expected = q_from_ac(a, c)
        errors.append(abs(q_signed(x) - expected) / max(1.0, abs(expected)))
    assert max(errors) < 1e-12


def test_extract_roundtrip():
    for _ in range(10_000):
        a = RNG.uniform(-5.0, 5.0)
        c = RNG.uniform(1e-6, 3.0)
        delta = RNG.uniform(0.0, 2.0 * np.pi)
        x = eigenvalues(a, c, delta)
        a2, c2, d2 = extract_ac_delta(x)
        assert abs(a2 - a) < 1e-12
        assert abs(c2 - c) < 1e-12
        # compare angles on the circle
        assert abs(np.exp(1j * (d2 - delta)) - 1.0) < 1e-12


def test_koide_condition():
    a = 2.0
    assert q_from_ac(a, 0.0) == pytest.approx(1.0 / 3.0)
    assert q_from_ac(a, a / np.sqrt(2.0)) == pytest.approx(2.0 / 3.0)
    assert q_from_ac(a, a) == pytest.approx(1.0)
    # signed-eigenvalue branch only: delta=0 gives eigenvalues (3a, 0, 0).
    x = eigenvalues(a, a, 0.0)
    assert q_signed(x) == pytest.approx(1.0)
    assert q_physical_from_roots(x) == pytest.approx(1.0)


def test_sign_caveat():
    # a=1, c=0.9, delta=pi -> signed roots (-0.8, 1.9, 1.9).
    x = eigenvalues(1.0, 0.9, np.pi)
    assert np.isclose(x[0], -0.8)
    assert np.allclose(x[1:], 1.9)
    assert q_signed(x) == pytest.approx(0.8733333333333333)
    assert q_physical_from_roots(x) == pytest.approx(0.3714555765595463)
    assert abs(q_signed(x) - q_physical_from_roots(x)) > 0.1


def test_bound_half_is_false():
    # Positive signed roots with c/a = 0.75 and Q < 1.
    x = np.array([10.0, 1.0, 1.0])
    a, c, delta = extract_ac_delta(x)
    assert a == pytest.approx(4.0)
    assert c / a == pytest.approx(0.75)
    assert q_physical_from_roots(x) == pytest.approx(102.0 / 144.0)
    assert c / a > 0.5


def test_fit_masses_exact():
    for _ in range(200):
        m = RNG.uniform(0.01, 1000.0, size=3)
        fit = fit_masses(m)
        assert fit["max_abs_residual"] < 1e-9
        assert fit["Q"] == pytest.approx(fit["Q_from_ac"], rel=1e-12)
        assert fit["all_roots_positive"]


def test_koide_tau_prediction():
    me, mmu = 0.51099895000, 105.6583755
    mtau = koide_tau_prediction(me, mmu)
    assert 1776.0 < mtau < 1778.0
    assert q_masses([me, mmu, mtau]) == pytest.approx(2.0 / 3.0, abs=1e-12)


@given(
    a=st.floats(min_value=0.1, max_value=10.0),
    c=st.floats(min_value=0.0, max_value=10.0),
    delta=st.floats(min_value=0.0, max_value=2 * np.pi),
)
@settings(max_examples=200, deadline=None)
def test_property_q_identity(a, c, delta):
    x = eigenvalues(a, c, delta)
    if abs(np.sum(x)) < 1e-9:
        return
    expected = q_from_ac(a, c)
    assert abs(q_signed(x) - expected) / max(1.0, abs(expected)) < 1e-10


@given(
    m1=st.floats(min_value=1e-3, max_value=1e3),
    m2=st.floats(min_value=1e-3, max_value=1e3),
    m3=st.floats(min_value=1e-3, max_value=1e3),
)
@settings(max_examples=200, deadline=None)
def test_property_fit(m1, m2, m3):
    fit = fit_masses([m1, m2, m3])
    assert fit["max_abs_residual"] < 1e-8
