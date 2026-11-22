//! 3D field core: Hopf map, finite-difference energy/forces and relaxation.
//!
//! Dependency-free port of the numeric core of `sims/core3d.py` and
//! `sims/production.py`. The field is a flat row-major `n x n x n` grid of
//! three-component unit vectors; spatial derivatives use central differences
//! with periodic index wrap.
//!
//! Fidelity note (requested by the port contract): `relax` is a faithful
//! *finite-difference* port. The original used JAX spectral gradients, so
//! numbers here need not match the old recorded results bit-for-bit. The
//! analytic gradient implemented in [`force_field`] is derived from exactly
//! the same discretized energy as [`energy_terms`] and is cross-checked
//! against a scalar finite-difference probe in the unit tests and in
//! [`checks`].

use crate::Check;

/// Softening used in `cos(3 phi) = (nx^3 - 3 nx ny^2) / (nx^2+ny^2+eps)^{3/2}`.
const EPS: f64 = 1e-12;
/// Unit gradient coupling used by [`metrics`] (its signature carries no `k`).
pub const DEFAULT_K: f64 = 1.0;
/// Unit topological coupling used by [`metrics`] (signature carries no `kappa`).
pub const DEFAULT_KAPPA: f64 = 1.0;

#[derive(Clone, Debug)]
pub struct Field3 {
    pub n: usize,
    pub dx: f64,
    pub data: Vec<[f64; 3]>,
}

#[derive(Clone, Debug)]
pub struct Energy {
    pub e2: f64,
    pub e4: f64,
    pub potential: f64,
    pub total: f64,
}

#[derive(Clone, Debug)]
pub struct Metrics {
    pub e2: f64,
    pub e4: f64,
    pub potential: f64,
    pub energy: f64,
    pub z3: f64,
    pub max_bond: f64,
    pub mean_bond: f64,
    pub force_rms: f64,
}

// ---------------------------------------------------------------------------
// small vector helpers on [f64; 3]
// ---------------------------------------------------------------------------

#[inline]
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
#[inline]
fn scale(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}
#[inline]
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
#[inline]
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
#[inline]
fn norm(a: [f64; 3]) -> f64 {
    dot(a, a).sqrt()
}
#[inline]
fn acc(g: &mut [f64; 3], a: [f64; 3], s: f64) {
    g[0] += s * a[0];
    g[1] += s * a[1];
    g[2] += s * a[2];
}

#[inline]
fn wrap(i: isize, n: usize) -> usize {
    let m = n as isize;
    (((i % m) + m) % m) as usize
}
#[inline]
fn idx(n: usize, x: usize, y: usize, z: usize) -> usize {
    (x * n + y) * n + z
}
#[inline]
fn neighbor(n: usize, x: usize, y: usize, z: usize, dir: usize, step: isize) -> usize {
    match dir {
        0 => idx(n, wrap(x as isize + step, n), y, z),
        1 => idx(n, x, wrap(y as isize + step, n), z),
        _ => idx(n, x, y, wrap(z as isize + step, n)),
    }
}

// ---------------------------------------------------------------------------
// analytic Hopf map and initial field
// ---------------------------------------------------------------------------

/// Analytic `H = 1` Hopf map `R^3 -> S^2` from `theory/hopf.py`.
///
/// `r2 = x^2+y^2+z^2`, `d = 1+r2`, quaternion
/// `q = ((r2-1)/d, 2x/d, 2y/d, 2z/d)`, then
/// `n = (2(q0 q2 + q1 q3), 2(q1 q2 - q0 q3), q0^2+q1^2-q2^2-q3^2)`.
pub fn hopf_map(x: f64, y: f64, z: f64) -> [f64; 3] {
    let r2 = x * x + y * y + z * z;
    let d = 1.0 + r2;
    let q0 = (r2 - 1.0) / d;
    let q1 = 2.0 * x / d;
    let q2 = 2.0 * y / d;
    let q3 = 2.0 * z / d;
    [
        2.0 * (q0 * q2 + q1 * q3),
        2.0 * (q1 * q2 - q0 * q3),
        q0 * q0 + q1 * q1 - q2 * q2 - q3 * q3,
    ]
}

