//! CLI for the Rust fluid-universe core. Every subcommand is bounded and silent
//! about dependencies: the crate has none.

use std::process::Command;

use fluid_universe::{
    Check, all_checks, audit, circulant, derrick, fingerprint, fluid, lorentz, reports, rotor,
    simulation, spectra, verify, water,
};

fn json_escape(text: &str) -> String {
    let mut out = String::new();
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn print_json(checks: &[Check]) {
    let mut out = String::from("{\"checks\":[");
    for (i, c) in checks.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&format!(
            "{{\"id\":\"{}\",\"value\":{:.15e},\"detail\":\"{}\"}}",
            json_escape(c.id), c.value, json_escape(&c.detail)
        ));
    }
    println!("{}]}}", out);
}

fn spawn_fingerprint() -> Option<String> {
    let exe = std::env::current_exe().ok()?;
    let out = Command::new(exe).arg("fingerprint").output().ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8(out.stdout).ok()?;
    let token = text.trim();
    if token.len() != 16 || !token.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    Some(token.to_string())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().map(String::as_str).unwrap_or("all");
    match command {
        "all" => {
            let checks = all_checks();
            for c in &checks {
                println!("{c}");
            }
            println!("{} check(s), fingerprint {}", checks.len(), fingerprint(&checks));
        }
        "circulant" | "spectra" | "rotor" | "derrick" | "fluid" | "water" | "lorentz"
        | "simulation" | "audit" | "reports" => {
            let checks: Vec<Check> = match command {
                "circulant" => circulant::checks(),
                "spectra" => spectra::checks(),
                "rotor" => rotor::checks(),
                "derrick" => derrick::checks(),
                "fluid" => fluid::checks(),
                "water" => water::checks(),
                "lorentz" => lorentz::checks(),
                "simulation" => simulation::checks(),
                "audit" => audit::checks(),
                "reports" => reports::checks(),
                _ => unreachable!(),
            };
            for c in &checks {
                println!("{c}");
            }
        }
        "json" => print_json(&all_checks()),
        "fingerprint" => println!("{}", fingerprint(&all_checks())),
        "verify" => {
            let first = spawn_fingerprint();
            let second = spawn_fingerprint();
            match (first, second) {
                (Some(a), Some(b)) => {
                    let result = verify::compare(&a, &b);
                    println!("G-001 two-process verification: run1={a}, run2={b}, agreement={}",
                             if result { "PASS" } else { "FAIL" });
                    std::process::exit(if result { 0 } else { 1 });
                }
                _ => {
                    eprintln!("Why: could not spawn two child processes for G-001.\nFix: rebuild with `cargo build --release` and run the binary, not a test harness.");
                    std::process::exit(2);
                }
            }
        }
        "claims" => {
            for record in audit::run_audit() {
                println!("{:<7} {:<44} {:<12.6e} {}", record.verdict, record.claim,
                         record.value, record.detail);
            }
        }
        "regenerate" => {
            let dir = args.get(1).map(std::path::PathBuf::from)
                .unwrap_or_else(|| std::path::PathBuf::from("results"));
            match reports::write_reports(&dir) {
                Ok(paths) => {
                    println!("wrote {} Rust result file(s) to {}", paths.len(), dir.display());
                    for path in &paths {
                        println!("  {}", path.display());
                    }
                }
                Err(error) => {
                    eprintln!("Why: result regeneration failed: {error}.\nFix: check the output directory is writable.");
                    std::process::exit(1);
                }
            }
        }
        "selftest" => {
            let result = verify::verify_in_process();
            println!("{}", result.line());
            std::process::exit(if result.agree { 0 } else { 1 });
        }
        other => {
            eprintln!("Why: unknown subcommand {other:?}.\nFix: choose all|circulant|spectra|rotor|derrick|fluid|water|lorentz|simulation|audit|reports|claims|regenerate|json|fingerprint|verify|selftest.");
            std::process::exit(2);
        }
    }
}
