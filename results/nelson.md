# Nelson stochastic kinematics (claim 21)

A Brownian particle with D=hbar/(2m) has the osmotic velocity u=D d_x ln(rho). The drift uses the exact quantum density.

## Harmonic-oscillator ground state

- omega=1; particles=50000; steps=5000; dt=1.0e-03.
- Variance: 0.500625 (expected 0.500000).
- Histogram L1 error: 0.0177.

## Free Gaussian packet

- t=1; sigma0=1; particles=50000; steps=1000.
- Histogram L1 error: 0.0195.

## Verdict

- Kinematics verified: **True**.
- Claim 21: **NUMERIC (stochastic kinematics verified)**.

The drift uses the exact quantum density, so this validates the Nelson kinematics, not a self-consistent derivation of the density. A modified diffusion law or a mean-field closure is the next falsifiable step.

- Config SHA-256: `f978c2a31cd0d1617c2d8ceb76a1c5d086692c3e8b55c0be2ea8356e403e9f32`
- Git: `5afa804e3d8ca053f2ef45e1fc1f30627425e4cc` (dirty=True)
