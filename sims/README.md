# Simulations — Not Started

There is **no** simulation result in this repository. Nothing here may be cited.

Earlier numbers from a lost script (\(|H|\approx0.16\), \(Z_3\approx0.72\) at \(N=36\)) are not reproduced and are explicitly excluded from the ledger and the paper.

## Required baselines before any production run

`sims/core.py` does not exist yet. When it does, it must pass all of the following, each as an automated test in `tests/`:

1. **1D sine-Gordon kink.** Profile and tension against the analytic \(T=(8/3)\sqrt{\mu}\).
2. **Gradient check.** Analytic force equals a finite difference of the discrete energy (relative error \(<10^{-6}\)) for the full Lagrangian including Skyrme and \(Z_3\) terms.
3. **Monotone energy.** Energy decreases monotonically under gradient flow; a failure means a bug or an unstable time step.
4. **Hopf-charge estimator.** Analytic Hopf map (\(H=1\)) on a grid; \(H\to1\) with convergence order in \(N\); sign flips under reflection; integer stability under smooth deformation. Two independent methods (field-theory integral and preimage linking number) must agree.
5. **Derrick virial.** On a relaxed soliton, the scaling identity for the exact convention holds within tolerance.
6. **Rotor cross-check.** Exact diagonalisation against a grid Schrödinger solver.
7. **Hessian.** Symmetric, zero modes counted, eigenvalues stable under epsilon and \(N\).

## Production rules

- Convergence study in \(N\), \(dt\), and step count; report the residual force norm.
- At least 10 random seeds for any spontaneous-ordering claim; report mean, standard deviation, and the ordered fraction.
- Log the \(Z_3\) order parameter, Hopf charge, energy, residual force, and virial ratio per run, with config hash and git commit.
- Extract a mass matrix from a field-theory Hessian only after the baselines pass; report whether it is circulant and whether \(|c|/a\) emerges rather than being fitted.
- Scale to GPU only after the CPU core is validated; target \(N\ge64\) for \(|H|\ge1\) coexistence.

## Known analytic obstacles (to be tested, not assumed away)

- A nematic director is headless (\(RP^2\)): \(\pi_1=Z_2\), \(\pi_2=Z\), \(\pi_3=Z\). A three-fold sector is not automatic.
- The rotor map with a real \(\cos3\phi\) potential has degenerate \(k=1,2\) sectors, so a naive three-eigenvalue map cannot produce three distinct lepton masses.
- Derrick scaling for the exact Lagrangian must be derived before any stable-Hopfion claim.