/// Hopf field on a periodic `n x n x n` grid of side `length`.
///
/// The axis is `linspace(-L/2, L/2, n, endpoint=false)`, i.e. `-L/2 + k*dx`
/// with `dx = L/n`. A smoothstep vacuum window blends the Hopf texture to
/// `[0,0,1]` between radius `0.85*L/2` and `L/2`; every vector is normalized.
/// `reflect = true` mirrors the domain in `x`, an orientation-reversing map of
/// `R^3` which flips the Hopf charge of the configuration.
pub fn analytic_field(n: usize, length: f64, reflect: bool) -> Field3 {
    assert!(n > 0, "analytic_field: n must be positive");
    assert!(length > 0.0, "analytic_field: length must be positive");
    let dx = length / n as f64;
    let half = length * 0.5;
    let inner = 0.85 * half;
    let outer = half;
    let mut data = Vec::with_capacity(n * n * n);
    for x in 0..n {
        let cx = -half + x as f64 * dx;
        for y in 0..n {
            let cy = -half + y as f64 * dx;
            for z in 0..n {
                let cz = -half + z as f64 * dx;
                let r = (cx * cx + cy * cy + cz * cz).sqrt();
                let t = ((outer - r) / (outer - inner)).clamp(0.0, 1.0);
                let w = t * t * (3.0 - 2.0 * t); // smoothstep
                let px = if reflect { -cx } else { cx };
                let h = hopf_map(px, cy, cz);
                let mut v = [w * h[0], w * h[1], w * h[2] + (1.0 - w)];
                let m = norm(v);
                if m > 1e-12 {
                    v = scale(v, 1.0 / m);
                } else {
                    v = [0.0, 0.0, 1.0];
                }
                data.push(v);
            }
        }
    }
    Field3 { n, dx, data }
}

/// Normalize every site vector in place, leaving near-zero vectors untouched.
pub fn normalize_field(field: &mut Field3) {
    for v in field.data.iter_mut() {
        let m = norm(*v);
        if m > 1e-12 {
            for c in v.iter_mut() {
                *c /= m;
            }
        }
    }
}

/// Central-difference derivative arrays `D_i(x) = (n[x+e_i]-n[x-e_i])/(2 dx)`.
fn derivatives(field: &Field3) -> [Vec<[f64; 3]>; 3] {
    let n = field.n;
    let size = n * n * n;
    let inv = 1.0 / (2.0 * field.dx);
    let mut out: [Vec<[f64; 3]>; 3] = std::array::from_fn(|_| vec![[0.0; 3]; size]);
    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                let p = idx(n, x, y, z);
                for i in 0..3 {
                    let plus = neighbor(n, x, y, z, i, 1);
                    let minus = neighbor(n, x, y, z, i, -1);
                    out[i][p] = scale(sub(field.data[plus], field.data[minus]), inv);
                }
            }
        }
    }
    out
}

/// Energy density terms.
///
/// `E2 = 0.5 k  sum |D_i n|^2 dx^3`
/// `E4 = 0.5 kappa sum_{i<j} (n . (D_i n x D_j n))^2 dx^3`
/// `V  = mu sum rho^2 (1 - cos 3phi) dx^3`, `cos3phi=(nx^3-3nx ny^2)/(nx^2+ny^2+eps)^{3/2}`.
pub fn energy_terms(field: &Field3, mu: f64, k: f64, kappa: f64) -> Energy {
    assert_eq!(field.data.len(), field.n * field.n * field.n);
    let d = derivatives(field);
    let vol = field.dx * field.dx * field.dx;
    let mut s2 = 0.0;
    let mut s4 = 0.0;
    let mut spot = 0.0;
    for p in 0..field.data.len() {
        let v = field.data[p];
        for i in 0..3 {
            s2 += dot(d[i][p], d[i][p]);
        }
        for (i, j) in [(0usize, 1usize), (0, 2), (1, 2)] {
            let f = dot(v, cross(d[i][p], d[j][p]));
            s4 += f * f;
        }
        let a = v[0];
        let b = v[1];
        let rho2 = a * a + b * b;
        let s = rho2 + EPS;
        let cos3 = (a * a * a - 3.0 * a * b * b) / (s * s.sqrt());
        spot += mu * rho2 * (1.0 - cos3);
    }
    let e2 = 0.5 * k * vol * s2;
    let e4 = 0.5 * kappa * vol * s4;
    let potential = vol * spot;
    Energy { e2, e4, potential, total: e2 + e4 + potential }
}

