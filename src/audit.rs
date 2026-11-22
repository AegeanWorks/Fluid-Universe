//! Independent, falsification-first audit of the frozen claim ledger.
//!
//! Port of `verification/audit.py`, rewritten with no external crates. Every
//! number below is *re-derived locally*: this module writes out its own Koide,
//! circulant, rotor, Derrick, linking-integral and RGE formulas and never calls
//! (or re-exports) `circulant::checks`, `spectra::checks`, `rotor::checks`, etc.
//! Only the generic numerical kernels in `crate::linalg` (Jacobi eigenvalues,
//! Simpson quadrature) are borrowed; the claim formulas themselves live here.
//!
//! A verdict describes the claim *as stated in the ledger*, not the numeric
//! result: e.g. "Q=2/3 for |c|/a=1/sqrt2" is PROVEN, while "the Z3 structure
//! alone fixes Q" and "exact one-loop r invariance" are FALSIFIED. The allowed
//! verdicts are PROVEN / FALSIFIED / NUMERIC / UNVERIFIED, matching `claims.md`.
//!
//! `run_audit` is pure (fixed inputs and a private deterministic LCG), so the
//! two-process rule G-001 sees the same `audit_fingerprint` on every call, in
//! any call order.

use std::f64::consts::{PI, TAU};

/// One re-derived claim result with its frozen-ledger verdict.
#[derive(Clone, Debug, PartialEq)]
pub struct AuditRecord {
    pub claim: &'static str,
    pub verdict: &'static str,
    pub detail: String,
    pub value: f64,
}

const KOIDE: f64 = 2.0 / 3.0;
const R_KOIDE: f64 = std::f64::consts::FRAC_1_SQRT_2;
/// Boltzmann constant in eV/K (CODATA 2018), as used by the water-Marcus scans.
const KB_EV_PER_K: f64 = 8.617333262145e-5;
const M_E: f64 = 0.51099895;
const M_MU: f64 = 105.6583755;
const M_TAU: f64 = 1776.93;

// ---------------------------------------------------------------------------
// Locally re-derived primitives (no reuse of the other modules' checks)
// ---------------------------------------------------------------------------

/// Koide ratio `sum(m) / (sum sqrt(m))^2` of three (possibly zero) masses.
fn q_from_masses(m: &[f64]) -> f64 {
    let sum: f64 = m.iter().sum();
    let root_sum: f64 = m.iter().map(|v| v.sqrt()).sum();
    sum / (root_sum * root_sum)
}

/// Signed-root ratio `sum(x^2) / (sum x)^2`.
fn q_signed(x: &[f64]) -> f64 {
    let sq: f64 = x.iter().map(|v| v * v).sum();
    let sum: f64 = x.iter().sum();
    sq / (sum * sum)
}

/// Algebraic identity `Q_signed = 1/3 + 2 c^2 / (3 a^2)` of a circulant.
fn q_from_ac(a: f64, c: f64) -> f64 {
    1.0 / 3.0 + 2.0 * c * c / (3.0 * a * a)
}

/// Signed eigenvalues `a + 2 c cos(delta + 2*pi*k/3)` of `circ(a, c e^{i d}, ...)`.
fn circ_roots(a: f64, c: f64, delta: f64) -> [f64; 3] {
    std::array::from_fn(|k| a + 2.0 * c * (delta + TAU * (k as f64) / 3.0).cos())
}

/// Inverse DFT: recover `(a, c, delta)` from three signed roots.
fn extract_ac(x: &[f64; 3]) -> (f64, f64, f64) {
    let a = x.iter().sum::<f64>() / 3.0;
    let (mut re, mut im) = (0.0, 0.0);
    for (k, &v) in x.iter().enumerate() {
        let phi = TAU * (k as f64) / 3.0;
        re += v * phi.cos();
        im -= v * phi.sin();
    }
    re /= 3.0;
    im /= 3.0;
    let c = re.hypot(im);
    let mut delta = im.atan2(re);
    if delta < 0.0 {
        delta += TAU;
    }
    (a, c, delta)
}

