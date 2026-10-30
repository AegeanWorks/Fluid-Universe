"""Stable fingerprints for the two-run claim audit.

The audit must reject stale results: if the code, docs, or physics outputs
change between the two runs, the two runs are not verifying the same object.
`source_fingerprint` covers code and prose; `results_fingerprint` covers the
generated physics outputs. The verification files themselves are included in
the source set, so editing the harness invalidates earlier runs.
"""

from __future__ import annotations

import hashlib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

EXCLUDED_DIRS = {
    ".git",
    ".aikit",
    "__pycache__",
    ".pytest_cache",
    ".hypothesis",
    ".mypy_cache",
    "results",
}

SOURCE_SUFFIXES = {".py", ".md", ".tex", ".toml", ".yaml", ".yml", ".txt", ".csv"}
SOURCE_NAMES = {"Makefile"}

RESULT_FILES = (
    "pdg_fit.json",
    "pdg_fit.md",
    "rg_run.json",
    "rg_run.md",
    "rotor.json",
    "rotor.md",
    "neutrinos.json",
    "neutrinos.md",
    "quarks.json",
    "quarks.md",
    "pmns.json",
    "pmns.md",
    "delta_search.json",
    "delta_search.md",
    "frank_oseen.json",
    "frank_oseen.md",
    "derrick.json",
    "derrick.md",
    "topology.json",
    "topology.md",
    "reduction.json",
    "reduction.md",
    "predictions.json",
    "predictions.md",
    "lorentz.json",
    "lorentz.md",
    "sim_core.json",
    "sim_core.md",
    "hopf.json",
    "hopf.md",
    "hopfion.json",
    "hopfion.md",
    "hopfion3d.json",
    "hopfion3d.md",
    "hopfion3d_n64.json",
    "hopfion3d_n64.md",
    "hopfion_scale.json",
    "hopfion_scale.md",
    "hopfion_stability.json",
    "hopfion_stability.md",
    "hopfion3d_production.json",
    "hopfion3d_production.md",
    "hopfion3d_convergence.json",
    "hopfion3d_convergence.md",
    "hopfion3d_temporal.json",
    "hopfion3d_temporal.md",
    "hopfion3d_extension.json",
    "hopfion3d_extension.md",
    "hopfion_purefn_stability.json",
    "hopfion_purefn_stability.md",
    "hopfion_saddle_escape.json",
    "hopfion_saddle_escape.md",
    "hopfion_length_scan.json",
    "hopfion_length_scan.md",
    "vortex_knot_spectrum.json",
    "vortex_knot_spectrum.md",
    "vortex_cascade.json",
    "vortex_cascade.md",
    "rotor_potential.json",
    "rotor_potential.md",
    "madelung.json",
    "madelung.md",
    "nelson.json",
    "nelson.md",
    "gaussian_fluid.json",
    "gaussian_fluid.md",
    "coherent_fluid.json",
    "coherent_fluid.md",
    "excited_fluid.json",
    "excited_fluid.md",
    "hydrogen_fluid.json",
    "hydrogen_fluid.md",
    "hydrogen_vortex.json",
    "hydrogen_vortex.md",
    "water_analogue.json",
    "water_analogue.md",
    "water_validity.json",
    "water_validity.md",
    "water_dispersive.json",
    "water_dispersive.md",
    "water_rays.json",
    "water_rays.md",
    "water_scattering.json",
    "water_scattering.md",
    "hopf_resolution.json",
    "hopf_resolution.md",
    "summary.md",
)


def _source_files() -> list[Path]:
    files: list[Path] = []
    for path in ROOT.rglob("*"):
        if not path.is_file():
            continue
        if any(part in EXCLUDED_DIRS for part in path.relative_to(ROOT).parts):
            continue
        if path.name in SOURCE_NAMES or path.suffix.lower() in SOURCE_SUFFIXES:
            files.append(path)
    return sorted(files, key=lambda p: p.relative_to(ROOT).as_posix())


def _digest(files: list[Path]) -> str:
    hasher = hashlib.sha256()
    for path in files:
        relative = path.relative_to(ROOT).as_posix().encode("utf-8")
        hasher.update(relative)
        hasher.update(b"\0")
        hasher.update(path.read_bytes())
        hasher.update(b"\0")
    return hasher.hexdigest()


def source_fingerprint() -> str:
    return _digest(_source_files())


def results_fingerprint() -> str:
    files = [ROOT / "results" / name for name in RESULT_FILES if (ROOT / "results" / name).is_file()]
    return _digest(files)


def fingerprints() -> dict[str, str]:
    return {
        "source_sha256": source_fingerprint(),
        "results_sha256": results_fingerprint(),
    }
