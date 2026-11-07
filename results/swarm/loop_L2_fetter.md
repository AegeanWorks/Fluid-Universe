# loop_L2_fetter — Literature sweep record (claim-61 input, no verdict)

Scope: Fluid Universe claim-61 planning. Target: Fetter (rotating helium /
trapped BEC vortex core structure): core density profile, core radius
definition, velocity field form inside vs outside core, equation/page numbers.
Analogy-only to quantum 2p vortex; no quantum claim is made here. No claim-61
verdict is recorded here.

Search date: 2026-11-07. Method: websearch unavailable (parallel.ai 429 on all
queries tried: Fetter/RMP/core/healing-length). Webfetch of open pages succeeded
(arXiv abs 0801.2952; arXiv HTML 0801.2952v1 full text; Wikipedia Quantum vortex;
Wikipedia Gross-Pitaevskii equation). No institutional access used. No APS
(Fetter 1965, Anderson 1966) or other paywalled full text beyond the open arXiv
preprint was parsed here. Same handling as sibling sweep loop_L2_reconnection.md:
RMP volume/page range is a bibliographic pointer; HTML Eq/section labels below
marked "verified in this sweep" are citable only as arXiv-HTML labels, NOT as
audited RMP page/equation numbers until checked against a library copy.

Config: `{"claim":61,"probe":"fetter-core-structure","scope":"core density profile; core radius definition; velocity inside vs outside; eq/page numbers","targets":["Fetter-RMP2009-III.1","Fetter-RMP2009-IV.2.3","rotating-He-bridge"]}`
- Git: fluid-universe worktree (results/swarm context)
- Generated UTC: 2026-11-07 (webfetch sweep, no code run)

## 1. Bibliography (pointers; interior RMP pages NOT verified here)

S1. A. L. Fetter, "Rotating trapped Bose-Einstein condensates",
  Rev. Mod. Phys. 81, 647-691 (2009). DOI: 10.1103/RevModPhys.81.647.
  Preprint verified in this sweep only as open access:
  arXiv:0801.2952 [cond-mat.stat-mech] (submitted 18 Jan 2008),
  abs page + HTML experimental (arxiv.org/html/0801.2952v1) parsed here.
  44 pages, 18 figures (per abs page). RMP interior pagination and
  RMP-printed equation numbers NOT verified here — use HTML labels §2-§5
  below as find-aids only.

S2. A. L. Fetter, "Vortices in an Imperfect Bose Gas. I. The Condensate With
  No Circulation" / sequel series, Phys. Rev. 138, A429 (1965) and follow-ups.
  Pointer — full text NOT parsed here. Listed because it is the original
  GP-vortex-line vehicle cited by S1 Sec. III.1 via Gross [S3] / Pitaevskii [S4].

S3. E. P. Gross, "Structure of a quantized vortex in boson systems,"
  Nuovo Cimento 20(3), 454-457 (1961). Pointer — NOT parsed here.
  S1 Sec. III.1 states Gross (+ S4) introduced the GP equation specifically
  to describe a single straight vortex in uniform dilute BEC.

S4. L. P. Pitaevskii, "Vortex lines in an imperfect Bose gas,"
  Sov. Phys. JETP 13(2), 451-454 (1961). Pointer — NOT parsed here.
  Paired with S3 as the GP-vortex origin in S1 Sec. III.1.

S5. Rotating-He bridge cited inside S1 (pointers, NOT parsed here):
  R. J. Donnelly, "Quantized Vortices in Helium II" (book, Chap./Sec.
  refs throughout S1, e.g. Secs. 2.5/5.1/5.2/4.5/Chap.5/6.1 per HTML);
  E. J. Yarmchuk et al., Phys. Rev. Lett. 43, 214 (1979) (S1 Fig.8,
  stable He-II vortex photographs vs BEC Fig.7);
  R. P. Feynman He-II vortex discussion (S1 Sec. II.2 via Ref.[91]).
  No He-II core-radius equation is transcribed from these primaries here.

