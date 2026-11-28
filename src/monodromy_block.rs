//! Rigorous SU(3)_3 conformal-block monodromy calculation.
//!
//! We build the modular S-matrix from the Kac-Peterson Weyl sum over the 6
//! elements of the A2 Weyl group, with the normalisation fixed by unitarity
//! and S^2 = C (charge conjugation). Then the Verlinde formula gives the
//! fusion □ x □ = (0,1) + (2,0), each with multiplicity 1. Four identical
//! fundamentals have no sphere block: their total center charge is nonzero.
//! The mixed □, □, □bar, □bar sphere block has two channels. The full
//! monodromy of the first two fields in an admissible channel nu is
//!   exp(2 pi i (h_nu - 2 h_box)),
//! and the exponent magnitudes below use the displayed weight representatives.
//! A single exchange is not the same operation as this full monodromy. At k=3:
//!   antisymmetric channel (0,1) = □bar:  label = |2/9 - 4/9| = 2/9 = delta_lepton
//!   symmetric channel     (2,0) = Sym^2: label = |5/9 - 4/9| = 1/9.
//!
//! Thus the charged-lepton Brannen phase is the magnitude of a rigorously
//! computed monodromy label of the mixed SU(3)_3 four-point block, not merely a
//! guessed number. HONEST CAVEAT: identifying the mass-matrix phase with this
//! monodromy label is still a structural hypothesis; the calculation proves
//! the LABEL is 2/9, not that nature must use it.

use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex { re: f64, im: f64 }
impl Complex {
    fn new(re: f64, im: f64) -> Self { Self { re, im } }
    fn mul(self, o: Self) -> Self {
        Self::new(self.re*o.re - self.im*o.im, self.re*o.im + self.im*o.re)
    }
    fn conj(self) -> Self { Self::new(self.re, -self.im) }
    fn expi(theta: f64) -> Self { Self::new(theta.cos(), theta.sin()) }
}

/// The 6 elements of the A2 Weyl group as matrices on Dynkin labels (p,q),
/// with their determinants.
fn weyl_group() -> [([i32; 4], f64); 6] {
    // generators: s1:(p,q)->(-p,p+q), s2:(p,q)->(p+q,-q)
    let gens = [([-1,0,1,1], -1.0), ([1,1,0,-1], -1.0)];
    let mut elems: Vec<([i32;4], f64)> = vec![([1,0,0,1], 1.0)];
    let mut changed = true;
    while changed {
        changed = false;
        let snapshot = elems.clone();
        for (m, det) in &snapshot {
            for (s, sd) in &gens {
                let (a,b,c,d) = (m[0],m[1],m[2],m[3]);
                let (e,f,g,h) = (s[0],s[1],s[2],s[3]);
                let prod = [e*a+f*c, e*b+f*d, g*a+h*c, g*b+h*d];
                if !elems.iter().any(|(x,_)| *x == prod) {
                    elems.push((prod, det*sd));
                    changed = true;
                }
            }
        }
    }
    let mut out = [([1,0,0,1],1.0); 6];
    for (i,e) in elems.into_iter().take(6).enumerate() { out[i] = e; }
    out
}

fn ip(p: i32, q: i32, r: i32, s: i32) -> f64 {
    // (omega_i, omega_j) = A2 inverse = [[2/3,1/3],[1/3,2/3]]
    (2.0/3.0)*p as f64*r as f64 + (1.0/3.0)*(p as f64*s as f64 + q as f64*r as f64)
        + (2.0/3.0)*q as f64*s as f64
}

/// Integrable primaries of SU(3)_3.
pub fn primaries() -> Vec<(i32, i32)> {
    let mut v = Vec::new();
    for p in 0..=3 { for q in 0..=(3-p) { v.push((p,q)); } }
    v
}

/// Modular S-matrix element via the Weyl sum (normalisation fixed to unitarity).
pub fn s_matrix_element(lam: (i32,i32), mu: (i32,i32)) -> Complex {
    let kbig = 6.0; // k + h^vee = 3 + 3
    let lp = (lam.0 + 1, lam.1 + 1);
    let mp = (mu.0 + 1, mu.1 + 1);
    let mut sum = Complex::new(0.0, 0.0);
    for (m, det) in weyl_group() {
        let wl = (m[0]*lp.0 + m[1]*lp.1, m[2]*lp.0 + m[3]*lp.1);
        let theta = -2.0*PI*ip(wl.0, wl.1, mp.0, mp.1)/kbig;
        let term = Complex::expi(theta);
        sum = Complex::new(sum.re + det*term.re, sum.im + det*term.im);
    }
    // -i/(6 sqrt3): the factor that makes S unitary with S^2 = C.
    let pref = 1.0/(6.0*3.0_f64.sqrt());
    Complex::new(pref*sum.im, -pref*sum.re)
}

