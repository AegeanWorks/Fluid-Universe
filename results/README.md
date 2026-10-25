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