S6. Controls verified in this sweep via open pages (no new numbers taken):
  Wikipedia "Quantum vortex" (quantization section); Wikipedia
  "Gross-Pitaevskii equation" (Form-of-equation + Healing-length +
  Soliton sections; Foot 2005 pp.231-240 cited there).

## 2. Core radius definition — healing length (verified HTML wording)

Verified in S1 HTML (arXiv:0801.2952v1) Sec. II.2, HTML Eqs. (21)/(31)/(32):

- (HTML Eq.21) Uniform condensate, bulk density n, coupling g = 4πaħ²/M:
  `ξ = ħ / sqrt(2 M g n) = 1 / sqrt(8π a n)`,
  "which characterizes the length scale over which a localized alteration
  in the condensate density heals back to its uniform value n".
  Corollary HTML Eq.(22): `n^{2/3} ξ² = 1/(8π n^{1/3} a) >> 1`,
  i.e. healing length must exceed interparticle spacing in any GP description;
  Bogoliubov small-depletion requirement `n a³ << 1`.

- (HTML Eqs.28-32, trapped TF condensate) Convention: use central density
  n(0) to define healing length:
  `ξ² = 1 / [8π a n(0)]` [HTML Eq.31].
  With TF radius R0 >> oscillator length d0 = sqrt(ħ/Mω0):
  `ξ² = d0⁴/R0²`, equivalently `ξ/d0 = d0/R0` [HTML Eq.32],
  hence TF scale separation `ξ << d0 << R0`.
  RMP page numbers for these equations NOT verified — cite only as
  S1-HTML Sec.II.2 Eqs.(21)/(31)/(32).

- (HTML Sec.III.1) "The resulting centrifugal barrier forces the amplitude
  f(r) to vanish linearly for r ≲ ξ, which characterizes the vortex core,
  and f(r) ≈ 1 for r >> ξ." Core radius ∼ ξ by definition.
  Particle-current maximum near r ≈ ξ (see §4).

- (HTML Sec.IV.2.3, rapid rotation) Rotation expands cloud, lowers n(0)
  and μ = g n(0), so ξ grows with Ω. Dimensionless fractional core area:
  `ξ²/l² = ħΩ/(2μ)` [HTML Eq.95 context], with vortex-cell radius
  `l = sqrt(ħ/MΩ)` [HTML Eq.75, Sec.IV.1 lattice-density derivation].
  TF form [HTML Eq.95]:
  `ξ²/l² = ħΩ/2μ = Ω̄/(1-Ω̄²)^{2/5} · [(ω⊥/ωz)(d⊥/15Na)]^{2/5}`,
  Ω̄ = Ω/ω⊥. Small until Ω̄→1, then grows rapidly (denominator).
  Variational correction (Fischer-Baym, Baym-Pethick refs in S1):
  ξ² treated as variational parameter, ratio saturates ≈ 1/2
  (core occupies significant fraction of cell; TF then fails).
  Coddington et al. Fig.14 (S1-HTML Sec.IV.2.3): fractional core area vs
  2ħΩ/μ, TF dashed vs LLL dotted; different fits/definitions give somewhat
  different numerical values (explicit S1 caveat). No single "core radius"
  number beyond ξ and ξ²/l² is asserted here.

Explicit negative: no He-II core radius in Å / coherence-length number is
transcribed from S1/S5 primaries in this sweep; the only verified length
definitions are the BEC ξ forms above (HTML labels). Do NOT cite an
"S1 Eq.(X) p.Y" RMP location from this note.

## 3. Core density profile (verified HTML wording)

Verified in S1 HTML Sec. III.1 (one vortex in unbounded condensate):

- Ansatz [HTML Sec.III.1, unnumbered display verified as page statement,
  HTML Eq.48 context]: condensate Ψ(r) = sqrt(n) χ(r) with
  `χ(r) = e^{iφ} f(r/ξ)`,
  r,φ plane-polar, f → 1 for large r. Bulk density n.
  Density: `n(r) = n |f(r/ξ)|²`, so ρ = 0 on axis, depletes for r < O(ξ).