/// Conformal weight h = C2/(k+h^vee) of a primary.
pub fn h_weight(l: (i32,i32)) -> f64 {
    let (p,q) = l;
    (p*p + q*q + p*q + 3*p + 3*q) as f64 / 3.0 / 6.0
}

/// Verlinde fusion coefficient N_{lam,mu}^nu.
pub fn fusion(lam: (i32,i32), mu: (i32,i32), nu: (i32,i32)) -> f64 {
    let prim = primaries();
    let mut total = 0.0_f64;
    for &s in &prim {
        let a = s_matrix_element(lam, s);
        let b = s_matrix_element(mu, s);
        let c = s_matrix_element(nu, s).conj();
        let d = s_matrix_element((0,0), s).re;
        let ab = a.mul(b).mul(c);
        total += ab.re / d;
    }
    total
}

/// Sphere four-point block dimension from pairwise Verlinde coefficients.
/// The inputs must be integrable SU(3)_3 primary labels.
pub fn sphere_four_point_dimension(
    a: (i32, i32), b: (i32, i32), c: (i32, i32), d: (i32, i32),
) -> f64 {
    primaries().iter().map(|&nu| {
        fusion(a, b, nu) * fusion(c, d, (nu.1, nu.0))
    }).sum()
}

/// Magnitude of the full-monodromy exponent; admissibility must be checked.
pub fn braiding_label(nu: (i32,i32)) -> f64 {
    (h_weight(nu) - 2.0*h_weight((1,0))).abs()
}

pub fn checks() -> Vec<crate::Check> {
    vec![
        crate::Check::new("monodromy_block.weyl_order", weyl_group().len() as f64,
                          "A2 Weyl group has 6 elements"),
        crate::Check::new("monodromy_block.s00", s_matrix_element((0,0),(0,0)).re,
                          "S_00 of SU(3)_3 (=1/6)"),
        crate::Check::new("monodromy_block.fusion_box_abar", fusion((1,0),(1,0),(0,1)),
                          "Verlinde coefficient for box x box -> boxbar (=1)"),
        crate::Check::new("monodromy_block.fusion_box_sym", fusion((1,0),(1,0),(2,0)),
                          "Verlinde coefficient for box x box -> Sym^2 (=1)"),
        crate::Check::new("monodromy_block.four_box_dimension", sphere_four_point_dimension((1,0),(1,0),(1,0),(1,0)),
                          "four-fundamental sphere block dimension = 0"),
        crate::Check::new("monodromy_block.mixed_dimension", sphere_four_point_dimension((1,0),(1,0),(0,1),(0,1)),
                          "two-fundamental two-antifundamental sphere block dimension = 2"),
        crate::Check::new("monodromy_block.label_antisym", braiding_label((0,1)),
                          "antisymmetric full-monodromy exponent magnitude = 2/9"),
        crate::Check::new("monodromy_block.label_sym", braiding_label((2,0)),
                          "symmetric full-monodromy exponent magnitude = 1/9"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_four_point_dimensions_respect_external_charges() {
        let f = (1, 0);
        let fbar = (0, 1);
        assert!(sphere_four_point_dimension(f, f, f, f).abs() < 1e-9);
        assert!((sphere_four_point_dimension(f, f, fbar, fbar) - 2.0).abs() < 1e-9);
        assert!((sphere_four_point_dimension(f, fbar, fbar, f) - 2.0).abs() < 1e-9);
    }

    #[test]
    fn weyl_group_has_six_elements() {
        assert_eq!(weyl_group().len(), 6);
    }

    #[test]
    fn s_matrix_is_unitary() {
        let p = primaries();
        for a in &p {
            let row: f64 = p.iter().map(|b| {
                let s = s_matrix_element(*a,*b); s.re*s.re + s.im*s.im
            }).sum();
            assert!((row - 1.0).abs() < 1e-9, "row norm {row} for {a:?}");
        }
    }

    #[test]
    fn verlinde_fusion_of_box_is_conjugates() {
        let boxrep = (1,0);
        let mut found = Vec::new();
        for &nu in &primaries() {
            let n = fusion(boxrep, boxrep, nu);
            if n > 0.5 { found.push((nu, n)); }
        }
        found.sort_by_key(|x| x.0);
        assert_eq!(found.len(), 2, "fusion = {found:?}");
        assert_eq!(found[0].0, (0,1));
        assert_eq!(found[1].0, (2,0));
        assert!((found[0].1 - 1.0).abs() < 1e-9 && (found[1].1 - 1.0).abs() < 1e-9,
                "fusion = {found:?}");
    }

    #[test]
    fn antisymmetric_braiding_label_is_two_ninths() {
        assert!((braiding_label((0,1)) - 2.0/9.0).abs() < 1e-12);
        assert!((braiding_label((2,0)) - 1.0/9.0).abs() < 1e-12);
    }
}