/// Solve `Q(m_e, m_mu, m_tau) = 2/3` for the larger root of the quadratic in
/// `sqrt(m_tau)`, then square it to get the tau mass prediction (MeV).
fn koide_tau_prediction(m_e: f64, m_mu: f64) -> f64 {
    let s = m_e.sqrt() + m_mu.sqrt();
    let b = -4.0 * s;
    let c = 3.0 * (m_e + m_mu) - 2.0 * s * s;
    let disc = b * b - 4.0 * c;
    let centre = -0.5 * b;
    let half = 0.5 * disc.max(0.0).sqrt();
    let root = centre + half;
    root * root
}

/// Eigenvalues of the 2x2 Frank-Oseen elastic matrix for a unit direction,
/// obtained from its trace and determinant (not from another module).
fn elastic_eigs2(k1: f64, k2: f64, k3: f64, kx: f64, ky: f64, kz: f64) -> [f64; 2] {
    let a11 = k1 * kx * kx + k2 * ky * ky + k3 * kz * kz;
    let a12 = (k1 - k2) * kx * ky;
    let a22 = k1 * ky * ky + k2 * kx * kx + k3 * kz * kz;
    let trace = a11 + a22;
    let det = a11 * a22 - a12 * a12;
    let gap = (trace * trace - 4.0 * det).max(0.0).sqrt();
    [0.5 * (trace - gap), 0.5 * (trace + gap)]
}

/// Minimal deterministic 64-bit LCG with a cached Box-Muller normal deviate.
struct Lcg {
    state: u64,
    spare: Option<f64>,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed, spare: None }
    }

    fn uniform(&mut self) -> f64 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.state >> 11) as f64) / ((1_u64 << 53) as f64)
    }

    fn normal(&mut self) -> f64 {
        if let Some(spare) = self.spare.take() {
            return spare;
        }
        let radius = (-2.0 * self.uniform().max(f64::MIN_POSITIVE).ln()).sqrt();
        let angle = TAU * self.uniform();
        self.spare = Some(radius * angle.sin());
        radius * angle.cos()
    }
}

/// Fractional peak-to-peak anisotropy of the two transverse speeds for Frank
/// constants `(k1,k2,k3)` over `n` deterministic unit directions. Exactly zero
/// at the isotropic point `k1 == k2 == k3`.
fn residual_anisotropy(k1: f64, k2: f64, k3: f64, n: usize) -> f64 {
    let mut rng = Lcg::new(20_261_025);
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut sum = 0.0;
    let mut count = 0usize;
    for _ in 0..n {
        let (mut x, mut y, mut z) = (rng.normal(), rng.normal(), rng.normal());
        let norm = (x * x + y * y + z * z).sqrt();
        x /= norm;
        y /= norm;
        z /= norm;
        for c2 in elastic_eigs2(k1, k2, k3, x, y, z) {
            min = min.min(c2);
            max = max.max(c2);
            sum += c2;
            count += 1;
        }
    }
    (max - min) / (sum / count as f64)
}

/// Gauss linking integral of two analytic Hopf fibers:
/// circle A(t) = (cos t, sin t, 0), circle B(s) = (1 + cos s, 0, ±sin s).
/// `reflect` mirrors B in z, which reverses its orientation and flips the sign.
/// A midpoint quadrature over a smooth, non-intersecting pair returns ±1.
fn gauss_linking(n: usize, reflect: bool) -> f64 {
    let sign = if reflect { -1.0 } else { 1.0 };
    let mut sum = 0.0;
    for i in 0..n {
        let t = TAU * (i as f64 + 0.5) / n as f64;
        let (ct, st) = (t.cos(), t.sin());
        for j in 0..n {
            let s = TAU * (j as f64 + 0.5) / n as f64;
            let (cs, ss) = (s.cos(), s.sin());
            // r1 = (ct, st, 0), r1' = (-st, ct, 0)
            // r2 = (1+cs, 0, sign*ss), r2' = (-ss, 0, sign*cs)
            let dx = ct - (1.0 + cs);
            let dy = st;
            let dz = -sign * ss;
            // (r1' x r2') = (sign*ct*cs, sign*st*cs, ct*ss): the whole
            // integrand picks up `sign`, so reflecting B flips the linking.
            let cx = sign * ct * cs;
            let cy = sign * st * cs;
            let cz = ct * ss;
            let dot = cx * dx + cy * dy + cz * dz;
            let r2 = dx * dx + dy * dy + dz * dz;
            sum += dot / (r2 * r2.sqrt());
        }
    }
    let h = TAU / n as f64;
    sum * h * h / (4.0 * PI)
}

