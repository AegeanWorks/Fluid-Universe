# results/

Machine-generated outputs only. Every file here is written by a script and
carries a config SHA-256, package versions, and the git commit (or `dirty`).
Do not edit these files by hand.

| File | Contents |
|---|---|
| `pdg_fit.json` | Pole-mass fit point values, Monte Carlo summary, input provenance |
| `pdg_fit.md` | Human-readable version of the same |
| `rg_run.json` | One-loop SM running table, drift, scheme checks, published-value comparison |
| `rg_run.md` | Human-readable version of the same |
| `rotor.json` | Rotor spectrum, degeneracy, convergence, domain-wall tension |
| `rotor.md` | Human-readable version of the same |
| `neutrinos.json` | Neutrino Q_nu scan, both orderings and 3-sigma corners |
| `neutrinos.md` | Human-readable version of the same |
| `quarks.json` | Quark Q_up/Q_down at six full-SM MS-bar scales |
| `quarks.md` | Human-readable version of the same |
| `pmns.json` | Minimal circulant PMNS branches and measured angles |
| `pmns.md` | Human-readable version of the same |
| `delta_search.json` | Delta closed-form search with prior and look-elsewhere penalty |
| `delta_search.md` | Human-readable version of the same |
| `frank_oseen.json` | Frank-Oseen elastic matrix, modes, anisotropy |
| `frank_oseen.md` | Human-readable version of the same |
| `derrick.json` | Derrick exponents, virial, scaling minima, parameter region |
| `derrick.md` | Human-readable version of the same |
| `topology.json` | RP2 homotopy, direct Z3 claim, standard extensions |
| `topology.md` | Human-readable version of the same |
| `reduction.json` | 2D lower bound and effective moment of inertia |
| `reduction.md` | Human-readable version of the same |
| `predictions.json` | Seven-class new-prediction audit |
| `predictions.md` | Human-readable version of the same |
| `lorentz.json` | Residual anisotropy and conditional LV bounds |
| `lorentz.md` | Human-readable version of the same |
| `sim_core.json` | 1D kink/gradient/flow/Hessian/rotor baselines |
| `sim_core.md` | Human-readable version of the same |
| `hopf.json` | Hopf preimage/grid estimator validation |
| `hopf.md` | Human-readable version of the same |
| `hopfion.json` | Static finite-energy H=1 configuration and scaling virial |
| `hopfion.md` | Human-readable version of the same |
| `hopfion3d.json` | Single-seed 3D E2+E4+Z3 pilot |
| `hopfion3d.md` | Human-readable version of the same |
| `hopf_resolution.json` | Grid Hopf charge vs N |
| `hopf_resolution.md` | Human-readable version of the same |
| `hopfion3d_n64.json` | Targeted N=64 diagnostic; not part of make results |
| `hopfion3d_n64.md` | Human-readable version of the same |
| `hopfion_scale.json` | Damped scale-iteration candidate; not part of make results |
| `hopfion_scale.md` | Human-readable version of the same |
| `hopfion_stability.json` | Dense-validated tangent Hessian of the scale candidate; not part of make results |
| `hopfion_stability.md` | Human-readable version of the same |
| `hopfion3d_production.json` | N=64 claim-15 production scan with 2x upsampled H, dt and seed checks (<15 min, G-003); not part of make results |
| `hopfion3d_production.md` | Human-readable version of the same |
| `hopfion3d_convergence.json` | N=64 vs N=96 claim-15 resolution check at the same physical time (<15 min, G-003); not part of make results |
| `hopfion3d_convergence.md` | Human-readable version of the same |
| `hopfion3d_temporal.json` | N=64 claim-15 timestep check at the same physical time (<15 min, G-003); not part of make results |
| `hopfion3d_temporal.md` | Human-readable version of the same |
| `summary.md` | Combined status sheet |
| `verification_run1.json` | Independent re-derivation, first separate run |
| `verification_run1.md` | Human-readable version of run 1 |
| `verification_run2.json` | Independent re-derivation, second separate run |
| `verification_run2.md` | Human-readable version of run 2 |
| `verification_single.json` | A single run, written only if the audit is called without `--tag`; explicitly NOT ACCEPTED |
| `verification.json` | Combined two-run per-claim comparison; accepted only if both runs agree |
| `verification.md` | Human-readable version of the combined result |

To regenerate the physics outputs:

```bash
python -m phenomenology.run_all
```

`python -m verification.verify` first regenerates the physics outputs, then runs the two audits and the comparison. To run the steps manually (two separate runs, then compare):

```bash
python -m verification.audit --tag run1
python -m verification.audit --tag run2
python -m verification.compare
```

`make results` regenerates the physics outputs; `python -m verification.verify` (or
`make verify`) regenerates them and then runs the two audits plus the comparison. Numbers quoted in the README, claims ledger,
or paper must match these files.
