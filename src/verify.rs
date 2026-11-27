//! Project rule G-001 requires per-claim agreement in two separate processes.
//!
//! The binary compares identified audit records from two separate processes.
//! Audit coverage of all accepted ledger claims has not yet been established.

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
            "In-process determinism check (not G-001 evidence): {} checks, run1={}, run2={}, agreement={}",
            self.checks,
            self.run1,
            self.run2,
            if self.agree { "PASS" } else { "FAIL" }
        )
    }
}

/// Re-derive all checks twice in the same process. Useful as a cheap determinism
/// smoke test; the binary's `verify` compares audit records from two processes.
/// NOT valid G-001 evidence: both derivations share one process.
/// The `verify` subcommand must spawn two child processes instead.
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

/// Compare complete, uniquely identified audit records without relying on hashes.
pub fn compare_audit_records(first: &str, second: &str) -> Result<usize, String> {
    fn validate(input: &str) -> Result<usize, String> {
        let mut ids = std::collections::HashSet::new();
        if input.is_empty() || !input.ends_with('\n') {
            return Err("empty or incomplete audit output".into());
        }
        for line in input.lines() {
            let mut fields = line.splitn(3, '\t');
            let id = fields.next().unwrap_or("");
            let bits = fields.next().unwrap_or("");
            let detail = fields.next().unwrap_or("");
            if !id.starts_with("audit.") || id.len() <= 6 || bits.len() != 16
                || !bits.bytes().all(|b| b.is_ascii_hexdigit()) || detail.is_empty()
                || !ids.insert(id)
            {
                return Err(format!("malformed or duplicate audit record: {id}"));
            }
        }
        Ok(ids.len())
    }
    let count = validate(first)?;
    validate(second)?;
    if first != second {
        return Err("audit records differ between processes".into());
    }
    Ok(count)
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
    fn audit_records_require_unique_complete_exact_matches() {
        let first = "audit.a\t3ff0000000000000\tPROVEN | a\naudit.b\t4000000000000000\tNUMERIC | b\n";
        assert_eq!(compare_audit_records(first, first), Ok(2));
        assert!(compare_audit_records(first, &first.replace("NUMERIC", "UNVERIFIED")).is_err());
        assert!(compare_audit_records(first, first.lines().next().unwrap()).is_err());
        assert!(compare_audit_records("", "").is_err());
        let duplicate = "audit.a\t3ff0000000000000\tx\naudit.a\t3ff0000000000000\tx\n";
        assert!(compare_audit_records(duplicate, duplicate).is_err());
        assert!(compare_audit_records("audit.a\tbad\tx\n", "audit.a\tbad\tx\n").is_err());
    }

    #[test]
    fn fingerprint_is_order_sensitive() {
        let a = vec![Check::new("a", 1.0, "x"), Check::new("b", 2.0, "y")];
        let b = vec![Check::new("b", 2.0, "y"), Check::new("a", 1.0, "x")];
        assert_ne!(fingerprint(&a), fingerprint(&b));
    }
}
