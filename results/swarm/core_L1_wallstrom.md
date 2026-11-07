# core_L1_wallstrom — Wallstrom-objection literature sweep (claim-60 input, no verdict)

Scope: Fluid Universe claim-60 (HO-1 finite-core node probe, eps0=0.1a0).
Requested fields: exact quantization-condition statement, loop exclusion of
rho=0, equation/page numbers, whether any finite-core regularization is
proposed. Figure-and-equation level. Explicit negative where absent.
No claim-60 verdict is recorded here.

Search date: 2026-11-06. Method: webfetch of open pages only
(Wikipedia Madelung equations + Stochastic quantum mechanics;
arXiv:2207.11367 abstract + full HTML v1).
Websearch was unavailable (Exa 402: free allowance exhausted, no credit
balance). No APS (Wallstrom) or OUP (Takabayasi) full text was parsed;
no Fritsche-Hasse full text was parsed; no institutional access used.
Page/equation numbers below are verified only where stated as
"verified in this sweep"; all other labels are bibliographic pointers
with explicit negatives.

Config: `{"claim":60,"eps0_a0":0.1,"probe":"HO-1 finite-core node probe","scope":"Wallstrom-objection loop-exclusion rho=0; quantization-condition statement; equation/page numbers; finite-core regularization check","targets":["Wallstrom1994","Takabayasi1952/53","Fritsche-Hasse","Nelson-open-closed-loops"]}`
- Config SHA-256: `4f0c9143eaebe7360c80185798b644a854214abab073377ba3d2682c7cce56aa`
- Git: `0d14fcc8f87a17a664b950750ddc0251be6b7906` (dirty=True)
- Generated UTC: 2026-11-06 (webfetch sweep, no code run)

## 1. Exact quantization-condition statement (verified wording)

Verified in this sweep from two open sources (same formula, different
notation):

- Reddiger–Poirier, arXiv:2207.11367v1, Eq.(1.3) (published version:
  J. Phys. A 56(19) 193001, 2023, DOI 10.1088/1751-8121/acc7db):
  `m/(2πħ) ∮ v⃗·d r⃗ ∈ ℤ`, "at any given time for it to be physically
  acceptable". Drift field is defined in Eq.(1.2):
  `v⃗ = ħ/m Im(∇Ψ/Ψ) = ħ/(2im)(∇Ψ/Ψ − ∇Ψ*/Ψ*)`.
  Madelung system is Eqs.(1.1a)–(1.1c):
  (1.1a) `m(∂v⃗/∂t+(v⃗·∇)v⃗) = −∇V + ħ²/(2m) ∇ Δ√ρ/√ρ`,
  (1.1b) `∂ρ/∂t + ∇·(ρ v⃗) = 0`,
  (1.1c) `∇×v⃗ = 0`.
- Wikipedia "Madelung equations" (fetched 2026-11-06), circulation paragraph:
  `Γ ≐ ∮ m v·dl = 2πnħ` "for all integers n", cited there to
  Białynicki-Birula–Cieplak–Kamiński, Theory of Quanta (OUP 1992),
  pp.90–91, and Wyatt, Quantum Dynamics with Trajectories (2005), §13.5.
  Treat those page/section numbers as Wikipedia's citations, NOT as
  primary verification in this sweep.

Takabayasi attribution (secondary, verified as Reddiger's statement;
primary page not parsed here):

- Reddiger–Poirier §1: "Takabayasi [137] is often credited ... he was the
  first to suggest that it ought to be added to the Madelung equations,
  Eqs.(1.1a)–(1.1c) above (cf. p.155 in Ref.[137])". Same paragraph:
  in quantum-vortex literature the credit is "mostly given to Dirac
  (cf. p.67 in Ref.[42])"; Wentzel [151] "should not be credited";
  Hirschfelder et al. [68] gave the most detailed 'derivation' but
  "lacked mathematical rigor". Takabayasi's own gloss, quoted there:
  Eq.(1.3) "is so to speak the 'quantum condition' for fluidal motion
  and of ad hoc and compromising character for our formulation, just as
  the [Bohr–Sommerfeld] quantum condition for old quantum theory."
  Reddiger identifies this gloss with "Wallstrom's central objection
  voiced in Refs.[147,146]".

## 2. Loop exclusion of rho=0 (verified)

Verified in this sweep from Reddiger–Poirier Prop.2.1 (§2.1):

- Hypotheses: n∈ℕ, n≥2; D open in ℝⁿ; Ψ∈C¹(D,ℂ) such that Ψ does not
  vanish on D; v⃗ defined as in Eq.(1.2). Conclusion: the integral in
  Eq.(1.3) over any C¹-curve γ in D on a closed interval [a,b] with
  γ(a)=γ(b) satisfies Eq.(1.3). In words: closed loops are taken
  strictly inside the non-vanishing domain D; nodes (ρ=0) are excluded
  by hypothesis because v⃗ from Eq.(1.2) is otherwise not well-defined
  and not guaranteed continuous (fundamental theorem of calculus step).
