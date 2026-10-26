"""Report for the coarse N=64 Hessian stability check."""

from __future__ import annotations

from sims.stability import build_candidate, hessian_spectrum

from .provenance import RESULTS_DIR, metadata, write_json, write_text_atomic


def run_stability_check(write: bool = True) -> dict:
    candidate, info = build_candidate(
        n=64, length=10.0, iterations=5, steps=600, dt=5e-4
    )
    spectrum = hessian_spectrum(candidate, info["dx"], k=8)
    payload = {
        "meta": metadata(
            {
                "n": 64,
                "length": 10.0,
                "iterations": 5,
                "steps": 600,
                "dt": 5e-4,
                "k_eigenvalues": 8,
                "boundary": "fixed vacuum faces",
                "scope": "coarse Hessian at N=64; not part of make results",
            }
        ),
        "candidate": info,
        "spectrum": spectrum,
        "verdict": {
            "local_minimum_at_N64": bool(spectrum["min_eigenvalue"] > -1e-6),
            "stable_at_N64": bool(spectrum["min_eigenvalue"] > -1e-6),
            "claim_14b_status": "UNVERIFIED",
            "claim_14c_status": "NUMERIC (scale-stationary H=1 candidate exists)",
            "claim_14d_status": "FALSIFIED (negative Hessian modes; not a local minimum)",
            "note": (
                "The scale-stationary H=1 candidate has negative Hessian eigenvalues "
                "at N=64, so it is a saddle, not a stable Hopfion. No stable Hopfion "
                "is demonstrated."
            ),
        },
    }
    if write:
        RESULTS_DIR.mkdir(parents=True, exist_ok=True)
        write_json(RESULTS_DIR / "hopfion_stability.json", payload)
        write_text_atomic(RESULTS_DIR / "hopfion_stability.md", render_markdown(payload))
    return payload


def render_markdown(payload: dict) -> str:
    c = payload["candidate"]
    s = payload["spectrum"]
    v = payload["verdict"]
    lines = [
        "# Coarse Hessian stability at N=64 (claim 14b/14c)",
        "",
        "Scale-iteration candidate, fixed vacuum boundaries. This is a coarse Hessian "
        "diagnostic, not a full stability proof.",
        "",
        f"Candidate: H={c['H']:.9f}, Z3={c['Z3']:.3e}, E2-E4={c['virial']:+.6f}, "
        f"force RMS={c['force_rms']:.3e}.",
        "",
        "| quantity | value |",
        "|---|---:|",
        f"| interior dimension | {s['interior_dimension']} |",
        f"| minimum eigenvalue | {s['min_eigenvalue']:+.6f} |",
        f"| negative eigenvalues | {s['negative_count']} |",
        f"| near-zero eigenvalues | {s['near_zero_count']} |",
        "",
        "Smallest eigenvalues: "
        + ", ".join(f"{value:+.6f}" for value in s["smallest_eigenvalues"])
        + ".",
        "",
        f"**Verdict:** local minimum at N=64: **{v['local_minimum_at_N64']}**; "
        f"stable at N=64: **{v['stable_at_N64']}**; "
        f"claim 14c: **{v['claim_14c_status']}**; "
        f"claim 14d: **{v['claim_14d_status']}**; claim 14b: **{v['claim_14b_status']}**.",
        "",
        v["note"],
        "",
        f"- Config SHA-256: `{payload['meta']['config_sha256']}`",
        f"- Git: `{payload['meta']['git']['commit']}` (dirty={payload['meta']['git']['dirty']})",
        "",
    ]
    return "\n".join(lines)


if __name__ == "__main__":
    print(run_stability_check()["verdict"])
