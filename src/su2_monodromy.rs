//! Exact SU(2) fusion and full-monodromy witness with higher-spin fields.
//!
//! Dynkin labels are twice the spin. The witness was selected after the target
//! phases were known; it is not a claim of physical sector assignment.

use crate::Check;

pub const LEVEL: u32 = 2338;
pub const EXTERNAL: u32 = 558;
pub const CHANNELS: [u32; 3] = [1070, 1044, 688];

/// SU(2)_k fusion multiplicities are zero or one. Inputs are Dynkin labels.
pub fn fusion_allowed(k: u32, a: u32, b: u32, c: u32) -> bool {
    if k == 0 || a > k || b > k || c > k { return false; }
    let (k, a, b, c) = (i128::from(k), i128::from(a), i128::from(b), i128::from(c));
    (a + b + c) % 2 == 0 && c >= (a - b).abs() && c <= (a + b).min(2 * k - a - b)
}

/// Reversed full-monodromy exponent modulo one, as an unreduced fraction.
/// Returns None when the channel is absent from the fusion of a with itself.
pub fn reversed_monodromy_residue(k: u32, a: u32, c: u32) -> Option<(i128, i128)> {
    if !fusion_allowed(k, a, a, c) { return None; }
    let (k, a, c) = (i128::from(k), i128::from(a), i128::from(c));
    let den = 4 * (k + 2);
    let forward = c * (c + 2) - 2 * a * (a + 2);
    Some(((-forward).rem_euclid(den), den))
}

/// Exhaust every identical external primary and allowed channel below the
/// witness level. All three reduced target denominators divide 4(k+2), so
/// 585 divides k+2; the only lower positive levels are 583, 1168 and 1753.
/// Each mask count records which of the three phases occur, allowing either
/// orientation independently. The channel count includes every external label.
pub fn lower_level_search() -> &'static [(u32, u64, [u32; 8]); 3] {
    static RESULT: std::sync::OnceLock<[(u32, u64, [u32; 8]); 3]> = std::sync::OnceLock::new();
    RESULT.get_or_init(|| [583_u32, 1168, 1753].map(|k| {
        let den = i64::from(4 * (k + 2));
        let mut visited = 0_u64;
        let mut counts = [0_u32; 8];
        for a in 0..=k {
            let mut mask = 0_usize;
            for c in (0..=2 * a.min(k - a)).step_by(2) {
                visited += 1;
                let (a, c) = (i64::from(a), i64::from(c));
                let value = (c * (c + 2) - 2 * a * (a + 2)).rem_euclid(den);
                for (bit, p, q) in [(1_usize, 2_i64, 9_i64), (2, 1, 10), (4, 2, 39)] {
                    if value == den * p / q || value == den * (q - p) / q { mask |= bit; }
                }
            }
            counts[mask] += 1;
        }
        (k, visited, counts)
    }))
}

pub fn checks() -> Vec<Check> {
    let mut out: Vec<Check> = CHANNELS.iter().enumerate().map(|(index, &channel)| {
        let (num, den) = reversed_monodromy_residue(LEVEL, EXTERNAL, channel).unwrap();
        Check::new(
            ["su2_monodromy.lepton", "su2_monodromy.down", "su2_monodromy.up"][index],
            num as f64 / den as f64,
            "post-hoc SU(2)_2338 higher-spin reversed full-monodromy residue",
        )
    }).collect();
    let rows = lower_level_search();
    out.push(Check::new("su2_monodromy.lower_level_channels", rows.iter().map(|row| row.1).sum::<u64>() as f64,
                        "fusion channels scanned at all three lower candidate levels"));
    out.push(Check::new("su2_monodromy.lower_level_triple_matches", rows.iter().map(|row| row.2[7]).sum::<u32>() as f64,
                        "lower-level external labels matching all three target phases"));
    out
}