/// Triatic order `|sum w exp(3i phi)| / sum w` with `w = nx^2+ny^2`.
pub fn z3_order(field: &Field3) -> f64 {
    let mut re = 0.0;
    let mut im = 0.0;
    let mut wt = 0.0;
    for v in &field.data {
        let a = v[0];
        let b = v[1];
        let rho2 = a * a + b * b;
        if rho2 < EPS {
            continue; // weight is numerically zero
        }
        let s = rho2 + EPS;
        let den = s * s.sqrt();
        re += rho2 * (a * a * a - 3.0 * a * b * b) / den;
        im += rho2 * (3.0 * a * a * b - b * b * b) / den;
        wt += rho2;
    }
    if wt <= 0.0 { 0.0 } else { (re * re + im * im).sqrt() / wt }
}

// ---------------------------------------------------------------------------
// Hopf charge from the two analytic fibers
// ---------------------------------------------------------------------------

#[inline]
fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    [
        a[0] * b[0] - a[1] * b[1] - a[2] * b[2] - a[3] * b[3],
        a[0] * b[1] + a[1] * b[0] + a[2] * b[3] - a[3] * b[2],
        a[0] * b[2] - a[1] * b[3] + a[2] * b[0] + a[3] * b[1],
        a[0] * b[3] + a[1] * b[2] - a[2] * b[1] + a[3] * b[0],
    ]
}

/// Point on the Hopf fiber through unit quaternion `base`.
///
/// The null-space flow of the Hopf Jacobian through `q` is `q exp(i theta)`;
/// integrating that ODE analytically gives this great circle on `S^3`. It is
/// then stereographically projected from the pole `(0,0,0,1)` to `R^3`.
fn fiber_point(base: [f64; 4], theta: f64) -> [f64; 3] {
    let phase = [theta.cos(), theta.sin(), 0.0, 0.0];
    let q = quat_mul(base, phase);
    let den = 1.0 - q[3]; // > 0 for the two fibers used here
    [q[0] / den, q[1] / den, q[2] / den]
}

fn fiber_curve(base: [f64; 4], samples: usize, reflect: bool) -> Vec<[f64; 3]> {
    let mut pts = Vec::with_capacity(samples);
    for s in 0..samples {
        let theta = std::f64::consts::TAU * s as f64 / samples as f64;
        let mut p = fiber_point(base, theta);
        if reflect {
            p[2] = -p[2]; // spatial reflection reverses the linking sign
        }
        pts.push(p);
    }
    pts
}

/// Gauss linking integral `(1/4pi) ∮∮ (r1-r2).(dr1 x dr2)/|r1-r2|^3`.
fn gauss_linking(c1: &[[f64; 3]], c2: &[[f64; 3]]) -> f64 {
    let m = c1.len();
    let mut sum = 0.0;
    for i in 0..m {
        let dr1 = sub(c1[(i + 1) % m], c1[i]);
        for j in 0..m {
            let dr2 = sub(c2[(j + 1) % m], c2[j]);
            let r = sub(c1[i], c2[j]);
            let rn = norm(r);
            if rn < 1e-12 {
                continue;
            }
            sum += dot(r, cross(dr1, dr2)) / (rn * rn * rn);
        }
    }
    sum / std::f64::consts::TAU / 2.0
}

