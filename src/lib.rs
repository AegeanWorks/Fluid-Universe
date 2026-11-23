//! fluid-universe: falsification-first research core, rewritten in Rust.
//!
//! The crate is dependency-free on purpose: a cold `cargo test` must finish in
//! milliseconds with no network, no Python and no native libraries. Every
//! module ports one family of checks from the former Python implementation;
//! `verify` re-derives the results in two separate processes (project rule G-001).

pub mod amplitude_seesaw;
pub mod anomaly;
pub mod audit;
pub mod circulant;
pub mod circulant_inverse;
pub mod classical_scan;
pub mod conformal_block;
pub mod compact_identity;
pub mod confinement;
pub mod d3_chain;
pub mod d3_neutrino;
pub mod d3_only_synthesis;
pub mod d3_running;
pub mod d3_spectrum;
pub mod d3_validation;
pub mod derrick;
pub mod dynamical_flow;
pub mod ew_from_planck;
pub mod exceptional_nonsimply;
pub mod exceptional_scan;
pub mod fib_significance;
pub mod fft;
pub mod fibonacci_phase;
pub mod gauge_flavor;
pub mod gauge_running;
pub mod fn_scan;
pub mod flux_quantization;
pub mod foot_geometry;
pub mod fluid;
pub mod geometry;
pub mod froggatt;
pub mod gravity_relation;
pub mod index_framework;
pub mod informational_geometry;
pub mod index_uncertainty;
pub mod flux_sector;
pub mod jordan_spectrum;
pub mod linalg;
pub mod lorentz;
pub mod mass_convention;
pub mod mass_spectrum;
pub mod monodromy_block;
pub mod modular;
pub mod nosusy_unification;
pub mod null_model;
pub mod monodromy;
pub mod poisson;
pub mod orthogonal;
pub mod phase_interpolation;
pub mod peer_reviewed_37;
pub mod potential_family;
pub mod phase_arithmetic;
pub mod puddle;
pub mod quark_phases;
pub mod quark_rg;
pub mod potts_sector;
pub mod rg_sector;
pub mod shape_freedom;
pub mod reports;
pub mod sector_count;
pub mod rep_scan;
pub mod rotor;
pub mod scale_choice;
pub mod simulation;
pub mod single_phase_rule;
pub mod spectra;
pub mod su3;
pub mod so10_gut;
pub mod spinodal;
pub mod two_component;
pub mod unified_model;
pub mod up_phase_resolution;
pub mod up_amplitude_gauge;
pub mod unification;
pub mod variation;
pub mod verify;
pub mod vortex_unification;
pub mod water;
pub mod water_mechanics;
pub mod water_unification;
pub mod wzw;
pub mod wzw_crosscheck;
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
    out.extend(unified_model::checks());
    out.extend(shape_freedom::checks());
    out.extend(flux_quantization::checks());
    out.extend(wzw::checks());
    out.extend(wzw_crosscheck::checks());
    out.extend(quark_phases::checks());
    out.extend(circulant_inverse::checks());
    out.extend(phase_arithmetic::checks());
    out.extend(conformal_block::checks());
    out.extend(monodromy::checks());
    out.extend(monodromy_block::checks());
    out.extend(mass_convention::checks());
    out.extend(fibonacci_phase::checks());
    out.extend(gauge_running::checks());
    out.extend(exceptional_scan::checks());
    out.extend(exceptional_nonsimply::checks());
    out.extend(so10_gut::checks());
    out.extend(amplitude_seesaw::checks());
    out.extend(index_framework::checks());
    out.extend(index_uncertainty::checks());
    out.extend(gravity_relation::checks());
    out.extend(compact_identity::checks());
    out.extend(classical_scan::checks());
    out.extend(dynamical_flow::checks());
    out.extend(null_model::checks());
    out.extend(fib_significance::checks());
    out.extend(gauge_flavor::checks());
    out.extend(scale_choice::checks());
    out.extend(informational_geometry::checks());
    out.extend(d3_validation::checks());
    out.extend(d3_chain::checks());
    out.extend(d3_spectrum::checks());
    out.extend(d3_neutrino::checks());
    out.extend(d3_running::checks());
    out.extend(foot_geometry::checks());
    out.extend(nosusy_unification::checks());
    out.extend(sector_count::checks());
    out.extend(ew_from_planck::checks());
    out.extend(d3_only_synthesis::checks());
    out.extend(phase_interpolation::checks());
    out.extend(peer_reviewed_37::checks());
    out.extend(up_phase_resolution::checks());
    out.extend(single_phase_rule::checks());
    out.extend(up_amplitude_gauge::checks());
    out.extend(rep_scan::checks());
    out.extend(jordan_spectrum::checks());
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
    out.extend(orthogonal::checks());
    out.extend(confinement::checks());
    out.extend(froggatt::checks());
    out.extend(fn_scan::checks());
    out.extend(modular::checks());
    out.extend(su3::checks());
    out.extend(geometry::checks());
    out.extend(anomaly::checks());
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
