//! Result regeneration for the Rust rewrite.
//!
//! Port of the former `phenomenology/*_report.py` scripts, `run_all.py` and
//! `verification/verify.py`. Every check family is rendered twice: as
//! deterministic JSON (`schema: 1`, FNV-1a `config_sha256`, G-001 provenance)
//! and as a Markdown table. All JSON is hand-escaped; the crate has no
//! dependencies.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

/// The ten check families in a fixed order; the order is part of every hash.
const FAMILIES: [&str; 10] = [
    "circulant",
    "spectra",
    "rotor",
    "derrick",
    "fluid",
    "radial_shell",
    "water",
    "lorentz",
    "simulation",
    "audit",
];

/// A fully rendered, self-contained report for one check family.
#[derive(Clone, Debug)]
pub struct GeneratedReport {
    pub name: String,
    pub json: String,
    pub markdown: String,
}

/// Dispatch a family name to that module's `checks()`.
///
/// The `simulation` and `audit` entries are supplied by the sibling modules of
/// the same crate; the parent session wires them into `lib.rs`.
fn family_checks(name: &str) -> Vec<crate::Check> {
    match name {
        "circulant" => crate::circulant::checks(),
        "spectra" => crate::spectra::checks(),
        "rotor" => crate::rotor::checks(),
        "derrick" => crate::derrick::checks(),
        "fluid" => crate::fluid::checks(),
        "radial_shell" => crate::radial_shell::checks(),
        "water" => crate::water::checks(),
        "lorentz" => crate::lorentz::checks(),
        "simulation" => crate::simulation::checks(),
        "audit" => crate::audit::checks(),
        _ => Vec::new(),
    }
}

/// FNV-1a over the bytes of `text`, rendered as 16 lowercase hex digits.
/// (The field is historically called `config_sha256` even though it is FNV-1a.)
fn fnv1a_16(text: &[u8]) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for &byte in text {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    format!("{hash:016x}")
}

/// Stable hash of a family name plus its ordered checks, using the same
/// canonical `id|value|detail` line format as `crate::fingerprint`.
fn config_hash(name: &str, checks: &[crate::Check]) -> String {
    let mut text = String::new();
    let _ = writeln!(text, "{name}");
    for check in checks {
        let _ = writeln!(text, "{}|{:.15e}|{}", check.id, check.value, check.detail);
    }
    fnv1a_16(text.as_bytes())
}

/// Minimal RFC-8259 string escaping; no serde needed.
fn json_escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out
}

/// Escape a value so it cannot break a Markdown table cell.
fn md_cell(text: &str) -> String {
    text.replace('|', "\\|")
        .replace('\n', " ")
        .replace('\r', " ")
}

fn json_report(
    name: &str,
    checks: &[crate::Check],
    hash: &str,
    verification: &crate::verify::Verification,
) -> String {
    let mut out = String::new();
    out.push_str("{\"schema\":1,\"generator\":\"fluid-universe-rust\"");
    let _ = write!(out, ",\"name\":\"{}\"", json_escape(name));
    let _ = write!(out, ",\"config_sha256\":\"{}\"", json_escape(hash));
    out.push_str(",\"checks\":[");
    for (index, check) in checks.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let _ = write!(
            out,
            "{{\"id\":\"{}\",\"value\":{:.15e},\"detail\":\"{}\"}}",
            json_escape(check.id),
            check.value,
            json_escape(&check.detail)
        );
    }
    out.push(']');
    let _ = write!(
        out,
        ",\"audit\":{{\"run1\":\"{}\",\"run2\":\"{}\",\"checks\":{},\"agree\":{},\"line\":\"{}\"}}",
        json_escape(&verification.run1),
        json_escape(&verification.run2),
        verification.checks,
        verification.agree,
        json_escape(&verification.line())
    );
    out.push('}');
    out
}

