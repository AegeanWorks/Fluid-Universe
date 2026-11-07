# loop_L3_holland — Literature sweep record (claim-61 input, no verdict)

Scope: Holland Quantum Theory of Motion 2p (m=1) trajectory / velocity
field v_phi only: formula, behavior near origin, any core discussion,
equation/page numbers. Explicit negative where absent (core regularization).
No claim-61 verdict is recorded here.

Search date: 2026-11-07. Method: webfetch of open pages only
(Wikipedia Madelung + de Broglie-Bohm + Quantum potential;
arXiv abs 2207.11367 + full HTML v1; arXiv abs quant-ph/0302088).
Websearch unavailable (429 on search provider, 3/3 queries).
No full-text copy of Holland (1993) or Bohm-Hiley (1993) was parsed here;
Holland page numbers below are from the publisher/Routledge TOC and the
PagePlace preview-PDF TOC plus secondaries that quote the equations.
Primary equation numbers inside Holland Ch.3 are NOT verified here.
Numdam Holland-Kyprianidis PDF fetch returned binary, not parsed.

## 1. Holland — The Quantum Theory of Motion (1993)

Bibliography: P. R. Holland, The Quantum Theory of Motion: An Account of the
de Broglie-Bohm Causal Interpretation of Quantum Mechanics, Cambridge
University Press, first published 1993; paperback 1995; ISBN 0-521-35404-8
(hbk) / 0-521-48543-6 (pbk). 618 pp. DOI: 10.1017/CBO9780511622687.

