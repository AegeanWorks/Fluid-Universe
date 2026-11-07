# loop_L1_takabayasi — Literature sweep record (claim-61 input, no verdict)

Scope: Takabayasi 1952 condition as single-component prior art for vortex
quantization. Requested fields: exact condition, single-component status
(no branch construct), vortex-quantization reading, nodal treatment,
equation/page pointers. Explicit negative where absent (original equation
label, finite-core numbers). No claim-61 verdict is recorded here.

Search date: 2026-11-07. Method: webfetch of open pages only
(Wikipedia Madelung equations; arXiv abs + full HTML v1 of
Reddiger–Poirier 2207.11367; OUP primary page access attempt).
Websearch unavailable (Exa 402: free allowance exhausted, no credit
balance — same outage as 2026-11-06 sweep). No full-text copy of
Takabayasi 1952/53 PTP was parsed here (OUP fetch 403 on 2026-11-07,
same as 2026-11-06); original equation numbers / page numbers below are
via the secondary that quotes the primary (Reddiger–Poirier 2023, citing
p.155). Primary-confirmation flag carried: exact original numbering needs
direct PTP check.

Local repo context only (no numbers taken): theory/node_defect.py,
theory/hydrogen_radial_nodes.py, theory/hydrogen_vortex.py,
results/node_defect.{json,md}, results/radial_node.{json,md},
results/hydrogen_vortex.{json,md}, claims 20/24/26/46/47.

## 1. Bibliography

- T. Takabayasi, "On the Formulation of Quantum Mechanics associated with
  Classical Pictures", Prog. Theor. Phys. 8(2), 143–182 (1952).
  DOI: 10.1143/PTP/8.2.143. ISSN 0033-068X.
  Condition first proposed p.155 (per Reddiger–Poirier 2023, Sec.1).
  Spelling note: PTP prints "Takabayasi"; Wikipedia page writes
  "Takehiko Takabayashi"; Reddiger–Poirier writes "Takabayasi [137]".
  Same referent.
- T. Takabayasi 1953 continuation(s) (hydrodynamical / stochastic-line
  follow-ups). Same single-component framework; no separate vortex-core
  model attributed in the material swept. Exact 1953 citation form
  (commonly cited as PTP 9, 187 (1953)) not verified here —
  primary-confirmation flag.
- Review control: M. Reddiger and B. Poirier, "Towards a mathematical
  Theory of the Madelung Equations: Takabayasi's Quantization Condition,
  Quantum Quasi-Irrotationality, ...", arXiv:2207.11367v1 [math-ph]
  (22 Jul 2022), published J. Phys. A 56, 193001 (2023).
  DOI: 10.1088/1751-8121/acc7db. Sec.1, Sec.2.1 Prop.2.1, Sec.2.3
  Cor.2.11 / Eq.(1.4), Sec.4 non-quantized solutions.
- Modern wordings of the same condition (not Takabayasi substitutes):
  I. Bialynicki-Birula, M. Cieplak, J. Kaminski, Theory of Quanta (OUP,
  1992), pp.90–91; R. E. Wyatt, Quantum Dynamics with Trajectories
  (Springer, 2005), Sec.13.5. Page/section numbers via Wikipedia
  Madelung-equations citations, verified as Wikipedia's citation text
  on 2026-11-07 fetch, not as primary parse here.

## 2. Exact condition (single-component, no branch construct)

Modern wording (Reddiger–Poirier Eq.(1.3), also Wikipedia Madelung-equations
"auxiliary quantization condition" citing Bialynicki-Birula pp.90–91 and
Wyatt Sec.13.5):

  m / (2π ħ) ∮ v · dr ∈ Z

for every C^1 closed curve γ in the domain D at fixed time, with

  v = (ħ/m) Im(∇Ψ/Ψ) = ħ/(2im) (∇Ψ/Ψ − ∇Ψ*/Ψ*)   [Reddiger Eq.(1.2)]

and circulation form Γ ≐ ∮ m v · dl = 2π n ħ (n ∈ Z).

Single-component content (why "no branch construct"):

- v is defined from ONE Ψ via Eq.(1.2) wherever Ψ ≠ 0. No s(x)·|Ψ| split,
  no two-sheet sign field, no inner/outer no-crossing doctrine is part of
  the condition.
- Takabayasi's own characterization (quoted in Reddiger Sec.1): Eq.(1.3) is
  "so to speak the 'quantum condition' for fluidal motion and of ad hoc and
  compromising character for our formulation, just as the [Bohr–Sommerfeld]
  quantum condition for old quantum theory." Reference omitted in secondary;
  wording needs primary check.
- Reddiger–Poirier Prop.2.1: for n ≥ 2, D open in R^n, Ψ ∈ C^1(D,C)
  non-vanishing on D, the integral over any C^1 closed γ ⊂ D satisfies
  Eq.(1.3). Proof is winding-number / degree content; the paper notes no
  global continuous S with Ψ = |Ψ| exp(iS/ħ) need exist (no lifting in
  general; footnote 8). That is: single-valued Ψ, possibly no single-valued S.
- Reddiger–Poirier naming note: credit for the condition in vortex literature
  often goes to Dirac (cf. p.67 in their Ref.[42]); Wentzel 192? is explicitly
  NOT to be credited (different 1D meromorphic/residue context). The name
  "Takabayasi's quantization condition" is kept because Takabayasi first
  proposed adding it to Madelung Eqs.(1.1a)–(1.1c) (p.155 in [137]).

Equation / page numbers (as reported, primary-confirmation flag):