fn markdown_report(
    name: &str,
    checks: &[crate::Check],
    hash: &str,
    verification: &crate::verify::Verification,
) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "# {name}");
    out.push('\n');
    out.push_str("| id | value | detail |\n");
    out.push_str("| --- | --- | --- |\n");
    for check in checks {
        let _ = writeln!(
            out,
            "| `{}` | `{:.15e}` | {} |",
            md_cell(check.id),
            check.value,
            md_cell(&check.detail)
        );
    }
    out.push('\n');
    out.push_str("## Provenance\n\n");
    out.push_str("- Generator: `fluid-universe-rust` (Rust edition 2024, no external crates).\n");
    let _ = writeln!(
        out,
        "- G-001 two-process fingerprint: run1={}, run2={}, agreement={}.",
        verification.run1,
        verification.run2,
        if verification.agree { "PASS" } else { "FAIL" }
    );
    let _ = writeln!(out, "- config_sha256 (FNV-1a): `{hash}`.");
    out
}

/// Build one JSON + Markdown report per check-family prefix.
pub fn build_reports() -> Vec<GeneratedReport> {
    // One in-process two-run derivation supplies the G-001 audit object for
    // every report; it is deterministic and does not recurse into this module.
    let verification = crate::verify::verify_in_process();
    FAMILIES
        .iter()
        .map(|&name| {
            let checks = family_checks(name);
            let hash = config_hash(name, &checks);
            GeneratedReport {
                name: name.to_string(),
                json: json_report(name, &checks, &hash, &verification),
                markdown: markdown_report(name, &checks, &hash, &verification),
            }
        })
        .collect()
}

/// Write `<dir>/rust_<name>.json` and `<dir>/rust_<name>.md` for every family.
/// Returns both paths per report, JSON first, in family order.
pub fn write_reports(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    std::fs::create_dir_all(dir)?;
    let mut paths = Vec::with_capacity(FAMILIES.len() * 2);
    for report in build_reports() {
        let json_path = dir.join(format!("rust_{}.json", report.name));
        let markdown_path = dir.join(format!("rust_{}.md", report.name));
        std::fs::write(&json_path, report.json.as_bytes())?;
        std::fs::write(&markdown_path, report.markdown.as_bytes())?;
        paths.push(json_path);
        paths.push(markdown_path);
    }
    Ok(paths)
}

