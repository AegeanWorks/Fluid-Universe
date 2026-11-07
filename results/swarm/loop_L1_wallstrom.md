# loop_L1_wallstrom — Literature sweep record (claim-61 input, no verdict)

Scope: Fluid Universe claim-61 (2p core-threading loop probe, delta0=0.1a0).
Target: Wallstrom 1994 quantization-condition paper — exact loop-exclusion
wording (which loops must avoid rho=0), statement form, page/equation
numbers. Fallback where primary is paywalled: Reddiger–Poirier 2023
Eqs.1.2–1.4 verified wording. Explicit negative where absent.
No claim-61 verdict is recorded here.

Search date: 2026-11-07. Method: webfetch of open pages only
(Wikipedia Madelung equations; arXiv abs + full HTML v1 of
Reddiger–Poirier 2207.11367; APS abstract page for Wallstrom 1994;
OUP primary page access attempt for Takabayasi 1952).
Websearch unavailable (Exa 402: free allowance exhausted, no credit
balance — same outage as 2026-11-06 sweep). No APS (Wallstrom) full text
was parsed; no OUP (Takabayasi) full text was parsed; no institutional
access used. Page/equation numbers below are verified only where stated
as "verified in this sweep"; all other labels are bibliographic pointers
with explicit negatives.

Config: `{"claim":61,"delta0_a0":0.1,"probe":"2p core-threading loop probe","scope":"Wallstrom-objection loop-exclusion rho=0; quantization-condition statement; equation/page numbers","targets":["Wallstrom1994","Takabayasi1952/53","Reddiger-Poirier2023-fallback"]}`
- Config SHA-256: `646fcf1253931eb2dc587bc5ca67ff0a90ea3efddccf09001fcc351c2775f7f2`
- Git: `95b5403e2c3a620131ebed326f9054fa6923b401` (clean=True)
- Generated UTC: 2026-11-07 (webfetch sweep, no code run)

Local repo context only (no numbers taken): theory/hydrogen_vortex.py,
theory/node_defect.py, theory/hydrogen_radial_nodes.py,
results/hydrogen_vortex.{json,md}, results/node_defect.{json,md},
results/radial_node.{json,md}, claims 20/24/26/46/47, SCORECARD claim-60
finite-core probe (eps0=0.1a0, HO-1 only; 2p/2s queued).

## 1. Wallstrom 1994 (bibliographic verified; internal equations NOT verified)

- Bibliography (verified via APS abstract page fetched 2026-11-07):
  T. C. Wallstrom, "Inequivalence between the Schrödinger equation and
  the Madelung hydrodynamic equations", Phys. Rev. A 49(3) 1613–1617
  (1994), DOI 10.1103/PhysRevA.49.1613. Received 2 June 1993.
  Published 1 March 1994. Theoretical Division, Los Alamos National
  Laboratory.
- Abstract (verified as APS abstract text, sic "amd" retained):
  "By differentiating the Schrödinger equation and separating the real
  amd imaginary parts, one obtains the Madelung hydrodynamic equations,
  which have inspired numerous classical interpretations of quantum
  mechanics. Such interpretations frequently assume that these equations
  are equivalent to the Schrödinger equation, and thus provide an
  alternative basis for quantum mechanics. This paper proves that this
  is incorrect: to recover the Schrödinger equation, one must add by
  hand a quantization condition, as in the old quantum theory."
- Access status (verified 2026-11-07): APS page returns
  "Authorization Required … Log in via your institution / APS Member /
  Buy Article". PDF link
  https://journals.aps.org/pra/pdf/10.1103/PhysRevA.49.1613 present but
  not fetched (paywalled, no institutional access). References section
  marked "Subscription Required".
- Explicit negative (scope: open sources only): no Wallstrom internal
  equation numbers, no internal page numbers, no figure numbers, no
  verbatim loop-exclusion sentence, and no finite-core / delta0-type
  regularization proposal were verified in this sweep. Do not cite any
  Wallstrom internal label from this note. Statement-form and
  loop-exclusion wording below are verified fallback (Reddiger–Poirier),
  not Wallstrom primary text.