/// Z3 order parameter `|sum_k w_k exp(3 i phi_k)| / sum_k w_k` with unit weights.
fn z3_order(phases: &[f64]) -> f64 {
    let (mut re, mut im) = (0.0, 0.0);
    for &phi in phases {
        re += (3.0 * phi).cos();
        im += (3.0 * phi).sin();
    }
    re.hypot(im) / phases.len() as f64
}

/// Fixed-lambda classical Marcus FWHM in eV: `2 sqrt(2 ln2) sqrt(2 lambda kB T)`.
fn marcus_fwhm(lambda_ev: f64, temp_k: f64) -> f64 {
    2.0 * (2.0 * 2.0_f64.ln()).sqrt() * (2.0 * lambda_ev * KB_EV_PER_K * temp_k).sqrt()
}

/// Ground state of one Fourier sector of the quantum rotor at I = mu = 1,
/// assembled locally and diagonalised with the shared Jacobi kernel.
fn rotor_sector_ground(mu: f64, sector: i32, cutoff: i32) -> f64 {
    let values: Vec<i32> = (-cutoff..=cutoff)
        .filter(|&n| n.rem_euclid(3) == sector.rem_euclid(3))
        .collect();
    let m = values.len();
    let mut h = vec![vec![0.0; m]; m];
    for (a, &n) in values.iter().enumerate() {
        let nf = n as f64;
        h[a][a] = nf * nf / 2.0 + mu;
        for shift in [3_i32, -3] {
            if let Ok(b) = values.binary_search(&(n + shift)) {
                h[a][b] -= 0.5 * mu;
            }
        }
    }
    crate::linalg::symmetric_eigenvalues(&h)[0]
}

/// One-loop lepton+quark Yukawa RHS, written out from scratch (16 pi^2 units).
fn rg_rhs(t: [f64; 8]) -> [f64; 8] {
    let [ye, ymu, ytau, yt, yb, g1, g2, g3] = t;
    let loop_factor = 16.0 * PI * PI;
    let big_t = 3.0 * yt * yt + 3.0 * yb * yb + ye * ye + ymu * ymu + ytau * ytau;
    [
        ye * (1.5 * ye * ye + big_t - 2.25 * g2 * g2 - 3.75 * g1 * g1) / loop_factor,
        ymu * (1.5 * ymu * ymu + big_t - 2.25 * g2 * g2 - 3.75 * g1 * g1) / loop_factor,
        ytau * (1.5 * ytau * ytau + big_t - 2.25 * g2 * g2 - 3.75 * g1 * g1) / loop_factor,
        yt * (1.5 * yt * yt - 1.5 * yb * yb + big_t - 8.0 * g3 * g3
            - 2.25 * g2 * g2 - 17.0 / 12.0 * g1 * g1) / loop_factor,
        yb * (1.5 * yb * yb - 1.5 * yt * yt + big_t - 8.0 * g3 * g3
            - 2.25 * g2 * g2 - 5.0 / 12.0 * g1 * g1) / loop_factor,
        41.0 / 10.0 * g1 * g1 * g1 / loop_factor,
        -19.0 / 6.0 * g2 * g2 * g2 / loop_factor,
        -7.0 * g3 * g3 * g3 / loop_factor,
    ]
}

/// `|c|/a` of the three charged leptons at a given Yukawa state.
fn rg_ratio(s: &[f64; 8]) -> f64 {
    let vew = 246.0;
    let m = [s[0], s[1], s[2]].map(|y| y * vew / 2.0_f64.sqrt());
    let roots = m.map(f64::sqrt);
    let (a, c, _) = extract_ac(&roots);
    c / a.abs()
}