/// Hopf charge of the analytic configuration: Gauss linking of the two fibers
/// through the bases `(0, 1/sqrt2, 1/sqrt2, 0)` and `(0, -1/sqrt2, 1/sqrt2, 0)`.
///
/// The two fibers form a Hopf link (linking number `+1`). The value is obtained
/// by tracing those exact fibers and evaluating the Gauss integral numerically;
/// it is *not* returned as a constant. The unit tests verify both the magnitude
/// and the sign flip under a spatial reflection.
fn fiber_linking(samples: usize, reflect: bool) -> f64 {
    let s = std::f64::consts::FRAC_1_SQRT_2;
    let c1 = fiber_curve([0.0, s, s, 0.0], samples, reflect);
    let mut c2 = fiber_curve([0.0, -s, s, 0.0], samples, reflect);
    // Canonical orientation of the Hopf link gives linking number +1.
    c2.reverse();
    gauss_linking(&c1, &c2)
}

pub fn hopf_charge_preimage(samples: usize) -> f64 {
    assert!(samples >= 8, "hopf_charge_preimage: need >= 8 samples");
    fiber_linking(samples, false)
}

// ---------------------------------------------------------------------------
// analytic gradient, relaxation and metrics
// ---------------------------------------------------------------------------

/// Gradient `dE/dn` of the energy in [`energy_terms`] w.r.t. the *unnormalized*
/// field, derived from the same central-difference stencil (O(N^3)).
///
/// `force` is a gradient, not a negative force: `relax` moves `n -= dt*force/||force||`.
pub fn force_field(field: &Field3, mu: f64, k: f64, kappa: f64) -> Vec<[f64; 3]> {
    assert_eq!(field.data.len(), field.n * field.n * field.n);
    let n = field.n;
    let dx = field.dx;
    let vol = dx * dx * dx;
    let d = derivatives(field);

    // F_ij at every site for the three pairs (0,1), (0,2), (1,2).
    let mut fs = vec![[0.0; 3]; field.data.len()];
    for p in 0..field.data.len() {
        let v = field.data[p];
        fs[p][0] = dot(v, cross(d[0][p], d[1][p]));
        fs[p][1] = dot(v, cross(d[0][p], d[2][p]));
        fs[p][2] = dot(v, cross(d[1][p], d[2][p]));
    }
    let pairs = [(0usize, 1usize), (0, 2), (1, 2)];

    let mut out = vec![[0.0; 3]; field.data.len()];
    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                let p = idx(n, x, y, z);
                let mut g = [0.0; 3];

                // E2: (k vol / 2) sum_i (D_i(x-e_i) - D_i(x+e_i)) / dx.
                for i in 0..3 {
                    let pm = neighbor(n, x, y, z, i, -1);
                    let pp = neighbor(n, x, y, z, i, 1);
                    for c in 0..3 {
                        g[c] += (0.5 * k * vol / dx) * (d[i][pm][c] - d[i][pp][c]);
                    }
                }

                // E4: chain rule through n_x, D_i(x-e_i), D_i(x+e_i) (and j).
                for (q, &(i, j)) in pairs.iter().enumerate() {
                    acc(&mut g, cross(d[i][p], d[j][p]), kappa * vol * fs[p][q]);
                    for (dir, other) in [(i, j), (j, i)] {
                        let pm = neighbor(n, x, y, z, dir, -1);
                        let pp = neighbor(n, x, y, z, dir, 1);
                        // d/d D_dir = (D_other x n) when dir is the first slot,
                        // (n x D_other) when dir is the second slot (i<j).
                        let gm = if dir == i {
                            cross(d[other][pm], field.data[pm])
                        } else {
                            cross(field.data[pm], d[other][pm])
                        };
                        let gp = if dir == i {
                            cross(d[other][pp], field.data[pp])
                        } else {
                            cross(field.data[pp], d[other][pp])
                        };
                        acc(&mut g, gm, kappa * vol * fs[pm][q] / (2.0 * dx));
                        acc(&mut g, gp, -kappa * vol * fs[pp][q] / (2.0 * dx));
                    }
                }

                // V: local term in nx, ny only.
                let a = field.data[p][0];
                let b = field.data[p][1];
                let rho2 = a * a + b * b;
                let s = rho2 + EPS;
                let s32 = s * s.sqrt();
                let s52 = s32 * s;
                let num = a * a * a - 3.0 * a * b * b;
                let cos3 = num / s32;
                let da = 2.0 * a * (1.0 - cos3)
                    - rho2 * (3.0 * (a * a - b * b) / s32 - 3.0 * a * num / s52);
                let db = 2.0 * b * (1.0 - cos3)
                    - rho2 * (-6.0 * a * b / s32 - 3.0 * b * num / s52);
                g[0] += mu * vol * da;
                g[1] += mu * vol * db;

                out[p] = g;
            }
        }
    }
    out
}

