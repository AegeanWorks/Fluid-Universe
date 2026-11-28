//! Project rule G-001 requires per-claim agreement in two separate processes.
//!
//! The binary compares identified audit records from two separate processes.
//! Audit coverage of all accepted ledger claims has not yet been established.

use crate::{Check, all_checks, fingerprint};

/// Expected audit identifiers in output order; update alongside audit rows.
pub const AUDIT_RECORD_IDS: &[&str] = &[
    "audit.circulant_matrix_structure", "audit.circulant_dft_eigenvectors",
    "audit.q_identity", "audit.koide_ratio_condition", "audit.z3_not_fixed",
    "audit.sign_caveat", "audit.tau_prediction", "audit.measured_pole_fit", "audit.measured_pole_parameters", "audit.r_invariance",
    "audit.neutrino_q_max", "audit.quark_q", "audit.rotor_degeneracy",
    "audit.wall_tension", "audit.derrick_virial", "audit.equal_k_isotropy",
    "audit.k123_anisotropy", "audit.delta_closed_form", "audit.hopf_preimage_charge",
    "audit.z3_limits", "audit.vortex_blob_fraction", "audit.marcus_width",
    "audit.radial_shell_scale", "audit.radial_shell_singular_slope", "audit.radial_shell_phase",
    "audit.radial_shell_quantum_pressure", "audit.radial_shell_gates",
    "audit.multi_radial_shell_nodes", "audit.multi_radial_shell_singular",
    "audit.multi_radial_shell_phase", "audit.multi_radial_shell_quantum_pressure",
    "audit.multi_radial_shell_gates", "audit.single_model_search_coverage",
    "audit.single_model_known_labels", "audit.single_model_exists",
    "audit.single_model_witness_su17", "audit.single_model_witness_su23",
    "audit.single_model_inverse_search", "audit.single_model_witness_su7",
    "audit.fundamental_pair_center_selection",
    "audit.su3_four_point_dimensions",
    "audit.single_model_relaxed_inverse", "audit.lower_rank_inverse_search",
    "audit.minimal_level_inverse_search", "audit.minimal_witness_search",
    "audit.minimal_witness_canonical", "audit.su7_conformal_weights",
    "audit.su7_quantum_dimensions", "audit.sm_conformal_embedding",
    "audit.sm_coset_central_charge", "audit.all_hosts_sm_embedding", "audit.su7_block_sm_coset",
    "audit.su7_hypercharge_census", "audit.su7_nonabelian_branching", "audit.su7_local_cubic_anomaly",
    "audit.su7_su2_doublet_parity", "audit.su7_subgroup_local_anomalies",
];

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

/// Require both exact agreement and the complete expected ordered ID list.
pub fn compare_expected_audit_records(first: &str, second: &str, expected: &[&str]) -> Result<usize, String> {
    let count = compare_audit_records(first, second)?;
    if expected.is_empty() || !first.lines().map(|line| line.split('\t').next().unwrap_or(""))
        .eq(expected.iter().copied()) {
        return Err("audit record IDs do not match the expected manifest".into());
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
    fn identical_partial_or_substituted_records_are_rejected() {
        let expected = ["audit.a", "audit.b"];
        let both = "audit.a\t3ff0000000000000\tx\naudit.b\t4000000000000000\ty\n";
        assert_eq!(compare_expected_audit_records(both, both, &expected), Ok(2));
        let partial = "audit.a\t3ff0000000000000\tx\n";
        assert!(compare_expected_audit_records(partial, partial, &expected).is_err());
        let substituted = both.replace("audit.b", "audit.c");
        assert!(compare_expected_audit_records(&substituted, &substituted, &expected).is_err());
    }

    #[test]
    fn fingerprint_is_order_sensitive() {
        let a = vec![Check::new("a", 1.0, "x"), Check::new("b", 2.0, "y")];
        let b = vec![Check::new("b", 2.0, "y"), Check::new("a", 1.0, "x")];
        assert_ne!(fingerprint(&a), fingerprint(&b));
    }
}
