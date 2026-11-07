# loop_L3_landau — Literature sweep record (claim-61 input, no verdict)

Scope: Fluid Universe claim-61. Target: Landau–Lifshitz hydrogen m=±1 (2p)
probability current density only: j_phi form, 1/r dependence, circulation
statement, equation/page numbers. Figure-and-equation level.
Explicit negative where absent (dedicated 2p j_phi numbered equation,
finite-core regularization). No claim-61 verdict is recorded here.

Search date: 2026-11-07. Method: webfetch of open pages only
(Wikipedia Course of Theoretical Physics + Probability current +
Hydrogen atom + Hydrogen-like atom + Madelung equations;
Wikibooks Probability flux; archive.org metadata page for LL Vol.3).
Websearch unavailable (Exa 402: free allowance exhausted, no credit
balance — same outage as 2026-11-06/07 sweeps).
No full-text copy of Landau–Lifshitz Vol.3 was parsed here
(archive.org PDF fetch 403 on 2026-11-07; Trent/Internet-Archive
borrow-restricted copy not parsed; no institutional access used).
Page/equation numbers below are verified only where stated as
"verified in this sweep"; all other labels are bibliographic pointers
with primary-confirmation flag.

Config: `{"claim":61,"probe":"Landau-Lifshitz hydrogen m=+-1 (2p) j_phi","scope":"j_phi form; 1/r dependence; circulation statement; equation/page numbers","targets":["LL-Vol3-general-current","LL-hydrogen-2p-wavefunction","LL-j_phi-application","LL-circulation-wording","finite-core-check"]}`
- Git: `95b5403e2c3a620131ebed326f9054fa6923b401` (dirty: untracked loop_* swarm notes only, projects/fluid-universe)
- Generated UTC: 2026-11-07 (webfetch sweep, no code run)

Local repo context only (no LL numbers taken): theory/hydrogen_vortex.py
(radial_2p, azimuthal_velocity, circulation, loop_circulation,
angular_momentum), results/hydrogen_vortex.{json,md}, claims 20/26/46,
sibling sweeps results/swarm/loop_L1_*.md, loop_L2_*.md, loop_L3_holland.md.

## 1. Bibliography

- L. D. Landau and E. M. Lifshitz, Quantum Mechanics: Non-Relativistic
  Theory, Course of Theoretical Physics Vol.3. 1st ed Pergamon 1958;
  2nd ed Pergamon 1965; 3rd ed Pergamon 1977 (revised with Pitaevskii).
  3rd ed: ISBN 0-08-020940-1, xiii+616 pp. Russian original:
  Kvantovaya Mekhanika (Nauka). Vol.3 covers QM without relativity.
  Verified in this sweep only as volume identity/edition list via
  Wikipedia "Course of Theoretical Physics" (fetched 2026-11-07);
  interior pages/equations NOT parsed here.
- Archive.org metadata control (fetched 2026-11-07, no PDF parsed):
  "Quantum mechanics: non-relativistic theory", Landau–Lifshitz,
  Pergamon/Addison-Wesley 1965, 642 scans, identifier
  quantummechanics0000land, access-restricted/borrow. Confirms a
  1965-edition holding exists; supplies no equation numbers.
- Secondary control actually parsed for the current formula:
  Wikipedia "Probability current", §"Spin-s particle in an
  electromagnetic field" (fetched 2026-11-07), which cites
  "see page 473, equation 115.4, L.D. Landau, E.M. Lifschitz,
  COURSE OF THEORETICAL PHYSICS Vol.3 — Quantum Mechanics
  (3rd ed.)" with PDF link
  https://ia803206.us.archive.org/4/items/landau-and-lifshitz-physics-textbooks-series/Vol%203%20-%20Landau%2C%20Lifshitz%20-%20Quantum%20mechanics%20-%20non-relativistic%20theory%20%283ed.%2C%201991%29.pdf
  (3rd ed.). Verified here ONLY as Wikipedia's citation text
  (p.473 / Eq.115.4 pointer for the spin electric-current case),
  NOT as a primary LL page check. Direct fetch of that PDF URL
  returned 403 in this sweep.

## 2. General probability current j (LL pointer; wording verified via open pages)

Standard spin-0 form (verified wording on Wikipedia "Probability current"
and Wikibooks "Probability flux", both fetched 2026-11-07; LL attribution
for the spin case only per §1):

  j = ħ/(2mi) (ψ* ∇ψ − ψ ∇ψ*) = ħ/m Im(ψ* ∇ψ)

in 3D, with continuity

  ∂ρ/∂t + ∇·j = 0,  ρ = |ψ|².