- Takabayasi 1952 proposal: p.155 in PTP 8(2) 143–182 (per Reddiger Sec.1).
  Original equation label on that page NOT verified here (OUP 403).
- Reddiger–Poirier: drift definition Eq.(1.2); condition Eq.(1.3); Madelung
  triple Eqs.(1.1a)–(1.1c) with third Eq.(1.1c) ∇×v = 0; Schrodinger Eq.(2.1);
  Sec.2.1 Prop.2.1; Ex.2.2 Eqs.(2.3a)–(2.3e) + Fig.1; Def.2.10
  quasi-irrotationality; Cor.2.11 / Eq.(1.4) distributional curl; Sec.4
  non-quantized 2D oscillator strong solutions.
- Tertiary wording: Γ = ∮ m v·dl = 2πnħ (Bialynicki-Birula pp.90–91;
  Wyatt Sec.13.5, via Wikipedia). Not Takabayasi original labels.
- Wikipedia History paragraph (verified 2026-11-07 fetch): "In 1994
  Timothy C. Wallstrom showed that an additional ad hoc quantization
  condition must be added to the Madelung equations ... His analysis
  paralleled earlier work by Takehiko Takabayashi on the hydrodynamic
  interpretation of Bohmian mechanics [8]" with [8] = Takabayasi 1952
  PTP citation above.

## 3. Vortex-quantization reading (prior-art status)

- What the condition supplies as prior art: integer circulation on loops
  avoiding nodes. In the hydrogen standard-solution instance the integer
  in Eq.(1.3) equals the magnetic quantum number μ (Reddiger Sec.1–2.3).
  Distributional form (Reddiger Eq.(1.4) / Cor.2.11, "quantum
  quasi-irrotationality" after Schönberg): for standard Ψ_nlμ (Coulomb),
  in physics delta notation,
  (∇×v)(t,x,y,z) = (2π μ ħ/m) (0, 0, δ(x)δ(y))^T.
  I.e. the third Madelung equation ∇×v = 0 fails distributionally on the
  vortex line; integer = μ in that example (which in turn forces n,l
  integers).
- Simply-connected proviso: if D (or each component) is simply connected,
  Stokes gives integral 0 (Reddiger Sec.2.1 discussion; Thm.3.2 in
  Ref.[114]). Nonzero integers arise only via non-trivial topology from the
  excluded nodal/vortex set. No branch cut is added by Takabayasi.
- What the condition does NOT supply (explicit negatives for claim-61
  scoping): no finite vortex-core radius, no regularized core profile, no
  healing length, no core-energy / defect-energy number. It gives integers
  n ∈ Z only (winding numbers; in the hydrogen example n = μ). No length
  scale accompanies the integer. In particular the hydrogen most-probable
  radius 4a_0 (2p, m = ±1) and the radial node 2a_0 (2s) are
  standard-Schrodinger numbers, not Takabayasi core-model predictions;
  Takabayasi supplies no core-size ladder, no m ∝ E^p mapping, and no
  truncated-Q cap.
- 1953 follow-up: no additional finite-core or vortex-core table found in
  the swept secondaries; treated here as same-condition continuation.
  A direct 1953 page-level check remains open (primary-confirmation flag),
  but no source in this sweep attributes finite-core numbers to it.

## 4. Nodal treatment

- Nodes excluded from domain: Prop.2.1 requires Ψ non-vanishing on D; v is
  undefined where Ψ = 0. Loops must lie in D (ρ > 0). No regularization,
  no core profile.
- Nodal-surface illustration (Reddiger Ex.2.2, Eqs.(2.3a)–(2.3e), Fig.1):
  family Φ_a of n > 1 hydrogen superpositions; at a = 0 the state is real,
  an x–z nodal plane develops, γ (Eq.(2.3d)) no longer lies in D_0, so I(0)
  is strictly undefined (formal v_0 = 0 gives 0 but the loop is inadmissible).
  I(a) = −1 for a ∈ [−1,0), +1 for a ∈ (0,1]: jump is by domain loss, not by
  continuous phase accumulation through the node.
- Sibling sweep context only (no numbers taken): Bohm–Hiley / Holland line
  excludes R≠0/ψ≠0 with Q→∞ at nodes and non-crossing (Holland §3.3.8, p.85);
  see core_L1_bohm.md. Takabayasi condition is consistent with that
  exclusion (loops in D) and adds the integer constraint; it does not add
  a core cutoff.

## 5. Source list (as fetched 2026-11-07)

- Madelung-equations summary (condition wording + Takabayasi 1952 citation +
  Reddiger–Poirier 2023 citation):
  https://en.wikipedia.org/wiki/Madelung_equations
- Reddiger–Poirier abstract page: https://arxiv.org/abs/2207.11367
- Reddiger–Poirier HTML Sec.1–2 quoted (Eqs.1.1a–c,1.2,1.3,1.4; Prop.2.1;
  Ex.2.2 + Fig.1; Def.2.10; Rem.2.7; Sec.4):
  https://arxiv.org/html/2207.11367v1
- OUP primary (fetch 403 on 2026-11-07, not parsed — needs direct check):
  https://academic.oup.com/ptp/article/8/2/143/1883374
- Local repo context only (no numbers taken): theory/node_defect.py,
  theory/hydrogen_radial_nodes.py, theory/hydrogen_vortex.py,
  results/node_defect.{json,md}, results/radial_node.{json,md},
  results/hydrogen_vortex.{json,md}, claims 20/24/26/46/47, ADR-038/039.

Record ends. No verdict.
