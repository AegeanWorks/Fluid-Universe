# loop_L2_reconnection — Literature sweep record (claim-61 input, no verdict)

Scope: Fluid Universe claim-61 planning. Target: quantum-vortex
reconnection cutoff scale (Nore et al. / Koplik–Levine GPE reconnection):
minimum resolved scale, cutoff handling, equation/page numbers.
Analogy-only; no quantum claim is made here. No claim-61 verdict is
recorded here.

Search date: 2026-11-07. Method: websearch unavailable (parallel.ai 429
on all three queries: Nore/cutoff, Koplik–Levine/healing-length,
minimum-resolved-scale). Webfetch of open pages succeeded
(Wikipedia Gross–Pitaevskii equation; Quantum vortex; Quantum turbulence).
No full text of Koplik–Levine PRL 1993, Nore–Abid–Brachet PRL/Phys-Fluids
1997, Pitaevskii 1961, or Gross 1961 was parsed here; no institutional
access used. Same handling as sibling sweep core_L4_blob.md:
bibliographic pointers with volume/page ranges are given; note-local
equation labels R1–R5 are NOT primary equation numbers until verified
against a library copy. Do not promote Eq/page pointers as audited.

## 1. Bibliography (pointers, library check required except §5)

S1. J. Koplik and H. Levine, "Vortex reconnection in superfluid helium,"
  Phys. Rev. Lett. 71(9), 1375–1378 (1993).
  DOI: 10.1103/PhysRevLett.71.1375.
  First GPE reconnection simulation. Pointer — full text NOT parsed here.

S2a. C. Nore, M. Abid, and M. E. Brachet, "Decaying Kolmogorov turbulence
  in a model of superflow," Phys. Rev. Lett. 78(20), 3896–3899 (1997).
  DOI: 10.1103/PhysRevLett.78.3896.
  Pointer — full text NOT parsed here.

S2b. C. Nore, M. Abid, and M. E. Brachet, "Kolmogorov turbulence in
  low-temperature superflows," Phys. Fluids 9(9), 2644–2669 (1997).
  DOI: 10.1063/1.869473 (standard citation form; verify against library).
  Long-form methods + spectra companion to S2a. Pointer — NOT parsed here.

S3. Vortex-core structure priors:
  E. P. Gross, "Structure of a quantized vortex in boson systems,"
  Nuovo Cimento 20(3), 454–457 (1961);
  L. P. Pitaevskii, "Vortex lines in an imperfect Bose gas,"
  Sov. Phys. JETP 13(2), 451–454 (1961).
  Stationary GPE vortex profile, core ~ healing length. Pointer — NOT
  parsed here.

S4. Healing-length / GPE form control (verified in this sweep via open page):
  Wikipedia "Gross–Pitaevskii equation", Form-of-equation + Healing-length
  sections (fetched 2026-11-07); C. J. Foot, Atomic Physics (Oxford, 2005),
  pp. 231–240 (cited there for healing-length derivation).
  GPE coupling g = 4πħ²a_s/m; healing length ξ = (8πn a_s)^(−1/2)
  from ħ²/(2mξ²) = g n. See §2.

S5. Quantum-turbulence control (verified in this sweep via open page):
  Wikipedia "Quantum turbulence" (fetched 2026-11-07):
  quantized circulation Γ = qκ, κ = h/m; straight-line v_θ = κ/(2πr);
  vortex-ring velocity v_R = κ/(4πR)[ln(8R/a_0) − 1/2], R >> a_0;
  Kelvin-wave ω ≈ (κk²/4π) ln(1/k a_0), k = 2π/λ;
  core sizes a_0 ≈ 1e-10 m (He II), 1e-8 m (3He-B), 1e-4 m (typical
  atomic condensates, as stated on that page);
  reconnection paragraph: travelling vortices interact, reconnect on
  collision, topology changes (Feynman Ref.[7] cited there).
  Treat page-section labels as Wikipedia pointers, NOT primary numbers.

## 2. GPE + healing length (verified wording, note-local labels R1–R3)

(R1) Time-dependent GPE (S4 page, verified form):
  iħ ∂Ψ(r,t)/∂t = (−ħ²/2m ∇² + V(r) + g|Ψ(r,t)|²) Ψ(r,t),
  g = 4πħ²a_s/m, N = ∫|Ψ|² d³r.
  Time-independent form μΨ = (−ħ²/2m ∇² + V + g|Ψ|²)Ψ.

(R2) Healing length (S4 page, verified derivation statement):
  ħ²/(2mξ²) = g n  ⟹  ξ = (8π n a_s)^(−1/2).
  Text: "minimum distance over which the order parameter can heal";
  "distance over which the wavefunction recovers from zero in the
  center of the vortex to the value in the bulk"; sets vortex size.
  Dark-soliton illustration on same page (verified form):
  ψ(x) = ψ_0 tanh(x/(√2 ξ)) for repulsive g > 0 (π phase slip across
  zero). Cited as soliton/topological-defect example, not reconnection.

