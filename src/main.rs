//! CLI for the Rust fluid-universe core. Every subcommand is bounded and silent
//! about dependencies: the crate has none.

use std::process::Command;

use fluid_universe::{
    Check, all_checks, anomaly, audit, circulant, confinement, derrick, fingerprint, fluid, flux_sector,
    fn_scan, froggatt, geometry, lorentz, mass_spectrum, modular, orthogonal, poisson, potential_family,
    potts_sector, puddle,
    quark_rg, reports, rg_sector, rotor, shape_freedom, simulation, spectra, spinodal, su3, two_component,
    unified_model, unification, variation, verify, vortex_unification, water, water_mechanics, water_unification,
    xy_z3,
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
        | "simulation" | "audit" | "reports" | "unification" | "mass_spectrum"
        | "potential_family" | "flux_sector" | "spinodal" | "rg_sector" | "two_component"
        | "potts_sector" | "xy_z3" | "quark_rg" | "water_mechanics" | "water_unification"
        | "puddle" | "vortex_unification" | "variation" | "poisson" | "orthogonal"
        | "confinement" | "froggatt" | "fn_scan" | "modular" | "su3" | "geometry" | "anomaly"
        | "unified_model" | "shape_freedom" => {
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
                "mass_spectrum" => mass_spectrum::checks(),
                "potential_family" => potential_family::checks(),
                "flux_sector" => flux_sector::checks(),
                "spinodal" => spinodal::checks(),
                "rg_sector" => rg_sector::checks(),
                "two_component" => two_component::checks(),
                "potts_sector" => potts_sector::checks(),
                "xy_z3" => xy_z3::checks(),
                "quark_rg" => quark_rg::checks(),
                "water_mechanics" => water_mechanics::checks(),
                "water_unification" => water_unification::checks(),
                "puddle" => puddle::checks(),
                "vortex_unification" => vortex_unification::checks(),
                "variation" => variation::checks(),
                "poisson" => poisson::checks(),
                "orthogonal" => orthogonal::checks(),
                "confinement" => confinement::checks(),
                "froggatt" => froggatt::checks(),
                "fn_scan" => fn_scan::checks(),
                "modular" => modular::checks(),
                "su3" => su3::checks(),
                "geometry" => geometry::checks(),
                "anomaly" => anomaly::checks(),
                "unified_model" => unified_model::checks(),
                "shape_freedom" => shape_freedom::checks(),
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
                "watercheck" => {
                    let lc = water_mechanics::capillary_length();
                    for d in [1.0_f64, 1.5, 2.0, 2.007084679301, 2.5, 3.0] {
                        let h = d * lc;
                        let k = water_mechanics::minimum_wave_number(h);
                        let klc = k * lc;
                        println!("h_over_lc={d:.9},k_lc={klc:.12},ratio={:.12}", klc * klc);
                    }
                }
                "fnscan" => {
                    println!("charges,eps,heavy,light,joint_error");
                    for a in fn_scan::scan(14).iter().take(12) {
                        println!("{}-{}-{}, {:.9},{:.9},{:.9},{:.9}",
                                 a.charges[0], a.charges[1], a.charges[2],
                                 a.eps, a.heavy, a.light, a.error);
                    }
                }
                "confinescan" => {
                    println!("lambda,E1,E2,E3,Q");
                    for index in 0..=40 {
                        let lambda = index as f64 * 0.1;
                        let v = confinement::levels(lambda, 50, 24.0);
                        println!("{lambda:.1},{:.9},{:.9},{:.9},{:.9}",
                                 v[0], v[1], v[2], confinement::q_of_levels(&v));
                    }
                }
                "water" => {
                    let lc = water_mechanics::capillary_length();
                    println!("depth_over_lc,ratio_at_minimum,phase_speed_squared");
                    for index in 0..=60 {
                        let d = 0.1_f64.powf(-2.0 + index as f64 / 20.0);
                        let h = d * lc;
                        let k = water_mechanics::minimum_wave_number(h);
                        println!("{d:.6},{:.9},{:.9}",
                                 water_mechanics::energy_ratio_at_minimum(h),
                                 water_mechanics::phase_speed_squared(k, h));
                    }
                }
                "bridge" => {
                    let n: usize = args.get(2).and_then(|v| v.parse().ok()).unwrap_or(16);
                    let mu: f64 = args.get(3).and_then(|v| v.parse().ok()).unwrap_or(10.0);
                    let steps: usize = args.get(4).and_then(|v| v.parse().ok()).unwrap_or(240);
                    let mut field = simulation::analytic_field(n, 8.0, false);
                    let baseline = simulation::hopf_charge_spectral(&field);
                    let target = 2.0_f64.powf(-0.5);
                    let mut previous: Option<(f64, f64)> = None;
                    println!("crossing_search,n={n},mu={mu},target_Z3={target:.9}");
                    for step in 0..=steps {
                        let z3 = simulation::z3_order(&field);
                        let h = simulation::hopf_charge_spectral(&field) / baseline;
                        if z3 >= target {
                            if let Some((z0, h0)) = previous {
                                let weight = if (z3 - z0).abs() > 1e-12 {
                                    (target - z0) / (z3 - z0)
                                } else { 0.0 };
                                let h_at = h0 + weight * (h - h0);
                                let deviation = h_at - 2.0 / 3.0;
                                println!(concat!("step={},Z3_before={:.9},",
                                                 "Z3_after={:.9},H_at_target={:.9},",
                                                 "deviation_from_2_3={:.9}"),
                                         step, z0, z3, h_at, deviation);
                            }
                            break;
                        }
                        previous = Some((z3, h));
                        if step < steps {
                            let (next, _, _) =
                                simulation::relax(&field, mu, 0.05, 1, 1.0, 1.0);
                            field = next;
                        }
                    }
                }
                "xyinit" => {
                    let n = 16usize;
                    let twist: Vec<f64> = (0..n * n)
                        .map(|i| std::f64::consts::TAU * (i % n) as f64 / n as f64)
                        .collect();
                    println!("twist winding: {}", xy_z3::winding_number(&twist, n));
                    let theta = xy_z3::seeded_vortex(n);
                    println!("vortex winding: {}", xy_z3::winding_number(&theta, n));
                }
                "xyscan" => {
                    println!("start,h,J,temperature,z3,winding,energy");
                    for start in ["smooth", "vortex"] {
                        for index in 0..=20 {
                            let h = index as f64 * 0.2;
                            let initial = if start == "vortex" {
                                xy_z3::seeded_vortex(16)
                            } else {
                                let mut smooth = Vec::with_capacity(16 * 16);
                                for y in 0..16 {
                                    for x in 0..16 {
                                        smooth.push(0.1 * ((x + y) as f64 - 16.0));
                                    }
                                }
                                smooth
                            };
                            let theta = xy_z3::relax_from(
                                &initial, 16, 1.0, h, 0.5, 60, 20261122);
                            println!("{start},{h:.1},1.0,0.5,{:.9},{:.9},{:.9}",
                                     xy_z3::z3_order(&theta),
                                     xy_z3::winding_density(&theta, 16),
                                     xy_z3::energy(&theta, 16, 1.0, h));
                        }
                    }
                }
                "potts" => {
                    println!("K,ratio,Q,delta_check");
                    for index in 0..=80 {
                        let k = index as f64 * 0.025;
                        let ratio = potts_sector::transfer_ratio(k);
                        let q = potts_sector::q_signed(&potts_sector::eigenvalues(k, 0.0));
                        println!("{k:.3},{ratio:.9},{:.9},{}",
                                 q.unwrap_or(f64::NAN),
                                 if q.is_some() { "positive" } else { "negative" });
                    }
                }
                "twocomp" => {
                    let n = 16usize;
                    let topo = simulation::analytic_field(n, 8.0, false);
                    let ordered = two_component::z3_layer(n, 8.0, 0.0);
                    let h0 = simulation::hopf_charge_spectral(&topo);
                    let z1 = simulation::z3_order(&ordered);
                    println!("eta,overlap,H_ratio,Z3_ratio,product");
                    for index in 0..=20 {
                        let eta = index as f64 * 0.05;
                        let mixed = two_component::blended(&topo, &ordered, eta);
                        let hr = simulation::hopf_charge_spectral(&mixed) / h0;
                        let zr = simulation::z3_order(&mixed) / z1;
                        println!("{eta:.2},{:.9},{:.9},{:.9},{:.9}",
                                 two_component::mean_overlap(&topo, &ordered),
                                 hr, zr, hr * zr);
                    }
                }
                "rgscan" => {
                    println!("delta0,delta_mz,drift,ratio_mz");
                    for index in 0..=40 {
                        let delta0 = index as f64 * 0.025;
                        let state = rg_sector::run_high_to_mz(
                            delta0, 2.0_f64.powf(-0.5));
                        println!("{delta0:.3},{:.9},{:+.3e},{:.9}",
                                 rg_sector::phase(&state),
                                 rg_sector::phase(&state) - delta0,
                                 rg_sector::effective_ratio(&state));
                    }
                }
                "spinodal" => {
                    println!("h,n_minima,curvature_1,curvature_2,curvature_3,Q");
                    for index in 0..=90 {
                        let h = index as f64 * 0.1;
                        let values: Vec<f64> = spinodal::minima(h)
                            .iter().map(|(_, c)| *c).collect();
                        let (a, b, c) = (values.first().copied().unwrap_or(0.0),
                                         values.get(1).copied().unwrap_or(0.0),
                                         values.get(2).copied().unwrap_or(0.0));
                        println!("{h:.1},{},{:.9},{:.9},{:.9},{:.9}",
                                 values.len(), a, b, c,
                                 spinodal::q_from_curvatures(&values));
                    }
                }
                "fluxscan" => {
                    println!("alpha,e0,e1,e2,split,Q,deviation_from_2_3");
                    for index in 0..=100 {
                        let alpha = index as f64 * 0.01;
                        let v = flux_sector::flux_levels(alpha, 1.0, 50);
                        let q = flux_sector::q_levels(&[v[0], v[1], v[2]]);
                        println!("{:.2},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9}",
                                 alpha, v[0], v[1], v[2], v[2] - v[1], q, q - 2.0 / 3.0);
                    }
                }
                "potentialfamily" => {
                    println!("p,e0,e1,e2,Q,deviation_from_2_3");
                    for index in 0..=40 {
                        let p = 0.25 + index as f64 * 0.05;
                        let v = potential_family::levels(p, 1.0, 50);
                        let q = potential_family::q_levels(&[v[0], v[1], v[2]]);
                        println!("{:.2},{:.9},{:.9},{:.9},{:.9},{:.9}",
                                 p, v[0], v[1], v[2], q, q - 2.0 / 3.0);
                    }
                }
                "tune" => {
                    println!("mu,I,h_crossing,eight_thirds,relative_gap");
                    for mu in [0.25_f64, 0.5, 1.0, 1.5, 2.0, 3.0, 4.0] {
                        for inertia in [0.5_f64, 1.0, 2.0] {
                            let q = |h: f64| {
                                let levels = mass_spectrum::rotor_levels(
                                    inertia, mu, h, 60, 3);
                                mass_spectrum::koide_from_levels(
                                    &[levels[0], levels[1], levels[2]])
                            };
                            let (mut lo, mut hi) = (0.5_f64, 8.0_f64);
                            if q(lo) >= 2.0 / 3.0 || q(hi) <= 2.0 / 3.0 {
                                println!("{mu:.2},{inertia:.1},NONE,{:.9},NONE", 8.0 / 3.0);
                                continue;
                            }
                            for _ in 0..70 {
                                let mid = 0.5 * (lo + hi);
                                if q(mid) < 2.0 / 3.0 { lo = mid; } else { hi = mid; }
                            }
                            let crossing = 0.5 * (lo + hi);
                            let target = 8.0 / 3.0;
                            let gap = (crossing - target).abs() / target;
                            println!("{mu:.2},{inertia:.1},{crossing:.9},{target:.9},{gap:.6}");
                        }
                    }
                }
                "solveh" => {
                    let solve = |use_spacings: bool| -> f64 {
                        let q = |h: f64| {
                            let levels = mass_spectrum::rotor_levels(1.0, 1.0, h, 80, 3);
                            let triple = [levels[0], levels[1], levels[2]];
                            if use_spacings {
                                mass_spectrum::koide_from_spacings(&triple)
                            } else {
                                mass_spectrum::koide_from_levels(&triple)
                            }
                        };
                        let mut lo = 2.0f64;
                        let mut hi = 3.2f64;
                        for _ in 0..80 {
                            let mid = 0.5 * (lo + hi);
                            if q(mid) < 2.0 / 3.0 { lo = mid; } else { hi = mid; }
                        }
                        0.5 * (lo + hi)
                    };
                    let h_levels = solve(false);
                    let h_spacings = solve(true);
                    let levels = mass_spectrum::rotor_levels(1.0, 1.0, 8.0 / 3.0, 80, 3);
                    println!("h_crossing_levels={h_levels:.9}");
                    println!("h_crossing_spacings={h_spacings:.9}");
                    println!("eight_thirds={:.9}", 8.0 / 3.0);
                    println!("Q_at_eight_thirds_levels={:.9}",
                             mass_spectrum::koide_from_levels(&[levels[0], levels[1], levels[2]]));
                    println!("Q_at_eight_thirds_spacings={:.9}",
                             mass_spectrum::koide_from_spacings(&[levels[0], levels[1], levels[2]]));
                }
                "predict" => {
                    // Parameter-free candidate: the proven Z3 wall tension T=(8/3)sqrt(mu)
                    // supplies the symmetry-breaking field h = T/sqrt(mu) = 8/3.
                    let h = 8.0 / 3.0;
                    println!("cutoff,h,e0,e1,e2,Q,deviation_from_2_3");
                    for cutoff in [20usize, 40, 60, 80, 120, 160] {
                        let levels = mass_spectrum::rotor_levels(1.0, 1.0, h, cutoff, 3);
                        let q = mass_spectrum::koide_from_levels(&[
                            levels[0], levels[1], levels[2]]);
                        println!("{},{:.12},{:.9},{:.9},{:.9},{:.9},{:.9}",
                                 cutoff, h, levels[0], levels[1], levels[2], q,
                                 q - 2.0 / 3.0);
                    }
                }
                "rotorh" => {
                    println!("h,e0,e1,e2,split,Q,deviation_from_2_3");
                    for index in 0..=120 {
                        let h = index as f64 * 0.05;
                        let levels = mass_spectrum::rotor_levels(1.0, 1.0, h, 60, 3);
                        let q = mass_spectrum::koide_from_levels(&[
                            levels[0], levels[1], levels[2]]);
                        println!("{:.2},{:.9},{:.9},{:.9},{:.9},{:.9},{:.9}",
                                 h, levels[0], levels[1], levels[2],
                                 levels[2] - levels[1], q, q - 2.0 / 3.0);
                    }
                }
                "phaseattr" => {
                    let n: usize = args.get(2).and_then(|v| v.parse().ok()).unwrap_or(32);
                    let mu: f64 = args.get(3).and_then(|v| v.parse().ok()).unwrap_or(5.0);
                    let steps = 160usize;
                    let baseline = simulation::hopf_charge_spectral(
                        &simulation::analytic_field(n, 8.0, false));
                    println!(concat!("n,mu,initial_theta,initial_phase,final_phase,",
                                     "phase_deg,z3,hopf_ratio,force_rms"));
                    for index in 0..12 {
                        let theta = std::f64::consts::PI * index as f64 / 6.0;
                        let seed = simulation::phase_rotated_field(n, 8.0, theta);
                        let initial_phase = simulation::z3_phase(&seed);
                        let (relaxed, _, m) =
                            simulation::relax(&seed, mu, 0.05, steps, 1.0, 1.0);
                        let final_phase = simulation::z3_phase(&relaxed);
                        let h = simulation::hopf_charge_spectral(&relaxed);
                        let (z3, force_rms) = (m.z3, m.force_rms);
                        println!(concat!("{},{:.1},{:.6},{:.9},{:.9},{:.3},",
                                         "{:.9},{:.9},{:.9}"),
                                 n, mu, theta, initial_phase, final_phase,
                                 final_phase.to_degrees(), z3, h / baseline, force_rms);
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
