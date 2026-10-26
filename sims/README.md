# Simulations — 1D Baseline Only

There is **no 3D or Hopfion result** in this repository. The 1D validation core exists; no production run has been done. Earlier lost-run values are not cited.

Earlier numbers from a lost script (\(|H|\approx0.16\), \(Z_3\approx0.72\) at \(N=36\)) are not reproduced and are explicitly excluded from the ledger and the paper.

## Required baselines before any production run

`sims/core.py` now implements the 1D baselines. It must still pass all of the following, each as an automated test in `tests/`:

1. **1D sine-Gordon kink.** PASS: profile and tension convergence at order ~2 (`tests/test_sim_core.py`).
2. **Gradient check.** PASS for the 1D sine-Gordon energy (relative error < 1e-6). The full 3D Skyrme+Z3 gradient check is not implemented.
3. **Monotone energy.** PASS for the 1D flow.
4. **Hopf-charge estimator.** PASS: preimage linking gives H=0.999958 at 400 samples (reflection flips sign; rotation/shear invariant), and the normalized grid Chern-Simons estimator gives 1.000000 at N=64. The two methods agree; baseline 4 passes.
5. **Derrick virial.** Scaling identity derived (`theory/derrick.py`); not yet checked on a relaxed 3D soliton.
6. **Rotor cross-check.** PASS: Fourier vs finite-difference spectra agree below 1e-3.
7. **Hessian.** PARTIAL: symmetric and stable in 1D; the translation mode is lifted by Dirichlet boundaries, so zero-mode counting is not exact.

## Production rules

- Convergence study in \(N\), \(dt\), and step count; report the residual force norm.
- At least 10 random seeds for any spontaneous-ordering claim; report mean, standard deviation, and the ordered fraction.
- Log the \(Z_3\) order parameter, Hopf charge, energy, residual force, and virial ratio per run, with config hash and git commit.
- Extract a mass matrix from a field-theory Hessian only after the baselines pass; report whether it is circulant and whether \(|c|/a\) emerges rather than being fitted.
- Scale to GPU only after the CPU core is validated; target \(N\ge64\) for \(|H|\ge1\) coexistence.

## Status

- 1D baselines 1, 2, 3, 6 pass; 5 and 7 are partial; 4 passes (preimage and normalized grid agree).
- A single-seed N=32 3D pilot shows H falling as Z3 rises; no coexistence. No production or spontaneous-order run. Claim 15 remains UNVERIFIED.

## Known analytic obstacles (to be tested, not assumed away)

- A nematic director is headless (\(RP^2\)): \(\pi_1=Z_2\), \(\pi_2=Z\), \(\pi_3=Z\). A three-fold sector is not automatic.
- The rotor map with a real \(\cos3\phi\) potential has degenerate \(k=1,2\) sectors, so a naive three-eigenvalue map cannot produce three distinct lepton masses.
- Derrick scaling for the exact Lagrangian must be derived before any stable-Hopfion claim.
