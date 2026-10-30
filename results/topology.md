# Nematic topology and Z3 (claim 13)

Order-parameter space: RP^2. Homotopy: {'pi1': 'Z2', 'pi2': 'Z', 'pi3': 'Z'}.

| extension | Z3 derived? | what it would require |
|---|---|---|
| rank_three_order_parameter | False | An explicit rank-3 tensor with a three-fold symmetric invariant. The Z3 is a property of the chosen invariant, not of RP^2. |
| three_coupled_nematics | False | An S3 permutation symmetry of the three fields. Its Z3 subgroup is imposed by the coupling, so the symmetry is inserted. |
| three_fold_lattice | False | A lattice with a three-fold axis. The lattice breaks continuous rotation and supplies the Z3 label; nematic topology alone does not. |

**Verdict:** direct Z3 from nematic topology: **False**; standard extensions: **False**. `cos(3 phi)` inserted by hand: **True**.

The strong claim is false: RP^2 has no Z3 factor and the cos(3 phi) term is inserted. None of the three standard extensions produces Z3 without an explicit three-fold structure. A different, specified extension is not excluded, but it would have to be constructed and tested explicitly.

- Config SHA-256: `a9fc09d913c1a4e299368f503f7e7cc9c2eeb49dc478bcfa3593cc10353b854b`
- Git: `e642476ad5a86f8326ffaf9e5b5b631faf454787` (dirty=True)
