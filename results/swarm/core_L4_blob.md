# core_L4_blob — vortex-blob regularization literature note (claim-60, L4)

Scope: literature sweep only for Fluid Universe claim-60 finite-core planning.
No numerics, no implementation, no verdict. Analogy to quantum core is
annotated as analogy only; no quantum claim is made here.

Search status (2026-11-06):
- Live web search unavailable (quota exhausted, Exa 402). No new crawl.
- This note is compiled from established textbook/canonical forms already in
  training memory. Bibliographic volume/page ranges are given; equation labels
  B1-B3 are note-local. Source-equation crosswalk must be verified against a
  library copy before promotion to theory/formulas.md. Do not promote Eq
  numbers as audited until that check is done.

## Sources (bibliographic only)

S1. A. J. Chorin, "Numerical study of slightly viscous flow,"
  J. Fluid Mech. 57(4), 785-796 (1973).
  Origin of vortex-blob / cutoff idea for 2D Euler/Navier-Stokes.

S2. R. Krasny, "Desingularization of periodic vortex sheet roll-up,"
  J. Comput. Phys. 65(2), 292-313 (1986).
  Periodic-sheet desingularization with parameter delta; N-convergence study
  at fixed delta.

S3. J. T. Beale and A. Majda, "Vortex methods I: Convergence in three
  dimensions," Math. Comp. 39, 1-27 (1982); "Vortex methods II: Higher order
  accuracy in two and three dimensions," Math. Comp. 39, 29-52 (1985).
  Convergence theory for blob/cutoff methods with moment conditions and
  delta-h scaling.

S4. Textbooks for cross-check (not primary evidence here):
  G.-H. Cottet and P. Koumoutsakos, Vortex Methods (Cambridge, 2000), Chap. 4;
  A. Majda and A. Bertozzi, Vorticity and Incompressible Flow (Cambridge,
  2002), Chap. 7.

## Blob kernel formula (note-local labels B1-B3)

Singular 2D Biot-Savart kernel (reference, not regularized):

  K(x) = (1/2pi) x_perp / |x|^2,  x_perp = (-y, x),  x in R^2.
  u(x) = sum_j Gamma_j K(x - x_j).

(B1) Algebraic blob — standard Chorin/Krasny/Beale-Majda 2nd-order form
cited as the Krasny desingularization in free space:

  K_delta(x) = (1/2pi) x_perp / (|x|^2 + delta^2)
             = K(x) * |x|^2 / (|x|^2 + delta^2).

  Azimuthal swirl for one blob of strength Gamma:
  u_theta(r) = Gamma/(2pi) * r / (r^2 + delta^2).
  Maximum Gamma/(4pi delta) at r = delta.
  Far field r >> delta: recovers Gamma/(2pi r).

  Cf. S2 desingularized Birkhoff-Rott (periodic form replaces |x|^2 by the
  periodic distance, e.g. sinh/cosh form; free-space algebraic form above is
  the kernel analyzed in S3). Cf. S4 Chap. 4 for the cutoff-function family
  K_delta = K * Psi(|x|/delta) with Psi(r) = r^2/(r^2+1) for (B1).

(B2) Rankine / solid-body core — original Chorin cutoff interpretation:

  u_theta(r) = Gamma r / (2pi sigma^2) for r < sigma,
               Gamma / (2pi r)         for r >= sigma.

  Cf. S1 cutoff-radius discussion (pp. 785-796); sigma is the cutoff radius.

(B3) Gaussian / Lamb-Oseen higher-order alternative (for moment-order
contrast; not the Krasny default):

  K_delta(x) = K(x) * (1 - exp(-|x|^2/delta^2)),
  u_theta(r) = Gamma/(2pi r) * (1 - exp(-r^2/delta^2)).

  Cf. S3-S4: (B1) satisfies 2nd-order moment conditions; (B3) is the smooth
  exponentially-decaying member used to illustrate higher-order cutoffs.

Equation/page crosswalk to verify (library copy required):
- S1: JFM 57, pp. 785-796; cutoff introduction ~pp. 786-787.
- S2: JCP 65, pp. 292-313; singular vs desingularized Birkhoff-Rott ~Sec. 2
  (standard citation: Eqs. 2.3-2.4 / 11-12 depending on reprint); N-study
  figures ~Sec. 3-4.
- S3: Math. Comp. 39, pp. 1-27 and 29-52; cutoff moment conditions and
  delta = h^q scaling theorems.
Until verified, treat the "~" locations as pointers, not audited pins.

## Core parameter role

- delta (also written sigma in S1) is the core radius / desingularization
  parameter with units of length.
- Removes the 1/r singularity: velocity capped at O(Gamma/delta) instead of
  diverging; vortex-sheet Birkhoff-Rott integral made non-singular.
- Sets the smallest resolved scale: features with radius << delta are
  smoothed; r >> delta recovers point-vortex / singular dynamics.
- delta -> 0 recovers the singular equations but reintroduces Kelvin-Helmholtz
  ill-posedness / unbounded high-wavenumber growth; delta large over-smooths
  and suppresses spiral roll-up turns.
- Practical coupling: h = initial inter-particle spacing (~L/N). Overlap
  condition requires h/delta -> 0 as h -> 0; standard choice delta = h^q with
  0 < q < 1 (S3), or in practice delta = C h with C > 1 held fixed while
  refining N at fixed delta first (S2 procedure).

## Convergence statement (classical fluid only)

- For fixed delta > 0: the N-point blob discretization converges to the
  desingularized continuum evolution as N -> infinity (h -> 0), provided
  overlap h/delta -> 0. Krasny S2 demonstrates this empirically: at fixed
  delta, curves for N = 100/200/400 coincide to plotting accuracy up to
  spiral roll-up time; refining N at fixed delta is the reported convergence
  test.
- Then delta -> 0: the desingularized continuum converges to the Euler
  solution while the Euler solution stays smooth (S3 theorems; rate depends
  on cutoff moment order and delta-h scaling). Without desingularization the
  point-vortex / Birkhoff-Rott discretization does not converge in general.
- Failure mode documented in S2: for a given N, taking delta too small
  (delta <= h) produces spurious oscillations / non-converged small scales;
  more spiral turns appear as delta decreases only if N is simultaneously
  increased to maintain overlap. Convergence is therefore a two-limit
  statement (N -> infinity at fixed delta, then delta -> 0), not a single
  N-only limit.

## Analogy to quantum core (analogy only, no quantum claim)

- Structural analogy only: a finite fluid core delta caps velocity/vorticity
  the way a finite quantum-core scale epsilon would cap quantum pressure /
  phase-gradient divergence at a node. Both introduce a length scale that
  regularizes a 1/r singularity and both require a two-limit convergence
  check (resolution -> 0 at fixed core, then core -> 0).
- No transfer is asserted: this note makes no claim that blob convergence
  implies Madelung-quantum convergence, that delta equals any healing length
  or Bohr-radius fraction, or that a finite core reproduces any defect number.
  Quantization, single-valuedness, and Wallstrom-type conditions have no
  counterpart in S1-S4.
- Use for claim-60 is restricted to motivating the pre-registered epsilon
  scan and the N-at-fixed-epsilon convergence protocol; it does not supply a
  pass/fail criterion.

No verdict rendered. No claim number assigned. Upgrade path: verify Eq/page
pointers against library copies of S1-S3, then promote kernel + scaling law
to theory/formulas.md as F44 candidate with audit checks.
