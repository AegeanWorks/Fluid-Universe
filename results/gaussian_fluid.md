# Gaussian Madelung reduction (claim 22)

With a Gaussian density and v=A(t)x, continuity and Euler reduce to the Ermakov equation sigma_ddot = -omega^2 sigma + hbar^2/(4m^2 sigma^3).

## Fixed point

- sigma^2 = 0.500000000000 (expected 0.500000000000).

## Large-amplitude oscillation

- sigma0=2 sigma_q; duration=9.424778; dt=1.0e-03.
- sigma range: [0.353553391, 1.414213562], quantum width 0.707106781.
- Energy drift: 2.754e-12.

## Small oscillation

- sigma0=1.02 sigma_q; duration=6.283185; dt=1.0e-04.
- Period: 3.141600000 (expected 3.141592654); relative error 2.338e-06.

## Verdict

- Fixed point verified: **True**.
- Claim 22: **PROVEN (Ermakov reduction; numerics confirm)**.

The Gaussian ansatz reduces the Madelung equations to the Ermakov equation. Its effective-potential minimum is the quantum ground-state width, derived from the fluid equations rather than inserted from the wavefunction. The Born rule and measurement remain outside this reduction.

- Config SHA-256: `89116e9e715c63218f8ae5a917863d17d8ae6be7ba2b320a2e8eba15114a2043`
- Git: `b218002a5d10cba8296f66cd0255c46f693a932c` (dirty=True)
