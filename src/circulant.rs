//! Exact algebra of the Hermitian 3x3 circulant mass matrix.
//!
//! Port of `theory/circulant.py`. With `M = circ(a, c*exp(i*delta),
//! c*exp(-i*delta))` the signed eigenvalues are
//!
//! ```text
//! lam_k = a + 2*c*cos(delta + 2*pi*k/3),  k = 0, 1, 2,
//! ```
//!
//! and the identity `Q_signed = 1/3 + 2*c^2/(3*a^2)` holds for those signed
//! roots. The physical Koide ratio uses `|lam_k|` and agrees only when every
//! root is non-negative.

use std::f64::consts::TAU;

/// Signed eigenvalues of the Hermitian circulant in the `k = 0, 1, 2` order.
pub fn eigenvalues(a: f64, c: f64, delta: f64) -> [f64; 3] {
    std::array::from_fn(|k| a + 2.0 * c * (delta + TAU * (k as f64) / 3.0).cos())
}

/// Q for signed square roots: `sum(x^2) / (sum(x))^2`.
pub fn q_signed(x: &[f64]) -> f64 {
    let sum_sq: f64 = x.iter().map(|v| v * v).sum();
    let sum: f64 = x.iter().sum();
    sum_sq / (sum * sum)
}

/// Physical Koide ratio for positive masses: `sum(m) / (sum(sqrt(m)))^2`.
pub fn q_masses(m: &[f64]) -> f64 {
    let sum: f64 = m.iter().sum();
    let sum_sqrt: f64 = m.iter().map(|v| v.sqrt()).sum();
    sum / (sum_sqrt * sum_sqrt)
}

/// Physical Q when `x` are signed square roots: `sum(x^2) / (sum(|x|))^2`.
pub fn q_physical_from_roots(x: &[f64]) -> f64 {
    let sum_sq: f64 = x.iter().map(|v| v * v).sum();
    let sum_abs: f64 = x.iter().map(|v| v.abs()).sum();
    sum_sq / (sum_abs * sum_abs)
}

/// The algebraic identity `Q = 1/3 + 2*c^2/(3*a^2)`.
pub fn q_from_ac(a: f64, c: f64) -> f64 {
    1.0 / 3.0 + 2.0 * c * c / (3.0 * a * a)
}

/// Inverse DFT: recover `(a, c, delta)` from three signed roots.
///
/// Since `sum_k x_k * exp(-i*2*pi*k/3) / 3 = c*exp(i*delta)`, the real and
/// imaginary parts of that sum give `c` and `delta` directly. The returned
/// phase is normalised to `[0, 2*pi)`; a vanishing `c` yields `delta = 0`.
/// Near equal roots the phase is ill-conditioned even when it is nonzero.
/// Non-finite roots or unrepresentable intermediate values return an error.
pub fn extract_ac_delta(x: &[f64]) -> Result<(f64, f64, f64), String> {
    if x.len() != 3 {
        return Err("expected exactly three roots".to_string());
    }
    if x.iter().any(|value| !value.is_finite()) {
        return Err("Why: roots must be finite. Fix: provide three finite roots.".to_string());
    }
    let a = x.iter().sum::<f64>() / 3.0;
    if !a.is_finite() {
        return Err("Why: inverse DFT overflowed. Fix: rescale the roots.".to_string());
    }
    if x[0] == x[1] && x[1] == x[2] {
        return Ok((a, 0.0, 0.0));
    }

    // conj(OMEGA)^k = cos(2*pi*k/3) - i*sin(2*pi*k/3).
    let (mut re, mut im) = (0.0, 0.0);
    for (k, &value) in x.iter().enumerate() {
        let phi = TAU * (k as f64) / 3.0;
        re += value * phi.cos();
        im -= value * phi.sin();
    }
    re /= 3.0;
    im /= 3.0;

    let c = re.hypot(im);
    if !c.is_finite() {
        return Err("Why: inverse DFT overflowed. Fix: rescale the roots.".to_string());
    }
    let mut delta = if c == 0.0 {
        0.0
    } else {
        let angle = im.atan2(re);
        if angle < 0.0 { angle + TAU } else { angle }
    };
    if delta >= TAU {
        delta -= TAU;
    }
    Ok((a, c, delta))
}

/// Result of fitting a circulant to three positive masses.
#[derive(Clone, Debug, PartialEq)]
pub struct Fit {
    pub a: f64,
    pub c: f64,
    pub delta: f64,
    pub r: f64,
    pub q: f64,
    pub q_from_ac: f64,
    pub max_abs_residual: f64,
    pub all_roots_positive: bool,
}