- Companion topological note in the same section (verified): if v⃗ is a
  C¹ field and D is simply-connected (or countably many simply-connected
  components), a bounding surface exists and Stokes gives integral zero;
  with a "line singularity" the surface is not compact and Stokes cannot
  be employed — "roughly speaking, this is how the integral in
  Eq.(1.3) can fail to be zero". Reddiger notes wrongful applications of
  Stokes in the Takabayasi-condition literature.
- Worked illustration: Example 2.2 + Figure 1 (verified in HTML).
  Family Φ_a of n>1 hydrogen-like energy eigenfunctions mixing
  Y_1^{−1}, Y_1^{+1} (Eqs.2.3a–2.3c); closed equatorial loop γ
  (Eq.2.3d); integral function I(a) (Eq.2.3e). Figure 1 plots 100
  equidistant numerically evaluated I(a) on [−1,1]: I=−1 for a∈[−1,0),
  undefined at a=0 (Φ_0 real, x–z plane nodal surface cuts γ so
  γ⊄D_0), jump to +1 for a∈(0,1]. Demonstrates that superposition does
  not break Prop.2.1 because the nodal surface removes the loop from
  the admissible domain. Directly relevant to claim-60 loop protocol:
  loops must be certified to lie in D (ρ>0 everywhere on γ).
- Distributional counterpart (verified, Cor.2.11 / Eq.1.4, §2.3):
  for standard hydrogen Ψ_{nlμ}, using physics-notation Dirac delta,
  `(∇×v⃗)(t,x,y,z) = 2πμħ/m (0,0,δ(x)δ(y))ᵀ`.
  Hence the third Madelung equation Eq.(1.1c) is in general NOT satisfied
  in the distributional sense; drift fields are "quasi-irrotational"
  (Def.2.10, term motivated by Schönberg [130]). Quantum vorticity
  coefficient depends on magnetic quantum number μ, whose quantization
  forces n,l to integer values. Implication for claim-60: point-vortex
  loops around ρ=0 carry the δ-string; excluding ρ=0 (strong-solution
  reading) vs. admitting it distributionally are different claims.

## 3. Wallstrom 1994 (bibliographic verified; internal equations NOT verified)

- Bibliography (verified via Wikipedia ref [7], fetched 2026-11-06):
  T. C. Wallstrom, "Inequivalence between the Schrödinger equation and
  the Madelung hydrodynamic equations", Phys. Rev. A 49(3) 1613–1617
  (1994), DOI 10.1103/PhysRevA.49.1613.
