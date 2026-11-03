"""Regression: run_all docstring must list every result file it writes.

Waves 0-30 audit: run_all grew from the early falsification core to cover
claims 9-16 (delta, Frank-Oseen, Derrick, topology, reduction, predictions,
Lorentz, sim core, Hopf, Hopfion, 3D pilot, resolution) but the module
docstring still lists only the original subset. README is complete; the
docstring is the stale copy.
"""

from __future__ import annotations

EXPECTED_STEMS = [
    "pdg_fit",
    "rg_run",
    "rotor",
    "neutrinos",
    "quarks",
    "pmns",
    "delta_search",
    "frank_oseen",
    "derrick",
    "topology",
    "reduction",
    "predictions",
    "lorentz",
    "sim_core",
    "hopf",
    "hopfion",
    "hopfion3d",
    "hopf_resolution",
    "summary",
]


def test_run_all_docstring_lists_every_result():
    import phenomenology.run_all as run_all

    doc = run_all.__doc__ or ""
    missing = [stem for stem in EXPECTED_STEMS if stem not in doc]
    assert not missing, f"run_all.py docstring omits result files: {missing}"
