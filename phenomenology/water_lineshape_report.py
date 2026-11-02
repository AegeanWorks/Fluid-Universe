"""Fixed-lambda cavity-Marcus line-shape prediction (claim 43, ADR-034)."""

from __future__ import annotations

import math

from theory.water_lineshape import (
    LAMBDA_EV,
    SHAPE_RATIO_TOL,
    SKEW_TOL,
    WIDTH_RATIO_TOL,
    lineshape_scales,
)

from .provenance import RESULTS_DIR, metadata, write_json, write_text_atomic


def run_water_lineshape_check(write: bool = True) -> dict:
    scales = lineshape_scales()
    moments = scales["moments"]
    prediction = scales["prediction"]
    expected_ratio = math.sqrt(
        (prediction["lambda_45c_ev"] * 318.15)
        / (prediction["lambda_0c_ev"] * 273.15)
    )
    result = {
        "scales": scales,
        "verdict": {
            "symmetry_exact": moments["symmetric"] is True
            and moments["skewness"] == 0.0
            and moments["half_width_ratio"] == 1.0,
            "widths_match_temperature_scan": abs(
                prediction["width_0c_ev"] - 0.653821691
            )
            < 1.0e-6
            and abs(prediction["width_45c_ev"] - 0.695194215) < 1.0e-6,
            "ratio_matches_sqrt_lambda_t": abs(
                prediction["width_ratio_45_over_0"] - expected_ratio
            )
            < 1.0e-9,
            "claim_43_status": (
                "UNVERIFIED (pre-registered prediction; no observed "
                "width-vs-T or shape comparator)"
            ),
            "note": (
                "With lambda fixed at 1.617132544 eV from the Born cavity, "
                "the classical Marcus Gaussian predicts W(0 C)=0.653822 eV, "
                "W(45 C)=0.695194 eV, ratio 1.0633, with exact symmetry "
                "(skew 0, half-width ratio 1). No observed width change or "
                "band-shape numbers exist in this repository, so this is a "
                "prediction with kill thresholds "
                f"(ratio {WIDTH_RATIO_TOL}, shape {SHAPE_RATIO_TOL}, skew "
                f"{SKEW_TOL}), never a comparison. A future comparator kills "
                "the sqrt(lambda T) scaling if |observed ratio - 1.0633| > "
                "0.05 and kills Gaussian-only shape if |observed half-width "
                "ratio - 1| > 0.15."
            ),
        },
    }
    config = {
        "lambda_ev": LAMBDA_EV,
        "width_ratio_tol": WIDTH_RATIO_TOL,
        "shape_ratio_tol": SHAPE_RATIO_TOL,
        "skew_tol": SKEW_TOL,
        "scope": "fixed-lambda Marcus line-shape prediction, no fit",
    }
    payload = {"meta": metadata(config), "result": result}
    if write:
        RESULTS_DIR.mkdir(parents=True, exist_ok=True)
        write_json(RESULTS_DIR / "water_lineshape.json", payload)
        write_text_atomic(
            RESULTS_DIR / "water_lineshape.md", render_markdown(payload)
        )
    return payload


def render_markdown(payload: dict) -> str:
    result = payload["result"]
    scales = result["scales"]
    prediction = scales["prediction"]
    moments = scales["moments"]
    thresholds = scales["thresholds"]
    verdict = result["verdict"]
    lines = [
        "# Fixed-lambda cavity-Marcus line-shape prediction (claim 43)",
        "",
        "Model: classical Marcus Gaussian with lambda fixed from claim 39.",
        "",
        f"\\(\\lambda={scales['lambda_ev']:.9f}\\) eV; "
        "\\(\\text{FWHM}=2\\sqrt{2\\ln2}\\sqrt{2\\lambda k_BT}\\).",
        "",
        "## Predicted widths",
        "",
        "| quantity | value |",
        "|---|---:|",
        f"| W(0 C) | {prediction['width_0c_ev']:.6f} eV |",
        f"| W(45 C) | {prediction['width_45c_ev']:.6f} eV |",
        f"| change | {prediction['width_change_ev']:.6f} eV |",
        f"| ratio W45/W0 | {prediction['width_ratio_45_over_0']:.6f} |",
        "",
        "## Symmetry (exact for a Gaussian)",
        "",
        "| quantity | value |",
        "|---|---:|",
        f"| skewness | {moments['skewness']:.6f} |",
        f"| half-width ratio | {moments['half_width_ratio']:.6f} |",
        f"| third central moment | {moments['third_central_moment']:.6f} |",
        "",
        "## Kill thresholds (for a future comparator)",
        "",
        "| quantity | value |",
        "|---|---:|",
        f"| width ratio tolerance | {thresholds['width_ratio_tol']:.2f} |",
        f"| shape ratio tolerance | {thresholds['shape_ratio_tol']:.2f} |",
        f"| skew tolerance | {thresholds['skew_tol']:.2f} |",
        "",
        "## Verdict",
        "",
        f"- Symmetry exact: **{verdict['symmetry_exact']}**.",
        f"- Widths match the temperature scan: "
        f"**{verdict['widths_match_temperature_scan']}**.",
        f"- Ratio matches sqrt(lambda T): "
        f"**{verdict['ratio_matches_sqrt_lambda_t']}**.",
        f"- Claim 43: **{verdict['claim_43_status']}**.",
        "",
        verdict["note"],
        "",
        f"- Config SHA-256: `{payload['meta']['config_sha256']}`",
        f"- Git: `{payload['meta']['git']['commit']}` "
        f"(dirty={payload['meta']['git']['dirty']})",
        "",
    ]
    return "\n".join(lines)


if __name__ == "__main__":
    payload = run_water_lineshape_check()
    print(payload["result"]["verdict"])
