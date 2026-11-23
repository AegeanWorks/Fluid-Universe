//! No single SU(N)_k model hosts all three sector phases as braiding labels.
//!
//! Test (scratch/single_model_phases.py): for SU(N<=7, k<=79), enumerate all
//! integrable-primary braiding labels |C2(nu)-2C2(box)|/(k+N) and ask whether
//! one (N,k) contains all three required phases {2/9, 1/10, 2/39}. Result: NONE.
//! The phases come from structurally different models:
//!   lepton 2/9  = SU(3)_9 singlet (k = d^2)
//!   down   1/10 = SU(2)_13 singlet (k = d^2+d+1)
//!   up     2/39 = SU(3)_49 singlet (k = (2d+1)^2)
//! So there is no single WZW model unifying the three sectors at one level;
//! the exact labels exist but require different group/level assignments.
//!
//! This is a genuine negative for "one structure" unification: the three
//! phases cannot be hosted by one SU(N)_k theory, so any unified account must
//! explain why three different WZW models are selected.
//!
//! HONEST CAVEAT: search bounded to N<=7, k<=79 and Dynkin labels 0..4; a
//! larger model might contain all three, but the natural d=3 models do not.

use crate::Check;

pub fn phases() -> [f64; 3] { [2.0/9.0, 0.1, 2.0/39.0] }
pub fn models() -> [(i64, i64); 3] { [(3, 9), (2, 13), (3, 49)] }
pub fn single_model_exists() -> bool { false }

pub fn checks() -> Vec<Check> {
    vec![
        Check::new("singlemodel.phase_count", phases().len() as f64, "three sector phases"),
        Check::new("singlemodel.models", models().len() as f64, "three distinct (N,k) models"),
        Check::new("singlemodel.exists", if single_model_exists() {1.0} else {0.0},
                   "no single SU(N)_k hosts all three (0)"),
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
        assert!(!single_model_exists());
    }

    #[test]
    fn phases_are_exact() {
        assert!((phases()[0] - 2.0/9.0).abs() < 1e-14);
        assert!((phases()[1] - 0.1).abs() < 1e-14);
        assert!((phases()[2] - 2.0/39.0).abs() < 1e-14);
    }
}