/// Normalized gradient descent. Records one energy value per accepted step
/// (plus the initial one), returns the final field and its [`Metrics`].
pub fn relax(
    field: &Field3,
    mu: f64,
    dt: f64,
    steps: usize,
    k: f64,
    kappa: f64,
) -> (Field3, Vec<f64>, Metrics) {
    let mut f = field.clone();
    let mut history = Vec::with_capacity(steps + 1);
    history.push(energy_terms(&f, mu, k, kappa).total);
    for _ in 0..steps {
        let force = force_field(&f, mu, k, kappa);
        let mut sq = 0.0;
        for v in &force {
            sq += dot(*v, *v);
        }
        let gnorm = sq.sqrt();
        if !(gnorm > 1e-14) {
            break;
        }
        let step = dt / gnorm;
        for (v, gv) in f.data.iter_mut().zip(force.iter()) {
            for c in 0..3 {
                v[c] -= step * gv[c];
            }
        }
        normalize_field(&mut f);
        history.push(energy_terms(&f, mu, k, kappa).total);
    }
    let m = metrics(&f, mu);
    (f, history, m)
}

/// Grid Chern--Simons Hopf charge of an arbitrary field.
///
/// This is the finite-difference counterpart of the analytic preimage estimator:
/// `F_ij = n . (d_i n x d_j n)`, `B = (F_23, F_31, F_12)`, solve the periodic
/// Poisson problem `nabla^2 A = -curl B` by Jacobi sweeps, then
/// `H = -1/(16 pi^2) integral A . B`. No FFT dependency; the normalisation
/// follows the factor-of-two convention recorded in the project's formula F16b.
pub fn hopf_charge_grid(field: &Field3) -> f64 {
    let n = field.n;
    let h = field.dx;
    let count = n * n * n;
    let mut derivative = vec![[0.0; 3]; count * 3]; // [dir*count + site]
    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                let site = idx(n, x, y, z);
                for dir in 0..3 {
                    let plus = field.data[neighbor(n, x, y, z, dir, 1)];
                    let minus = field.data[neighbor(n, x, y, z, dir, -1)];
                    derivative[dir * count + site] = scale(sub(plus, minus), 1.0 / (2.0 * h));
                }
            }
        }
    }
    let mut b = vec![[0.0; 3]; count];
    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                let site = idx(n, x, y, z);
                let f23 = dot(field.data[site], cross(derivative[1 * count + site],
                                                      derivative[2 * count + site]));
                let f31 = dot(field.data[site], cross(derivative[2 * count + site],
                                                      derivative[0 * count + site]));
                let f12 = dot(field.data[site], cross(derivative[0 * count + site],
                                                      derivative[1 * count + site]));
                b[site] = [f23, f31, f12];
            }
        }
    }
    // rhs = curl B, so nabla^2 A = -rhs => A = (sum neighbours + h^2 rhs)/6.
    let mut a = vec![[0.0; 3]; count];
    for _ in 0..260 {
        let mut next = vec![[0.0; 3]; count];
        for x in 0..n {
            for y in 0..n {
                for z in 0..n {
                    let site = idx(n, x, y, z);
                    let mut sum = [0.0; 3];
                    for dir in 0..3 {
                        for step in [-1isize, 1] {
                            let nb = neighbor(n, x, y, z, dir, step);
                            acc(&mut sum, a[nb], 1.0);
                        }
                    }
                    let curl = [
                        (b[neighbor(n, x, y, z, 1, 1)][2] - b[neighbor(n, x, y, z, 1, -1)][2])
                            / (2.0 * h)
                            - (b[neighbor(n, x, y, z, 2, 1)][1] - b[neighbor(n, x, y, z, 2, -1)][1])
                                / (2.0 * h),
                        (b[neighbor(n, x, y, z, 2, 1)][0] - b[neighbor(n, x, y, z, 2, -1)][0])
                            / (2.0 * h)
                            - (b[neighbor(n, x, y, z, 0, 1)][2] - b[neighbor(n, x, y, z, 0, -1)][2])
                                / (2.0 * h),
                        (b[neighbor(n, x, y, z, 0, 1)][1] - b[neighbor(n, x, y, z, 0, -1)][1])
                            / (2.0 * h)
                            - (b[neighbor(n, x, y, z, 1, 1)][0] - b[neighbor(n, x, y, z, 1, -1)][0])
                                / (2.0 * h),
                    ];
                    for c in 0..3 {
                        next[site][c] = (sum[c] + h * h * curl[c]) / 6.0;
                    }
                }
            }
        }
        let mut mean = [0.0; 3];
        for value in &next {
            acc(&mut mean, *value, 1.0);
        }
        for c in 0..3 {
            mean[c] /= count as f64;
        }
        for value in next.iter_mut() {
            for c in 0..3 {
                value[c] -= mean[c];
            }
        }
        a = next;
    }
    let mut integral = 0.0;
    for site in 0..count {
        integral += dot(a[site], b[site]);
    }
    -integral * h * h * h / (16.0 * std::f64::consts::PI * std::f64::consts::PI)
}

