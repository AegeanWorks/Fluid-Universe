//! fluid-universe: falsification-first research core, rewritten in Rust.
//!
//! The crate is dependency-free on purpose: a cold `cargo test` must finish in
//! milliseconds with no network, no Python and no native libraries. Every
//! module ports one family of checks from the former Python implementation;
//! `verify` re-derives the results in two separate processes (project rule G-001).

pub mod audit;
pub mod circulant;
pub mod derrick;
pub mod fft;
pub mod fluid;
pub mod flux_sector;
pub mod linalg;
pub mod lorentz;
pub mod mass_spectrum;
pub mod poisson;
pub mod potential_family;
pub mod puddle;
pub mod quark_rg;
pub mod potts_sector;
pub mod rg_sector;
pub mod reports;
pub mod rotor;
pub mod simulation;
pub mod spectra;
pub mod spinodal;
pub mod two_component;
pub mod unification;
pub mod variation;
pub mod verify;
pub mod vortex_unification;
pub mod water;
pub mod water_mechanics;
pub mod water_unification;
pub mod xy_z3;

use std::fmt;

/// One independently checkable result line, used by the CLI and the two-run audit.
#[derive(Clone, Debug, PartialEq)]
pub struct Check {
    pub id: &'static str,
    pub detail: String,
    pub value: f64,
}

impl Check {
    pub fn new(id: &'static str, value: f64, detail: impl Into<String>) -> Self {
        Self { id, value, detail: detail.into() }
    }
}

impl fmt::Display for Check {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<28} {:<22.12e}  {}", self.id, self.value, self.detail)
    }
}

/// Every check the CLI can reproduce; the order is part of the audit fingerprint.
pub fn all_checks() -> Vec<Check> {
    let mut out = Vec::new();
    out.extend(circulant::checks());
    out.extend(spectra::checks());
    out.extend(rotor::checks());
    out.extend(derrick::checks());
    out.extend(fluid::checks());
    out.extend(water::checks());
    out.extend(lorentz::checks());
    out.extend(simulation::checks());
    out.extend(audit::checks());
    out.extend(reports::checks());
    out.extend(unification::checks());
    out.extend(mass_spectrum::checks());
    out.extend(potential_family::checks());
    out.extend(flux_sector::checks());
    out.extend(spinodal::checks());
    out.extend(rg_sector::checks());
    out.extend(two_component::checks());
    out.extend(potts_sector::checks());
    out.extend(xy_z3::checks());
    out.extend(quark_rg::checks());
    out.extend(water_mechanics::checks());
    out.extend(water_unification::checks());
    out.extend(puddle::checks());
    out.extend(vortex_unification::checks());
    out.extend(variation::checks());
    out.extend(poisson::checks());
    out
}

/// Stable text fingerprint of all checks, used by the two-process comparison.
pub fn fingerprint(checks: &[Check]) -> String {
    use std::fmt::Write;
    let mut text = String::new();
    for check in checks {
        let _ = writeln!(text, "{}|{:.15e}|{}", check.id, check.value, check.detail);
    }
    // FNV-1a, no dependency.
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in text.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{:016x}", hash)
}
