//! CLI for the Rust fluid-universe core. Every subcommand is bounded and silent
//! about dependencies: the crate has none.

use std::process::Command;

use fluid_universe::{
    Check, all_checks, audit, circulant, derrick, fingerprint, fluid, lorentz, reports, rotor,
    simulation, spectra, unification, verify, water,
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
        | "simulation" | "audit" | "reports" | "unification" => {
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
                "unification" => unification::checks(),
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
        "simulate" => {
            match args.get(1).map(String::as_str).unwrap_or("") {
                "muscan" => {
                    let n: usize = 10;
                    let length = 8.0;
                    let steps = 200usize;
                    let dt = 0.05;
                    println!("mu,n,steps,dt,e2,e4,potential,energy,z3,force_rms,max_bond,mean_bond,energy_drop");
                    for index in 0..=20usize {
                        let mu = index as f64; // 0..20
                        let field = simulation::analytic_field(n, length, false);
                        let (_, history, m) = simulation::relax(&field, mu, dt, steps, 1.0, 1.0);
                        let drop = history.first().copied().unwrap_or(0.0)
                            - history.last().copied().unwrap_or(0.0);
                        println!("{:.1},{},{},{:.3},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9}",
                                 mu, n, steps, dt, m.e2, m.e4, m.potential, m.energy,
                                 m.z3, m.force_rms, m.max_bond, m.mean_bond, drop);
                    }
                }
                "search" => {
                    let n = 16usize;
                    let steps = 120usize;
                    let baseline = simulation::hopf_charge_spectral(
                        &simulation::analytic_field(n, 8.0, false));
                    println!(concat!("mu,K,kappa,init,hopf_baseline,hopf_ratio,z3,",
                                     "force_rms,max_bond"));
                    for mu in [1.0_f64, 2.0, 5.0, 10.0] {
                        for k in [0.5_f64, 1.0, 2.0] {
                            for kappa in [0.25_f64, 0.5, 1.0, 2.0] {
                                for init in ["analytic", "perturbed"] {
                                    let seed = if init == "analytic" {
                                        simulation::analytic_field(n, 8.0, false)
                                    } else {
                                        simulation::perturbed_field(n, 8.0, 20261122, 0.05)
                                    };
                                    let (relaxed, _, m) =
                                        simulation::relax(&seed, mu, 0.05, steps, k, kappa);
                                    let h = simulation::hopf_charge_spectral(&relaxed);
                                    let (z3, force_rms, max_bond) = (m.z3, m.force_rms, m.max_bond);
                                    println!(concat!("{:.1},{:.1},{:.2},{},{:.9},{:.9},",
                                                     "{:.9},{:.9},{:.9}"),
                                             mu, k, kappa, init, baseline, h / baseline,
                                             z3, force_rms, max_bond);
                                }
                            }
                        }
                    }
                }
                "evolve" => {
                    let n: usize = args.get(2).and_then(|v| v.parse().ok()).unwrap_or(12);
                    let mu: f64 = args.get(3).and_then(|v| v.parse().ok()).unwrap_or(20.0);
                    let kappa: f64 = args.get(4).and_then(|v| v.parse().ok()).unwrap_or(1.0);
                    let steps: usize = args.get(5).and_then(|v| v.parse().ok()).unwrap_or(400);
                    let dt: f64 = args.get(6).and_then(|v| v.parse().ok()).unwrap_or(0.05);
                    let baseline = simulation::hopf_charge_grid(
                        &simulation::analytic_field(n, 8.0, false));
                    let mut field = simulation::analytic_field(n, 8.0, false);
                    println!("n,mu,kappa,step,hopf_baseline,hopf_grid,hopf_ratio,z3,energy,force_rms");
                    for step in 0..=steps {
                        if step % 10 == 0 || step == steps {
                            let h = simulation::hopf_charge_grid(&field);
                            let m = simulation::metrics(&field, mu);
                            let ratio = h / baseline;
                            println!(concat!("{},{:.1},{:.1},{},{:.9},{:.9},{:.9},",
                                             "{:.9},{:.9},{:.9}"),
                                     n, mu, kappa, step, baseline, h, ratio, m.z3,
                                     m.energy, m.force_rms);
                        }
                        if step < steps {
                            let (next, _, _) = simulation::relax(&field, mu, dt, 1, 1.0, kappa);
                            field = next;
                        }
                    }
                }
                "phase" => {
                    let n = 12usize;
                    let steps = 100usize;
                    let baseline = simulation::hopf_charge_grid(
                        &simulation::analytic_field(n, 8.0, false));
                    println!(concat!("mu,kappa,n,hopf_baseline,hopf_grid,hopf_ratio,z3,",
                                     "e2,e4,potential,force_rms,max_bond,energy_drop"));
                    for mu in [0.0_f64, 1.0, 2.0, 5.0, 10.0, 15.0, 20.0] {
                        for kappa in [0.5_f64, 1.0, 2.0] {
                            let field = simulation::analytic_field(n, 8.0, false);
                            let (relaxed, history, m) =
                                simulation::relax(&field, mu, 0.05, steps, 1.0, kappa);
                            let h = simulation::hopf_charge_grid(&relaxed);
                            let ratio = if baseline.abs() > 1e-12 { h / baseline } else { 0.0 };
                            let drop = history.first().copied().unwrap_or(0.0)
                                - history.last().copied().unwrap_or(0.0);
                            let (z3, e2, e4, potential, force_rms, max_bond) =
                                (m.z3, m.e2, m.e4, m.potential, m.force_rms, m.max_bond);
                            println!(concat!("{:.1},{:.1},{},{:.9},{:.9},{:.9},{:.9},",
                                             "{:.9},{:.9},{:.9},{:.9},{:.9},{:.9}"),
                                     mu, kappa, n, baseline, h, ratio, z3, e2, e4,
                                     potential, force_rms, max_bond, drop);
                        }
                    }
                }
                "hopfcheck" => {
                    println!("n,hopf_jacobi,hopf_spectral,hopf_preimage,z3");
                    for n in [8usize, 16, 32, 64, 128] {
                        let field = simulation::analytic_field(n, 8.0, false);
                        println!("{},{:.9},{:.9},{:.9},{:.9}", n,
                                 simulation::hopf_charge_grid(&field),
                                 simulation::hopf_charge_spectral(&field),
                                 simulation::hopf_charge_preimage(200),
                                 simulation::z3_order(&field));
                    }
                }
                "history" => {
                    println!("mu,step,energy");
                    for mu in [0.0_f64, 5.0, 20.0] {
                        let field = simulation::analytic_field(10, 8.0, false);
                        let (_, history, _) = simulation::relax(&field, mu, 0.05, 200, 1.0, 1.0);
                        for (step, energy) in history.iter().enumerate() {
                            println!("{mu:.1},{step},{energy:.9}");
                        }
                    }
                }
                other => {
                    eprintln!("Why: unknown simulation {other:?}.\nFix: choose `muscan|history`.");
                    std::process::exit(2);
                }
            }
        }
        "scan" => {
            match args.get(1).map(String::as_str).unwrap_or("") {
                "neutrino" => {
                    println!("ordering,m_light_eV,Q");
                    for step in 0..=2000usize {
                        let m0 = 0.2 * step as f64 / 2000.0;
                        for inverted in [false, true] {
                            let masses = spectra::neutrino_masses(m0, 7.41e-5, 2.507e-3, inverted);
                            println!("{},{:.8},{:.12}",
                                     if inverted { "IO" } else { "NO" }, m0,
                                     spectra::q_from_masses(&masses));
                        }
                    }
                }
                other => {
                    eprintln!("Why: unknown scan {other:?}.\nFix: choose `neutrino`.");
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
            eprintln!("Why: unknown subcommand {other:?}.\nFix: choose all|circulant|spectra|rotor|derrick|fluid|water|lorentz|simulation|audit|reports|claims|scan|simulate|regenerate|json|fingerprint|verify|selftest.");
            std::process::exit(2);
        }
    }
}
