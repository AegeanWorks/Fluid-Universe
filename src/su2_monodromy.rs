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

pub fn checks() -> Vec<Check> {
    CHANNELS.iter().enumerate().map(|(index, &channel)| {
        let (num, den) = reversed_monodromy_residue(LEVEL, EXTERNAL, channel).unwrap();
        Check::new(
            ["su2_monodromy.lepton", "su2_monodromy.down", "su2_monodromy.up"][index],
            num as f64 / den as f64,
            "post-hoc SU(2)_2338 higher-spin reversed full-monodromy residue",
        )
    }).collect()
}