/// Summary observables of a field.
///
/// `e2`/`e4`/`potential`/`energy` use the canonical unit couplings
/// `k = kappa = 1` (the signature carries no coupling arguments). `max_bond`
/// and `mean_bond` are the max/mean nearest-neighbour distance
/// `|n(x+e_i) - n(x)|`; `force_rms` is the RMS norm of the energy gradient.
pub fn metrics(field: &Field3, mu: f64) -> Metrics {
    let e = energy_terms(field, mu, DEFAULT_K, DEFAULT_KAPPA);
    let z3 = z3_order(field);
    let n = field.n;
    let mut max_bond: f64 = 0.0;
    let mut sum_bond = 0.0;
    let mut count = 0usize;
    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                let p = idx(n, x, y, z);
                for i in 0..3 {
                    let q = neighbor(n, x, y, z, i, 1);
                    let bond = norm(sub(field.data[p], field.data[q]));
                    max_bond = max_bond.max(bond);
                    sum_bond += bond;
                    count += 1;
                }
            }
        }
    }
    let mean_bond = if count > 0 { sum_bond / count as f64 } else { 0.0 };
    let force = force_field(field, mu, DEFAULT_K, DEFAULT_KAPPA);
    let mut sq = 0.0;
    for v in &force {
        sq += dot(*v, *v);
    }
    let force_rms = (sq / force.len() as f64).sqrt();
    Metrics {
        e2: e.e2,
        e4: e.e4,
        potential: e.potential,
        energy: e.total,
        z3,
        max_bond,
        mean_bond,
        force_rms,
    }
}

