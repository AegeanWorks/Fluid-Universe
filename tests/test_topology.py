"""Nematic topology tests (claim 13)."""

from __future__ import annotations

from theory.topology import HOMOTOPY, direct_result, extensions, has_z3_factor
from phenomenology.uv_negatives import run_topology_check


def test_homotopy_groups_are_recorded():
    assert HOMOTOPY == {"pi1": "Z2", "pi2": "Z", "pi3": "Z"}


def test_no_z3_factor_in_rp2():
    assert not has_z3_factor()
    result = direct_result()
    assert result["z3_factor_present"] is False
    assert result["direct_claim_falsified"] is True
    assert result["cos3phi_inserted_by_hand"] is True


def test_standard_extensions_do_not_derive_z3():
    ext = extensions()
    assert set(ext) == {"rank_three_order_parameter", "three_coupled_nematics", "three_fold_lattice"}
    assert all(item["z3_derived"] is False for item in ext.values())


def test_verdict_and_json_shape():
    payload = run_topology_check(write=False)
    v = payload["verdict"]
    assert v["13a_direct_z3_from_nematic_topology"] is False
    assert v["13b_natural_extension_yields_z3_without_insertion"] is False
    assert v["direct_claim_status"] == "FALSIFIED"
    assert "not excluded" in v["note"]
