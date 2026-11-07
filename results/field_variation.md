# Unstructured field variation (claim 58)

Adversarial-start field optimization against the Gaussian bridge bank.

| check | value |
|---|---|
| quantum bank | -0.3906340559 |
| coarse gauss_linear | J=-3.3548040775, grad=1.812e+02, conv=False |
| coarse bump | J=-11196111287.6981239319, grad=1.113e+10, conv=False |
| coarse bimodal | J=-14068314954.4627342224, grad=1.364e+10, conv=False |
| coarse wide | J=-62236515807.5005874634, grad=3.373e+10, conv=False |
| fine bridge_warm | J=-1.5507885565, grad=6.252e+01, conv=False |
| fine wide | J=-99136423776.8857116699, grad=2.525e+11, conv=False |
| attribution | 1.661e+01 |
| classical bank / nu=0 best | -0.2385281374 / -0.2384368584 |

## Verdict

- Claim 58: **FALSIFIED (unstructured field escapes far below the bank; trial-class global minimality dead; escape_below; path_match; scaling_match)**.

The fixed-endpoint G-M field does not re-select the Gaussian: every tier escapes far below the bank, still descending; the bridge Hessian has negative directions; the escape exploits near-vacuum cells. Stationarity results of claims 51-57 stand; only the global-minimum reading dies.

- Config SHA-256: `a5f51d455af69679b631624ac8680a9fabfc4e4abbdb4ab58cebda740f1e3e7f`
- Git: `0d14fcc8f87a17a664b950750ddc0251be6b7906` (dirty=True)
