//! Two-component ansatz: a topological H=1 layer and a Z3-ordered layer.
//!
//! Instead of asking one field to carry both observables, we couple two unit
//! fields on the same grid: `n` carries the Hopf texture and `m` is driven to
//! the Z3 minima. The coupling is a linear alignment term of strength eta.
//! This module tests whether coexistence is possible when the two sectors are
//! allowed to be distinct but coupled.

use crate::Check;
use crate::simulation::{Field3, analytic_field, hopf_charge_spectral, normalize_field, z3_order};

/// A spatially varying Z3-ordered target: three azimuthal minima of 1-cos(3phi).
pub fn z3_layer(n: usize, length: f64, phase: f64) -> Field3 {
    let dx = length / n as f64;
    let half = 0.5 * length;
    let mut data = Vec::with_capacity(n * n * n);
    for x in 0..n {
        let cx = -half + x as f64 * dx;
        for y in 0..n {
            let cy = -half + y as f64 * dx;
            for _z in 0..n {
                let phi = cy.atan2(cx) + phase;
                let rho = (cx * cx + cy * cy).sqrt();
                let w = (rho / (1.0 + rho)).clamp(0.0, 1.0);
                let (s, c) = (3.0 * phi).sin_cos();
                let v = [w * c, w * s, (1.0 - w * w).sqrt().max(0.0)];
                let magnitude = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
                data.push(if magnitude > 1e-12 {
                    [v[0] / magnitude, v[1] / magnitude, v[2] / magnitude]
                } else {
                    [0.0, 0.0, 1.0]
                });
            }
        }
    }
    Field3 { n, dx, data }
}

/// Overlap of two fields at every site, averaged over the grid.
pub fn mean_overlap(a: &Field3, b: &Field3) -> f64 {
    let mut total = 0.0;
    for (u, v) in a.data.iter().zip(b.data.iter()) {
        total += u[0] * v[0] + u[1] * v[1] + u[2] * v[2];
    }
    total / a.data.len() as f64
}

/// Blend the two layers with weight `eta` in [0,1] and renormalise.
pub fn blended(topological: &Field3, ordered: &Field3, eta: f64) -> Field3 {
    let mut out = topological.clone();
    for (site, target) in out.data.iter_mut().zip(ordered.data.iter()) {
        for c in 0..3 {
            site[c] = (1.0 - eta) * site[c] + eta * target[c];
        }
    }
    normalize_field(&mut out);
    out
}

pub fn checks() -> Vec<Check> {
    let n = 16usize;
    let topo = analytic_field(n, 8.0, false);
    let ordered = z3_layer(n, 8.0, 0.0);
    let h_topo = hopf_charge_spectral(&topo);
    let z_ordered = z3_order(&ordered);
    let mixed = blended(&topo, &ordered, 0.5);
    vec![
        Check::new("two_component.hopf_topological", h_topo, "H of the pure topological layer"),
        Check::new("two_component.z3_ordered", z_ordered, "Z3 of the pure ordered layer"),
        Check::new("two_component.z3_mixed", z3_order(&mixed), "Z3 at a 50/50 blend"),
        Check::new("two_component.overlap", mean_overlap(&topo, &ordered),
                   "mean site overlap of the two layers"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linalg::approx;

    fn small() -> (Field3, Field3) {
        (analytic_field(8, 8.0, false), z3_layer(8, 8.0, 0.0))
    }

    #[test]
    fn layers_are_unit_fields() {
        let (a, b) = small();
        assert!(a.data.iter().all(|v| approx(
            (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt(), 1.0, 1e-12)));
        assert!(b.data.iter().all(|v| approx(
            (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt(), 1.0, 1e-12)));
    }

    #[test]
    fn ordered_layer_is_anisotropic() {
        let (_, b) = small();
        // The radial weight makes the Z3 order finite but not unity; the
        // important structural fact is that it is small, not that it saturates.
        let value = z3_order(&b);
        assert!(value > 0.0 && value < 1.0, "Z3 = {value}");
    }

    #[test]
    fn blend_changes_both_observables() {
        let n = 16usize;
        let topo = analytic_field(n, 8.0, false);
        let ordered = z3_layer(n, 8.0, 0.0);
        let h0 = hopf_charge_spectral(&topo);
        let z0 = z3_order(&ordered);
        let mixed = blended(&topo, &ordered, 0.5);
        assert!(z3_order(&mixed) != z0);
        assert!(hopf_charge_spectral(&mixed) != h0);
    }

    #[test]
    fn overlap_is_bounded() {
        let (a, b) = small();
        let value = mean_overlap(&a, &b);
        assert!((-1.0..=1.0).contains(&value));
    }
}
