# loop_L1_circulation — superfluid circulation quantum literature sweep (claim-61 input, no verdict)

Scope: Fluid Universe claim-61 (finite-core probe context).
Requested targets: superfluid circulation quantum (Feynman/Onsager, Anderson phase-slip, Fetter rotating-He):
exact quantization statement kappa=h/m, any core-size measurement, equation/page numbers.
Figure-and-equation level. Explicit negative where absent.
No claim-61 verdict is recorded here.

Search date: 2026-11-07. Method: webfetch of open pages only
(Wikipedia Quantum vortex + Macroscopic quantum phenomena + Quantum turbulence + Superfluid helium-4 +
Superfluidity; arXiv abstract cond-mat/9912015; Alexander Fetter bio page).
Websearch was unavailable (parallel.ai 429 on 4/4 queries: Feynman/Onsager, Anderson, Fetter, core-size).
No APS (Anderson, Fetter 1965, Avenel-Varoquaux, Rayfield-Reif) or other paywalled full text was parsed;
no Feynman 1955 or Onsager 1949 primary text was parsed; no institutional access used.
Page/equation numbers below are verified only where stated as "verified in this sweep";
all other labels are bibliographic pointers with explicit negatives.

Config: `{"claim":61,"probe":"circulation-quantum kappa=h/m finite-core context","scope":"exact quantization statement; loop exclusion / multiply-connected domain; Anderson phase-slip Josephson relation; Fetter rotating-He vortex lattice; core-size measurement with equation/page numbers","targets":["Feynman1955/Onsager1949","Anderson1966-phase-slip","Fetter-rotating-He","core-size-measurement"]}`
- Git: `95b5403e2c3a620131ebed326f9054fa6923b401` (clean, projects/fluid-universe)
- Generated UTC: 2026-11-07 (webfetch sweep, no code run)

## 1. Exact quantization statement kappa=h/m (verified wording, 3 open sources)

Verified in this sweep from three open pages (same formula, different notation):

- Wikipedia "Quantum vortex", section "Vortex-quantisation in a superfluid" (fetched 2026-11-07):
  `∮_C v·dl = (ħ/m) ∮_C ∇φ_v·dl = (ħ/m) Δ^tot φ_v`,
  with `Δ^tot φ_v = 2πn` (n integer, single-valued wavefunction, cf. Bohr model),
  thus `∮_C v·dl = (2πħ/m) n`.
  Quantum of circulation per unit winding is therefore `κ = h/m = 2πħ/m`.
  Simply-connected enclosed region gives zero; hole/vortex (rod through fluid, vortex line)
  gives the non-zero quantized value above.
- Wikipedia "Macroscopic quantum phenomena", section "Superfluidity", Eqs.(9)-(12),(14) (fetched 2026-11-07):
  Eq.(9) `v_s = (1/m4)(h/2π) ∇φ` (neutral, q=0, m=m4);
  Eq.(10) `∮ v_s·ds = (h/2πm4) ∮ ∇φ·ds`;
  Eq.(11a) `∮ ∇φ·ds = 2πn`; Eq.(11b) `∮ v_s·ds = (h/m4) n`;
  Eq.(12) `κ = h/m4 ≈ 1.0×10^-7 m^2/s`;
  Eq.(13) `∮ v_s·ds = 2π v_s r` (circular motion);
  Eq.(14) `v_s = κ/(2πr)` for single quantum (n=1).
- Wikipedia "Quantum turbulence", section "Quantized circulation" (fetched 2026-11-07):
  `Γ = ∮_C v·dr`; `v(x,t) = (ħ/m) ∇φ`; `n(x,t) = |Ψ|^2`;
  `Γ = (ħ/m) ∮_C ∇S·dr = q κ` where `κ = h/m` is the quantum of circulation
  and integer `q = 0,1,2,...` is the charge (winding number).
  Multiply-charged vortices (q>1) in He II stated as unstable: energetically favourable
  to split into q singly-charged vortices; most practical applications q=1.
  Simply-connected surface + Stokes gives zero; multiply-connected (phase defect)
  gives the 2πq phase condition above. Fig.1 schematic (simply vs multiply connected)
  verified as page figure, not a primary measurement.

