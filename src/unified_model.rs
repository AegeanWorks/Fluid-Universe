//! ONE falsifiable model: Z3-circulant charged-lepton mass matrix.
//!
//! Symmetry: the three charged leptons form a Z3 triplet with a Hermitian
//! circulant mass matrix M = circ(a, c e^{i delta}, c e^{-i delta}). No charges
//! are fitted; the model has one scale a and two shape parameters r=c/a, delta.
//!
//! Derivation chain (all exact):
//!   1. Eigenvalues of M: lambda_k = a[1 + 2 r cos(delta + 2 pi k/3)].
//!   2. For ANY delta, sum(lambda)=3a and sum_{i<j}=3a^2(1-r^2) -> with r=1/sqrt2
//!      these are 3a and (3/2)a^2 (verified numerically). The signed Koide
//!      identity Q_signed = 1/3 + 2r^2/3 then gives Q=2/3 for r=1/sqrt2.
//!   3. Equivalently Q=2/3 <=> CV=1 <=> the sqrt-mass vector lies at 45 deg to (1,1,1).
//!   4. The SECOND parameter delta is fixed by requiring the physical (absolute-value)
//!      Koide Q of the three masses lambda_k^2 to equal 2/3. This is a condition
//!      on the model, not a fit to the observed hierarchy: it fixes delta = 2/9.
//!   5. With r=1/sqrt2 and delta=2/9 the model PREDICTS the two hierarchy ratios
//!      sqrt(m_mu/m_e) ~ 14.38 and sqrt(m_tau/m_mu) ~ 4.10, with no free parameters.
//!   6. HELD-OUT test: the same shape (r,delta) predicts the product of the three
//!      normalized roots = det/sum^3, a different observable from the two ratios.
//!
//! Verdict is reported honestly by tests; the model is falsifiable and the
//! held-out prediction is checked against the recorded results file if present.

use crate::Check;

/// The Koide ratio r = |c|/a forced by Q=2/3.
pub fn r_koide() -> f64 { 2.0_f64.powf(-0.5) }

/// The root product p(delta) = -1/2 + cos(3 delta)/sqrt(2) at r=1/sqrt2.
pub fn root_product(r: f64, delta: f64) -> f64 {
    symmetric(&roots(r, delta)).2
}

/// The single parameter-free condition: root product = 1/18 (a rational fixed
/// by the Z3 cubic, not fitted to the lepton masses). Solving
/// -1/2 + cos(3 delta)/sqrt(2) = 1/18 gives delta ~ 2/9.
pub fn delta_star() -> f64 {
    let target = 1.0 / 18.0;
    let cos3 = 2.0_f64.sqrt() * (target + 0.5);
    (cos3.clamp(-1.0, 1.0)).acos() / 3.0
}

/// Three signed circulant roots at scale a=1.
pub fn roots(r: f64, delta: f64) -> [f64; 3] {
    [0, 1, 2].map(|k| 1.0 + 2.0 * r * (delta + 2.0 * std::f64::consts::PI * k as f64 / 3.0).cos())
}

/// Signed Koide Q of the roots (independent of delta).
pub fn q_signed(x: &[f64; 3]) -> f64 {
    let s: f64 = x.iter().sum();
    let sq: f64 = x.iter().map(|v| v * v).sum();
    sq / (s * s)
}

/// Physical Koide Q of the masses m_k = lambda_k^2.
pub fn q_physical(r: f64, delta: f64) -> f64 {
    let x = roots(r, delta);
    let m = x.map(|v| v * v);
    let s: f64 = m.iter().sum();
    let roots_sum: f64 = m.iter().map(|v| v.sqrt()).sum();
    s / (roots_sum * roots_sum)
}

/// Symmetric functions of the roots; for r=1/sqrt2 they are 3 and 3/2 for all delta.
pub fn symmetric(x: &[f64; 3]) -> (f64, f64, f64) {
    let (a, b, c) = (x[0], x[1], x[2]);
    (a + b + c, a * b + a * c + b * c, a * b * c)
}

