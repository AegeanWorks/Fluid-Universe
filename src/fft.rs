//! Minimal radix-2 FFT for power-of-two 3D grids. No external crates.
//!
//! Used by the spectral Hopf estimator to solve the periodic Poisson problem
//! exactly for each Fourier mode, replacing the slow Jacobi iteration.

use std::f64::consts::PI;

fn bit_reversed(n: usize, i: usize) -> usize {
    let mut value = 0usize;
    let mut index = i;
    let mut bits = n.trailing_zeros();
    while bits > 0 {
        bits -= 1;
        value = (value << 1) | (index & 1);
        index >>= 1;
    }
    value
}

/// In-place complex radix-2 FFT along one line. `inverse=true` divides by n.
pub fn fft1d(re: &mut [f64], im: &mut [f64], inverse: bool) {
    let n = re.len();
    assert!(n.is_power_of_two() && n > 1 && im.len() == n);
    for i in 0..n {
        let j = bit_reversed(n, i);
        if j > i {
            re.swap(i, j);
            im.swap(i, j);
        }
    }
    let mut len = 2;
    while len <= n {
        let angle = if inverse { 2.0 * PI / len as f64 } else { -2.0 * PI / len as f64 };
        let (wr_step, wi_step) = (angle.cos(), angle.sin());
        for start in (0..n).step_by(len) {
            let mut wr = 1.0f64;
            let mut wi = 0.0f64;
            for k in 0..len / 2 {
                let a = start + k;
                let b = a + len / 2;
                let vr = re[b] * wr - im[b] * wi;
                let vi = re[b] * wi + im[b] * wr;
                let ur = re[a];
                let ui = im[a];
                re[a] = ur + vr;
                im[a] = ui + vi;
                re[b] = ur - vr;
                im[b] = ui - vi;
                let next = wr * wr_step - wi * wi_step;
                wi = wr * wi_step + wi * wr_step;
                wr = next;
            }
        }
        len *= 2;
    }
    if inverse {
        let scale = 1.0 / n as f64;
        for i in 0..n {
            re[i] *= scale;
            im[i] *= scale;
        }
    }
}

/// 3D FFT over a flat row-major `(x*n + y)*n + z` field.
pub fn fft3d(re: &mut [f64], im: &mut [f64], n: usize, inverse: bool) {
    assert!(n.is_power_of_two() && n >= 2, "fft3d: n must be a power of two >= 2");
    assert_eq!(re.len(), n * n * n);
    assert_eq!(im.len(), re.len());
    for axis in 0..3 {
        let mut line_r = vec![0.0; n];
        let mut line_i = vec![0.0; n];
        match axis {
            0 => {
                for y in 0..n {
                    for z in 0..n {
                        for x in 0..n {
                            let at = (x * n + y) * n + z;
                            line_r[x] = re[at];
                            line_i[x] = im[at];
                        }
                        fft1d(&mut line_r, &mut line_i, inverse);
                        for x in 0..n {
                            let at = (x * n + y) * n + z;
                            re[at] = line_r[x];
                            im[at] = line_i[x];
                        }
                    }
                }
            }
            1 => {
                for x in 0..n {
                    for z in 0..n {
                        for y in 0..n {
                            let at = (x * n + y) * n + z;
                            line_r[y] = re[at];
                            line_i[y] = im[at];
                        }
                        fft1d(&mut line_r, &mut line_i, inverse);
                        for y in 0..n {
                            let at = (x * n + y) * n + z;
                            re[at] = line_r[y];
                            im[at] = line_i[y];
                        }
                    }
                }
            }
            _ => {
                for x in 0..n {
                    for y in 0..n {
                        let base = (x * n + y) * n;
                        fft1d(&mut re[base..base + n], &mut im[base..base + n], inverse);
                    }
                }
            }
        }
    }
}

/// Signed wave-number index for a Fourier mode `m` on a periodic grid.
pub fn signed_k(m: usize, n: usize) -> isize {
    if m <= n / 2 { m as isize } else { m as isize - n as isize }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fft_round_trip() {
        let n = 8usize;
        let mut re: Vec<f64> = (0..n * n * n).map(|i| ((i % 17) as f64) * 0.01).collect();
        let original = re.clone();
        let mut im = vec![0.0; re.len()];
        fft3d(&mut re, &mut im, n, false);
        fft3d(&mut re, &mut im, n, true);
        for (a, b) in re.iter().zip(original.iter()) {
            assert!((a - b).abs() < 1e-12);
        }
    }

    #[test]
    fn fft_detects_constant_mode() {
        let n = 8usize;
        let mut re = vec![1.0; n * n * n];
        let mut im = vec![0.0; re.len()];
        fft3d(&mut re, &mut im, n, false);
        assert!((re[0] - (n * n * n) as f64).abs() < 1e-9);
        for (i, value) in re.iter().enumerate().skip(1) {
            assert!(value.abs() < 1e-9, "mode {i} = {value}");
        }
    }
}