(R3) Vortex-line velocity (S5 page, verified form):
  v_θ = κ/(2πr); density depletion for r < a_0 with ρ = 0 on the line;
  Rankine combination (solid-body core + 1/r tail) shown as realistic
  classical model; quantum core shown as density-depletion profile
  n_s far-field vs 0 on axis (Fig.2 on that page).

What this fixes for the sweep: the GPE has NO explicit blob-δ. The
regularizer is physical: quantum pressure ∼ ħ²/(2mξ²) balances
interaction g n at scale ξ; velocity law is cut by density depletion
(ρ → 0 on axis), not by a K_δ kernel. Numerical cutoff is therefore
a resolution requirement (dx vs ξ, k_max vs 1/ξ), not a model parameter.

## 3. Koplik–Levine 1993 — reconnection result (pointer, standard account)

Standard account (to be confirmed against PRL 71:1375 full text):

- First direct GPE demonstration that two quantized vortex filaments
  reconnect even without viscosity, unlike classical Euler where
  Helmholtz/Kelvin theorems forbid it at finite resolution.
- Mechanism: when filament separation d becomes O(ξ), cores overlap,
  the ψ = 0 lines exchange partners through a transient cusp/bridge,
  and a sound (phonon) pulse is emitted carrying away the lost
  filament energy/length. Circulation stays q = 1 per filament;
  multiply-charged (q > 1) splits (cf. S5 multiply-charged note).
- Minimum resolved scale: reconnection is a core-scale event at
  d ∼ few × ξ. There is no smaller "reconnection cutoff" parameter in
  the model; ξ itself is the scale. Simulations must have grid spacing
  dx << ξ (in practice several points per ξ; exact points-per-ξ and
  the Koplik–Levine box/grid numbers NOT verified here — library check).
- Cutoff handling: none beyond GPE + boundary conditions. Topology
  change is allowed because ψ vanishes at the reconnection point, so
  v = (ħ/m)∇φ is undefined there exactly as in the Takabayasi/Wallstrom
  domain rule (sibling sweeps core_L1_takabayasi.md, core_L1_wallstrom.md:
  loops must lie in D with ρ > 0). The "cutoff" is the healing length,
  not an imposed truncation.
- Equation/page pointer (UNVERIFIED here): GPE/advected-filament setup
  and reconnection figures ~pp. 1375–1378; do NOT cite internal Eq./Fig.
  numbers from this note.

Explicit negative: no Koplik–Levine internal equation number, figure
number, grid size, dx/ξ ratio, or timestep was verified in this sweep.

## 4. Nore et al. 1997 — decaying superflow turbulence (pointer, standard account)

Standard account (to be confirmed against PRL 78:3896 + Phys Fluids
9:2644 full texts):

- Model: 3D periodic GPE (S2a/S2b), pseudo-spectral in space, i.e. the
  same (R1) with V = 0, studied as a "model of superflow" at T ≈ 0.
- Claimed result: Taylor–Green / random-phase initial data develop a
  vortex tangle whose incompressible kinetic-energy spectrum is
  consistent with Kolmogorov k^(−5/3) over the resolved inertial range,
  with dissipation at small scales by phonon/sound emission from
  reconnections and Kelvin-wave cascades — the GPE analogue of viscous
  cutoff, with ξ playing the role of the dissipative microscale.
- Minimum resolved scale: the inertial range terminates at kξ ∼ O(1);
  scales kξ >> 1 are sound-dominated, not vortex-dominated. Finer
  filaments/rings with R ∼ ξ shrink and radiate away (cf. S5 ring
  velocity v_R ∝ 1/R, faster at small R). Exact kξ break values and
  spectrum figures NOT verified here — library check.
- Cutoff handling (numerical, standard pseudo-spectral account):
  2/3-dealiasing cutoff k_max (k_max = N/3 per side in the usual
  convention, or (2/3)(N/2) depending on normalization — convention
  NOT verified here), plus the physical regulator ξ. Convergence
  requires k_max ξ >> 1 (i.e. dx << ξ) so the core is resolved; then
  dealiasing removes wraparound, not physics. Exact N, box L, L/ξ,
  k_max ξ, and dt numbers for S2a/S2b NOT verified here.
- Equation/page pointer (UNVERIFIED here): GPE + energy decompositions
  (compressible/incompressible kinetic, quantum, internal) and spectrum
  definitions ~S2b Secs. II–IV; Kolmogorov-plot figures in S2a/S2b;
  do NOT cite internal Eq./Fig. numbers from this note.

Explicit negative: no Nore et al. internal equation number, page number,
figure number, resolution table (N, L/ξ, k_max ξ, dx/ξ), or spectrum
slope-with-errorbar was verified in this sweep.