- Objection summary as characterized by Reddiger–Poirier §1 (verified
  as Reddiger's characterization, not as Wallstrom primary text):
  (i) Refs.[147,146]: an additional ad hoc quantization condition must
  be imposed for equivalence (see §1 above); (ii) Ref.[148]: Madelung
  equations fail to yield unique time-evolution from initial data when
  zero-density connected components merge over time. Remark 3.4
  (verified label) terms (ii) the "Wallstrom phenomenon" and argues it
  is the failure of Schrödinger theory to discern physically equivalent
  yet mathematically inequivalent states, related to the Pauli problem
  (Refs.[136,150,149,5]).
- Explicit negative: no internal Wallstrom equation/page numbers, no
  figure numbers, and no finite-core regularization proposal were
  verified in this sweep (APS full text not accessed). Do not cite
  Wallstrom internal labels from this note.

## 4. Takabayasi 1952/53 (bibliographic verified; primary equation NOT verified)

- Bibliography (verified via Wikipedia ref [8], fetched 2026-11-06):
  T. Takabayasi, "On the Formulation of Quantum Mechanics associated
  with Classical Pictures", Prog. Theor. Phys. 8(2) 143–182 (1952),
  DOI 10.1143/PTP/8.2.143. Condition location cited second-hand as
  p.155 in Reddiger–Poirier §1 (see §1 above); primary p.155 not parsed
  here.
- 1953 companion paper (commonly cited as PTP 9, 187 (1953) in the
  literature): NOT verified in this sweep — no volume/page/equation
  numbers are recorded here. Treat any 1953 equation label as unverified.
- Explicit negative: no Takabayasi primary equation number, figure, or
  finite-core regularization was verified in this sweep.

## 5. Fritsche–Hasse (explicit negative with scope)

- Scope swept: open HTML/abstract-level references encountered via
  Reddiger–Poirier and stochastic-mechanics pages (Derakshani
  Refs.[40,39]; Grössing et al. [56]; de la Peña et al. [38] Eq.4.37 /
  §4.7 process-assumption route, Note 37 in Ref.[8]).
- Explicit negative: no Fritsche–Hasse primary equation numbers, page
  numbers, loop statements, or finite-core (eps0-type) regularization
  were verified in this sweep. No Fritsche–Hasse full text was parsed
  (search outage + no institutional access). In particular, no source
  swept proposes eps0=0.1a0 or any Gaussian/healing-length core that
  preserves the condition in §1. Reddiger's assessment of this response
  line (verified as Reddiger's opinion, §1): addressing Wallstrom by
  imposing stochastic-process assumptions "seems unlikely to convince
  contemporary critical voices"; the phenomenological-level (no a priori
  stochastic processes) refutation has not been established as futile.

## 6. Nelson open/closed loops (explicit negative with scope)

- What was verified: Wikipedia "Stochastic quantum mechanics" (fetched
  2026-11-06) documents Nelson's stochastic-quantization postulate
  (trajectory = real projection of complex semimartingale; extremized
  action S=E[∫L dt]; martingale quadratic variation
  d[M^i,M^j]=αħ/m δ^{ij} dt; time-reversed process exists with same
  laws), Nelson 1966 + Nelson 1985 lineage, Yasue 1979 terminology note,
  and velocity fields w_±, w_a, w_∘ with second-order field
  w_2^{ij}=αħ/m δ^{ij}. No loop-quantization equation appears on the
  fetched page.
- Explicit negative: no Nelson primary open-loop vs. closed-loop
  equation numbers, page numbers, or node-loop exclusion statement were
  verified in this sweep. The closed-loop condition in §1 belongs to the
  Madelung/Takabayasi line (Reddiger Eq.1.3), not to a Nelson equation
  verified here. Any claim that Nelson 1966 or Nelson 1985 contains a
  specific open/closed-loop equation must not cite this note.

## 7. Finite-core regularization (explicit negative — central for claim-60)

- No finite vortex-core radius, no eps0=0.1a0 value, no Gaussian-smoothed
  core, no healing-length regulator, no truncated-Q cap, and no
  "core cutoff preserves quantization" theorem was found in any source
  parsed in this sweep (Wikipedia Madelung + Stochastic pages;
  arXiv:2207.11367 abstract + full HTML v1).
- What the parsed sources do instead:
  (a) exclude ρ=0 from the strong-solution domain (Reddiger Prop.2.1 D;
  cf. sibling sweep core_L1_bohm.md: R≠0/ψ≠0, Holland §3.3.8, Q→∞);
  (b) show distributional failure at the axis (Reddiger Eq.1.4,
  quasi-irrotationality, §2.3); (c) construct explicit non-quantized
  stationary strong solutions for the 2D isotropic harmonic oscillator
  (Reddiger §4, claimed by Wallstrom [146]) — i.e. without the condition
  there are MORE Madelung solutions, not a core that derives the
  condition; (d) note C¹ regularity is already too restrictive
  (Smolin Sec.IV [134]; Reddiger §2.2 / Remark 2.7: "serious doubt
  whether the condition can be appropriately extended" to distributions).
- Therefore, for the HO-1 probe with eps0=0.1a0: the literature swept
  here supplies the condition to beat (§1), the loop-exclusion rule
  (§2), and the distributional/extra-solution warnings above, but
  supplies no finite-core prescription, no eps0 value, and no
  "regularization preserves quantization" result. Any eps0=0.1a0 choice
  is a new local probe choice, not a literature value recovered here.
- Scope limit: absences above are relative to the open-access set
  actually parsed (listed in §8) under the 2026-11-06 search outage;
  paywalled full texts (Wallstrom PRA, Takabayasi PTP, Fritsche–Hasse
  monograph/papers, Nelson monograph) were not parsed and could in
  principle contain additional discussion — this sweep makes no claim
  about their unparsed interiors beyond the secondary characterizations
  quoted with provenance.

## 8. Source list (as fetched)

- Wikipedia Madelung equations (condition + refs [7,8,9]):
  https://en.wikipedia.org/wiki/Madelung_equations
- Wikipedia Stochastic quantum mechanics (Nelson postulate block):
  https://en.wikipedia.org/wiki/Stochastic_quantum_mechanics
- Reddiger–Poirier abstract:
  https://arxiv.org/abs/2207.11367
- Reddiger–Poirier full HTML v1 (Eqs.1.1a–c,1.2,1.3,1.4; Prop.2.1;
  Ex.2.2 + Fig.1; Def.2.10; Rem.2.7; Rem.3.4; §4):
  https://arxiv.org/html/2207.11367v1
- Bibliographic pointers NOT parsed (listed for control, no equation
  numbers taken): Wallstrom PRA 49:1613 (DOI above); Takabayasi PTP
  8:143 (DOI above); Białynicki-Birula et al. 1992 pp.88–91 / Wyatt 2005
  §13.5 (via Wikipedia); Fritsche–Hasse / Nelson 1966/1985 full texts.

Record ends. No verdict.
