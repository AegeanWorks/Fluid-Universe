//! Independent local-energy finite-difference check of the guarded probe.

/// Fixed short-probe result; the implementation does not call the production
/// field, force, energy, relaxation or homotopy-guard functions.
#[derive(Clone, Debug)]
pub struct IndependentProbe {
    pub steps: usize,
    pub minimum_margin: f64,
    pub mean_minimum_margin: f64,
    pub mean_inconclusive_steps: usize,
    pub inconclusive_steps: usize,
    pub first_inconclusive_step: Option<usize>,
    pub start_energy: f64,
    pub end_energy: f64,
}

fn initial_samples(n: usize) -> Vec<[f64; 3]> {
    let mut data = Vec::with_capacity(n * n * n);
    let dx = 8.0 / n as f64;
    for x in 0..n { for y in 0..n { for z in 0..n {
        let (x, y, z) = (x as f64 * dx - 4.0, y as f64 * dx - 4.0, z as f64 * dx - 4.0);
        let r2 = x*x + y*y + z*z;
        let den = 1.0 + r2;
        let (q0, q1, q2, q3) = ((r2 - 1.0)/den, 2.0*x/den, 2.0*y/den, 2.0*z/den);
        let t = ((4.0 - r2.sqrt()) / (4.0 - 0.85 * 4.0)).clamp(0.0, 1.0);
        let w = t*t*(3.0 - 2.0*t);
        let mut v = [2.0*w*(q0*q2 + q1*q3), 2.0*w*(q1*q2 - q0*q3),
                     w*(q0*q0 + q1*q1 - q2*q2 - q3*q3) + 1.0 - w];
        let len = (v[0]*v[0] + v[1]*v[1] + v[2]*v[2]).sqrt();
        if len > 1e-12 { for component in &mut v { *component /= len; } }
        else { v = [0.0, 0.0, 1.0]; }
        data.push(v);
    }}}
    data
}

/// Energy contribution at one periodic cell for the fixed mu=20, K=kappa=1.
fn cell_energy(n: usize, field: &[[f64; 3]], site: usize) -> f64 {
    let (x, y, z) = (site / (n*n), (site / n) % n, site % n);
    let idx = |x: usize, y: usize, z: usize| (x*n + y)*n + z;
    let plus = [idx((x+1)%n,y,z), idx(x,(y+1)%n,z), idx(x,y,(z+1)%n)];
    let minus = [idx((x+n-1)%n,y,z), idx(x,(y+n-1)%n,z), idx(x,y,(z+n-1)%n)];
    let dx = 8.0 / n as f64;
    let mut d = [[0.0; 3]; 3];
    for dir in 0..3 { for c in 0..3 { d[dir][c] = (field[plus[dir]][c] - field[minus[dir]][c]) / (2.0*dx); } }
    let v = field[site];
    let mut e2 = 0.0;
    let mut e4 = 0.0;
    for deriv in &d { for c in deriv { e2 += c*c; } }
    for (i, j) in [(0,1),(0,2),(1,2)] {
        let cross = [d[i][1]*d[j][2]-d[i][2]*d[j][1], d[i][2]*d[j][0]-d[i][0]*d[j][2], d[i][0]*d[j][1]-d[i][1]*d[j][0]];
        let f = v[0]*cross[0] + v[1]*cross[1] + v[2]*cross[2];
        e4 += f*f;
    }
    let rho2 = v[0]*v[0] + v[1]*v[1];
    let s = rho2 + 1e-12;
    let cos3 = (v[0]*v[0]*v[0] - 3.0*v[0]*v[1]*v[1]) / (s*s.sqrt());
    dx*dx*dx * (0.5*e2 + 0.5*e4 + 20.0*rho2*(1.0-cos3))
}

/// Centered differences of only the seven affected local energies at each site.
fn finite_difference_gradient(n: usize, field: &mut [[f64; 3]], epsilon: f64) -> Vec<[f64; 3]> {
    assert!(n >= 3 && field.len() == n*n*n && epsilon > 0.0);
    let idx = |x: usize, y: usize, z: usize| (x*n + y)*n + z;
    let mut gradient = vec![[0.0; 3]; field.len()];
    for x in 0..n { for y in 0..n { for z in 0..n {
        let site = idx(x,y,z);
        let affected = [site, idx((x+1)%n,y,z), idx((x+n-1)%n,y,z),
            idx(x,(y+1)%n,z), idx(x,(y+n-1)%n,z), idx(x,y,(z+1)%n), idx(x,y,(z+n-1)%n)];
        for axis in 0..3 {
            let old = field[site][axis];
            field[site][axis] = old + epsilon;
            let plus: f64 = affected.iter().map(|&p| cell_energy(n, field, p)).sum();
            field[site][axis] = old - epsilon;
            let minus: f64 = affected.iter().map(|&p| cell_energy(n, field, p)).sum();
            field[site][axis] = old;
            gradient[site][axis] = (plus - minus) / (2.0*epsilon);
        }
    }}}
    gradient
}