## 5. Equation/page crosswalk (what is verified vs pointer)

VERIFIED in this sweep (open pages, citable as page statements only):
- (R1) GPE time-dependent + stationary forms, g, N-normalization — S4
  Form-of-equation section.
- (R2) ξ balance ħ²/(2mξ²) = gn ⟹ ξ = (8πna_s)^(−1/2); "heals from zero
  in the vortex center"; dark soliton ψ_0 tanh(x/√2ξ) — S4
  Healing-length + Soliton sections; Foot pp. 231–240 via S4 citation.
- (R3) Γ = qκ, κ = h/m; v_θ = κ/2πr; ring v_R formula; Kelvin ω formula;
  core sizes 1e-10 / 1e-8 / 1e-4 m; Feynman-reconnection topology-change
  sentence — S5 sections Quantized-circulation / Vortex-line-properties /
  Kelvin-waves-and-reconnections (with Fig.2/4/5 captions as page context).

POINTERS requiring library copy before promotion to theory/formulas.md:
- S1: PRL 71, pp. 1375–1378; setup equations, grid/dx/ξ, reconnection
  time-series figures.
- S2a: PRL 78, pp. 3896–3899; S2b: Phys Fluids 9, pp. 2644–2669;
  GPE + spectral-decomposition equations, N/L/ξ/k_max tables, kξ break.
- S3: Gross Nuovo Cimento 20:454–457; Pitaevskii JETP 13:451–454;
  stationary vortex profile f(r/ξ) equation and core-radius convention
  (ξ vs a_0 vs √2ξ — convention clash explicitly flagged, do NOT merge
  numbers without checking which convention each source uses).
- S4 textbook: Foot pp. 231–240 exact equation labels.
Until checked, treat all "~pp./Eq." locations as find-aids, not audits.

## 6. Analogy to claim-61 (analogy only, no transfer)

- Structural analogy only: a GPE core of radius ∼ ξ caps the 1/r
  velocity by density depletion, the way a finite claim-61 core scale
  would cap phase-gradient/quantum-pressure divergence at a node.
  Both introduce ONE length that regularizes a line singularity and
  both require a two-limit convergence check (resolution → 0 at fixed
  core, then core → 0) — cf. sibling L4 blob note (delta → 0 after
  N → ∞ at fixed delta; Krasny fixed-delta N-study; Beale–Majda
  δ = h^q scaling).
- Reconnection-specific analogy: topology change goes THROUGH ψ = 0
  (undefined velocity point), never around it — the same domain rule
  as the Wallstrom/Takabayasi loop exclusion (ρ > 0 on γ). A finite
  core does not "derive" quantization or reconnection; it resolves the
  event so the code can step through it, with the excess length
  radiated as sound.
- No transfer is asserted: this note makes no claim that blob-δ equals
  ξ, that ξ equals any claim-61 epsilon, that GPE sound emission
  predicts any Madelung-defect energy, or that Kolmogorov k^(−5/3)
  constrains any claim-61 number. Quantization, Wallstrom-type
  conditions, and single-valuedness have no counterpart in S1–S5
  numerics beyond (R3).
- Use for claim-61 is restricted to motivating a pre-registered
  core-scale + N-at-fixed-core protocol and a kξ-style resolution
  diagnostic (resolved only if core spans several grid points and the
  dealiasing/physical cutoffs are separated); it supplies no pass/fail
  criterion.

## 7. Source list (as fetched)

- Wikipedia Gross–Pitaevskii equation (R1/R2 verified forms):
  https://en.wikipedia.org/wiki/Gross%E2%80%93Pitaevskii_equation
- Wikipedia Quantum vortex (circulation quantum context):
  https://en.wikipedia.org/wiki/Quantum_vortex
- Wikipedia Quantum turbulence (R3 + core sizes + reconnection sentence):
  https://en.wikipedia.org/wiki/Quantum_turbulence
- Bibliographic pointers NOT parsed (control only, no numbers taken):
  Koplik–Levine PRL 71:1375 (1993), DOI 10.1103/PhysRevLett.71.1375;
  Nore–Abid–Brachet PRL 78:3896 (1997), DOI 10.1103/PhysRevLett.78.3896;
  Nore–Abid–Brachet Phys Fluids 9:2644 (1997), DOI 10.1063/1.869473;
  Gross Nuovo Cimento 20:454 (1961); Pitaevskii JETP 13:451 (1961).
- Local repo context only (no numbers taken): results/swarm/core_L4_blob.md,
  results/swarm/core_L1_bohm.md, results/swarm/core_L1_wallstrom.md,
  results/swarm/core_L1_takabayasi.md, theory/node_defect.py,
  theory/hydrogen_radial_nodes.py, claims 20/24/26/46/47.

Record ends. No verdict.