/// Fit `(a, c, delta)` to three positive finite masses and report Q and residuals.
/// Returns an error if the calculation cannot be represented by finite values.
pub fn fit_masses(m: &[f64]) -> Result<Fit, String> {
    if m.len() != 3 {
        return Err("expected exactly three masses".to_string());
    }
    if m.iter().any(|&value| !value.is_finite() || value <= 0.0) {
        return Err("Why: masses must be finite and positive. Fix: provide three finite positive masses.".to_string());
    }
    let roots: Vec<f64> = m.iter().map(|&value| value.sqrt()).collect();
    let (a, c, delta) = extract_ac_delta(&roots)?;
    let fitted = eigenvalues(a, c, delta);
    if fitted.iter().any(|value| !value.is_finite()) {
        return Err("Why: fitted roots overflowed. Fix: rescale the masses.".to_string());
    }
    let max_abs_residual = fitted
        .iter()
        .zip(roots.iter())
        .map(|(&f, &r)| (f - r).abs())
        .fold(0.0_f64, f64::max);

    let fit = Fit {
        a,
        c,
        delta,
        r: c / a,
        q: q_masses(m),
        q_from_ac: q_from_ac(a, c),
        max_abs_residual,
        all_roots_positive: fitted.iter().all(|&v| v >= 0.0),
    };
    if [fit.a, fit.c, fit.delta, fit.r, fit.q, fit.q_from_ac, fit.max_abs_residual]
        .iter().any(|value| !value.is_finite())
    {
        return Err("Why: fit arithmetic overflowed or underflowed. Fix: rescale the masses.".to_string());
    }
    Ok(fit)
}

/// Solve `Q = 2/3` for `m_tau` given `m_e` and `m_mu` (MeV).
///
/// The quadratic in `x = sqrt(m_tau)` has two positive roots; the physical
/// branch is the larger one (the smaller root lies below `sqrt(m_mu)`).
pub fn koide_tau_prediction(m_e: f64, m_mu: f64) -> f64 {
    let s = m_e.sqrt() + m_mu.sqrt();
    let b = -4.0 * s;
    let c = 3.0 * (m_e + m_mu) - 2.0 * s * s;
    let disc = b * b - 4.0 * c;
    let half = 0.5 * disc.max(0.0).sqrt();
    let centre = -0.5 * b;
    let larger = (centre + half).max(centre - half);
    if larger > 0.0 { larger * larger } else { 0.0 }
}