All three agree: circulation per winding `κ = h/m` (He-4: `h/m4`).

## 2. Feynman 1955 / Onsager 1949 (bibliographic verified; primary equations NOT verified)

- Bibliography verified via Wikipedia "Quantum vortex" refs [2][3] (fetched 2026-11-07):
  L. Onsager, "Statistical Hydrodynamics", Nuovo Cimento 6 Suppl.2 (2):279-287 (1949),
  doi:10.1007/BF02780991 (prediction of quantized vortices from single-valued
  superfluid order parameter; excitation-conjecture for superfluid transitions
  per that page's summary);
  R. P. Feynman, "Application of quantum mechanics to liquid helium",
  Prog. Low Temp. Phys. 1:17-53 (1955), doi:10.1016/S0079-6417(08)60077-3
  (development of Onsager ideas; vortex-ring / turbulence-reconnection suggestion
  per Quantum turbulence page §"Kelvin waves and vortex reconnections" citing Feynman Ref.[7]).
- Explicit negative: no Feynman-1955 or Onsager-1949 primary equation numbers,
  page numbers, figure numbers, or core-radius values were verified in this sweep
  (no primary full text parsed). Do not cite internal labels from this note.
  The quantization formula in §1 is verified only as the open-page synthesis above,
  not as a transcription of a specific Feynman/Onsager page.

## 3. Anderson phase-slip 1966 (bibliographic pointer; equation NOT verified)

- Common citation (NOT verified interior in this sweep):
  P. W. Anderson, "Considerations on the flow of superfluid helium",
  Rev. Mod. Phys. 38, 298 (1966). No volume/page/equation numbers beyond this
  pointer are recorded here; APS full text not parsed.
- What WAS verified in this sweep (Josephson-effect context, not Anderson interior):
  (a) Wikipedia "Superfluid helium-4", History section (fetched 2026-11-07) cites
  O. Avenel & E. Varoquaux, "Observation of Singly Quantized Dissipation Events
  Obeying the Josephson Frequency Relation in the Critical Flow of Superfluid ^4He
  through an Aperture", Phys. Rev. Lett. 55, 2704 (1985) — singly-quantized
  dissipation obeying the Josephson frequency relation; full text NOT parsed,
  no equation/page verified here.
  (b) Wikipedia "Macroscopic quantum phenomena", §§"Interrupted ring, weak links"
  + "DC/AC Josephson relations" (fetched 2026-11-07): Eqs.(28)-(32) verified as
  page equations only — Eq.(28) `is = i1 sin(Δφ*)`, Eq.(29) `V = (1/2π)(h/2e) dΔφ*/dt`,
  Eq.(32) `ν = 2eV/h = V/Φ0` (Josephson frequency), with `Φ0 = h/2e`.
  These are superconductor-Josephson page equations, NOT Anderson 1966 equations.
- Explicit negatives: no Anderson-1966 equation number, page number, "2π phase-slip"
  formula transcription, or He-II aperture derivation was verified in this sweep.
  No statement of the form "Anderson Eq.(X) p.Y gives κ=h/m" is supported by this note.
  Any claim that Anderson 1966 contains a specific numbered equation must not cite this note.

## 4. Fetter rotating-He (bibliographic pointers; primary equations NOT verified)

- Pointers (NOT parsed in this sweep, no equation/page verified):
  A. L. Fetter, "Vortices in an Imperfect Bose Gas. I. The Condensate With No Circulation",
  Phys. Rev. 138, A429 (1965) (and sequel series); "Nonuniform states of an imperfect
  Bose gas", Ann. Phys. 1972 (listed on Fetter bio page fetched 2026-11-07);
  commonly cited rotating-He/BEC review A. L. Fetter, "Rotating trapped Bose-Einstein
  condensates", Rev. Mod. Phys. 81, 647 (2009) — NOT parsed here.
  Fetter bio page fetched 2026-11-07 confirms identity (Stanford, condensed-matter /
  superconductivity) and the Ann. Phys. 1972 title only; it contains no circulation-quantum
  equation and no core-size number.
- What WAS verified (rotating-container context, attribution to Fetter NOT verified):
  (a) Wikipedia "Superfluid helium-4", Rotation section: rotating container below first
  critical angular velocity stays stationary; above it forms quantized vortices whose
  strength is quantized ("only certain allowed values"); more vortices with faster
  rotation in Abrikosov-lattice-like patterns. No equation/number verified on that page
  in this sweep.
  (b) Wikipedia "Quantum turbulence", Vortex-lattice section: critical velocity for a
  cylindrical vessel radius R stated as `Ω_c1 = (κ/R^2) ln(R/a0)`, from minimising
  `F' = F − L_v·Ω`; higher `Ω_c2, Ω_c3, ...` add vortices in ordered lattices.
  Verified as a page formula only; the page does NOT attribute this numbered formula
  to Fetter in the fetched text, so do NOT cite it as "Fetter Eq.(X)".
  (c) Condensate rotation verification (open abstract only): Madison et al.,
  arXiv:cond-mat/9912015 (= PRL 84, 806 (2000)), "Vortex Formation in a Stirred
  Bose-Einstein Condensate": stirring ^87Rb above a critical frequency produces
  one vortex, up to four at higher rotation; lifetime measured. Abstract verified;
  no κ=h/m equation and no core-size number in the abstract.
- Explicit negatives: no Fetter primary equation number, page number, critical-velocity
  derivation, or healing-length/core-radius formula was verified in this sweep.
  Do not cite any Fetter internal label from this note.

## 5. Core-size measurement (order-of-magnitude statements verified; precision measurement NOT verified)

Verified statements (open pages only, no uncertainties, not precision measurements):

- Wikipedia "Quantum vortex", Vortex-quantisation section: "thickness of the vortex
  depends on a variety of factors; in liquid helium, the thickness is of the order
  of a few Angstroms." Verified wording; no equation, no error bar, no method.
- Wikipedia "Macroscopic quantum phenomena", Superfluidity section + Fig.2 caption:
  vortex lines "have a vacuum core with a diameter of about 1 Å
  (which is smaller than the average particle distance). The superfluid helium
  rotates around the core with very high speeds. Just outside the core (r = 1 Å),
  the velocity is as large as 160 m/s." With Eq.(14) `v=κ/2πr` this is the
  `κ ≈ 1.0×10^-7 / (2π×10^-10) ≈ 159 m/s` evaluation (arithmetic not printed
  on the page; recorded here as consistency check, not a page equation).
  Vortex-line density increases with rotation speed (Fig.2 left-to-right);
  two right panels each contain six lines in different stable patterns.
- Wikipedia "Quantum turbulence", §§"Properties of vortex lines" + Fig.2/Fig.3:
  `ρ = 0` on the vortex line; density depletion for `r < a0`; `v_θ = κ/(2πr)`;
  vortex-ring self-velocity `v_R = κ/(4πR)[ln(8R/a0) − 1/2]` for `R >> a0`;
  core size `a0 = 10^-10 m` for He II, `10^-8 m` for ^3He-B, `10^-4 m` for typical
  atomic condensates; Fig.3 caption: "for helium II the thickness is approximately
  10^-10 m". Verified as page values; no measurement method, no uncertainties.
- History pointers relevant to a future measurement search (cited on pages,
  NOT parsed here, no core numbers verified): H. E. Hall & W. F. Vinen,
  Proc. Roy. Soc. A 238, 215 (1956) (mutual friction / rotation);
  G. Rayfield & F. Reif, Phys. Rev. 136, A1194 (1964) (quantized vortex rings);
  R. E. Packard, Physica B 109-110, 1474 (1982) (vortex photography);
  E. J. Yarmchuk & R. E. Packard, J. Low Temp. Phys. 46, 479 (1982)
  (photographic studies of quantized vortex lines);
  G. P. Bewley, D. P. Lathrop & K. R. Sreenivasan, Nature 441, 588 (2006)
  (visualization with solid-hydrogen tracers).
- Explicit negatives (central for claim-61 core use):
  No direct core-radius measurement with uncertainties, no fitted healing length ξ,
  no interferometric or density-depletion profile with error bars, and no
  "core cutoff preserves κ=h/m" theorem with equation/page numbers was verified
  in any source parsed in this sweep.
  In particular: no source swept gives a measured He-II core radius beyond
  "few Å / ~1 Å / 10^-10 m" order-of-magnitude wording; no Fetter or Anderson
  core-size equation was verified; no eps0-type regulator appears in any parsed
  source. Any finite-core value used for claim-61 is a new local probe choice,
  not a literature measurement recovered here.
- Scope limit: absences above are relative to the open-access set actually parsed
  (listed in §7) under the 2026-11-07 search outage; paywalled measurement papers
  (Vinen, Rayfield-Reif, Packard/Yarmchuk-Packard, Bewley et al., Avenel-Varoquaux,
  Fetter/Anderson primaries) were not parsed and could in principle contain
  additional numbers — this sweep makes no claim about their unparsed interiors
  beyond the bibliographic pointers quoted with provenance.

## 6. Consolidated explicit negatives (do-not-cite list)

- No Onsager-1949 or Feynman-1955 internal equation/page/figure numbers verified.
- No Anderson-1966 equation/page numbers verified; no "Anderson Eq.(X)" statement supported.
- No Fetter primary equation/page numbers verified (1965 / 1972 / 2009); the
  `Ω_c1 = (κ/R^2)ln(R/a0)` formula is a page formula without Fetter attribution here.
- No precision vortex-core measurement (value ± uncertainty + method + equation/page)
  verified; only "few Å / ~1 Å / 10^-10 m (He II)" order-of-magnitude page wording.
- No finite-core regularization / healing-length cutoff theorem preserving κ verified.
- Websearch unavailable (429 ×4); upgrade path: institutional access + APS/RMP
  full texts + Donnelly "Quantized Vortices in He II" + Rayfield-Reif / Packard /
  Bewley measurement sections for core numbers.

## 7. Source list (as fetched)

- Wikipedia Quantum vortex (quantization § + refs [2][3] Onsager/Feynman):
  https://en.wikipedia.org/wiki/Quantum_vortex
- Wikipedia Macroscopic quantum phenomena (Superfluidity Eqs.9-14, κ, core ~1 Å,
  Josephson Eqs.28-32, Fig.2 vortex lines):
  https://en.wikipedia.org/wiki/Macroscopic_quantum_phenomena
- Wikipedia Quantum turbulence (circulation Γ=qκ, a0 values, v_θ/v_R/Ω_c1 formulas,
  Feynman reconnection note, Figs.1-3):
  https://en.wikipedia.org/wiki/Quantum_turbulence
- Wikipedia Superfluid helium-4 (Rotation §, History: Hall-Vinen / Rayfield-Reif /
  Packard / Avenel-Varoquaux PRL 55,2704 Josephson-relation citation):
  https://en.wikipedia.org/wiki/Superfluid_helium-4
- Wikipedia Superfluidity (Kapitsa/Allen-Misener 1937, Landau 1941 context):
  https://en.wikipedia.org/wiki/Superfluidity
- Wikipedia Josephson effect (Josephson Eqs.1-2, KJ/Φ0 reference; Anderson-course
  history anecdote, not Anderson 1966 interior):
  https://en.wikipedia.org/wiki/Josephson_effect
- Wikipedia Alexander Fetter (identity + Ann. Phys. 1972 title only):
  https://en.wikipedia.org/wiki/Alexander_L._Fetter
- arXiv cond-mat/9912015 abstract (Madison et al., stirred-BEC vortices, PRL 84,806):
  https://arxiv.org/abs/cond-mat/9912015
- Bibliographic pointers NOT parsed (listed for control, no equation numbers taken):
  Onsager Nuovo Cimento 6 S2:279 (1949); Feynman Prog. Low Temp. Phys. 1:17 (1955);
  Anderson Rev. Mod. Phys. 38,298 (1966); Fetter Phys. Rev. 138 A429 (1965);
  Fetter Ann. Phys. 1972; Fetter Rev. Mod. Phys. 81,647 (2009);
  Hall-Vinen PRS A238,215 (1956); Rayfield-Reif PR 136 A1194 (1964);
  Packard Physica B 109-110,1474 (1982); Yarmchuk-Packard JLTP 46,479 (1982);
  Avenel-Varoquaux PRL 55,2704 (1985); Bewley et al. Nature 441,588 (2006).

Record ends. No verdict.
