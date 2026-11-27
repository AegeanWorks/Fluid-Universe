# Audit coverage inventory

This is a working inventory of G-001 evidence, not a claim-acceptance report.
On 2026-11-27, `claims.md` contained 59 numbered rows marked PROVEN, NUMERIC,
or VERIFIED; `results/rust_audit.json` now contains 52 audit records. Their
counts cannot be compared directly: a claim can need several checks, a record
can cover only part of a claim, and a record can test a different proposition.
The two-process `verify` command compares audit records; it does not establish
that every accepted claim has been independently re-derived.

The following links are **related evidence only**, not a finding that the full
claim satisfies G-001. Audit identifiers are in `src/audit.rs` and the generated
`results/rust_audit.json`; ledger statements and caveats are in `claims.md`.

| Ledger ID | Related audit record(s) | Coverage still to assess |
| --- | --- | --- |
| 1a | `audit.q_identity`, `audit.circulant_dft_eigenvectors` | Sampled Q and matrix eigenvalue residuals do not establish the universal symbolic statement. |
| 1c | `audit.koide_ratio_condition` | Checks the ratio condition; assess the complete iff statement and assumptions. |
| 2a | `audit.circulant_matrix_structure` | Checks sampled Hermiticity and one generic nonsymmetry example; a universal symbolic derivation remains to be assessed. |
| 2b | `audit.circulant_dft_eigenvectors` | Checks the DFT equation on sampled inputs; parameter independence for all inputs remains to be established symbolically. |
| 3a | `audit.measured_pole_fit`, `audit.measured_pole_parameters` | Check measured-mass a, c, phase, ratio, Q and linearized uncertainties using independent errors. The original Monte Carlo remains to be assessed. The separate `audit.tau_prediction` checks a different proposition. |
| 6a | `audit.wall_tension` | Checks the integral at one parameter value; assess the general scaling statement. |
| 7a | `audit.neutrino_q_max` | The audit tests an oppositely worded proposition and labels it FALSIFIED; verify the full scan and uncertainty corners before mapping its verdict to the ledger claim. |
| 10 | `audit.equal_k_isotropy` | Checks equal-K isotropy; assess the full dispersion statement. |
| 14a | `audit.derrick_virial` | Checks the d=3 scaling and virial example; assess the complete stated relation. |
| 40 | `audit.marcus_width` | Checks the specified width and scaling; assess the stated reference comparison. |
| 65 | `audit.single_model_witness_su17`, `audit.single_model_witness_su23` | Check both explicit hosts; assess the ledger's accompanying bounded-search details. |
| 66 | `audit.single_model_witness_su7`, `audit.single_model_relaxed_inverse` | Check the explicit witness and relaxed search; assess the full stated domain and uniqueness. |
| 68 | `audit.minimal_level_inverse_search`, `audit.lower_rank_inverse_search`, `audit.single_model_witness_su7` | Check the component searches and existence; assess the combined bounded minimality statement. |
| 69 | `audit.minimal_witness_search` | Tests the opposite smaller-witness proposition; assess the mapping and bounded search. |
| 70 | `audit.minimal_witness_canonical` | Checks counts and canonical representatives; assess the full claim against the search domain. |
| 71 | `audit.su7_conformal_weights`, `audit.su7_quantum_dimensions` | Check the quoted weights, dimensions and ordering; assess complete claim coverage. |
| 73 | `audit.su7_block_sm_coset` | Checks chosen generator properties and central-charge arithmetic; the embedding index and coset construction need separate assessment. |
| 74 | `audit.su7_hypercharge_census` | Independently checks the chosen generator's 42-weight charge census; the nonabelian multiplet assignment is addressed separately in claim 75. |
| 75 | `audit.su7_nonabelian_branching` | Independently derives representation multiplicities through SU(5) exterior products; physical realization and anomalies are outside the claim. |

The remaining accepted IDs have **not yet been reviewed for audit coverage**:
4b, 5a, 5b, 6b, 12b, 14c, 20, 21, 22, 23, 25, 26, 27, 28,
29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 41, 42, 48, 49, 50, 51, 52,
53, 54, 55, 56, 57, 60, 62 and 63. This list is not a determination that
audit evidence is absent. Review must identify the exact claim, independently
re-derive its full content, and compare the resulting records in two separate
processes before treating it as G-001 evidence.