/// Independent one-loop drift of `r = |c|/a` from M_Z to 10^16 GeV via RK4.
fn rg_r_drift() -> f64 {
    let vew = 246.0;
    let scale = 2.0_f64.sqrt() / vew;
    let mut s = [
        scale * 0.00048307,
        scale * 0.101766,
        scale * 1.72856,
        scale * 168.26,
        scale * 2.839,
        (5.0_f64 / 3.0).sqrt() * 0.357254,
        0.65100,
        1.2104,
    ];
    let r_start = rg_ratio(&s);
    let t0 = 91.1876_f64.ln();
    let t1 = 1.0e16_f64.ln();
    let steps = 800;
    let h = (t1 - t0) / steps as f64;
    for _ in 0..steps {
        let k1 = rg_rhs(s);
        let s2 = std::array::from_fn(|i| s[i] + 0.5 * h * k1[i]);
        let k2 = rg_rhs(s2);
        let s3 = std::array::from_fn(|i| s[i] + 0.5 * h * k2[i]);
        let k3 = rg_rhs(s3);
        let s4 = std::array::from_fn(|i| s[i] + h * k3[i]);
        let k4 = rg_rhs(s4);
        s = std::array::from_fn(|i| {
            s[i] + h / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i])
        });
    }
    (rg_ratio(&s) - r_start).abs()
}

/// All audit rows, derived from scratch in a fixed order. Kept private so the
/// public API cannot diverge between `run_audit` and `checks`.
struct Row {
    id: &'static str,
    claim: &'static str,
    verdict: &'static str,
    detail: String,
    value: f64,
}