Polar form j = ρ ∇S/m with ψ = R exp(iS/ħ), v = ∇S/m
(verified derivation on the same pages; cf. Madelung page
J = (1/m)ρ∇S = ρv, fetched 2026-11-07).

Spin/electromagnetic extension (the ONLY LL equation label verified
as a citation string in this sweep, not as interior text):

- Wikipedia "Probability current" states: per Landau–Lifshitz Course,
  in Gaussian units,
  j_e = q/(2m)[(ψ* p̂ψ − ψ p̂ψ*) − (2q/c)A|ψ|²]
        + (μ_S c/sħ) ∇×(ψ* S ψ),
  and in SI the corresponding j_e/q form. Citation given as
  LL Vol.3 p.473 Eq.115.4 (3rd ed.). Treat "p.473 / 115.4" as
  Wikipedia's pointer, NOT as a primary-verified LL interior.
  For field-free spin-0 hydrogen (A=0, spin term dropped),
  j_e = q j with q = −|e| for the electron; probability current
  j is the q-independent factor above.

LL pointer with primary-confirmation flag (NOT verified interior here):

- The spin-0 j + continuity pair is commonly cited as LL Vol.3 §19,
  Eqs.(19.1)–(19.2)-type (edition-dependent numbering). No §19 page
  number and no (19.x) label was parsed from LL in this sweep.
  Do NOT cite a specific (19.x)/page from this note; library check
  of a 2nd/3rd-ed copy is the upgrade path.
- Hydrogen-atom eigenfunctions are commonly cited as LL Vol.3 §36
  ("The hydrogen atom", edition-dependent). No §36 page/equation
  label was parsed from LL here; same library-check flag.

## 3. Hydrogen 2p m=±1 wavefunction (standard form; LL interior NOT parsed)

Forms below are verified wording from Wikipedia "Hydrogen atom"
(§Schrödinger equation / Wavefunction, fetched 2026-11-07) and
"Hydrogen-like atom" (R_nl + Y_lm separation, fetched 2026-11-07),
plus local repo implementation (context only). They are NOT verified
here as LL §36 interior text.

  ψ_nlm(r,θ,φ) = R_nl(r) Y_l^m(θ,φ).

For n=2, l=1, m=±1 (Z=1, a0 = Bohr radius):

  R_21(r) = 1/(2√6 a0^{3/2}) (r/a0) exp(−r/2a0),
    normalized as ∫_0^∞ r² R_21² dr = 1
    (repo theory/hydrogen_vortex.py::radial_2p implements exactly this).

  Y_1^{±1}(θ,φ) = ∓√(3/8π) sinθ exp(±iφ).

Hence (∓ overall sign convention):

  ψ_21±1 = ∓ 1/(8√π a0^{3/2}) (r/a0) exp(−r/2a0) sinθ exp(±iφ),

  |ψ_21±1|² = 1/(64π a0³) (r/a0)² exp(−r/a0) sin²θ.

Checks: R_21(0)=0 so origin is a node; sinθ=0 so the entire
z-axis is nodal; radial probability P(r)=r²R_21² peaks at 4a0
(repo most_probable_radius()=4.0a0; claim-26 context, not LL).

Phase (up to additive constant):

  S = ±ħφ − E_2 t,  ∇S = ±ħ/(r sinθ) e_φ.

## 4. j_phi form (application of general j to §3; NO separate LL number)

No LL-book 2p-specific j_phi page/equation was verified in this sweep
(see §7 explicit negative). The formula below is the textbook
application j = ħ/m Im(ψ*∇ψ) to the §3 eigenfunction; it is what the
"Landau–Lifshitz hydrogen m=±1 current" target reduces to once the
general-j pointer (§2) and the hydrogen eigenfunction (§3) are combined.

With ∇ → e_φ (1/r sinθ)∂_φ on ψ_21±1:

  v = j/ρ = ∇S/m_e = mħ/(m_e r sinθ) e_φ,  m = ±1,

i.e.

  v_φ(r,θ) = mħ/(m_e r sinθ).

  j_φ(r,θ) = ρ v_φ = mħ/(m_e r sinθ) |ψ_21±1|².

Explicit density-inserted form (SI, field-free, spin-0):

  j_φ = mħ/(64π m_e a0³) (r/a0)² exp(−r/a0) sin²θ /(r sinθ)
      = mħ r sinθ exp(−r/a0) /(64π m_e a0⁵),  m = ±1.

At the equator (θ=π/2, the claim-26/claim-61 loop plane):

  v_φ = mħ/(m_e r),
  j_φ = mħ r exp(−r/a0) /(64π m_e a0⁵).