- Shape (verified wording): centrifugal barrier ⇒ f vanishes linearly for
  r ≲ ξ; f ≈ 1 for r >> ξ. No analytic closed form; numerical solution of
  radial GP equation required (S1 cites e.g. Pethick-Smith Sec.9.2 Fig.9.1).

- Variational fit [HTML Sec.III.1, Ref.[77] attribution]:
  scaled variable x = r/ξ,
  `f_var(x) = x / sqrt(x² + 2)`.
  S1 wording: "provides a good fit to the numerical solution". No fit-error
  number verified here.

- Energy per unit length [HTML Sec.III.1 verified form]:
  `E_v ≈ (πħ²n/M) ln(1.46 R/ξ)`, R large-distance cutoff;
  "Apart from the additive numerical constant, this result is essentially
  the integral of (1/2)Mv²n over a large circle of radius R with a
  short-distance cutoff of order ξ" (Ginzburg-Pitaevskii Ref.[99] context).
  RMP page/equation number for E_v NOT verified — HTML label only.

- Trapped case [HTML Sec.III.2 verified wording]: same phase e^{iφ},
  hydrodynamic velocity again v = ħ φ̂/(Mr) [HTML Eq.49 context];
  centrifugal barrier creates axial node; topology changes from ellipsoidal
  to toroidal "because of the hole along the vortex core". TF density taken
  as unperturbed parabolic [HTML Eq.27] except core cut off at ξ
  (log-divergent circulating energy cut off at ξ).

- Sound-speed restatement [HTML Sec.III.1 verified]:
  s = sqrt(gn/M) = ħ/(√2 M ξ), so circulating flow (Eq.49) becomes
  supersonic for r ≲ ξ; S1 phrase: "local acoustic cavitation can be
  considered the source of the vortex core".

Explicit negatives: no Gross-1961 / Pitaevskii-1961 interior radial-equation
number, no Fig.9.1 numbers from Pethick-Smith, no Coddington fit-function or
table values, and no RMP page for f_var were verified here.

## 4. Velocity field inside vs outside core (verified HTML wording)

- Outside / far field (r >> ξ), singly quantized straight line:
  [HTML Sec.III.1 Eq.49 context, also Sec.III.2 for trapped]:
  `v = ħ φ̂ / (M r)` (azimuthal, irrotational, circular streamlines,
  magnitude diverges formally as r → 0).
  Circulation `κ = ∮ dl·v = 2πħ/M` (S1 Sec.II.2 hydrodynamic derivation
  via single-valued Ψ; Feynman/London/Donnelly refs cited there).
  Singular vorticity [HTML Sec.III.1 Eq.50 context + Sec.IV.1 Eq.75 context]:
  `∇×v = (2πħ/M) δ^{(2)}(r-r0) ẑ` for vortex at r0.
  Stokes: contour with Nv vortices gives Γ = (2πħ/M)Nv vs classical 2ΩA,
  yielding mean areal density nv = MΩ/(πħ), cell radius l above.

- Inside / core (r ≲ ξ): the 1/r velocity formula is NOT replaced by a
  different v(r) law in S1; it is regularized by density depletion.
  Verified S1 statements:
  (a) f ∝ r (linear) ⇒ n(r) ∝ r², so mass current
  `|j(r)| = n|f|² ħ/(Mr)` vanishes both at center and far away, with
  maximum near r ≈ ξ;
  (b) BEC-side remark [HTML Sec.IV.2.6 verified]: "condensate density n(r)
  around a vortex decreases rapidly from its bulk value n for r ≲ ξ inside
  the vortex core. Consequently, the circulating mass current
  j(r) = M n(r)v(r) = n(r)ħ/r reaches a maximum 'critical' value
  jc ∼ nħ/ξ for r ≈ ξ [compare discussion below Eq.(51)]";
  (c) supersonic r ≲ ξ per s above — core as cavitation, not a solid-body
  patch. No Rankine combined (solid-body core + 1/r tail) profile is
  attributed to the quantum core in S1; that combination appears only as a
  classical-fluid contrast in sibling controls (see loop_L2_reconnection.md
  R3), not as S1's BEC core law.

