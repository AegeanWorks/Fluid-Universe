//! Single-WZW-model existence and bounded non-existence evidence.
//!
//! Two explicit exact witnesses exist: SU(17)_1153 and SU(23)_1147 each contain
//! three integrable primaries whose braiding labels are 2/9, 1/10 and 2/39.
//! This proves existence and non-uniqueness, not physical naturalness or sector
//! selection.
//!
//! Two bounded searches supply scope evidence:
//! * N=2..7, k=1..79, Dynkin labels 0..4: 474 models, 1,337,784 primaries,
//!   no all-phase host.
//! * N=2..7, every positive level, Dynkin labels 0..4: 19,530 primaries,
//!   694 models with any phase, no all-phase host.
//!
//! HONEST CAVEAT: SU(17)_1153 is a large, post-hoc host.  No rule selects it,
//! no dynamics follows from it, and it is not itself a unified field theory.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use crate::Check;

const MAX_N: i64 = 7;
const MAX_K: i64 = 79;
const MAX_DYNKIN: i64 = 4;
const TARGETS: [(i64, i64); 3] = [(2, 9), (1, 10), (2, 39)];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Witness {
    pub n: i64,
    pub k: i64,
    pub dynkin: &'static [&'static [i64]],
}

pub const WITNESSES: [Witness; 2] = [
    Witness {
        n: 17,
        k: 1153,
        dynkin: &[
            &[0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 2, 1, 0, 0, 0, 1],
            &[0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 0, 0, 0, 1],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1],
        ],
    },
    Witness {
        n: 23,
        k: 1147,
        dynkin: &[
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1, 0, 2, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
            &[0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2],
        ],
    },
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchSummary {
    pub models_tested: u64,
    pub labels_tested: u64,
    pub hit_models: Vec<(i64, i64)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InverseSearchSummary {
    pub labels_tested: u64,
    pub models_with_any_phase: u64,
    pub hit_models: Vec<(i64, i64)>,
    pub phase_masks: BTreeMap<(i64, i64), u8>,
}

static SEARCH: OnceLock<SearchSummary> = OnceLock::new();
static INVERSE_SEARCH: OnceLock<InverseSearchSummary> = OnceLock::new();

pub fn phases() -> [f64; 3] {
    [2.0 / 9.0, 0.1, 2.0 / 39.0]
}

pub fn models() -> [(i64, i64); 3] {
    [(3, 9), (2, 13), (3, 49)]
}

/// Numerator of `|C2-2*C2(box)|/(k+N)` before the level denominator.
fn braiding_numerator(n: i64, dynkin: &[i64]) -> i64 {
    assert!(n >= 2 && n <= 23);
    assert_eq!(dynkin.len(), n as usize - 1);
    assert!(dynkin.iter().all(|value| *value >= 0 && *value <= MAX_DYNKIN));

    let mut rows = [0_i64; 23];
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
    (n * a - boxes * boxes - 2 * (n * n - 1)).abs()
}

/// Exact unreduced numerator/denominator of the braiding label for one primary.
///
/// `C2 = (N*A - boxes^2)/(2N)` with
/// `A = sum_i lambda_i*(lambda_i+N+1-2i)`, so the label is
/// `abs(N*A-boxes^2-2*(N^2-1)) / (2N(k+N))`.
pub fn braiding_fraction(n: i64, k: i64, dynkin: &[i64]) -> (i64, i64) {
    assert!(k >= 1);
    (braiding_numerator(n, dynkin), 2 * n * (k + n))
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

/// Run the pre-registered bounded level scan once per process.
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

fn required_level(n: i64, numerator: i64, label_sum: i64, p: i64, q: i64) -> Option<i64> {
    let denominator = 2 * n * p;
    let scaled = numerator * q;
    if scaled % denominator != 0 {
        return None;
    }
    let k = scaled / denominator - n;
    if k > 0 && label_sum <= k {
        Some(k)
    } else {
        None
    }
}

fn enumerate_inverse_primaries(
    n: i64,
    index: usize,
    dynkin: &mut [i64],
    label_sum: i64,
    labels_tested: &mut u64,
    phase_masks: &mut BTreeMap<(i64, i64), u8>,
) {
    if index == dynkin.len() {
        *labels_tested += 1;
        let numerator = braiding_numerator(n, dynkin);
        for (target_index, &(p, q)) in TARGETS.iter().enumerate() {
            if let Some(k) = required_level(n, numerator, label_sum, p, q) {
                *phase_masks.entry((n, k)).or_default() |= 1 << target_index;
            }
        }
        return;
    }
    for value in 0..=MAX_DYNKIN {
        dynkin[index] = value;
        enumerate_inverse_primaries(
            n,
            index + 1,
            dynkin,
            label_sum + value,
            labels_tested,
            phase_masks,
        );
    }
}

/// All-positive-level inverse search for N<=7 and Dynkin labels 0..4.
pub fn inverse_search() -> &'static InverseSearchSummary {
    INVERSE_SEARCH.get_or_init(|| {
        let mut labels_tested = 0_u64;
        let mut phase_masks = BTreeMap::new();
        for n in 2..=MAX_N {
            let mut dynkin = vec![0_i64; n as usize - 1];
            enumerate_inverse_primaries(
                n,
                0,
                &mut dynkin,
                0,
                &mut labels_tested,
                &mut phase_masks,
            );
        }
        let hit_models = phase_masks
            .iter()
            .filter_map(|(&model, &mask)| (mask == 0b111).then_some(model))
            .collect();
        InverseSearchSummary {
            labels_tested,
            models_with_any_phase: phase_masks.len() as u64,
            hit_models,
            phase_masks,
        }
    })
}

pub fn witness_fractions(index: usize) -> [(i64, i64); 3] {
    let witness = WITNESSES[index];
    witness
        .dynkin
        .iter()
        .map(|dynkin| braiding_fraction(witness.n, witness.k, dynkin))
        .collect::<Vec<_>>()
        .try_into()
        .expect("each witness has exactly three primaries")
}

pub fn witness_exact(index: usize) -> bool {
    let witness = WITNESSES[index];
    witness.dynkin.iter().zip(TARGETS).all(|(dynkin, (p, q))| {
        let (numerator, denominator) = braiding_fraction(witness.n, witness.k, dynkin);
        numerator * q == p * denominator && dynkin.iter().sum::<i64>() <= witness.k
    })
}

pub fn all_witnesses_exact() -> bool {
    (0..WITNESSES.len()).all(witness_exact)
}

pub fn single_model_exists() -> bool {
    all_witnesses_exact() || !search().hit_models.is_empty() || !inverse_search().hit_models.is_empty()
}

pub fn checks() -> Vec<Check> {
    let result = search();
    let inverse = inverse_search();
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
            "two explicit single-model witnesses exist (SU(17)_1153 and SU(23)_1147)",
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
        Check::new(
            "singlemodel.witness_count",
            WITNESSES.len() as f64,
            "number of explicit all-phase single-model witnesses",
        ),
        Check::new(
            "singlemodel.witness0_n",
            WITNESSES[0].n as f64,
            "first witness SU(17) rank",
        ),
        Check::new(
            "singlemodel.witness0_k",
            WITNESSES[0].k as f64,
            "first witness level 1153",
        ),
        Check::new(
            "singlemodel.witness0_exact",
            if witness_exact(0) { 1.0 } else { 0.0 },
            "all three SU(17)_1153 labels exact and integrable",
        ),
        Check::new(
            "singlemodel.witness1_n",
            WITNESSES[1].n as f64,
            "second witness SU(23) rank",
        ),
        Check::new(
            "singlemodel.witness1_k",
            WITNESSES[1].k as f64,
            "second witness level 1147",
        ),
        Check::new(
            "singlemodel.witness1_exact",
            if witness_exact(1) { 1.0 } else { 0.0 },
            "all three SU(23)_1147 labels exact and integrable",
        ),
        Check::new(
            "singlemodel.witness_all_exact",
            if all_witnesses_exact() { 1.0 } else { 0.0 },
            "both explicit witnesses exact and integrable",
        ),
        Check::new(
            "singlemodel.inverse_labels",
            inverse.labels_tested as f64,
            "all-level N<=7 inverse-search primaries",
        ),
        Check::new(
            "singlemodel.inverse_models_any",
            inverse.models_with_any_phase as f64,
            "N<=7 models hosting at least one phase at any level",
        ),
        Check::new(
            "singlemodel.inverse_exists",
            if inverse.hit_models.is_empty() { 0.0 } else { 1.0 },
            "no N<=7, Dynkin<=4 all-phase host at any level",
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

    #[test]
    fn explicit_witnesses_are_exact() {
        assert!(all_witnesses_exact());
        for index in 0..WITNESSES.len() {
            assert!(witness_exact(index));
            let fractions = witness_fractions(index);
            assert_eq!(fractions[0].0 * 9, 2 * fractions[0].1);
            assert_eq!(fractions[1].0 * 10, fractions[1].1);
            assert_eq!(fractions[2].0 * 39, 2 * fractions[2].1);
        }
    }
}
