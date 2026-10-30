# Radial rays at the water horizon (claim 30)

Depth 0.1 m, flow rate 0.05 m^3/s; r_H=8.034440e-02 m, c=0.990454441 m/s.

## Speed across the horizon

| r/r_H | outgoing speed (m/s) | ingoing speed (m/s) |
|---:|---:|---:|
| 0.9 | -0.110050493 | -2.090959376 |
| 1 | +0.000000000 | -1.980908882 |
| 1.1 | +0.090041313 | -1.890867569 |

## Ray times to 2 r_H

| r0/r_H | analytic (s) | numeric (s) | relative error |
|---:|---:|---:|---:|
| 1.01 | 0.453873085 | 0.453874000 | 2.016e-06 |
| 1.1 | 0.259789627 | 0.259790000 | 1.436e-06 |

- Peeling rate: 12.209936 1/s; surface gravity: 12.209936 m/s².
- Analogue temperature: 1.484316e-11 K.

## Verdict

- Separatrix verified: **True**.
- Claim 30: **PROVEN (radial separatrix; peeling rate)**.

The outgoing radial speed changes sign at r_H and the peeling rate equals the surface gravity c^2/r_H. The numeric ray times match the analytic solution. This is geometric optics; a wave-scattering test remains the next step.

- Config SHA-256: `70ee05c0318d8aca2ab3f3f49bb8762de08a2c0202fa18900eb8babe8d122387`
- Git: `8f6aa4109092e00442a0fe6fe3ca6db3110c98de` (dirty=True)