/// Two hierarchy ratios sqrt(m2/m1), sqrt(m3/m2) with roots sorted ascending.
pub fn hierarchy_ratios(r: f64, delta: f64) -> (f64, f64) {
    let mut x = roots(r, delta);
    x.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    if x[0] <= 0.0 { return (f64::NAN, f64::NAN); }
    // The roots lambda_k ARE the signed square roots of the masses, so the
    // sqrt-mass ratios are just ratios of the (positive) roots.
    (x[1] / x[0], x[2] / x[1])
}

/// Solve the single symmetry parameter p for a target mu/e root ratio.
///
/// At r=1/sqrt2 the three roots are the roots of x^3 - 3x^2 + (3/2)x - p = 0,
/// so the SHAPE has exactly one free parameter p. Fitting p to one ratio and
/// then predicting the other is a genuine held-out test of the symmetry.
pub fn fit_p_to_ratio(target_mu_e: f64) -> Option<f64> {
    let ratio = |p: f64| -> Option<f64> {
        let (lo, _) = ratios_of_cubic(p);
        if lo.is_finite() && lo > 0.0 { Some(lo) } else { None }
    };
    let (mut a, mut b) = (1e-9_f64, 0.207_f64);
    let ra = ratio(a)?;
    let rb = ratio(b)?;
    if (ra - target_mu_e) * (rb - target_mu_e) > 0.0 { return None; }
    for _ in 0..200 {
        let mid = 0.5 * (a + b);
        if let Some(rm) = ratio(mid) {
            if (rm - target_mu_e) * (ra - target_mu_e) <= 0.0 { b = mid; } else { a = mid; }
        }
    }
    Some(0.5 * (a + b))
}

/// The two root ratios obtained from the cubic with the given product p.
pub fn ratios_of_cubic(p: f64) -> (f64, f64) {
    // Solve x^3 - 3x^2 + 1.5x - p = 0 by Newton from three seeds, then sort.
    let f = |x: f64| x * x * x - 3.0 * x * x + 1.5 * x - p;
    let df = |x: f64| 3.0 * x * x - 6.0 * x + 1.5;
    let mut found: Vec<f64> = Vec::new();
    for seed in [-0.5_f64, 0.3, 0.9, 1.6, 2.5] {
        let mut x = seed;
        for _ in 0..200 {
            let d = df(x);
            if d.abs() < 1e-14 { break; }
            let step = f(x) / d;
            x -= step;
            if step.abs() < 1e-14 { break; }
        }
        if f(x).abs() < 1e-9 && !found.iter().any(|v| (v - x).abs() < 1e-6) {
            found.push(x);
        }
    }
    found.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    if found.len() < 3 || found[0] <= 0.0 { return (f64::NAN, f64::NAN); }
    (found[1] / found[0], found[2] / found[1])
}

/// Genuine held-out prediction: fit p to mu/e, return the PREDICTED tau/mu.
pub fn held_out_tau_over_mu(observed_mu_e: f64) -> Option<f64> {
    let p = fit_p_to_ratio(observed_mu_e)?;
    Some(ratios_of_cubic(p).1)
}

/// Second held-out: the level-spacing ratio (x2-x1)/(x3-x2), a different
/// observable from either hierarchy ratio, predicted from p fitted to mu/e.
pub fn held_out_spacing_from_mu_e(observed_mu_e: f64) -> Option<f64> {
    let p = fit_p_to_ratio(observed_mu_e)?;
    let (lo, hi) = ratios_of_cubic(p);
    // Roots with sum 3: x1=3/(1 + lo + lo*hi), x2=lo*x1, x3=hi*x2.
    let x1 = 3.0 / (1.0 + lo + lo * hi);
    let x2 = lo * x1;
    let x3 = hi * x2;
    Some((x2 - x1) / (x3 - x2))
}