fn rows() -> Vec<Row> {
    let mut out = Vec::new();

    // 1a/F2: signed-root Q identity, spot-checked on a deterministic grid.
    let (mut max_q, mut max_eig) = (0.0_f64, 0.0_f64);
    for a in [0.5, 1.0, 2.0, 3.0] {
        for c in [0.1, 0.5, 0.9] {
            for step in 0..24 {
                let delta = TAU * step as f64 / 24.0;
                let roots = circ_roots(a, c, delta);
                max_q = max_q.max((q_signed(&roots) - q_from_ac(a, c)).abs());
                // The three roots sum to 3a and their squares to 3a^2+6c^2:
                // an independent algebraic restatement of the identity.
                let sum: f64 = roots.iter().sum();
                let sq: f64 = roots.iter().map(|v| v * v).sum();
                max_eig = max_eig.max((sum - 3.0 * a).abs().max((sq - (3.0 * a * a + 6.0 * c * c)).abs()));
            }
        }
    }
    out.push(Row {
        id: "audit.q_identity",
        claim: "1a/F2: circulant Q identity for signed eigenvalues",
        verdict: "PROVEN",
        detail: format!("max |Q_signed - (1/3+2c^2/3a^2)| = {max_q:.3e}; algebraic sum/sum-sq residual {max_eig:.3e}"),
        value: max_q,
    });

    // 1c/F3: Q = 2/3 iff |c|/a = 1/sqrt(2).
    out.push(Row {
        id: "audit.koide_ratio_condition",
        claim: "1c: Q=2/3 for |c|/a=1/sqrt2",
        verdict: "PROVEN",
        detail: format!("q_from_ac(1, 1/sqrt2) = {:.12}; c^2 = a^2/2 solves 1/3+2c^2/3a^2 = {KOIDE:.12}", q_from_ac(1.0, R_KOIDE)),
        value: R_KOIDE,
    });

    // 1b: the Z3/circulant structure does NOT by itself fix Q (three parameters
    // fit three arbitrary masses, so Q is free).
    let triples: [[f64; 3]; 5] = [
        [1.0, 4.0, 9.0],
        [1.0, 1.0, 4.0],
        [1.0, 100.0, 10_000.0],
        [2.0, 2.0, 2.0],
        [1.0, 2.0, 3.0],
    ];
    let qs: Vec<f64> = triples.iter().map(|m| q_from_masses(m)).collect();
    let spread = qs.iter().copied().fold(0.0_f64, f64::max)
        - qs.iter().copied().fold(f64::INFINITY, f64::min);
    out.push(Row {
        id: "audit.z3_not_fixed",
        claim: "1b: Z3/circulant structure alone fixes Q=2/3",
        verdict: "FALSIFIED",
        detail: format!("five positive-root triples span Q in [{:.6}, {:.6}] (spread {:.6})",
            qs.iter().copied().fold(f64::INFINITY, f64::min),
            qs.iter().copied().fold(0.0_f64, f64::max), spread),
        value: spread,
    });

    // 1d: signed vs physical Q at a=c counterexample (a=1, c=0.9, delta=pi).
    let signed_roots = circ_roots(1.0, 0.9, PI);
    let q_signed_bad = q_signed(&signed_roots);
    let q_physical_bad = q_from_masses(&signed_roots.map(|v| v * v));
    out.push(Row {
        id: "audit.sign_caveat",
        claim: "1d: signed vs physical Q caveat for signed roots",
        verdict: "FALSIFIED",
        detail: format!("a=1, c=0.9, delta=pi: signed Q={q_signed_bad:.10}, physical Q={q_physical_bad:.10}"),
        value: q_signed_bad,
    });

    // 3a: tau mass prediction from Q=2/3 with measured e and mu.
    let tau = koide_tau_prediction(M_E, M_MU);
    let q_tau = q_from_masses(&[M_E, M_MU, tau]);
    out.push(Row {
        id: "audit.tau_prediction",
        claim: "3a: tau mass predicted from Q=2/3",
        verdict: "NUMERIC",
        detail: format!("m_tau = {tau:.6} MeV; Q(m_e,m_mu,m_tau) = {q_tau:.12}"),
        value: tau,
    });

    // 4a: exact one-loop invariance of r is FALSIFIED (the 3/2 y_k^2 term is
    // generation-dependent; RK4 gives a small but nonzero drift).
    let dr = rg_r_drift();
    out.push(Row {
        id: "audit.r_invariance",
        claim: "4a: exact r invariance at one loop",
        verdict: "FALSIFIED",
        detail: format!("|r(10^16 GeV) - r(M_Z)| = {dr:.3e} (nonzero; generation-dependent RHS)"),
        value: dr,
    });

    // 7a: neutrino Q max reaches 2/3 is FALSIFIED (max ~0.5856, NO, m0=0).
    let (dm21, dm31) = (7.41e-5_f64, 2.507e-3_f64);
    let q_no = q_from_masses(&[0.0, dm21.sqrt(), dm31.sqrt()]);
    let q_io = q_from_masses(&[dm31.abs().sqrt(), (dm31.abs() + dm21).sqrt(), 0.0]);
    let q_nu_max = q_no.max(q_io);
    out.push(Row {
        id: "audit.neutrino_q_max",
        claim: "7a: neutrino Q max reaches 2/3",
        verdict: "FALSIFIED",
        detail: format!("Q_nu(NO,m0=0)={q_no:.12}, Q_nu(IO,m0=0)={q_io:.12}; max={q_nu_max:.12} < 2/3"),
        value: q_nu_max,
    });

    // 8: direct Koide extension to quarks is FALSIFIED (both Q's above 2/3).
    let q_up = q_from_masses(&[0.00123, 0.620, 168.26]);
    let q_down = q_from_masses(&[0.00267, 0.05316, 2.839]);
    out.push(Row {
        id: "audit.quark_q",
        claim: "8: quark Koide Q reaches 2/3",
        verdict: "FALSIFIED",
        detail: format!("Huang-Zhou M_Z masses: Q_up={q_up:.12}, Q_down={q_down:.12}, both > 2/3"),
        value: q_up,
    });

    // 5c: rotor sectors 1 and 2 are exactly degenerate (parity n -> -n).
    let e1 = rotor_sector_ground(1.0, 1, 40);
    let e2 = rotor_sector_ground(1.0, 2, 40);
    let degap = (e1 - e2).abs();
    out.push(Row {
        id: "audit.rotor_degeneracy",
        claim: "5c: rotor sectors 1 and 2 are exactly degenerate",
        verdict: "PROVEN",
        detail: format!("E1={e1:.12}, E2={e2:.12}, |E1-E2|={degap:.3e} (exact by H(phi)=H(-phi))"),
        value: degap,
    });

    // 6a: domain-wall tension T = (8/3) sqrt(mu), checked by local Simpson.
    let tension = crate::linalg::simpson(
        |phi| (2.0 * (1.0 - (3.0 * phi).cos())).max(0.0).sqrt(),
        0.0,
        TAU / 3.0,
        1024,
    );
    out.push(Row {
        id: "audit.wall_tension",
        claim: "6a: domain wall tension T=(8/3)sqrt(mu)",
        verdict: "PROVEN",
        detail: format!("Simpson T(1)={tension:.12} vs (8/3)={:.12}", 8.0 / 3.0),
        value: tension,
    });

    // 14a: Derrick exponents (d-2,d-4,d) and the virial dE/dlambda = E2-E4+3V.
    let (e2, e4, vpot) = (2.0_f64, 1.0_f64, 0.5_f64);
    let scaling = |l: f64| l * e2 + e4 / l + l * l * l * vpot;
    let h = 1e-7;
    let slope = (scaling(1.0 + h) - scaling(1.0 - h)) / (2.0 * h);
    let virial = e2 - e4 + 3.0 * vpot;
    out.push(Row {
        id: "audit.derrick_virial",
        claim: "14a: Derrick exponents and virial relation",
        verdict: "PROVEN",
        detail: format!("d=3 exponents [1,-1,3]; numeric dE/dlambda={slope:.9}, virial={virial:.9}, residual {:.2e}", (slope - virial).abs()),
        value: (slope - virial).abs(),
    });

    // 10: equal Frank constants give an exactly isotropic quadratic dispersion.
    let iso = residual_anisotropy(2.0, 2.0, 2.0, 64);
    out.push(Row {
        id: "audit.equal_k_isotropy",
        claim: "10: equal-K isotropy (omega^2=c^2 k^2)",
        verdict: "PROVEN",
        detail: format!("xi(K=K=K)={iso:.1e}; the elastic matrix is 2K*I, two degenerate transverse modes"),
        value: iso,
    });

    // 11a: K=(1,2,3) breaks Lorentz invariance (O(1) residual anisotropy).
    let xi = residual_anisotropy(1.0, 2.0, 3.0, 1000);
    out.push(Row {
        id: "audit.k123_anisotropy",
        claim: "11a: K=(1,2,3) emergent metric",
        verdict: "FALSIFIED",
        detail: format!("xi(K=1,2,3)={xi:.6}; axis speeds along x=[1,sqrt2], z=[sqrt3,sqrt3]: speeds split"),
        value: xi,
    });

    // 9: delta closed-form search is UNVERIFIED (best rational 2/9 is post-hoc).
    let roots = [M_E.sqrt(), M_MU.sqrt(), M_TAU.sqrt()];
    let (_, _, delta) = extract_ac(&roots);
    let span = TAU / 3.0;
    let delta_mod = delta.rem_euclid(span);
    let (mut best_diff, mut best_label) = (f64::INFINITY, String::from("none"));
    for q in 1..=20 {
        for p in 1..=((span * q as f64) as usize + 2) {
            let value = p as f64 / q as f64;
            if value > 0.0 && value <= span {
                let diff = (value - delta_mod).abs();
                if diff < best_diff {
                    best_diff = diff;
                    best_label = format!("{p}/{q}");
                }
            }
        }
    }
    out.push(Row {
        id: "audit.delta_closed_form",
        claim: "9: delta has a closed form (rational multiple of pi)",
        verdict: "UNVERIFIED",
        detail: format!("delta_mod(2pi/3)={delta_mod:.9}; nearest {best_label} at {best_diff:.3e}; post-hoc, not promoted"),
        value: delta_mod,
    });

    // 15/4: Hopf preimage charge = +1, re-derived as a Gauss linking integral
    // of two analytic linked circles (reflection flips the sign).
    let link = gauss_linking(256, false);
    out.push(Row {
        id: "audit.hopf_preimage_charge",
        claim: "15/4: Hopf preimage charge = +1",
        verdict: "NUMERIC",
        detail: format!("Gauss linking of two linked circles = {link:.6}; reflection gives {}", gauss_linking(256, true)),
        value: link.abs(),
    });

    // 15: Z3 order parameter is bounded in [0,1] and attains both limits.
    let z3_mixed = z3_order(&[0.0, PI / 3.0, 2.0 * PI / 3.0, PI]);
    let z3_pure = z3_order(&[0.2, 0.2, 0.2]);
    out.push(Row {
        id: "audit.z3_limits",
        claim: "15: Z3 order parameter limits",
        verdict: "NUMERIC",
        detail: format!("four evenly spaced phases -> Z3={z3_mixed:.3e} (min); single phase -> Z3={z3_pure:.3e} (max 1)"),
        value: z3_mixed,
    });

    // 17/18: vortex core-threading blob fraction r^2/(r^2+delta^2).
    let delta_core: f64 = 0.1;
    let blob = (2.0 * delta_core).powi(2) / ((2.0 * delta_core).powi(2) + delta_core * delta_core);
    out.push(Row {
        id: "audit.vortex_blob_fraction",
        claim: "17/18: vortex blob fractions",
        verdict: "NUMERIC",
        detail: format!("r^2/(r^2+delta^2) at r=2*delta = {blob:.12} (delta point 0.5, 20-delta point 400/401)"),
        value: blob,
    });

    // 40: Marcus FWHM = 2 sqrt(2 ln2) sqrt(2 lambda kB T) ~ 0.681 eV at 300 K.
    let width = marcus_fwhm(1.6171325440878093, 300.0);
    out.push(Row {
        id: "audit.marcus_width",
        claim: "40: Marcus band width at 300 K",
        verdict: "NUMERIC",
        detail: format!("FWHM(lambda=1.617132544 eV, T=300K)={width:.9} eV; scales as sqrt(T)"),
        value: width,
    });

    out
}

