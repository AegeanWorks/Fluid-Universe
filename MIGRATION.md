# fluid-universe: Rust rewrite

The Python packages were removed from the tree; their history remains in git. The
**whole project is now the dependency-free Rust crate** declared in `Cargo.toml`.
The rewrite was started because the Python audit/repair loop took hours for 99 commits;
the full Rust suite runs in seconds and `cargo run -- verify` compares the available
audit records in two separate processes. Complete G-001 claim coverage remains open.

## Layout

| Rust module | Python reference | Scope ported |
|---|---|---|
| `src/circulant.rs` | `theory/circulant.py` | circulant eigenvalues, signed/physical Koide Q, inverse DFT `(a,c,delta)`, mass fit, tau prediction |
| `src/spectra.rs` | `phenomenology/neutrinos.py`, `quarks.py`, `pmns.py` | neutrino ordering masses and Q scan, quark MZ Q regression, PMNS angles |
| `src/rotor.rs` | `theory/rotor.py` | sector Hamiltonians, lowest levels via Jacobi eigensolver, free-rotor limit, domain-wall tension, parity obstruction |
| `src/derrick.rs` | `theory/derrick.py` | scaling exponents/energy, virial, curvature, stationary scale and minimum, stability region |
| `src/fluid.rs` | `theory/*fluid*.py`, vortex modules | Ermakov fixed point, G-M action, Pinney branches, vortex core velocity/Q, blob loop fraction, Marcus width |
| `src/radial_shell.rs` | claim-62 extension | HO-2 complex-shift radial shell, full radial-Laplacian quantum pressure, phase/velocity/Q/norm gates |
| `src/multi_radial_shell.rs` | claim-63 extension | HO-3 double complex-shift shell, exact 3s nodes, product phase and node-local convergence gates |
| `src/water.rs` | `theory/water_analogue.py` | sound speed, horizon radius, surface gravity, analogue Hawking temperature, capillary length/ratio |
| `src/lorentz.rs` | `theory/lorentz.py` | 2x2 elastic eigenvalues, residual anisotropy over deterministic unit directions, equal-K isotropy |
| `src/linalg.rs` | (new) | cyclic Jacobi symmetric eigensolver, Simpson quadrature, bisection, tolerance helper |
| `src/simulation.rs` | `sims/core3d.py`, `sims/production.py`, `theory/hopf.py` | analytic H=1 field, central-difference E2/E4/V, Z3 order, Hopf linking, analytic force, relaxation and field metrics |
| `src/audit.rs` | `verification/audit.py` | 46 independently re-derived audit records with frozen verdicts and a deterministic fingerprint |
| `src/reports.rs` | `phenomenology/*_report.py`, `phenomenology/run_all.py` | deterministic `rust_*.json\|md` generation with config hash and in-process provenance |
| `src/verify.rs` | `verification/verify.py` | exact comparison of two process audit records; ledger coverage remains open |

## Commands

```bash
cargo test                      # unit + integration suite (no network, no Python)
cargo run -- all                # print every reproducible check
cargo run -- <module>           # circulant|spectra|rotor|derrick|fluid|radial_shell|multi_radial_shell|water|lorentz|simulation|audit|reports
cargo run -- claims             # independent per-claim verdicts
cargo run -- regenerate [dir]   # write rust_*.json|md (default: results/)
cargo run -- json               # machine-readable check list
cargo run -- fingerprint        # FNV-1a of the ordered check list
cargo run -- verify             # compare available audit records in two child processes
cargo run --release -- all      # optimised binary
```

## Design decisions

1. **Zero dependencies.** No `ndarray`, no `serde`, no linalg backend. A Jacobi
   eigensolver and Simpson quadrature cover the dense matrices and integrals this
   core needs, so a cold build works offline and releases are reproducible.
2. **Checks, not prose.** Every module exports `checks() -> Vec<Check>` with a stable
   id, a value and a human detail. The CLI and the two-process verifier consume the
   same list, so a number cannot drift between the report and the audit.
3. **Exactness preserved.** Algebraic identities (Q = 1/3 + 2c²/3a², Derrick
   exponents, rotor sector-1/sector-2 degeneracy, Pinney determinant) are asserted
   with machine-precision tolerances in unit tests, matching the Python ledger.
4. **Falsification status preserved.** The Rust core reports numbers and identities;
   it does not promote a claim. Verdicts remain in `claims.md` / `SCORECARD.md`
   and are driven by the same gates as before.
5. **Practical exact verification.** Dev/test builds use `opt-level=2` because the
   bounded SU(N)_k searches enumerate tens of millions of primaries. This changes
   optimization only; assertions, domains, tolerances and fingerprints remain the
   evidence contract.

## Migration status — complete

The whole project is now Rust. The Python packages (`theory/`, `sims/`,
`phenomenology/`, `verification/` and the Python tests) were removed from the tree;
their history remains in git. `src/simulation.rs` ports the 3D field core, energy,
Hopf/Z3 estimators and relaxation. `src/audit.rs` independently re-derives the claim
ledger with frozen verdicts. `src/reports.rs` regenerates results as
`results/rust_<family>.json|md`; the old Python-generated result files are kept only
as a historical ledger.

Verified: **612 reproducible checks**, **373 passing tests** (341 unit + 7 existing
integration + 6 single-shell + 7 double-shell + 12 single-WZW-search integration)
and a two-process comparison of 46 matching audit records. The audit has not been mapped to every accepted claim, so complete G-001 coverage remains outstanding.

On this host the MSVC `link.exe` is absent, so the registered test command is
`cargo test --target x86_64-pc-windows-gnu`. On a host with Visual Studio Build
Tools, plain `cargo test` works unchanged.