/// Reproducible result lines for this module (ids prefixed with `circulant.`).
pub fn checks() -> Vec<crate::Check> {
    use crate::Check;

    let (m_e, m_mu) = (0.51099895_f64, 105.6583755_f64);
    let tau = koide_tau_prediction(m_e, m_mu);
    let koide = q_masses(&[m_e, m_mu, tau]);

    let (a, c, delta) = (2.0_f64, 0.5_f64, 0.7_f64);
    let roots = eigenvalues(a, c, delta);
    let identity_residual = (q_signed(&roots) - q_from_ac(a, c)).abs();
    let roundtrip = extract_ac_delta(&roots).map_or(f64::NAN, |(ea, ec, ed)| {
        (ea - a).abs().max((ec - c).abs()).max((ed - delta).abs())
    });

    let fit = fit_masses(&[m_e, m_mu, tau]).expect("Koide masses must fit");
    let signed_koide = q_physical_from_roots(&roots);

    vec![
        Check::new("circulant.q_from_ac", q_from_ac(1.0, 0.5), "q_from_ac(1, 1/2) = 1/2"),
        Check::new("circulant.q_identity_residual", identity_residual, "|Q_signed - Q(a,c)|"),
        Check::new("circulant.ac_delta_roundtrip", roundtrip, "max inverse-DFT parameter error"),
        Check::new("circulant.q_koide", koide - 2.0 / 3.0, "Q(m_e, m_mu, m_tau) - 2/3"),
        Check::new("circulant.tau_prediction", tau, "predicted m_tau (MeV)"),
        Check::new("circulant.fit_r", fit.r, "c/a of fitted charged leptons"),
        Check::new("circulant.fit_max_residual", fit.max_abs_residual, "max |fitted - sqrt(m)|"),
        Check::new("circulant.fit_q_from_ac", fit.q_from_ac - fit.q, "Q_from_ac - Q of fitted masses"),
        Check::new("circulant.fit_all_positive", if fit.all_roots_positive { 1.0 } else { 0.0 }, "all fitted roots >= 0"),
        Check::new("circulant.q_physical_signed", signed_koide, "Q from |roots| of test circulant"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    /// Deterministic LCG: no RNG crate, identical stream on every platform.
    struct Lcg(u64);
    impl Lcg {
        fn unit(&mut self) -> f64 {
            self.0 = self
                .0
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            ((self.0 >> 11) as f64) / ((1_u64 << 53) as f64)
        }
    }

    #[test]
    fn signed_root_q_identity_over_many_draws() {
        let mut rng = Lcg(0x5eed_1234_5678_9abc);
        for _ in 0..500 {
            let a = 0.5 + 2.0 * rng.unit();
            let c = rng.unit(); // c <= 1 < a/2 is not required for the identity
            let delta = TAU * rng.unit();
            let x = eigenvalues(a, c, delta);
            assert!(approx(q_signed(&x), q_from_ac(a, c), 1e-11));
        }
        // Explicit reference point from the contract.
        assert!(approx(q_from_ac(1.0, 0.5), 0.5, 1e-15));
    }

    #[test]
    fn eigenvalues_and_q_definitions() {
        let x = eigenvalues(1.0, 0.5, 0.0);
        assert!(approx(x[0], 2.0, 1e-14));
        assert!(approx(x[1], 0.5, 1e-14));
        assert!(approx(x[2], 0.5, 1e-14));
        assert!(approx(q_signed(&x), 0.5, 1e-14));
        assert!(approx(q_masses(&[1.0, 1.0, 1.0]), 1.0 / 3.0, 1e-14));

        // Physical ratio ignores signs; the raw signed ratio does not.
        let signed = [-1.0, 2.0, 3.0];
        let positive = [1.0, 2.0, 3.0];
        assert!(approx(q_physical_from_roots(&signed), q_physical_from_roots(&positive), 1e-14));
        assert!(!approx(q_signed(&signed), q_signed(&positive), 1e-6));
    }

    #[test]
    fn inverse_dft_roundtrip_and_fit() {
        let (a, c, delta) = (3.0_f64, 0.75_f64, 1.234_f64);
        let x = eigenvalues(a, c, delta);
        let (a2, c2, delta2) = extract_ac_delta(&x).unwrap();
        assert!(approx(a2, a, 1e-12));
        assert!(approx(c2, c, 1e-12));
        assert!(approx(delta2, delta, 1e-12));

        let m_e = 0.51099895_f64;
        let m_mu = 105.6583755_f64;
        let tau = koide_tau_prediction(m_e, m_mu);
        let fit = fit_masses(&[m_e, m_mu, tau]).unwrap();
        assert!(fit.max_abs_residual < 1e-9, "residual {}", fit.max_abs_residual);
        assert!(fit.all_roots_positive);
        assert!(approx(fit.q, 2.0 / 3.0, 1e-12));
        assert!(approx(fit.q_from_ac, fit.q, 1e-12));
        assert!(approx(fit.r, 1.0 / 2.0_f64.sqrt(), 1e-9));
    }

    #[test]
    fn inverse_phase_and_fit_reconstruct_at_small_scales() {
        let baseline = fit_masses(&[1.0, 4.0, 9.0]).unwrap();
        let tiny = fit_masses(&[1e-40, 4e-40, 9e-40]).unwrap();
        assert!((tiny.delta - baseline.delta).abs() < 1e-12);
        assert!(tiny.max_abs_residual / 3e-20 < 1e-12);
        for root in [1.0, 1e-20, 1e20] {
            let (_, c, phase) = extract_ac_delta(&[root, root, root]).unwrap();
            assert_eq!(c, 0.0);
            assert_eq!(phase, 0.0);
        }
    }

    #[test]
    fn koide_tau_prediction_reference() {
        let tau = koide_tau_prediction(0.51099895, 105.6583755);
        assert!(approx(tau, 1776.93, 1e-3), "predicted {}", tau);
        assert!(approx(q_masses(&[0.51099895, 105.6583755, tau]), 2.0 / 3.0, 1e-12));
    }

    #[test]
    fn invalid_inputs_are_rejected() {
        assert!(extract_ac_delta(&[1.0, 2.0]).is_err());
        assert!(fit_masses(&[1.0, 2.0]).is_err());
        assert!(fit_masses(&[1.0, -2.0, 3.0]).is_err());
        assert!(fit_masses(&[1.0, 0.0, 3.0]).is_err());
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(extract_ac_delta(&[1.0, value, 3.0]).is_err());
            assert!(fit_masses(&[1.0, value, 3.0]).is_err());
        }
        assert!(extract_ac_delta(&[f64::MAX; 3]).is_err());
        assert!(fit_masses(&[f64::MAX; 3]).is_err());
    }
}