- Classical contrast verified in S1 (for separation, NOT a quantum claim):
  classical precession of straight vortex in cylinder via image vortex
  [S1 Sec.III.2.2, cf. Pethick-Smith Sec.9.4]; Kelvin-wave / Tkachenko
  treatments that start from incompressible classical flow then add
  compressibility [S1 Secs.III.4.5/IV.2.4]. These are cited here only to
  prevent misreading a classical solid-body interior as Fetter's quantum
  core law — S1's quantum core is density-depletion, not solid-body rotation.

Explicit negative: no "Fetter interior v(r) = Ω×r for r<ξ" formula exists in
the material verified here; any such Rankine assignment would be a classical
analogue, not S1's GP core. No RMP page number for Eqs.(49)/(50)/(75) is
verified here — HTML labels only.

## 5. Equation/page crosswalk (what is verified vs pointer)

VERIFIED in this sweep (open arXiv HTML + abs, citable as HTML-section
statements only):

- S1 identity: arXiv:0801.2952 = RMP 81,647 (2009), 44pp/18figs (abs page).
- S1-HTML Sec.II.2: GP functional Eq.19; g = 4πaħ²/M; healing length Eq.21;
  TF Eqs.24-27; trapped ξ Eq.31; scale separation Eq.32; hydrodynamic
  v = (ħ/M)∇S / circulation κ = 2πħ/M derivation; continuity + generalized
  Bernoulli/quantum-pressure discussion (Feynman/Landau refs cited therein).
- S1-HTML Sec.III.1: χ = e^{iφ}f(r/ξ) ansatz; v = ħφ̂/(Mr) [Eq.49 context];
  localized vorticity δ-form [Eq.50 context]; f linear for r≲ξ, →1 for r>>ξ;
  j(r) max near ξ; s = ħ/(√2Mξ), supersonic r≲ξ; f_var = x/√(x²+2);
  E_v ≈ (πħ²n/M)ln(1.46R/ξ) with ξ short-distance cutoff.
- S1-HTML Sec.III.2: trapped same v law; toroidal topology sentence; ξ cutoff
  for TF log divergence; off-center phase S = arctan[(y-y0)/(x-x0)];
  critical Ωc = (5/2)(ħ/MR⊥²)ln(R⊥/ξ) [Eq.57], metastability
  Ωm = (3/2)(ħ/MR⊥²)ln(R⊥/ξ) = (3/5)Ωc [Eq.58].
- S1-HTML Sec.IV.1/IV.2.3/IV.2.6: l = sqrt(ħ/MΩ) [Eq.75]; ξ²/l² = ħΩ/2μ
  with TF explicit Eq.95; saturation ≈1/2 (Fischer-Baym/Baym-Pethick);
  Coddington Fig.14 caveat; ENS core 0.2-0.4 μm too small for direct imaging
  (expansion imaging note, Sec.III.4.2); jc ∼ nħ/ξ remark.
- S6 controls: GPE/ξ forms as in sibling sweeps (no new numbers taken here).

POINTERS requiring library copy before promotion to theory/formulas.md:

- RMP 81,647-691 interior pagination for any of the above; RMP-printed
  Eq./Fig. numbers (HTML Eq numbers ≠ RMP Eq numbers until checked).
- S2 (Phys.Rev.138 A429), S3/S4 (1961) interior equations/pages/figures.
- S5 (Donnelly book sections, Yarmchuk PRL 43:214 photographs,
  Packard/Rayfield-Reif core-visualization numbers) — no core-size number
  taken here.
- Pethick-Smith Fig.9.1 / Sec.9.2 fit details; Foot pp.231-240 derivation;
  Coddington/Baym-Pethick/Fischer-Baym fit functions and tables.