/// Independent distance-based hemisphere margin for normalized samples.
fn distance_margin(n: usize, first: &[[f64; 3]], second: &[[f64; 3]]) -> f64 {
    let idx = |x: usize, y: usize, z: usize| (x*n + y)*n + z;
    let mut minimum = 1.0_f64;
    for x in 0..n { for y in 0..n { for z in 0..n {
        let a = first[idx(x,y,z)];
        for dx in 0..=1 { for dy in 0..=1 { for dz in 0..=1 {
            let site = idx((x+dx)%n,(y+dy)%n,(z+dz)%n);
            for v in [first[site], second[site]] {
                let distance2: f64 = (0..3).map(|i| (a[i]-v[i]).powi(2)).sum();
                minimum = minimum.min(1.0 - distance2/2.0);
            }
        }}}
    }}}
    minimum
}

// Compute projections from distances to the normalized sample centroid.
fn centroid_distance_margin(n: usize, first: &[[f64; 3]], second: &[[f64; 3]]) -> f64 {
    let mut minimum = 1.0_f64;
    for x in 0..n { for y in 0..n { for z in 0..n {
        let mut corners = Vec::with_capacity(16);
        for dx in 0..=1 { for dy in 0..=1 { for dz in 0..=1 {
            let i = (((x+dx)%n)*n + (y+dy)%n)*n + (z+dz)%n;
            corners.push(first[i]); corners.push(second[i]);
        }}}
        let sum = [0,1,2].map(|axis| corners.iter().map(|v| v[axis]).sum::<f64>());
        let length = sum.iter().map(|v| v*v).sum::<f64>().sqrt();
        if length <= 1e-12 { return 0.0; }
        let direction = sum.map(|v| v / length);
        for v in corners {
            let distance2 = (0..3).map(|i| (direction[i]-v[i]).powi(2)).sum::<f64>();
            minimum = minimum.min(1.0 - distance2/2.0);
        }
    }}}
    minimum
}

/// Independently rederive an N=32 probe using epsilon=1e-5, for 1..=200 steps.
pub fn run_independent_probe(steps: usize) -> IndependentProbe {
    assert!((1..=200).contains(&steps), "independent probe needs 1..=200 steps");
    let (n, epsilon, dt) = (32_usize, 1e-5_f64, 0.05_f64);
    let mut field = initial_samples(n);
    let energy = |f: &[[f64; 3]]| (0..f.len()).map(|p| cell_energy(n, f, p)).sum::<f64>();
    let start_energy = energy(&field);
    let mut minimum_margin = 1.0_f64;
    let mut mean_minimum_margin = 1.0_f64;
    let mut mean_inconclusive_steps = 0;
    let mut inconclusive_steps = 0;
    let mut first_inconclusive_step = None;
    for step in 1..=steps {
        let gradient = finite_difference_gradient(n, &mut field, epsilon);
        let norm = gradient.iter().flat_map(|v| v.iter()).map(|v| v*v).sum::<f64>().sqrt();
        assert!(norm.is_finite() && norm > 1e-14, "independent gradient must be finite and nonzero");
        let mut next = field.clone();
        for (v, g) in next.iter_mut().zip(gradient) {
            for c in 0..3 { v[c] -= dt * g[c] / norm; }
            let len = (v[0]*v[0] + v[1]*v[1] + v[2]*v[2]).sqrt();
            for c in v { *c /= len; }
        }
        let margin = distance_margin(n, &field, &next);
        let mean = centroid_distance_margin(n, &field, &next);
        mean_minimum_margin = mean_minimum_margin.min(mean);
        if mean <= 1e-12 { mean_inconclusive_steps += 1; }
        minimum_margin = minimum_margin.min(margin);
        if margin <= 1e-12 {
            inconclusive_steps += 1;
            if first_inconclusive_step.is_none() { first_inconclusive_step = Some(step); }
        }
        field = next;
    }
    IndependentProbe { steps, minimum_margin, mean_minimum_margin, mean_inconclusive_steps, inconclusive_steps, first_inconclusive_step, start_energy, end_energy: energy(&field) }
}

#[cfg(test)]
mod tests {
    use super::{cell_energy, finite_difference_gradient, initial_samples};

    #[test]
    fn local_gradient_matches_full_energy_difference_on_small_grid() {
        let n = 4;
        let mut field = initial_samples(n);
        let original = field.clone();
        let gradient = finite_difference_gradient(n, &mut field, 1e-5);
        assert_eq!(field, original);
        let total = |f: &[[f64; 3]]| (0..f.len()).map(|p| cell_energy(n, f, p)).sum::<f64>();
        for (site, axis) in [(0, 0), (7, 1), (35, 2)] {
            let value = field[site][axis];
            field[site][axis] = value + 1e-5;
            let plus = total(&field);
            field[site][axis] = value - 1e-5;
            let minus = total(&field);
            field[site][axis] = value;
            assert!((gradient[site][axis] - (plus - minus) / 2e-5).abs() < 1e-7);
        }
    }
}