/// Re-derive every ledger claim independently, in a fixed deterministic order.
pub fn run_audit() -> Vec<AuditRecord> {
    rows()
        .into_iter()
        .map(|r| AuditRecord {
            claim: r.claim,
            verdict: r.verdict,
            detail: r.detail,
            value: r.value,
        })
        .collect()
}

/// FNV-1a over `claim|verdict|value|detail` lines, rendered as 16 hex chars.
///
/// Mirrors `crate::fingerprint` so the two-process audit can compare a single
/// stable token; no hidden state and no dependence on call order.
pub fn audit_fingerprint() -> String {
    use std::fmt::Write;
    let mut text = String::new();
    for row in run_audit() {
        let _ = writeln!(
            text,
            "{}|{}|{:.15e}|{}",
            row.claim, row.verdict, row.value, row.detail
        );
    }
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in text.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100_0000_01b3);
    }
    format!("{hash:016x}")
}

/// Audit results exposed as the crate's common `Check` type (ids prefixed
/// `audit.`), with the verdict and detail carried in the text field.
pub fn checks() -> Vec<crate::Check> {
    rows()
        .into_iter()
        .map(|r| {
            crate::Check::new(
                r.id,
                r.value,
                format!("{} | {}", r.verdict, r.detail),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    const ALLOWED: [&str; 4] = ["PROVEN", "FALSIFIED", "NUMERIC", "UNVERIFIED"];

    #[test]
    fn fingerprint_is_deterministic_and_independent_of_call_order() {
        let first = audit_fingerprint();
        assert_eq!(first.len(), 16);
        assert!(first.bytes().all(|b| b.is_ascii_hexdigit()));
        // Interleave other public entry points; they must not perturb state.
        let _ = checks();
        let _ = run_audit();
        assert_eq!(audit_fingerprint(), first);
        assert_eq!(run_audit().len(), checks().len());
    }

    #[test]
    fn every_verdict_is_allowed_and_all_four_appear() {
        let records = run_audit();
        assert!(records.len() >= 15, "expected at least 15 claims, got {}", records.len());
        let mut seen = std::collections::HashSet::new();
        for r in &records {
            assert!(ALLOWED.contains(&r.verdict), "bad verdict {}", r.verdict);
            seen.insert(r.verdict);
        }
        for verdict in ALLOWED {
            assert!(seen.contains(verdict), "missing verdict {verdict}");
        }
    }

    #[test]
    fn frozen_ledger_expected_verdicts() {
        let records = run_audit();
        let verdict = |needle: &str| -> &'static str {
            records
                .iter()
                .find(|r| r.claim.contains(needle))
                .unwrap_or_else(|| panic!("missing claim containing {needle:?}"))
                .verdict
        };
        // The five named reference claims from the contract.
        assert_eq!(verdict("Q=2/3 for |c|/a=1/sqrt2"), "PROVEN");
        assert_eq!(verdict("exact r invariance"), "FALSIFIED");
        assert_eq!(verdict("neutrino Q max"), "FALSIFIED");
        assert_eq!(verdict("equal-K isotropy"), "PROVEN");
        assert_eq!(verdict("K=(1,2,3)"), "FALSIFIED");
        // Additional frozen-ledger verdicts.
        assert_eq!(verdict("Q identity"), "PROVEN");
        assert_eq!(verdict("signed vs physical"), "FALSIFIED");
        assert_eq!(verdict("quark Koide Q"), "FALSIFIED");
        assert_eq!(verdict("rotor sectors"), "PROVEN");
        assert_eq!(verdict("domain wall tension"), "PROVEN");
        assert_eq!(verdict("Derrick exponents"), "PROVEN");
    }

    #[test]
    fn no_record_value_is_nan_or_infinite() {
        for r in run_audit() {
            assert!(r.value.is_finite(), "{}: {}", r.claim, r.value);
            assert!(!r.detail.is_empty());
        }
        for c in checks() {
            assert!(c.value.is_finite(), "{}: {}", c.id, c.value);
        }
    }

    #[test]
    fn locally_rederived_physics_matches_references() {
        let records = run_audit();
        let value = |needle: &str| -> f64 {
            records.iter().find(|r| r.claim.contains(needle)).unwrap().value
        };
        assert!(approx(value("circulant Q identity"), 0.0, 1e-9));
        assert!(approx(value("tau mass predicted"), 1776.93, 1e-2));
        let nu = value("neutrino Q max");
        assert!(approx(nu, 0.585612, 1e-3) && nu < KOIDE);
        let quark = value("quark Koide Q");
        assert!(approx(quark, 0.887569, 1e-3) && quark > KOIDE);
        assert!(value("rotor sectors") < 1e-8);
        assert!(approx(value("domain wall tension"), 8.0 / 3.0, 1e-8));
        assert!(value("Derrick exponents") < 1e-6);
        assert!(value("equal-K isotropy") < 1e-12);
        assert!(value("K=(1,2,3)") > 0.5);
        assert!(approx(value("Marcus band width"), 0.680915, 1e-3));
        assert!(approx(value("vortex blob fractions"), 0.8, 1e-12));
        assert!(approx(value("Z3 order parameter limits"), 0.0, 1e-12));
    }

    #[test]
    fn gauss_linking_is_one_and_flips_under_reflection() {
        let link = gauss_linking(256, false);
        let reflected = gauss_linking(256, true);
        assert!(approx(link.abs(), 1.0, 2e-2), "link = {link}");
        assert!(approx(link + reflected, 0.0, 2e-2), "{link} vs {reflected}");
    }

    #[test]
    fn circulant_identity_and_sign_caveat_are_rederived() {
        let roots = circ_roots(1.0, 0.9, PI);
        assert!(approx(q_signed(&roots), 0.8733333333, 1e-9));
        assert!(approx(q_from_masses(&roots.map(|v| v * v)), 0.3714555766, 1e-9));
        assert!(approx(q_from_ac(1.0, R_KOIDE), KOIDE, 1e-14));
        let (a, c, delta) = extract_ac(&[1.0, 2.0, 3.0].map(|v| v * v * v));
        assert!(a > 0.0 && c > 0.0 && (0.0..TAU).contains(&delta));
    }

    #[test]
    fn checks_are_stable_unique_and_prefixed() {
        let first = checks();
        let second = checks();
        assert_eq!(first.len(), second.len());
        let mut ids = std::collections::HashSet::new();
        for (a, b) in first.iter().zip(second.iter()) {
            assert_eq!(a.id, b.id);
            assert_eq!(a.value.to_bits(), b.value.to_bits());
            assert_eq!(a.detail, b.detail);
            assert!(a.id.starts_with("audit."));
            assert!(ids.insert(a.id));
        }
    }
}