Atomic units (ħ=m_e=a0=1, cf. repo which runs ħ=m=1):

  v_φ = m/(r sinθ),  j_φ = m r sinθ exp(−r)/(64π),
  equator: v_φ = m/r, j_φ = m r exp(−r)/(64π).

Electric-current reading (where needed): j_e,φ = q j_φ with q=−|e|;
the sign flips for the electron charge. LL Gaussian/SI spin-term
forms in §2 reduce to this when A=0 and the spin term is dropped.

Repo cross-check (context only, not LL): theory/hydrogen_vortex.py::
azimuthal_velocity() implements v_φ = ħm/(m_e r sinθ) with
ValueError for r≤0 or |sinθ|<1e-12 (domain exclusion, not a core);
loop_circulation() integrates v_φ r sinθ dφ. No LL number is taken
from the repo.

## 5. 1/r dependence (two senses — must not be conflated)

- Velocity / j-per-density: v_φ = mħ/(m_e r sinθ) ∝ 1/(r sinθ)
  = 1/ρ_cyl with cylindrical radius ρ_cyl = r sinθ. This is the
  "1/r dependence" in the circulation sense: at fixed θ=π/2,
  |v| ∝ 1/r → ∞ as r→0. Verified as algebra from §§3–4; same
  1/ρ_cyl law appears on open pages as v_s=κ/(2πr) (superfluid
  synthesis, loop_L1_circulation.md) and v_θ=κ/(2πr) (Quantum
  turbulence page); those are NOT LL statements.
- Probability current density itself: j_φ = ρ v_φ ∝ r sinθ exp(−r/a0)
  (SI: ∝ r sinθ/a0⁵). It VANISHES on the axis (sinθ=0) and at the
  origin (R_21=0), peaks at finite r, and decays exponentially.
  The 1/(r sinθ) factor is cancelled by the ρ ∝ r²sin²θ factor.
  Conflating "j_φ ∝ 1/r" without the ρ factor is incorrect.

## 6. Circulation statement (derived; NOT LL vortex language)

Derived loop integral at fixed (r,θ) with dl = r sinθ dφ e_φ:

  ∮ v·dl = ∫_0^{2π} [mħ/(m_e r sinθ)] r sinθ dφ = 2πmħ/m_e.

For m=±1: ±2πħ/m_e (atomic units: ±2π; repo circulation()
returns 6.283185307180 for m=1, loop at r=2,θ=π/2 equal within
1e-6 — repo numbers, not LL). Mass circulation
Γ = ∮ m_e v·dl = 2πmħ; angular momentum L_z = ħm.

LL-wording negative: LL as swept supplies the j definition and the
hydrogen eigenfunctions from which the above follows, but no
"quantized-vortex / circulation-quantum κ=h/m" sentence, no
∮-loop equation with integer winding, and no distributional-curl
(∇×v ∝ δ(x)δ(y)) statement was verified as LL interior in this sweep.
That language belongs to the Takabayasi/Reddiger line
(Eq.(1.3) m/(2πħ)∮v·dr∈ℤ; Eq.(1.4) distributional curl; see
loop_L1_takabayasi.md and loop_L3_holland.md), not to LL as parsed here.
Do NOT cite §§4–6 above as "LL Eq.(X) p.Y"; they are a
general-j + hydrogen-wavefunction application recorded here for
claim-61 loop-protocol use.

## 7. Equation/page numbers requested (verified vs needs-library-check)

Verified in this sweep (as citation strings / open-page equations,
NOT as LL interior parses):

- LL spin-current pointer: p.473 Eq.115.4 (3rd ed.) — verified ONLY
  as Wikipedia "Probability current" citation text (Ref.[4] on that
  page). Interior NOT parsed (PDF 403).
- General j wording + continuity + polar j=ρ∇S/m — verified as
  open-page equations (Wikipedia Probability current / Madelung
  equations / Wikibooks Probability flux). LL §19-type labels
  NOT verified.
- Hydrogen ψ_nlm = R_nl Y_lm, R_21, Y_1^{±1}, 2p forms — verified as
  open-page / repo-context formulas (§3). LL §36-type labels NOT
  verified.
- Circulation integers / κ=h/m / v=κ/(2πr) wordings — verified ONLY
  on superfluid pages (loop_L1_circulation.md), NOT as LL statements.

Needs direct library check of a 2nd/3rd-ed LL Vol.3 copy (upgrade path):

- Exact §19 number(s) for j = ħ/2mi(ψ*∇ψ−ψ∇ψ*) and ∂ρ/∂t+∇·j=0,
  with 2nd- vs 3rd-ed pagination.
- Exact §36 (or adjacent) numbers/pages for R_nl / Y_lm / hydrogen
  levels, and whether any worked j_φ / magnetic-moment example
  carries its own number (orbital-moment discussion, if any).
