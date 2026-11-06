# Yasue-bridge analytic family (claim 56)

c<0 Pinney form with endpoint pinning: B_1 admissible (recorded arc), B_2 inadmissible.

| check | value |
|---|---|
| disc (recorded point) | 3.000000 |
| B_1 / B_2 | 0.3660254037844386 / -1.3660254037844386 |
| B_2 min s2 (must be < 0) | -0.06582624879369803 |
| ODE FD max residual | 5.503e-09 |
| constraint deviation | 0.000e+00 |
| attribution max-abs | 6.258e-08 |
| Jacobi first zero | None |
| Q range on arc | [-2.000, -0.766] |
| nu=0.5 | attained=True, peak=1.1415890569364793, res=7.468e-11 |
| nu=0.25 | attained=True, peak=1.0986840851938602, res=4.461e-11 |
| nu=0.125 | attained=True, peak=1.0865796187813457, res=4.842e-11 |
| nu=0.0 | attained=True, peak=1.0823921437274908, res=4.642e-11 |
| scaling B_1 / attrib / jacobi | 0.3660254037844386 / 4.425459354528982e-08 / None |

## Verdict

- Claim 56: **NUMERIC (Yasue-bridge analytic family demonstrated)**.

c<0 Pinney form satisfies the Yasue ODE; endpoint pinning gives B_1 admissible (recorded arc) and B_2 inadmissible; Jacobi zero-free; scaling preserves structure; nu->0 is the oscillator bridge.

- Config SHA-256: `7a1605b2c7afd4384c37674caf5c4e0938fb1782ce03e55a2ae91c87caf7e464`
- Git: `96eb57c0dc34b84b6a1d948bd8b69f8cd870e1dc` (dirty=True)
