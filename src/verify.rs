//! Project rule G-001: every check must be re-derived in two SEPARATE processes.
//!
//! The library offers a pure comparator; the binary's `verify` subcommand runs
//! itself twice with `--json`, so the two results cannot share mutable state.

use crate::{Check, all_checks, fingerprint};

#[derive(Clone, Debug)]
pub struct Verification {
    pub run1: String,
    pub run2: String,
    pub checks: usize,
    pub agree: bool,
}

impl Verification {
    pub fn line(&self) -> String {
        format!(
            "G-001 two-process verification: {} checks, run1={}, run2={}, agreement={}",
            self.checks,
            self.run1,
            self.run2,
            if self.agree { "PASS" } else { "FAIL" }
        )
    }
}

/// Re-derive all checks twice in the same process. Useful as a cheap determinism
/// smoke test; delivery uses `verify` in the binary, which spawns two processes.
pub fn verify_in_process() -> Verification {
    let first = all_checks();
    let second = all_checks();
    let run1 = fingerprint(&first);
    let run2 = fingerprint(&second);
    let agree = run1 == run2;
    Verification { run1, run2, checks: first.len(), agree }
}

/// Compare two externally produced fingerprints. No hidden state, no mutation.
pub fn compare(run1: &str, run2: &str) -> bool {
    run1 == run2
}

/// Run a caller-supplied derivation twice and report whether it is reproducible.
pub fn verify_with(derive: impl Fn() -> Vec<Check>) -> Verification {
    let first = derive();
    let second = derive();
    let run1 = fingerprint(&first);
    let run2 = fingerprint(&second);
    let agree = run1 == run2;
    Verification { run1, run2, checks: first.len(), agree }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Check;

    #[test]
    fn identical_runs_agree() {
        let v = verify_with(|| vec![Check::new("x", 1.0, "constant")]);
        assert!(v.agree);
        assert!(v.line().contains("PASS"));
    }

    #[test]
    fn different_runs_disagree() {
        assert!(!compare("abc", "abd"));
    }

    #[test]
    fn fingerprint_is_order_sensitive() {
        let a = vec![Check::new("a", 1.0, "x"), Check::new("b", 2.0, "y")];
        let b = vec![Check::new("b", 2.0, "y"), Check::new("a", 1.0, "x")];
        assert_ne!(fingerprint(&a), fingerprint(&b));
    }
}
