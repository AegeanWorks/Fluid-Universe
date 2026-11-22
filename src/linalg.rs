//! Small numerical kernels used by the physics modules. No external crates.

/// Ascending eigenvalues of a small real symmetric matrix, Jacobi rotations.
///
/// The matrices here are at most a few hundred rows (rotor sector Hamiltonians),
/// so a dense Jacobi sweep is both simpler and faster to compile than an LAPACK
/// dependency. Convergence is to machine precision in O(6-10) sweeps.
pub fn symmetric_eigenvalues(matrix: &[Vec<f64>]) -> Vec<f64> {
    let n = matrix.len();
    assert!(n > 0 && matrix.iter().all(|row| row.len() == n), "matrix must be square");
    let mut a = matrix.to_vec();
    // Cyclic Jacobi: one *sweep* rotates every upper-triangle pair. A 107x107
    // rotor sector needs thousands of rotations, so a single largest-pivot
    // rotation per iteration would stop far short of machine precision.
    for _sweep in 0..100 {
        let mut off = 0.0f64;
        for i in 0..n {
            for j in (i + 1)..n {
                if a[i][j].abs() < 1e-18 {
                    continue;
                }
                off = off.max(a[i][j].abs());
                let theta = (a[j][j] - a[i][i]) / (2.0 * a[i][j]);
                let t = if theta >= 0.0 {
                    1.0 / (theta + (1.0 + theta * theta).sqrt())
                } else {
                    -1.0 / (-theta + (1.0 + theta * theta).sqrt())
                };
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = t * c;
                let app = a[i][i];
                let aqq = a[j][j];
                let apq = a[i][j];
                a[i][i] = app - t * apq;
                a[j][j] = aqq + t * apq;
                a[i][j] = 0.0;
                a[j][i] = 0.0;
                for k in 0..n {
                    if k == i || k == j {
                        continue;
                    }
                    let akp = a[k][i];
                    let akq = a[k][j];
                    a[k][i] = c * akp - s * akq;
                    a[i][k] = a[k][i];
                    a[k][j] = s * akp + c * akq;
                    a[j][k] = a[k][j];
                }
            }
        }
        if off < 1e-14 {
            break;
        }
    }
    let mut values: Vec<f64> = (0..n).map(|i| a[i][i]).collect();
    values.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    values
}

/// Composite Simpson quadrature on `[a, b]` with an even number of panels.
pub fn simpson(f: impl Fn(f64) -> f64, a: f64, b: f64, panels: usize) -> f64 {
    let n = panels + (panels % 2);
    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);
    for i in 1..n {
        let weight = if i % 2 == 0 { 2.0 } else { 4.0 };
        sum += weight * f(a + h * i as f64);
    }
    sum * h / 3.0
}

/// Bisection root of a sign-changing function; `None` when the bracket is invalid.
pub fn bisect(f: impl Fn(f64) -> f64, mut lo: f64, mut hi: f64, tol: f64) -> Option<f64> {
    let flo = f(lo);
    let fhi = f(hi);
    if flo * fhi > 0.0 {
        return None;
    }
    for _ in 0..256 {
        let mid = 0.5 * (lo + hi);
        let fm = f(mid);
        if fm.abs() < tol || (hi - lo).abs() < tol {
            return Some(mid);
        }
        if flo * fm <= 0.0 {
            hi = mid;
        } else {
            lo = mid;
            // flo is only read once; recompute so a non-monotone bracket still works.
        }
    }
    Some(0.5 * (lo + hi))
}

/// Relative-or-absolute closeness with a caller-chosen tolerance.
pub fn approx(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol * b.abs().max(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jacobi_diagonal_symmetric() {
        let m = vec![vec![2.0, 1.0], vec![1.0, 2.0]];
        let v = symmetric_eigenvalues(&m);
        assert!(approx(v[0], 1.0, 1e-12));
        assert!(approx(v[1], 3.0, 1e-12));
    }

    #[test]
    fn simpson_exact_for_quadratic() {
        let got = simpson(|x| x * x, 0.0, 1.0, 64);
        assert!(approx(got, 1.0 / 3.0, 1e-12));
    }

    #[test]
    fn bisect_finds_root() {
        let root = bisect(|x| x * x - 2.0, 1.0, 2.0, 1e-12).unwrap();
        assert!(approx(root, 2.0_f64.sqrt(), 1e-10));
    }
}