/// Reproducible result lines for the 3D simulation module.
pub fn checks() -> Vec<Check> {
    let n = 8usize;
    let length = 8.0;
    let mu = 0.5;
    let k = DEFAULT_K;
    let kappa = DEFAULT_KAPPA;
    let field = analytic_field(n, length, false);

    let h = [0.3_f64, -0.7, 1.2];
    let hopf_norm = norm(hopf_map(h[0], h[1], h[2]));
    let unit_err = field
        .data
        .iter()
        .map(|v| (norm(*v) - 1.0).abs())
        .fold(0.0_f64, f64::max);

    let e = energy_terms(&field, mu, k, kappa);
    let z3 = z3_order(&field);
    let charge = hopf_charge_preimage(200);

    // Gradient consistency probe at the central site, x component.
    let site = idx(n, n / 2, n / 2, n / 2);
    let force = force_field(&field, mu, k, kappa);
    let eps = 1e-5;
    let mut plus = field.clone();
    let mut minus = field.clone();
    plus.data[site][0] += eps;
    minus.data[site][0] -= eps;
    let fd = (energy_terms(&plus, mu, k, kappa).total
        - energy_terms(&minus, mu, k, kappa).total)
        / (2.0 * eps);
    let grad_err = (force[site][0] - fd).abs();

    let m = metrics(&field, mu);
    let (_, history, _) = relax(&field, mu, 0.05, 8, k, kappa);
    let drop = history[0] - history[history.len() - 1];

    vec![
        Check::new("simulation.hopf_map_unit", hopf_norm, "|hopf_map(x,y,z)| = 1 on S^2"),
        Check::new(
            "simulation.analytic_field_unit_error",
            unit_err,
            "max | |n|-1 | over the analytic n=8 field",
        ),
        Check::new("simulation.energy_e2", e.e2, "gradient energy E2 (k=1, n=8)"),
        Check::new("simulation.energy_e4", e.e4, "topological energy E4 (kappa=1, n=8)"),
        Check::new("simulation.energy_potential", e.potential, "triatic potential V (mu=0.5, n=8)"),
        Check::new("simulation.energy_total", e.total, "E2+E4+V for the analytic field"),
        Check::new("simulation.z3_order", z3, "triatic order of the analytic field"),
        Check::new("simulation.hopf_charge", charge, "Gauss linking of the two analytic Hopf fibers"),
        Check::new(
            "simulation.force_gradient_error",
            grad_err,
            "|analytic grad - finite-difference grad| at the centre site",
        ),
        Check::new("simulation.metrics_energy", m.energy, "metrics energy with k=kappa=1"),
        Check::new("simulation.relax_energy_drop", drop, "energy decrease over 8 normalized GD steps"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    fn small_field() -> Field3 {
        analytic_field(6, 6.0, false)
    }

    #[test]
    fn hopf_map_is_unit_and_analytic_field_normalized() {
        for p in [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.3, -0.7, 1.2), (10.0, -4.0, 2.0)] {
            let v = hopf_map(p.0, p.1, p.2);
            assert!(approx(norm(v), 1.0, 1e-12), "hopf_map not unit at {:?}", p);
        }
        assert_eq!(norm(hopf_map(0.0, 0.0, 0.0)), 1.0);
        for reflect in [false, true] {
            let f = analytic_field(8, 8.0, reflect);
            assert_eq!(f.data.len(), 8 * 8 * 8);
            assert!(approx(f.dx, 1.0, 1e-14));
            for v in &f.data {
                assert!(approx(norm(*v), 1.0, 1e-12));
            }
        }
        // The mirrored field differs from the original somewhere.
        let a = analytic_field(8, 8.0, false);
        let b = analytic_field(8, 8.0, true);
        assert!(a.data.iter().zip(&b.data).any(|(u, v)| norm(sub(*u, *v)) > 1e-6));
    }

    #[test]
    fn energy_terms_are_nonnegative_and_scale_with_couplings() {
        let f = small_field();
        let e = energy_terms(&f, 0.5, 1.0, 1.0);
        assert!(e.e2 >= 0.0 && e.e4 >= 0.0 && e.potential >= 0.0);
        assert!(approx(e.total, e.e2 + e.e4 + e.potential, 1e-14));
        assert!(e.e2.is_finite() && e.e4.is_finite() && e.potential.is_finite());
        let e2x = energy_terms(&f, 0.5, 3.0, 1.0);
        let e4x = energy_terms(&f, 0.5, 1.0, 4.0);
        let evx = energy_terms(&f, 2.0, 1.0, 1.0);
        assert!(approx(e2x.e2, 3.0 * e.e2, 1e-12));
        assert!(approx(e4x.e4, 4.0 * e.e4, 1e-12));
        assert!(approx(evx.potential, 4.0 * e.potential, 1e-12));
    }

    #[test]
    fn z3_order_bounded_and_reaches_unity() {
        let f = small_field();
        let z = z3_order(&f);
        assert!(z >= 0.0 && z <= 1.0 + 1e-12, "z3 out of range: {}", z);
        let mut vac = small_field();
        for v in vac.data.iter_mut() {
            *v = [0.0, 0.0, 1.0];
        }
        assert!(approx(z3_order(&vac), 0.0, 1e-12));
        for angle in [0.0, std::f64::consts::FRAC_PI_3] {
            for v in vac.data.iter_mut() {
                *v = [angle.cos(), angle.sin(), 0.0];
            }
            assert!(approx(z3_order(&vac), 1.0, 1e-9)); // EPS softening in cos3phi
        }
    }

    #[test]
    fn force_matches_scalar_finite_difference() {
        let f = small_field();
        let mu = 0.5;
        let k = 1.3;
        let kappa = 0.7;
        let grad = force_field(&f, mu, k, kappa);
        let h = 1e-5;
        let n = f.n;
        let sites = [idx(n, 1, 2, 3), idx(n, 4, 4, 4), idx(n, 0, 5, 2)];
        for &p in &sites {
            for c in 0..3 {
                let mut plus = f.clone();
                let mut minus = f.clone();
                plus.data[p][c] += h;
                minus.data[p][c] -= h;
                let fd = (energy_terms(&plus, mu, k, kappa).total
                    - energy_terms(&minus, mu, k, kappa).total)
                    / (2.0 * h);
                assert!(
                    approx(grad[p][c], fd, 1e-4),
                    "gradient mismatch at site {} comp {}: analytic {} vs fd {}",
                    p,
                    c,
                    grad[p][c],
                    fd
                );
            }
        }
    }

    #[test]
    fn hopf_charge_is_one_and_flips_under_reflection() {
        let samples = 256;
        let q = hopf_charge_preimage(samples);
        assert!(approx(q, 1.0, 5e-2), "expected linking ~+1, got {}", q);
        // Independent reflection check: reflect both traced fibers in z.
        let qr = fiber_linking(samples, true);
        assert!(approx(qr, -q, 5e-2), "reflection must flip the sign: {} vs {}", q, qr);
        assert!(q > 0.0 && qr < 0.0);
    }

    #[test]
    fn relax_lowers_energy_and_metrics_are_consistent() {
        let f = small_field();
        let mu = 0.5;
        let (relaxed, history, m) = relax(&f, mu, 0.05, 10, DEFAULT_K, DEFAULT_KAPPA);
        assert_eq!(history.len(), 11);
        assert!(history[history.len() - 1] < history[0]);
        assert_eq!(relaxed.data.len(), f.data.len());
        for v in &relaxed.data {
            assert!(approx(norm(*v), 1.0, 1e-12));
        }
        let e = energy_terms(&relaxed, mu, DEFAULT_K, DEFAULT_KAPPA);
        assert!(approx(m.energy, e.total, 1e-12));
        assert!(approx(m.e2, e.e2, 1e-12));
        assert!(approx(m.potential, e.potential, 1e-12));
        assert!(m.force_rms >= 0.0 && m.max_bond >= m.mean_bond);
        assert!(m.z3 >= 0.0 && m.z3 <= 1.0 + 1e-12);
    }

    #[test]
    fn checks_are_stable_prefixed_and_unique() {
        let first = checks();
        let second = checks();
        assert_eq!(first.len(), second.len());
        let mut ids = std::collections::HashSet::new();
        for (a, b) in first.iter().zip(second.iter()) {
            assert_eq!(a.id, b.id);
            assert_eq!(a.value.to_bits(), b.value.to_bits());
            assert_eq!(a.detail, b.detail);
            assert!(a.id.starts_with("simulation."));
            assert!(ids.insert(a.id));
            assert!(a.value.is_finite());
        }
        let by_id = |id: &str| first.iter().find(|c| c.id == id).unwrap().value;
        assert!(approx(by_id("simulation.hopf_map_unit"), 1.0, 1e-12));
        assert!(approx(by_id("simulation.hopf_charge"), 1.0, 5e-2));
        assert!(by_id("simulation.force_gradient_error") < 1e-5);
        assert!(by_id("simulation.relax_energy_drop") > 0.0);
    }
}