TOC locations (from PagePlace preview PDF of the CUP edition,
https://api.pageplace.de/preview/DT0400.9780511874321_A23680233/preview-9780511874321_A23680233.pdf):

- Ch.3 "Elements of the quantum theory of motion", p.66 (§3.1 basic postulates, p.66).
- §3.2 "Wave and particle equations of motion", p.68; §3.2.1 "Reformulation of the
  Schrodinger equation", p.68; §3.2.2 "Single-valuedness of the wavefunction", p.70;
  §3.2.3 "Introduction of the particle", p.72.
- §3.3 "Properties of the model", p.78.
- §3.3.8 "Trajectories do not cross, or pass through nodes", p.85.
- §3.4 "Some properties of the quantum potential", p.89.
- Ch.4 "Simple applications", p.136; §4.1 "Stationary states", p.136 (§4.1.1 General
  properties, p.136).
- Ch.6 particle-in-a-box stationary discussion §6.5, p.239 (§6.5.1 Stationary states, p.239).

General guidance law as reported by secondaries citing Holland (NOT Holland
book equation numbers):

- Velocity field v = ∇S/m; quantum potential Q = −ħ²/(2mR) ∇²R; guidance law
  dx/dt = (ħ/m) Im(∇Ψ/Ψ) = ∇S/m. Equation labels Eqs.(6)/(7)/(10) in
  de Gosson–Hiley, Phys. Lett. A short-time propagator paper (2013)
  (https://pmc.ncbi.nlm.nih.gov/articles/PMC3820027/ and
  https://eprints.bbk.ac.uk/id/eprint/8102/7/8102.pdf); those authors cite Holland
  [7,8] (Nuovo Cimento B 116 (2001) 1043–1070; 1143–1172) for the Hamiltonian
  trajectory reading. Treat those Eq. numbers as de Gosson–Hiley labels, NOT as
  Holland book equation numbers.
- Non-crossing rule in the same source: "The Bohmian equations of motion
  are a priori only defined when R≠0 (that is, outside the nodes of the wave
  function)" and "An important feature of the quantum trajectories ... is that they
  cannot cross". Modern normalizing-flow restatement cites the same Holland
  section: "trajectories can-not cross or pass through nodes [14, §3.3.8]"
  (arXiv:2604.25137, § citing Holland book, p.2, Eq.(5) dx/dt=v, m dv/dt=−∇(V+Q)).
- Independent trajectory-methods review (Wyatt/Bittner, quant-ph/0302088, abs
  fetched 2026-11-07; full text not parsed here): p.2 per prior sweep:
  "The quantum trajectories obey two important noncrossing rules: (1) they cannot
  cross nodal surfaces (along which ψ=0); (2) they cannot cross each other."
  Same review (p.7–9 per prior sweep): "the quantum potential becomes large around
  wavefunction nodes and quasinodes", "Near nodes ... propagation of Bohmian
  trajectories becomes problematic ... trajectories tend to avoid nodal regions ...
  under-sampling". Cited as numerical-behaviour note, not as a Holland core-cutoff
  prescription.

## 2. 2p (m=1) velocity field v_phi — formula (application of Holland general law)

No Holland-book 2p-specific page/equation was verified in this sweep (see §4
explicit negative). The formula below is the standard hydrogen eigenfunction
with Holland's general law v=∇S/m applied; wording verified via Reddiger–Poirier
HTML v1 (fetched 2026-11-07) + local repo implementation (context only, not Holland):

- Standard form (Reddiger–Poirier Eq.(2.3a), verified in HTML:
  Ψ_{nlμ}(r,θ,φ) = R_{nl}(r) Y_l^μ(θ,φ), with n∈ℕ, l∈{0,…,n−1}, μ∈{−l,…,l}.
  For 2p m=±1: R_{21}(r) real radial factor; angular factor
  Y_1^{+1}(θ,φ) = −√(3/8π) sinθ e^{+iφ} = −(Y_1^{−1})* (Reddiger Eq.(2.3c), verified).
  Local repo form (theory/hydrogen_vortex.py::radial_2p, context only):
  R_{21}(r) = 1/(2√6 a0^{3/2}) (r/a0) exp(−r/2a0), normalized as ∫ r²R²dr=1.
- Phase: S = ±ħφ − E_{n}t (up to additive constant), so ∇S = ±ħ/(r sinθ) e_φ.
  Hence drift/Bohm velocity (Reddiger Eq.(1.2) v⃗=ħ/m Im(∇Ψ/Ψ) applied):
  v⃗ = ħμ/(m_e r sinθ) e_φ, i.e. v_φ = ħμ/(m_e r sinθ).
  At equator θ=π/2: v_φ = ħμ/(m_e r); repo azimuthal_velocity() implements
  exactly this with ValueError for r≤0 or |sinθ|<1e-12 (domain exclusion, not a core).
- Circulation (verified): Reddiger Eq.(1.3) m/(2πħ)∮v⃗·d r⃗ ∈ℤ; for hydrogen
  the integer is μ. Loop integral at fixed (r,θ=π/2): ∮ v·dl = 2πμħ/m_e
  (repo loop_circulation() returns 6.283185307180 at m=1, ħ=m=1; most-probable
  radius 4.0 a0; angular momentum ħμ = 1.0 — repo numbers, not Holland).
- Trajectories: circles about z-axis at constant (r,θ) with angular frequency
  ω = v_φ/(r sinθ) = ħμ/(m_e r² sin²θ). No radial or polar motion for the pure
  eigenstate (stationary density).
- Distributional curl (verified, Reddiger Eq.(1.4)/Cor.2.11):
  (∇×v⃗)(t,x,y,z) = 2πμħ/m (0,0,δ(x)δ(y))ᵀ in physics delta notation.
  I.e. third Madelung equation ∇×v⃗=0 (Eq.(1.1c)) fails distributionally on the
  vortex line; drift fields termed "quasi-irrotational" (Def.2.10, after Schönberg).
  Quantum vorticity coefficient depends on μ, whose quantization forces n,l integers.

## 3. Behavior near origin / axis

- As reported in the swept material: v_φ ∝ 1/(r sinθ) diverges toward the
  z-axis (sinθ→0) and toward r→0. At equator, |v| ∝ 1/r → ∞ as r→0.
  The origin r=0 is a node (R_{21}(0)=0; density zero) and the entire z-axis
  is nodal (sinθ=0 ⇒ Y=0); v is undefined there by Eq.(1.2) (division by Ψ).
- What the sources do instead of regularizing: exclude nodes from domain
  (Prop.2.1: D open, Ψ∈C¹(D,ℂ) non-vanishing on D; loops γ⊂D; v continuous
  so fundamental theorem applies). Reddiger Ex.2.2 + Fig.1 (verified):
  family Φ_a mixing Y_1^{−1},Y_1^{+1} (Eqs.2.3b–2.3e); at a=0 the state is real,
  an x–z nodal plane develops, γ (Eq.(2.3d)) no longer lies in D_0 so I(0) is
  strictly undefined (formal v_0=0 gives 0 but loop inadmissible); I=−1 for
  a∈[−1,0), +1 for a∈(0,1]. Directly relevant to claim-61 loop protocol:
  loops must be certified to lie in D (ρ>0 everywhere on γ).
- Numerical-trajectory note (Wyatt/Bittner lineage, prior sweep, not re-parsed
  full text here): Q becomes large around nodes/quasinodes; propagation becomes
  problematic near nodes; trajectories avoid nodal regions (under-sampling).
  Cited as behaviour note, not a core prescription.
- Quantum-potential page (fetched 2026-11-07): Q = −ħ²/2m ∇²R/R; quantum force
  F_Q = −∇Q; properties (scale-invariance in R, non-falloff, whole-arrangement
  information) per Hiley. No hydrogen-2p core formula on the fetched page.

## 4. Core discussion — explicit negative (central for claim-61)

- No finite vortex-core radius, no healing-length-type regulator, no truncated-Q
  cap, no eps0-type smoothed core, and no "core cutoff preserves quantization"
  theorem attributed to Holland (1993) or to Bohm–Hiley (1993) was found in the
  material actually parsed in this sweep.
- In particular: no Holland 2p (m=1)-specific core equation, core figure, core
  radius value, or regularized v_φ(0) prescription was verified here. The 2p
  formula in §2 is singular at origin/axis in every source swept; no source
  swept replaces it with a finite-core profile.
- What the sources do instead: exclude R=0/ψ=0 from the strong-solution domain
  (Reddiger Prop.2.1 D; de Gosson–Hiley R≠0 passage; Holland §3.3.8, p.85 per
  TOC + citing secondary), let Q = −ħ²/2m (∇²R/R) diverge (Wikipedia de
  Broglie–Bohm + Madelung summaries; "infinitely repulsive" in Dennis et al.
  2014, p.6–7 per prior sweep; "becomes large around nodes" in Wyatt/Bittner),
  keep v=∇S/m single-valued so integral curves cannot cross or pass through a
  node (Holland §3.3.8; de Gosson–Hiley 2013; arXiv:2604.25137 citing §3.3.8).
- Computational fixes discussed in trajectory literature (adaptive grids,
  equidistribution, cluster modelling in Wyatt/Bittner; energy-conserving Q
  approximations in Garashchuk–Rassolov lineage) are later numerical devices,
  not a Bohm–Hiley/Holland core cutoff (same negative as sibling sweep
  core_L1_bohm.md).
- Scope limit: absences above are relative to the open-access set actually
  parsed (listed in §5) under the 2026-11-07 websearch outage; paywalled
  full texts (Holland book interior, Bohm–Hiley book interior, Wallstrom PRA,
  Takabayasi PTP, Fritsche–Hasse, Nelson monograph) were not parsed and could
  in principle contain additional discussion — this sweep makes no claim about
  their unparsed interiors beyond the secondary characterizations quoted with
  provenance. Any eps0-type finite core for the 2p probe is a new local probe
  choice, not a Holland value recovered here.

## 5. Source list (as fetched 2026-11-07)

- Wikipedia Madelung equations (Eqs.1.1a–c,1.2,1.3, circulation Γ=∮m v·dl=2πnħ):
  https://en.wikipedia.org/wiki/Madelung_equations
- Wikipedia de Broglie–Bohm theory (guiding equation dQ/dt=ħ/m Im(∇ψ/ψ)):
  https://en.wikipedia.org/wiki/De_Broglie%E2%80%93Bohm_theory
- Wikipedia Quantum potential (Q=−ħ²/2m ∇²R/R, v=∇S/m, QHJE):
  https://en.wikipedia.org/wiki/Quantum_potential
- Reddiger–Poirier abstract: https://arxiv.org/abs/2207.11367
- Reddiger–Poirier full HTML v1 (Eqs.1.1a–c,1.2,1.3,1.4; Prop.2.1; Ex.2.2 + Fig.1;
  Eqs.2.3a–e; Def.2.10; Cor.2.11; Rem.2.7; Rem.3.4; §4):
  https://arxiv.org/html/2207.11367v1
- Wyatt–Bittner abs only (full text NOT parsed here):
  https://arxiv.org/abs/quant-ph/0302088
- Bibliographic pointers NOT parsed (listed for control, no equation numbers
  taken): Holland book interior; Bohm–Hiley book interior; Wallstrom PRA 49:1613;
  Takabayasi PTP 8:143; Holland–Kyprianidis 1988 (Numdam PDF fetch returned binary);
  Dabin diss.; Lawyer thesis; Dennis et al. 2014; de Gosson–Hiley 2013;
  Białynicki-Birula et al. 1992 pp.88–91; Wyatt 2005 §13.5/§2.2/p.48.
- Local repo context only (no Holland numbers taken): theory/hydrogen_vortex.py
  (radial_2p, azimuthal_velocity, circulation, loop_circulation, angular_momentum),
  results/hydrogen_vortex.{json,md}, claims 20/24/26/46, ADR-017; sibling sweeps
  results/swarm/core_L1_bohm.md, core_L1_wallstrom.md, core_L1_takabayasi.md.

Record ends. No verdict.
