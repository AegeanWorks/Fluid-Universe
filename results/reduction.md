# Dimensional reduction and moment of inertia (claim 6b)

1D wall tension at mu=1: T = 2.666666666667.
Lower-bound example: T * L_y (L_y=2) = 5.333333333333.

| tail rho ~ r^-alpha | I_eff status |
|---:|---|
| 2 | power-divergent (integrand ~ r^1) |
| 4 | log-divergent (integrand ~ r^-1) |
| 5 | finite (integrand ~ r^-2) |

I_eff = 2*pi int rho(r) r^3 dr; a tail steeper than r^-4 is required for convergence.

**Verdict:** trivial lower bound: **True**; exact dimensional reduction: **False**; finite I_eff for the stated profile: **False** (log-divergent).

E_2D >= T L_y is a trivial positive-gradient bound, not an exact reduction. The stated 1/r^4 profile makes I_eff logarithmically divergent; a finite I_eff requires a tail steeper than r^-4.

- Config SHA-256: `a812ba2cc84d36853e2798d2ed3276bfc5a3a12a134e263720c7c31588c4c0ec`
- Git: `d181dc54f1ad61bbd055f9763999de91a71ad048` (dirty=True)
