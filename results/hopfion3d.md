# 3D E2+E4+Z3 pilot (claim 15)

Single-seed pilot, N=32, L=10, dt=5e-4, 1200 gradient steps. The initial condition is the analytic H=1 map with a smooth vacuum compactification. This is not a production run and is not converged (see the convergence table).

Initial: H = 1.000141490, Z3 = 2.319e-18.

| mu | E2 | E4 | V | H | Z3 | monotone | max dE |
|---:|---:|---:|---:|---:|---:|---|---:|
| 0 | 180.401271 | 77.438890 | 0.000000 | 0.749758423 | 0.000000 | True | -2.059e-02 |
| 1 | 177.565978 | 74.551703 | 200.159919 | 0.699891236 | 0.092980 | True | -4.953e-02 |
| 5 | 176.219739 | 63.651258 | 577.797653 | 0.380794012 | 0.435641 | True | -3.924e-01 |
| 20 | 224.610469 | 55.960656 | 148.360395 | 0.052372282 | 0.960169 | True | -5.049e-01 |

## Step-count convergence at mu=20

| steps | E | H | Z3 | force RMS |
|---:|---:|---:|---:|---:|
| 300 | 2588.354810 | 0.852975980 | 0.443234 | 3.497e-01 |
| 600 | 1267.363301 | 0.403630875 | 0.745832 | 2.454e-01 |
| 1200 | 428.931520 | 0.052372282 | 0.960169 | 1.012e-01 |

Converged (force RMS < 1e-2): **False**.

**Verdict:** at 1200 steps, coexistence is False as a numerical diagnostic, but the pilot is unresolved and no coexistence conclusion is drawn. Claim 15: **UNVERIFIED (resolution-limited pilot; no conclusion)**.

The N=32 pilot is not converged: H and Z3 change with step count and resolution (a manual N=64 check gives different values). The 300-step trade-off is transient. No coexistence conclusion is drawn; this is not a production or spontaneous-order claim.

- Config SHA-256: `bbebe2555ce49d9eae94b9aae4a4b19e12fcde1b391073fd59beff6247b8def0`
- Git: `5ed248e6c0f4b1329b217ae0d2b7da8facf7bbb9` (dirty=True)
