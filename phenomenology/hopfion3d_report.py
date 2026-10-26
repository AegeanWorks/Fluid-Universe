"""3D pilot report for the E2+E4+Z3 model."""

from __future__ import annotations

from sims.core3d import pilot_scan

from .provenance import RESULTS_DIR, metadata, write_json, write_text_atomic


def run_hopfion3d_check(write: bool = True) -> dict:
    payload = {
        "meta": metadata(
            {
                "model": "E2 + Skyrme E4 + mu rho^2 (1-cos3phi)",
                "K": 1.0,
                "kappa": 1.0,
                "n": 32,
                "length": 10.0,
                "steps": 300,
                "dt": 5e-4,
                "seed": "deterministic analytic H=1 initial condition",
                "scope": "single-seed pilot; not production",
            }
        ),
        **pilot_scan(n=32, length=10.0, mu_values=(0.0, 1.0, 5.0, 20.0), steps=300, dt=5e-4),
    }
    if write:
        RESULTS_DIR.mkdir(parents=True, exist_ok=True)
        write_json(RESULTS_DIR / "hopfion3d.json", payload)
        write_text_atomic(RESULTS_DIR / "hopfion3d.md", render_markdown(payload))
    return payload


def render_markdown(payload: dict) -> str:
    v = payload["verdict"]
    initial = payload["initial"]
    lines = [
        "# 3D E2+E4+Z3 pilot (claim 15)",
        "",
        "Single-seed pilot, N=32, L=10, dt=5e-4, 300 gradient steps. "
        "The initial condition is the analytic H=1 map with a smooth vacuum compactification. "
        "This is not a production run.",
        "",
        f"Initial: H = {initial['H']:.9f}, Z3 = {initial['Z3']:.3e}.",
        "",
        "| mu | E2 | E4 | V | H | Z3 | monotone | max dE |",
        "|---:|---:|---:|---:|---:|---:|---|---:|",
    ]
    for row in payload["rows"]:
        lines.append(
            f"| {row['mu']:.3g} | {row['E2']:.6f} | {row['E4']:.6f} | {row['V']:.6f} | "
            f"{row['H']:.9f} | {row['Z3']:.6f} | {row['monotone']} | "
            f"{row['max_energy_increase']:+.3e} |"
        )
    lines += [
        "",
        f"**Verdict:** coexistence |H|>=1 and Z3>=0.5: **{v['coexistence_H_and_Z3']}**. "
        f"Maximum Z3 = {v['max_Z3']:.6f} at mu={v['mu_at_max_Z3']:.3g}, "
        f"where H = {v['H_at_max_Z3']:.6f}.",
        "",
        f"Claim 15: **{v['claim_15_status']}**.",
        "",
        v["note"],
        "",
        f"- Config SHA-256: `{payload['meta']['config_sha256']}`",
        f"- Git: `{payload['meta']['git']['commit']}` (dirty={payload['meta']['git']['dirty']})",
        "",
    ]
    return "\n".join(lines)