- Confirmation that NO dedicated "2p m=±1 j_φ" numbered equation
  exists (provisional negative below), and exact wording of any
  m≠0 azimuthal-current / magnetic-moment remark if present.

## 8. Explicit negatives (central for claim-61 scoping)

- No dedicated LL "hydrogen 2p m=±1 j_phi" numbered equation,
  figure, or page was verified in this sweep. The j_phi in §4 is an
  application, not a recovered LL-numbered result. Any citation of
  the form "LL Eq.(X) gives j_φ = ..." must not cite this note;
  cite §§2–3 pointers plus a library-checked LL copy instead.
- No LL circulation-quantization sentence (∮v·dl = 2πmħ/m_e as a
  vortex postulate), no multiply-connected-domain loop-exclusion
  rule, and no (∇×v)-delta-line statement was verified as LL
  interior here. Those belong to Takabayasi/Reddiger/quantum-vortex
  pages; see sibling sweeps.
- No finite vortex-core radius, no healing-length-type regulator,
  no truncated-Q cap, no eps0/delta0-type smoothed core, no
  "core cutoff preserves circulation" theorem, and no 1/r-cutoff
  velocity cap was found in any LL material as represented in this
  sweep. The §4 v_φ is singular on the axis (∝1/sinθ) and at r→0
  (∝1/r at fixed θ); j_φ vanishes there only via the ρ factor.
  What LL supplies is the singular-domain formula; regularization
  is a new local probe choice (cf. claim-60/61 eps0/delta0),
  not an LL value recovered here.
- In particular: no LL source swept gives eps0=0.1a0, any Gaussian/
  healing core, any core energy, or any "regularization preserves
  2π" result. Any finite-core value used for claim-61 is a new
  local probe choice, not a literature value recovered here.
- Scope limit: absences above are relative to the open-access set
  actually parsed (listed in §9) under the 2026-11-07 search outage;
  the unparsed LL interior (borrow-restricted full text),
  paywalled Wallstrom/Takabayasi/Fritsche–Hasse/Nelson primaries,
  and Holland/Bohm–Hiley interiors were not parsed and could in
  principle contain additional discussion — this sweep makes no
  claim about their unparsed interiors beyond the secondary
  characterizations quoted with provenance.

## 9. Source list (as fetched 2026-11-07)

- Wikipedia Course of Theoretical Physics (Vol.3 identity/edition list):
  https://en.wikipedia.org/wiki/Course_of_Theoretical_Physics
- Wikipedia Probability current (general j; LL p.473 Eq.115.4 citation
  for spin electric current; continuity derivation):
  https://en.wikipedia.org/wiki/Probability_current
- Wikipedia Madelung equations (J=(ħ/2mi)(ψ*∇ψ−ψ∇ψ*)=(1/m)ρ∇S;
  Γ=∮mv·dl=2πnħ auxiliary condition; Takabayasi/Wallstrom refs):
  https://en.wikipedia.org/wiki/Madelung_equations
- Wikipedia Hydrogen atom (2s/2p ψ forms; quantum numbers; R/Y separation):
  https://en.wikipedia.org/wiki/Hydrogen_atom
- Wikipedia Hydrogen-like atom (ψ_nlm=R_nl Y_lm; R_nl Laguerre form;
  angular-momentum L²/Lz eigenvalues):
  https://en.wikipedia.org/wiki/Hydrogen-like_atom
- Wikibooks Probability flux (continuity + j divergence derivation):
  https://en.wikibooks.org/wiki/This_Quantum_World/Implications_and_applications/Probability_flux
- Archive.org metadata for LL Vol.3 1965 holding (no PDF parsed):
  https://archive.org/details/quantummechanics0000land
- LL PDF URL attempted (403, NOT parsed — recorded for upgrade path):
  https://ia803206.us.archive.org/4/items/landau-and-lifshitz-physics-textbooks-series/Vol%203%20-%20Landau%2C%20Lifshitz%20-%20Quantum%20mechanics%20-%20non-relativistic%20theory%20%283ed.%2C%201991%29.pdf
- Bibliographic pointers NOT parsed (listed for control, no LL numbers taken):
  LL Vol.3 §19-type (general current) + §36-type (hydrogen atom),
  2nd ed 1965 / 3rd ed 1977 paginations; sibling-sweep sources in
  loop_L1_circulation.md, loop_L1_takabayasi.md, loop_L1_wallstrom.md,
  loop_L2_reconnection.md, loop_L3_holland.md.
- Local repo context only (no LL numbers taken):
  theory/hydrogen_vortex.py, results/hydrogen_vortex.{json,md},
  claims 20/24/26/46/47.

Record ends. No verdict.
