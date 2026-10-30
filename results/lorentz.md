# Residual anisotropy and Lorentz-violation bounds (claims 11-12)

The two transverse director modes have direction-dependent speeds unless K1=K2=K3. The residual anisotropy is xi = (c_max^2 - c_min^2)/mean(c^2), sampled over 4000 directions.

| K | xi | c2 min | c2 max |
|---|---:|---:|---:|
| (2,2,2) | 0.000e+00 | 2.000000 | 2.000000 |
| (1,2,3) | 1.003480 | 1.000000 | 2.999705 |

## Experimental limits

| bound | quantity | order | applies directly? | why |
|---|---|---:|---|---|
| vacuum birefringence (spectropolarimetry) | dimensionless LV coefficient | -34 | False | The bound is on photon-polarisation coefficients; director modes are neutral. |
| GRB dispersion (Fermi-LAT, linear, 10 GeV) | Delta c / c | -18 | False | The bound is on photon time-of-flight; no photon coupling in the model. |
| CMB isotropy | Delta T / T | -5 | False | Indirect, and requires the director modes to source the CMB; not modelled. |
| clock/cavity comparison | Delta c / c | -17 | False | Bounds photon/cavity propagation; no coupling to the director sector. |

## Conditional check

If a model identified the director modes with photons with O(1) coupling:

| bound | limit | xi (K=1,2,3) | passes? |
|---|---:|---:|---|
| GRB dispersion (Fermi-LAT, linear, 10 GeV) | 1e-18 | 1.003480 | False |
| clock/cavity comparison | 1e-17 | 1.003480 | False |

**Verdict.** Emergent metric at tree level: **False**. One-loop RG to a Lorentz-invariant IR point: **UNVERIFIED**. Direct application of photon bounds: **False**. Conditional equal-K pass: **False**; conditional O(1) anisotropy excluded: **True**.

The linearised model has a preferred frame and no metric. The two transverse modes have equal speed only for K1=K2=K3. The director modes are neutral, so photon-birefringence and GRB bounds do not directly apply. If a coupling to photons were added, equal K would pass all bounds exactly, while an O(1) anisotropy would be excluded by the GRB/clock/birefringence limits.

- Config SHA-256: `b3c75e3d54892bab8af69f107828f2dfb371a883f2703defb09b30ebb7408f3d`
- Git: `4e390abf8d5e278fb31cc8284ec35d9470782e56` (dirty=True)
