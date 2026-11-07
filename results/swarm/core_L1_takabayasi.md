# core_L1_takabayasi — Literature sweep record (claim-60 input, no verdict)

Scope: Takabayasi 1952/53 quantization condition only, as single-component
prior art (no branch construct). Requested fields: exact condition, nodal
treatment, equation/page numbers. Explicit negative where absent (finite-core
numbers). No claim-60 verdict is recorded here.

Search date: 2026-11-06. Method: webfetch of Madelung-equations summary +
arXiv HTML of Reddiger–Poirier review (2207.11367v1) + bibliography control.
No full-text copy of Takabayasi 1952/53 was parsed here (OUP fetch 403);
original equation numbers / page numbers below are via the secondary that
quotes the primary (Reddiger–Poirier 2023, citing p.155). Primary-confirmation
flag carried: exact original numbering needs direct PTP check.

## 1. Bibliography

- T. Takabayasi, "On the Formulation of Quantum Mechanics associated with
  Classical Pictures", Prog. Theor. Phys. 8(2), 143–182 (1952).
  DOI: 10.1143/PTP/8.2.143. ISSN 0033-068X.
  Condition first proposed p.155 (per Reddiger–Poirier 2023, Sec.1).
- T. Takabayasi 1953 continuation(s) (hydrodynamical / stochastic-line
  follow-ups). Same single-component framework; no separate core model
  attributed in the material swept. Exact 1953 citation form not verified
  here — primary-confirmation flag.
- Review control: M. Reddiger and B. Poirier, "Towards a mathematical Theory
  of the Madelung Equations: Takabayasi's Quantization Condition, Quantum
  Quasi-Irrotationality, ...", arXiv:2207.11367v1 [math-ph] (22 Jul 2022),
  published J. Phys. A 56, 193001 (2023). DOI: 10.1088/1751-8121/acc7db.
  Sec.1 (pp. quoted), Sec.2.1 Prop.2.1, Sec.2.3 Cor.2.11 / Eq.(1.4).
- Auxiliary statements of the same condition: I. Bialynicki-Birula et al.,
  Theory of Quanta (1992), pp.90–91; R. E. Wyatt, Quantum Dynamics with
  Trajectories (2005), Sec.13.5. Cited here only for modern wording, not as
  Takabayasi substitutes.

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
  Original equation label on that page NOT verified here.
- Reddiger–Poirier: drift definition Eq.(1.2); condition Eq.(1.3); Madelung
  triple Eqs.(1.1a)–(1.1c) with third Eq.(1.1c) ∇×v = 0; Schrodinger Eq.(2.1).
- Hydrogen-vortex distributional curl Eq.(1.4) / Cor.2.11 (see Sec.3 below).
- Tertiary wording: Γ = ∮ m v·dl = 2πnħ (Bialynicki-Birula pp.90–91;
  Wyatt Sec.13.5, via Wikipedia). Not Takabayasi original labels.

## 3. Nodal treatment

- Nodes excluded from domain: Prop.2.1 requires Ψ non-vanishing on D; v is
  undefined where Ψ = 0. Loops must lie in D (ρ > 0). No regularization, no
  core profile, no healing length.
- Nodal-surface illustration (Reddiger Ex.2.2, Eqs.(2.3a)–(2.3e), Fig.1):
  family Φ_a of n > 1 hydrogen superpositions; at a = 0 the state is real,
  an x–z nodal plane develops, γ (Eq.(2.3d)) no longer lies in D_0, so I(0)
  is strictly undefined (formal v_0 = 0 gives 0 but the loop is inadmissible).
  I(a) = −1 for a ∈ [−1,0), +1 for a ∈ (0,1]: jump is by domain loss, not by
  continuous phase accumulation through the node.
- Distributional consequence (Reddiger Eq.(1.4) / Cor.2.11, "quantum
  quasi-irrotationality" after Schönberg): for standard Ψ_nlμ (Coulomb),
  in physics delta notation,
  (∇×v)(t,x,y,z) = (2π μ ħ/m) (0, 0, δ(x)δ(y))^T.
  I.e. the third Madelung equation ∇×v = 0 fails distributionally on the
  vortex line; integer in Eq.(1.3) equals magnetic quantum number μ in that
  example (which in turn forces n,l integers). No finite core is introduced
  to obtain this; it is a line singularity.
- Simply-connected proviso: if D (or each component) is simply connected,
  Stokes gives integral 0 (Reddiger Sec.2.1 discussion; Thm.3.2 in
  Ref.[114]). Nonzero integers arise only via non-trivial topology from the
  excluded nodal/vortex set. No branch cut is added by Takabayasi.

## 4. Finite-core numbers — explicit negative

- No finite vortex-core radius, no regularized core profile, no core-energy /
  defect-energy number, and no quantum-defect number from a finite core was
  found in the Takabayasi 1952/53 material as represented in this sweep.
- What the condition gives instead: integers n ∈ Z only (winding numbers;
  in the hydrogen example n = μ). No length scale accompanies the integer.
- In particular: the hydrogen most-probable radius 4a_0 (2p, m = ±1) and the
  radial node 2a_0 (2s) are standard-Schrödinger numbers, not Takabayasi
  core-model predictions; Takabayasi supplies no core-size ladder, no
  m ∝ E^p mapping, and no truncated-Q cap.
- 1953 follow-up: no additional finite-core table found in the swept
  secondaries; treated here as same-condition continuation. A direct 1953
  page-level check remains open (primary-confirmation flag), but no source
  in this sweep attributes finite-core numbers to it.

## 5. Source list (as fetched)

- Madelung-equations summary (condition wording + Takabayasi 1952 citation +
  Reddiger–Poirier 2023 citation):
  https://en.wikipedia.org/wiki/Madelung_equations
- Reddiger–Poirier abstract page: https://arxiv.org/abs/2207.11367
- Reddiger–Poirier HTML (Sec.1–2 quoted above):
  https://arxiv.org/html/2207.11367v1
- OUP primary (fetch 403, not parsed — needs direct check):
  https://academic.oup.com/ptp/article/8/2/143/1883374
- Local repo context only (no numbers taken): theory/node_defect.py,
  theory/hydrogen_radial_nodes.py, results/node_defect.{json,md},
  results/radial_node.{json,md}, claims 20/24/26/46/47, ADR-038/039.

Record ends. No verdict.