- Secondary characterization (verified as Reddiger–Poirier's
  characterization, not as Wallstrom primary text; arXiv HTML v1 §1):
  (i) Refs.[147,146]: additional ad hoc quantization condition must be
  imposed for equivalence; (ii) Ref.[148]: Madelung equations fail to
  yield unique time-evolution from initial data when zero-density
  connected components merge over time (Remark 3.4 "Wallstrom
  phenomenon", related to Pauli problem). Takabayasi gloss quoted
  there — Eq.(1.3) "is so to speak the 'quantum condition' for fluidal
  motion and of ad hoc and compromising character … just as the
  [Bohr–Sommerfeld] quantum condition" — is identified as "Wallstrom's
  central objection voiced in Refs.[147,146]".

## 2. Statement form (verified fallback: Reddiger–Poirier Eqs.1.1–1.3)

Verified in this sweep from two open sources (same formula, different
notation):

- Reddiger–Poirier, arXiv:2207.11367v1, §1 (published version:
  J. Phys. A 56(19) 193001, 2023, DOI 10.1088/1751-8121/acc7db):
  Madelung triple Eqs.(1.1a)–(1.1c):
  (1.1a) `m(∂v⃗/∂t+(v⃗·∇)v⃗) = −∇V + ħ²/(2m) ∇ Δ√ρ/√ρ`,
  (1.1b) `∂ρ/∂t + ∇·(ρ v⃗) = 0`,
  (1.1c) `∇×v⃗ = 0`.
  Drift field Eq.(1.2):
  `v⃗ = ħ/m Im(∇Ψ/Ψ) = ħ/(2im)(∇Ψ/Ψ − ∇Ψ*/Ψ*)`.
  Quantization condition Eq.(1.3):
  `m/(2πħ) ∮ v⃗·d r⃗ ∈ ℤ`,
  "at any given time for it to be physically acceptable".
- Wikipedia "Madelung equations", circulation paragraph (fetched
  2026-11-07): `Γ ≐ ∮ m v·dl = 2πnħ` "for all integers n", cited there
  to Białynicki-Birula–Cieplak–Kamiński, Theory of Quanta (OUP 1992),
  pp.90–91, and Wyatt, Quantum Dynamics with Trajectories (2005),
  §13.5. Those page/section numbers are Wikipedia's citations, NOT
  primary verification in this sweep.
- Takabayasi attribution (secondary, verified as Reddiger's statement;
  primary p.155 not parsed here — OUP fetch 403 on 2026-11-07):
  "Takabayasi [137] … was the first to suggest that it ought to be
  added to the Madelung equations, Eqs.(1.1a)–(1.1c) above
  (cf. p.155 in Ref.[137])". Vortex-literature credit mostly to Dirac
  (cf. p.67 in their Ref.[42]); Wentzel [151] "should not be credited"
  (different 1D meromorphic/residue context).

## 3. Loop exclusion of rho=0 (verified fallback: Prop.2.1 + Ex.2.2 + Eq.1.4)

Verified in this sweep from Reddiger–Poirier HTML v1 §2:

- Prop.2.1 (§2.1) hypotheses: n∈ℕ, n≥2; D open in ℝⁿ; Ψ∈C¹(D,ℂ) such
  that Ψ does not vanish on D; v⃗ defined as in Eq.(1.2). Conclusion:
  the integral in Eq.(1.3) over any C¹-curve γ in D on a closed interval
  [a,b] with γ(a)=γ(b) satisfies Eq.(1.3). In words: closed loops are
  taken strictly inside the non-vanishing domain D; nodes (ρ=0) are
  excluded by hypothesis because v⃗ from Eq.(1.2) is otherwise not
  well-defined and not guaranteed continuous (fundamental-theorem step).
  This is the operative answer to "which loops must avoid rho=0": ALL
  loops entering the condition must satisfy γ ⊂ D, i.e. ρ>0 everywhere
  on γ. A loop meeting ρ=0 is inadmissible in the strong-solution
  reading — the integral is not defined there, not merely non-integer.
- Topological note in same section (verified): if v⃗ is a C¹ field and D
  is simply-connected (or countably many simply-connected components),
  a bounding surface exists and Stokes gives integral zero; with a
  "line singularity" the surface is not compact and Stokes cannot be
  employed — "roughly speaking, this is how the integral in Eq.(1.3)
  can fail to be zero". Notes wrongful Stokes applications in the
  Takabayasi-condition literature.
- Worked illustration: Example 2.2 + Figure 1 (verified in HTML).
  Family Φ_a of n>1 hydrogen-like energy eigenfunctions mixing
  Y_1^{−1}, Y_1^{+1} (Eqs.2.3a–2.3c); closed equatorial loop γ
  (Eq.2.3d); integral function I(a) (Eq.2.3e). Figure 1 plots 100
  equidistant numerically evaluated I(a) on [−1,1]: I=−1 for a∈[−1,0),
  undefined at a=0 (Φ_0 real, x–z nodal surface cuts γ so γ⊄D_0), jump
  to +1 for a∈(0,1]. Superposition does not break Prop.2.1 because the
  nodal surface removes the loop from the admissible domain. Directly
  relevant to claim-61 loop protocol: loops must be certified to lie in
  D (ρ>0 everywhere on γ); apparent discontinuities are domain loss,
  not phase accumulation through a node.
- Distributional counterpart (verified, Cor.2.11 / Eq.1.4, §2.3):
  for standard hydrogen Ψ_{nlμ}, using physics-notation Dirac delta,
  `(∇×v⃗)(t,x,y,z) = 2πμħ/m (0,0,δ(x)δ(y))ᵀ`.
  Hence the third Madelung equation Eq.(1.1c) is in general NOT
  satisfied in the distributional sense; drift fields are
  "quasi-irrotational" (Def.2.10, term motivated by Schönberg [130]).
  Quantum vorticity coefficient depends on magnetic quantum number μ,
  whose quantization forces n,l to integer values. Implication for
  claim-61: point-vortex loops around ρ=0 carry the δ-string; excluding
  ρ=0 (strong-solution reading) vs. admitting it distributionally are
  different claims. A loop of radius delta0>0 encircling the axis stays
  in D; the axis itself (delta0=0) is excluded.

## 4. Takabayasi 1952/53 (bibliographic verified; primary equation NOT verified)

- Bibliography (verified via Wikipedia ref [8], fetched 2026-11-07):
  T. Takabayasi, "On the Formulation of Quantum Mechanics associated
  with Classical Pictures", Prog. Theor. Phys. 8(2) 143–182 (1952),
  DOI 10.1143/PTP/8.2.143. Condition location cited second-hand as
  p.155 in Reddiger–Poirier §1; primary p.155 not parsed here
  (OUP fetch 403 on 2026-11-07, same as 2026-11-06 sweep).
- 1953 companion (commonly cited as PTP 9, 187 (1953)): NOT verified
  in this sweep — no volume/page/equation numbers recorded here.
- Explicit negative: no Takabayasi primary equation number, figure, or
  finite-core / delta0-type regularization was verified in this sweep.

## 5. Finite-core / delta0=0.1a0 (explicit negative — central for claim-61)

- No finite vortex-core radius, no delta0=0.1a0 value, no
  Gaussian-smoothed core, no healing-length regulator, no truncated-Q
  cap, and no "core cutoff preserves quantization" theorem was found in
  any source parsed in this sweep (Wikipedia Madelung page;
  arXiv:2207.11367 abstract + full HTML v1; APS abstract page).
- What the parsed sources do instead:
  (a) exclude ρ=0 from the strong-solution domain (Prop.2.1 D;
  cf. sibling sweeps core_L1_bohm.md: R≠0/ψ≠0, Holland §3.3.8, Q→∞;
  loop_L1_takabayasi.md §§2–4);
  (b) show distributional failure at the axis (Eq.1.4,
  quasi-irrotationality, §2.3); (c) construct explicit non-quantized
  stationary strong solutions for the 2D isotropic harmonic oscillator
  (§4, claimed by Wallstrom [146]) — i.e. without the condition there
  are MORE Madelung solutions, not a core that derives the condition;
  (d) note C¹ regularity is already too restrictive (Smolin Sec.IV
  [134]; §2.2 / Remark 2.7: "serious doubt whether the condition can
  be appropriately extended" to distributions).
- Therefore, for the 2p core-threading probe with delta0=0.1a0: the
  literature swept here supplies the condition to beat (§2 above), the
  loop-exclusion rule (§3 above: γ ⊂ D, ρ>0 on γ; delta0>0 circle
  around the 2p m=±1 vortex line is admissible, the axis itself is
  not), and the distributional/extra-solution warnings above, but
  supplies no finite-core prescription, no delta0 value, and no
  "regularization preserves quantization" result. Any delta0=0.1a0
  choice is a new local probe choice, not a literature value recovered
  here. Standard-Schrödinger numbers (2p most-probable radius 4a0) are
  not core-model predictions.
- Scope limit: absences above are relative to the open-access set
  actually parsed (listed in §6) under the 2026-11-07 search outage;
  paywalled full texts (Wallstrom PRA, Takabayasi PTP, Fritsche–Hasse,
  Nelson monograph) were not parsed and could in principle contain
  additional discussion — this sweep makes no claim about their
  unparsed interiors beyond the secondary characterizations quoted with
  provenance.

## 6. Source list (as fetched 2026-11-07)

- Wikipedia Madelung equations (History + circulation + refs [7,8,9]):
  https://en.wikipedia.org/wiki/Madelung_equations
- Reddiger–Poirier abstract:
  https://arxiv.org/abs/2207.11367
- Reddiger–Poirier full HTML v1 (Eqs.1.1a–c,1.2,1.3,1.4; Prop.2.1;
  Ex.2.2 + Fig.1; Def.2.10; Rem.2.7; Rem.3.4; §4):
  https://arxiv.org/html/2207.11367v1
- APS abstract page (bibliographic + abstract verified; full text
  Authorization Required, not parsed):
  https://journals.aps.org/pra/abstract/10.1103/PhysRevA.49.1613
- OUP primary (fetch 403 on 2026-11-07, not parsed — needs direct check):
  https://academic.oup.com/ptp/article/8/2/143/1883374
- Bibliographic pointers NOT parsed (listed for control, no equation
  numbers taken): Wallstrom PRA 49:1613 (DOI above); Takabayasi PTP
  8:143 (DOI above); Białynicki-Birula et al. 1992 pp.88–91 / Wyatt 2005
  §13.5 (via Wikipedia).

Record ends. No verdict.
