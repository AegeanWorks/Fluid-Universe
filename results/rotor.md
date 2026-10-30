# Rotor and domain-wall results (claim 5, claim 6 partial)

Fourier-basis exact diagonalisation of H = L^2/(2I) + mu(1-cos 3phi), I = 1, mu = 1.

| sector k | lowest level |
|---:|---:|
| 0 | 0.891186159736 |
| 1 | 1.317034514042 |
| 2 | 1.317034514042 |

- E1 - E2 = 0.000e+00 (exact degeneracy by phi -> -phi).
- Naive circulant map implied roots: 1.458983965, 1.033135611, 1.033135611 (2 distinct; 3 needed).
- Therefore the naive rotor-to-circulant map cannot give three lepton masses.

## Truncation convergence

| n_cutoff | E0 | E1 | E1 - E2 |
|---:|---:|---:|---:|
| 10 | 0.891186159750 | 1.317034514186 | 0.000e+00 |
| 20 | 0.891186159741 | 1.317034514042 | 0.000e+00 |
| 40 | 0.891186159741 | 1.317034514041 | 0.000e+00 |
| 80 | 0.891186159740 | 1.317034514041 | 0.000e+00 |
| 160 | 0.891186159737 | 1.317034514041 | 0.000e+00 |

## Domain-wall tension

- analytic (8/3) sqrt(mu) = 2.666666666667
- quadrature = 2.666666666667

The tension identity is verified. The full dimensional-reduction/BPS claim (claim 6) and the finiteness of the effective moment of inertia remain unverified.

- Config SHA-256: `f2888e101ebeb74e127a8b2da829432a558f9e67550cced73735333a7aaf2ecd`
- Git: `377fcd7076c00f31b35f08fe2b698974b903a362` (dirty=True)
