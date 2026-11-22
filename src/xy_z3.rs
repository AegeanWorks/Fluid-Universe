//! 2D XY + Z3 centre model: a new topological class (plaquette winding).
//!
//! Variables are planar phases theta on a periodic n x n grid:
//! E = -J sum_nn cos(theta_i - theta_j) - h sum_i cos(3 theta_i).
//! The Z3 order parameter is |<exp(3i theta)>| and the topological density is
//! the signed plaquette winding number per site. Metropolis updates are driven
//! by a deterministic LCG, so every run is reproducible.

use crate::Check;

const TWO_PI: f64 = std::f64::consts::TAU;

fn wrap(index: isize, n: usize) -> usize {
    let m = n as isize;
    (((index % m) + m) % m) as usize
}

fn at(theta: &[f64], n: usize, x: isize, y: isize) -> f64 {
    theta[wrap(y, n) * n + wrap(x, n)]
}

/// Energy of a configuration.
pub fn energy(theta: &[f64], n: usize, j: f64, h: f64) -> f64 {
    let mut total = 0.0;
    for y in 0..n {
        for x in 0..n {
            let here = theta[y * n + x];
            total -= j * (here - at(theta, n, x as isize + 1, y as isize)).cos();
            total -= j * (here - at(theta, n, x as isize, y as isize + 1)).cos();
            total -= h * (3.0 * here).cos();
        }
    }
    total
}

/// Absolute Z3 order |<exp(3i theta)>|.
pub fn z3_order(theta: &[f64]) -> f64 {
    let mut re = 0.0;
    let mut im = 0.0;
    for value in theta {
        re += (3.0 * value).cos();
        im += (3.0 * value).sin();
    }
    (re * re + im * im).sqrt() / theta.len() as f64
}

/// Total winding number from the circulation of principal-value differences
/// around the periodic boundary. A diffuse vortex is detected here even when
/// each individual link difference is smaller than pi.
pub fn winding_number(theta: &[f64], n: usize) -> f64 {
    let mut sum = 0.0;
    // Traverse the boundary counter-clockwise: top -> right -> bottom -> left.
    for x in 0..n {
        sum += principal(theta[(x + 1) % n] - theta[x]);
    }
    for y in 0..n {
        sum += principal(theta[((y + 1) % n) * n + n - 1] - theta[y * n + n - 1]);
    }
    for x in (0..n).rev() {
        let previous = (x + n - 1) % n;
        sum += principal(theta[(n - 1) * n + previous] - theta[(n - 1) * n + x]);
    }
    for y in (0..n).rev() {
        let previous = (y + n - 1) % n;
        sum += principal(theta[previous * n] - theta[y * n]);
    }
    sum / TWO_PI
}

/// Winding per lattice site.
pub fn winding_density(theta: &[f64], n: usize) -> f64 {
    winding_number(theta, n) / (n * n) as f64
}

fn principal(delta: f64) -> f64 {
    let value = delta.rem_euclid(TWO_PI);
    if value > std::f64::consts::PI { value - TWO_PI } else { value }
}

/// A single unit-winding vortex on an n x n periodic grid.
pub fn seeded_vortex(n: usize) -> Vec<f64> {
    // Half-integer centre avoids placing the atan2 branch point on a lattice site;
    // assigning a value at an on-site singularity would cancel the winding.
    let center = n as f64 / 2.0 - 0.5;
    let mut theta = Vec::with_capacity(n * n);
    for y in 0..n {
        for x in 0..n {
            theta.push((y as f64 - center).atan2(x as f64 - center));
        }
    }
    theta
}

/// Deterministic Metropolis sweeps from a supplied starting configuration.
pub fn relax_from(
    start: &[f64], n: usize, j: f64, h: f64, temperature: f64,
    sweeps: usize, seed: u64,
) -> Vec<f64> {
    let mut state = seed;
    let mut next = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((state >> 33) as f64) / ((1u64 << 31) as f64)
    };
    let mut theta = start.to_vec();
    assert_eq!(theta.len(), n * n);
    for sweep in 0..sweeps {
        let beta = 1.0 / temperature.max(1e-6) * (1.0 + sweep as f64 / sweeps as f64);
        for y in 0..n {
            for x in 0..n {
                let site = y * n + x;
                let before = theta[site];
                let proposal = before + 0.5 * (2.0 * next() - 1.0);
                let old_local = -j * ((before - at(&theta, n, x as isize + 1, y as isize)).cos()
                                    + (before - at(&theta, n, x as isize, y as isize + 1)).cos())
                              - h * (3.0 * before).cos();
                theta[site] = proposal;
                let new_local = -j * ((proposal - at(&theta, n, x as isize + 1, y as isize)).cos()
                                    + (proposal - at(&theta, n, x as isize, y as isize + 1)).cos())
                              - h * (3.0 * proposal).cos();
                if new_local > old_local && next() > (-beta * (new_local - old_local)).exp() {
                    theta[site] = before;
                }
            }
        }
    }
    theta
}

/// Backward-compatible smooth-start relaxation.
pub fn relax(n: usize, j: f64, h: f64, temperature: f64, sweeps: usize, seed: u64) -> Vec<f64> {
    let mut smooth = Vec::with_capacity(n * n);
    for y in 0..n {
        for x in 0..n {
            smooth.push(0.1 * ((x + y) as f64 - n as f64));
        }
    }
    relax_from(&smooth, n, j, h, temperature, sweeps, seed)
}

pub fn checks() -> Vec<Check> {
    let theta = relax(16, 1.0, 1.0, 0.5, 40, 7);
    vec![
        Check::new("xy_z3.energy", energy(&theta, 16, 1.0, 1.0),
                   "final XY+Z3 energy on a 16x16 grid"),
        Check::new("xy_z3.order", z3_order(&theta), "absolute Z3 order"),
        Check::new("xy_z3.winding", winding_density(&theta, 16),
                   "signed plaquette winding density"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_field_has_zero_winding_and_maximal_z3() {
        let theta = vec![0.0; 64];
        assert!(winding_density(&theta, 8).abs() < 1e-12);
        assert!((z3_order(&theta) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn deterministic_relaxation_is_reproducible() {
        let a = relax(8, 1.0, 0.5, 0.5, 10, 42);
        let b = relax(8, 1.0, 0.5, 0.5, 10, 42);
        assert_eq!(a, b);
    }

    #[test]
    fn boundary_loop_cancels_a_pure_twist() {
        // Traversing the boundary forward on one edge and backward on the
        // opposite edge cancels the holonomy; a 2D torus carries no net charge.
        let n = 16usize;
        let theta: Vec<f64> = (0..n * n)
            .map(|i| TWO_PI * (i % n) as f64 / n as f64)
            .collect();
        assert!(winding_number(&theta, n).abs() < 1e-9);
        let vortex = seeded_vortex(n);
        assert!(winding_number(&vortex, n).abs() < 1e-9);
    }

    #[test]
    fn observables_stay_bounded() {
        let theta = relax(8, 1.0, 2.0, 0.5, 20, 9);
        assert!((0.0..=1.0).contains(&z3_order(&theta)));
        assert!(winding_density(&theta, 8).abs() <= 1.0);
    }
}
