//! Reproducible bounded search: no single SU(N)_k model hosts all three sector
//! phases as braiding labels in the pre-registered domain.
//!
//! For SU(N<=7, k<=79), enumerate every integrable primary with Dynkin labels
//! 0..4 and label sum <=k.  Compare the exact rational braiding label
//! `|C2(nu)-2*C2(box)|/(k+N)` against {2/9, 1/10, 2/39} by integer
//! cross-multiplication.  The search covers 474 models and 1,337,784 primaries;
//! no model contains all three phases.
//!
//! The three natural assignments remain distinct:
//!   lepton 2/9  = SU(3)_9 singlet (k = d^2)
//!   down   1/10 = SU(2)_13 singlet (k = d^2+d+1)
//!   up     2/39 = SU(3)_49 singlet (k = (2d+1)^2)
//!
//! HONEST CAVEAT: this is a bounded negative.  Larger N, k or Dynkin labels are
//! not excluded, and the search does not derive why the three sector assignments
//! differ.

use std::sync::OnceLock;

use crate::Check;

const MAX_N: i64 = 7;
const MAX_K: i64 = 79;
const MAX_DYNKIN: i64 = 4;
const TARGETS: [(i64, i64); 3] = [(2, 9), (1, 10), (2, 39)];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchSummary {
    pub models_tested: u64,
    pub labels_tested: u64,
    pub hit_models: Vec<(i64, i64)>,
}

static SEARCH: OnceLock<SearchSummary> = OnceLock::new();

pub fn phases() -> [f64; 3] {
    [2.0 / 9.0, 0.1, 2.0 / 39.0]
}

pub fn models() -> [(i64, i64); 3] {
    [(3, 9), (2, 13), (3, 49)]
}

/// Exact unreduced numerator/denominator of the braiding label for one primary.
///
/// `C2 = (N*A - boxes^2)/(2N)` with
/// `A = sum_i lambda_i*(lambda_i+N+1-2i)`, so the label is
/// `abs(N*A-boxes^2-2*(N^2-1)) / (2N(k+N))`.
pub fn braiding_fraction(n: i64, k: i64, dynkin: &[i64]) -> (i64, i64) {
    assert!(n >= 2 && n <= MAX_N);
    assert!(k >= 1 && k <= MAX_K);
    assert_eq!(dynkin.len(), n as usize - 1);
    assert!(dynkin.iter().all(|value| *value >= 0 && *value <= MAX_DYNKIN));
    assert!(dynkin.iter().sum::<i64>() <= k);

    let mut rows = [0_i64; 7];
    let mut running = 0_i64;
    for i in (0..dynkin.len()).rev() {
        running += dynkin[i];
        rows[i] = running;
    }
    let boxes = rows.iter().sum::<i64>();
    let a = rows
        .iter()
        .take(dynkin.len())
        .enumerate()
        .map(|(index, row)| row * (row + n + 1 - 2 * (index as i64 + 1)))
        .sum::<i64>();
    let numerator = (n * a - boxes * boxes - 2 * (n * n - 1)).abs();
    let denominator = 2 * n * (k + n);
    (numerator, denominator)
}

fn enumerate_primaries(
    n: i64,
    k: i64,
    index: usize,
    remaining: i64,
    dynkin: &mut [i64],
    found: &mut u8,
    labels_tested: &mut u64,
) {
    if index == dynkin.len() {
        *labels_tested += 1;
        let (numerator, denominator) = braiding_fraction(n, k, dynkin);
        for (target_index, &(p, q)) in TARGETS.iter().enumerate() {
            if numerator * q == p * denominator {
                *found |= 1 << target_index;
            }
        }
        return;
    }
    let upper = remaining.min(MAX_DYNKIN);
    for value in 0..=upper {
        dynkin[index] = value;
        enumerate_primaries(
            n,
            k,
            index + 1,
            remaining - value,
            dynkin,
            found,
            labels_tested,
        );
    }
}

/// Run the pre-registered bounded search once per process.
pub fn search() -> &'static SearchSummary {
    SEARCH.get_or_init(|| {
        let mut summary = SearchSummary {
            models_tested: 0,
            labels_tested: 0,
            hit_models: Vec::new(),
        };
        for n in 2..=MAX_N {
            for k in 1..=MAX_K {
                summary.models_tested += 1;
                let mut dynkin = vec![0_i64; n as usize - 1];
                let mut found = 0_u8;
                enumerate_primaries(
                    n,
                    k,
                    0,
                    k,
                    &mut dynkin,
                    &mut found,
                    &mut summary.labels_tested,
                );
                if found == 0b111 {
                    summary.hit_models.push((n, k));
                }
            }
        }
        summary
    })
}

pub fn single_model_exists() -> bool {
    !search().hit_models.is_empty()
}

pub fn checks() -> Vec<Check> {
    let result = search();
    vec![
        Check::new(
            "singlemodel.phase_count",
            phases().len() as f64,
            "three sector phases",
        ),
        Check::new(
            "singlemodel.models",
            models().len() as f64,
            "three distinct natural (N,k) models",
        ),
        Check::new(
            "singlemodel.exists",
            if single_model_exists() { 1.0 } else { 0.0 },
            "no single searched SU(N)_k hosts all three (0)",
        ),
        Check::new(
            "singlemodel.models_tested",
            result.models_tested as f64,
            "bounded model count N=2..7, k=1..79",
        ),
        Check::new(
            "singlemodel.labels_tested",
            result.labels_tested as f64,
            "integrable primaries with Dynkin labels 0..4",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinct_models_required() {
        let m = models();
        assert_ne!(m[0], m[1]);
        assert_ne!(m[1], m[2]);
    }

    #[test]
    fn phases_are_exact() {
        assert!((phases()[0] - 2.0 / 9.0).abs() < 1e-14);
        assert!((phases()[1] - 0.1).abs() < 1e-14);
        assert!((phases()[2] - 2.0 / 39.0).abs() < 1e-14);
    }

    #[test]
    fn known_singlet_labels_are_exact_rationals() {
        let lepton = braiding_fraction(3, 9, &[0, 0]);
        let down = braiding_fraction(2, 13, &[0]);
        let up = braiding_fraction(3, 49, &[0, 0]);
        assert_eq!(lepton.0 * 9, 2 * lepton.1);
        assert_eq!(down.0 * 10, down.1);
        assert_eq!(up.0 * 39, 2 * up.1);
    }
}
