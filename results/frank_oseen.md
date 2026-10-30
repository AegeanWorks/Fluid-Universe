# Frank-Oseen linearised modes (claim 10)

Background n0 = z, plane wave, kinetic term (rho/2)(dt n)^2. The elastic matrix is the Hessian of the quadratic director energy.

- K1=K2=K3=2, k=1: omega^2 = [2.0, 2.0] (two degenerate modes).
- Unequal K=(1,2,3), k along x: omega^2 = [1.0, 2.0].
- Unequal K=(1,2,3), k along z: omega^2 = [3.0, 3.0].
- Symbolic Hessian equals the numeric matrix: **True**.

## Modes

- Propagating modes: 2.
- Polarisation: two transverse director components (a,b); the longitudinal component is constrained by |n|=1.
- Gauge field / vector potential: False / False.
- Can be a photon: **False**. Frank-Oseen describes neutral director fluctuations. A photon requires an independent U(1) gauge field and a coupling to it; neither is present.

**Verdict:** equal-K dispersion proven: **True**; emergent Lorentz invariance: **False**.

K1=K2=K3 gives omega^2=(K/rho)k^2 for the two transverse modes. The background n0 selects a preferred frame, and no gauge field is present.

- Config SHA-256: `6b4200a1c07612333ee6dd6a4d24df1b98eb0708c678a629b869bb85484c1bb5`
- Git: `8f6aa4109092e00442a0fe6fe3ca6db3110c98de` (dirty=True)
