# Moving-center Gaussian reduction (claim 23)

With a displaced Gaussian and v=x0_dot+A(x-x0), the fluid equations decouple into the classical center and the Ermakov width.

## Fixed width, moving center

- x0(0)=1, sigma0=sigma_q=0.707106781; dt=1.0e-03.
- x0(pi)=-0.999999917; x0(2pi)=0.999999983.
- Center period: 6.284000000 (expected 6.283185307).
- Width error: 0.000e+00; energy drift: 7.105e-15.

## Displaced width, moving center

- sigma0=2 sigma_q; sigma range: [0.353553501, 1.414213562].
- Energy drift: 2.678e-12.

## Verdict

- Reduction verified: **True**.
- Claim 23: **PROVEN (moving-center reduction; numerics confirm)**.

The moving-center Gaussian decouples into the classical oscillator x0''=-omega^2 x0 and the Ermakov width. The coherent-state trajectory and quantum width are derived from the fluid equations, not inserted from the wavefunction. The first excited state still has a node singularity.

- Config SHA-256: `1dd22946f116eedadf1884b72303d6451966bdc1a16ae5a4ee1191c96e2c7596`
- Git: `4e390abf8d5e278fb31cc8284ec35d9470782e56` (dirty=True)