Until checked, treat all "~p./Eq." locations as find-aids, not audits.

## 6. Analogy to claim-61 (analogy only, no transfer)

- Structural analogy only: a GP vortex of radius ∼ ξ caps the formal 1/r
  velocity divergence by density depletion (n → 0 on axis, j → 0 at center
  with max ∼ nħ/ξ at r ≈ ξ), the way a finite claim-61 core scale would cap
  phase-gradient/quantum-pressure divergence at a node. Both introduce ONE
  length that regularizes a line singularity and both require the same
  two-limit discipline (resolution → 0 at fixed core, then core → 0) —
  cf. sibling L4 blob note (δ → 0 after N → ∞ at fixed δ) and
  loop_L2_reconnection.md (dx << ξ, k_max ξ >> 1).
- No transfer is asserted: this note makes no claim that blob-δ equals ξ,
  that ξ equals any claim-61 epsilon, that E_v log-cutoff predicts any
  Madelung-defect energy, that variational f_var constrains any quantum 2p
  radial function, or that Tkachenko/Kelvin dispersion constrains any
  claim-61 number. Quantization itself (κ = h/M vs κ = h/m4) is cited only
  as the S1-HTML derivation from single-valued Ψ, with no extension to the
  Wallstrom/Takabayasi domain rule beyond the shared "loops must lie where
  ρ > 0" exclusion already recorded in core_L1_takabayasi.md /
  core_L1_wallstrom.md / loop_L1_circulation.md.
- Use for claim-61 is restricted to motivating a pre-registered core-scale +
  N-at-fixed-core protocol and a ξ-style resolution diagnostic (core spans
  several grid points; physical vs numerical cutoffs separated, with
  ξ²/l²-style fractional-area watch for core overlap at rapid rotation);
  it supplies no pass/fail criterion and no finite-core value for the 2p
  vortex. The hydrogen most-probable radius 4a0 and 2p circulation 2π remain
  standard-Schrödinger numbers, not Fetter core-model predictions.
- Rotating-He photographs (S1 Figs.7-8, Yarmchuk) are cited only as
  existence proof that quantized vortices form lattices under rotation in
  both He II and BEC; no He-II core profile or core-size measurement is
  recovered here.

## 7. Source list (as fetched)

- arXiv abs 0801.2952 (RMP identity, 44pp/18figs):
  https://arxiv.org/abs/0801.2952
- arXiv HTML 0801.2952v1 (Secs.II.2/III.1/III.2/III.4.2/IV.1/IV.2.3/IV.2.6,
  Eqs.19/21/22/31/32/49/50/57/58/75/95/102, Figs.7/8/11/14 contexts):
  https://arxiv.org/html/0801.2952v1
- Wikipedia Gross-Pitaevskii equation (GPE/healing-length/soliton control):
  https://en.wikipedia.org/wiki/Gross%E2%80%93Pitaevskii_equation
- Wikipedia Quantum vortex (quantization context):
  https://en.wikipedia.org/wiki/Quantum_vortex
- Wikipedia Quantum turbulence (a0 values / vθ/ring/Kelvin context, cf.
  loop_L2_reconnection.md S5 — no numbers taken here):
  https://en.wikipedia.org/wiki/Quantum_turbulence
- Bibliographic pointers NOT parsed (control only, no interior numbers taken):
  Fetter Phys.Rev.138 A429 (1965); Gross Nuovo Cimento 20:454 (1961);
  Pitaevskii JETP 13:451 (1961); Donnelly Quantized Vortices in He II;
  Yarmchuk et al. PRL 43:214 (1979); Coddington et al.; Fischer-Baym;
  Baym-Pethick; Pethick-Smith; Foot Oxford 2005 pp.231-240.
- Local repo context only (no numbers taken): results/swarm/loop_L1_circulation.md,
  results/swarm/loop_L2_reconnection.md, results/swarm/core_L1_takabayasi.md,
  results/swarm/core_L1_wallstrom.md, results/swarm/core_L4_blob.md.

Record ends. No verdict.
