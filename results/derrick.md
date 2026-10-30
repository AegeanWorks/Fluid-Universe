# Derrick scaling (claim 14)

d=3: E(lambda) = lambda E2 + lambda^-1 E4 + lambda^3 V.
Exponents: {'gradient': 1, 'skyrme': -1, 'potential': 3}.
Virial example (E2=2, E4=1, V=0.5): E2 - E4 + 3V = 2.5.
Second derivative at lambda=1: 2 E4 + 6 V = 5.0.

## Minima

- V=0: lambda^2 = 4.000000, minimum = {'lambda': 2.0, 'lambda_squared': 4.0, 'energy': 8.0, 'curvature': 16.0, 'is_minimum': True, 'virial_at_lambda_one': -6.0}.
- V=0.5: lambda^2 = 1.737034, minimum = {'lambda': 1.3179659265863664, 'lambda_squared': 1.7370341836426595, 'energy': 9.850567441059047, 'curvature': 19.0, 'is_minimum': True, 'virial_at_lambda_one': -4.5}.

## Parameter region (necessary conditions)

| case | bounded below | scaling minimum |
|---|---|---|
| kappa_negative | False | False |
| kappa_zero | True | False |
| kappa_positive | True | True |

**Verdict:** scaling exponents and virial relation proven: **True / True**. Necessary region: **K>0, kappa>0, mu>=0**. Hopfion existence proved: **False**.

Derrick scaling gives a necessary condition, not an existence proof. No Hopfion solution is claimed.

- Config SHA-256: `d0ef56d8f61041f2785a740d2c835b801287e3e90b5944232a80aa8534d0eb69`
- Git: `8f6aa4109092e00442a0fe6fe3ca6db3110c98de` (dirty=True)