/// Reproducible result lines for this module (ids prefixed with `reports.`).
///
/// Deliberately independent of `build_reports()` so that adding this module to
/// `crate::all_checks()` cannot recurse through `verify_in_process()`.
pub fn checks() -> Vec<crate::Check> {
    use crate::Check;

    let mut total = 0usize;
    let mut prefix_ok = 0usize;
    let mut hash_stable = 0usize;
    for &name in &FAMILIES {
        let checks = family_checks(name);
        total += checks.len();
        let prefix = format!("{name}.");
        if !checks.is_empty() && checks.iter().all(|check| check.id.starts_with(&prefix)) {
            prefix_ok += 1;
        }
        let first = config_hash(name, &checks);
        let second = config_hash(name, &family_checks(name));
        if first == second {
            hash_stable += 1;
        }
    }

    vec![
        Check::new("reports.families", FAMILIES.len() as f64, "number of generated report families"),
        Check::new("reports.checks", total as f64, "total checks rendered across all families"),
        Check::new("reports.prefix_ok", prefix_ok as f64, "families whose check ids carry the family prefix"),
        Check::new("reports.hash_stable", hash_stable as f64, "families whose config_sha256 is stable across rebuilds"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static TEMP_SEQ: AtomicU64 = AtomicU64::new(0);

    /// RAII temp directory under `std::env::temp_dir()`; removed on drop even
    /// when a test panics.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(tag: &str) -> Self {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_or(0, |duration| duration.as_nanos());
            let seq = TEMP_SEQ.fetch_add(1, Ordering::Relaxed);
            Self(std::env::temp_dir().join(format!(
                "fu_reports_{tag}_{}_{nanos}_{seq}",
                std::process::id()
            )))
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn quoted_field<'a>(json: &'a str, key: &str) -> &'a str {
        let marker = format!("\"{key}\":\"");
        let start = json.find(&marker).expect("missing JSON field") + marker.len();
        let rest = &json[start..];
        &rest[..rest.find('"').expect("unterminated JSON string")]
    }

    #[test]
    fn every_family_produces_a_non_empty_report() {
        let reports = build_reports();
        assert_eq!(reports.len(), FAMILIES.len());
        for (report, &name) in reports.iter().zip(FAMILIES.iter()) {
            assert_eq!(report.name, name);
            assert!(!report.json.is_empty(), "empty JSON for {name}");
            assert!(!report.markdown.is_empty(), "empty Markdown for {name}");
            assert_eq!(
                report.json.matches("\"id\":").count(),
                family_checks(name).len(),
                "unexpected check count in {name}"
            );
        }
    }

    #[test]
    fn json_has_schema_generator_checks_and_audit() {
        for report in build_reports() {
            let json = &report.json;
            assert!(json.starts_with('{') && json.ends_with('}'));
            assert!(json.contains("\"schema\":1"));
            assert!(json.contains("\"generator\":\"fluid-universe-rust\""));
            assert_eq!(quoted_field(json, "name"), report.name);
            assert_eq!(quoted_field(json, "config_sha256").len(), 16);
            assert!(json.contains("\"checks\":["));
            assert!(json.contains("\"audit\":{"));
            assert!(json.contains("\"run1\":"));
            assert!(json.contains("\"run2\":"));
            assert!(json.contains("\"agree\":true"));
        }
    }

    #[test]
    fn markdown_has_heading_table_and_provenance() {
        for report in build_reports() {
            let md = &report.markdown;
            assert!(md.starts_with(&format!("# {}", report.name)));
            assert!(md.contains("| id | value | detail |"));
            assert!(md.contains("| --- | --- | --- |"));
            assert!(md.contains("## Provenance"));
            assert!(md.contains("fluid-universe-rust"));
            assert!(md.contains("G-001 two-process fingerprint"));
            assert!(md.contains("PASS"));
        }
    }

    #[test]
    fn config_hash_is_stable_across_two_builds() {
        let first = build_reports();
        let second = build_reports();
        assert_eq!(first.len(), second.len());
        for (a, b) in first.iter().zip(second.iter()) {
            assert_eq!(a.name, b.name);
            assert_eq!(quoted_field(&a.json, "config_sha256"), quoted_field(&b.json, "config_sha256"));
            assert_eq!(a.json, b.json);
        }
    }

    #[test]
    fn json_escape_handles_special_characters() {
        let escaped = json_escape("a\"b\\c\nd\re\tf\u{01}g|h");
        assert_eq!(escaped, "a\\\"b\\\\c\\nd\\re\\tf\\u0001g|h");
        assert_eq!(md_cell("a|b\nc"), "a\\|b c");
    }

    #[test]
    fn write_reports_creates_two_files_per_report_and_cleans_up() {
        let dir = TempDir::new("write");
        let dir_path = dir.0.clone();
        let paths = write_reports(&dir.0).expect("write_reports failed");
        assert_eq!(paths.len(), FAMILIES.len() * 2);

        let reports = build_reports();
        for (report, pair) in reports.iter().zip(paths.chunks(2)) {
            assert!(pair[0].exists() && pair[1].exists());
            assert_eq!(
                pair[0].file_name().unwrap().to_string_lossy(),
                format!("rust_{}.json", report.name)
            );
            assert_eq!(
                pair[1].file_name().unwrap().to_string_lossy(),
                format!("rust_{}.md", report.name)
            );
            assert_eq!(std::fs::read_to_string(&pair[0]).unwrap(), report.json);
            assert_eq!(std::fs::read_to_string(&pair[1]).unwrap(), report.markdown);
        }

        drop(dir);
        assert!(!dir_path.exists(), "temp directory was not cleaned up");
    }

    #[test]
    fn checks_report_coverage_and_determinism() {
        let checks = super::checks();
        assert_eq!(checks.len(), 4);
        assert!(checks.iter().all(|check| check.id.starts_with("reports.")));
        assert!(checks.iter().all(|check| check.value.is_finite()));
        let families = checks.iter().find(|c| c.id == "reports.families").unwrap();
        assert_eq!(families.value, FAMILIES.len() as f64);
        let stable = checks.iter().find(|c| c.id == "reports.hash_stable").unwrap();
        assert_eq!(stable.value, FAMILIES.len() as f64);
        assert_eq!(checks, super::checks());
    }
}