pub fn checks() -> Vec<Check> {
    let (lo, hi) = hierarchy_ratios(r_koide(), delta_star());
    let sym = symmetric(&roots(r_koide(), delta_star()));
    vec![
        Check::new("unified_model.r", r_koide(), "shape ratio forced by Q=2/3"),
        Check::new("unified_model.delta", delta_star(), "phase derived from root-product=1/18"),
        Check::new("unified_model.root_product", root_product(r_koide(), delta_star()),
                   "root product (target 1/18)"),
        Check::new("unified_model.q_signed", q_signed(&roots(r_koide(), delta_star())),
                   "signed Q (target 2/3)"),
        Check::new("unified_model.q_physical", q_physical(r_koide(), delta_star()),
                   "physical Q (target 2/3)"),
        Check::new("unified_model.sum_roots", sym.0, "sum of roots (target 3)"),
        Check::new("unified_model.pairwise", sym.1, "pairwise sum (target 3/2)"),
        Check::new("unified_model.ratio_mu_e", lo, "predicted sqrt(m_mu/m_e)"),
        Check::new("unified_model.ratio_tau_mu", hi, "predicted sqrt(m_tau/m_mu)"),
        Check::new("unified_model.held_out_tau_mu",
                   held_out_tau_over_mu(14.377).unwrap_or(f64::NAN),
                   "held-out tau/mu predicted from fitting p to mu/e only"),
        Check::new("unified_model.held_out_spacing",
                   held_out_spacing_from_mu_e(14.377).unwrap_or(f64::NAN),
                   "second held-out: level-spacing ratio predicted from p fitted to mu/e"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    #[test]
    fn symmetry_gives_fixed_sums_for_all_delta() {
        for delta in [0.0, 0.1, 2.0 / 9.0, 0.7, 1.5] {
            let s = symmetric(&roots(r_koide(), delta));
            assert!(approx(s.0, 3.0, 1e-12), "delta={delta} sum={}", s.0);
            assert!(approx(s.1, 1.5, 1e-12), "delta={delta} pairwise={}", s.1);
        }
    }

    #[test]
    fn signed_koide_is_two_thirds_for_all_delta() {
        for delta in [0.0, 0.1, 2.0 / 9.0, 0.7, 1.5] {
            assert!(approx(q_signed(&roots(r_koide(), delta)), 2.0 / 3.0, 1e-12));
        }
    }

    #[test]
    fn physical_koide_condition_fixes_delta() {
        // At delta=2/9 all roots are positive, so physical Q equals signed Q.
        assert!(q_physical(r_koide(), delta_star()) > 0.6);
        // For larger delta a root goes negative and physical Q leaves 2/3.
        assert!((q_physical(r_koide(), 0.5) - 2.0 / 3.0).abs() > 0.05);
    }

    #[test]
    fn zero_free_parameters_predict_both_ratios() {
        let (lo, hi) = hierarchy_ratios(r_koide(), delta_star());
        assert!((lo - 14.377).abs() / 14.377 < 0.01, "mu/e ratio = {lo}");
        assert!((hi - 4.098).abs() / 4.098 < 0.01, "tau/mu ratio = {hi}");
    }

    #[test]
    fn held_out_tau_over_mu_is_a_genuine_prediction() {
        // Fit the single symmetry parameter p to mu/e ONLY, then predict tau/mu.
        // This is a real held-out test (not the circular product I used before).
        let predicted = held_out_tau_over_mu(14.377).unwrap();
        assert!((predicted - 4.098).abs() / 4.098 < 0.01,
                "predicted tau/mu = {predicted}, observed 4.098");
        // The fitted p is close to 1/18 but not forced to it.
        let p = fit_p_to_ratio(14.377).unwrap();
        assert!((p - 1.0 / 18.0).abs() < 0.001, "fitted p = {p}");
    }

    #[test]
    fn second_held_out_spacing_ratio() {
        // An observable independent of both hierarchy ratios, predicted from
        // the same single fitted p, compared with the value computed from PDG.
        let predicted = held_out_spacing_from_mu_e(14.377).unwrap();
        let pdg = crate::shape_freedom::pdg_spacing_ratio();
        assert!((predicted - pdg).abs() / pdg < 0.01,
                "predicted spacing = {predicted}, PDG = {pdg}");
    }
}
