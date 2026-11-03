"""G-001: two-run verification must use two distinct tags."""

import pytest

from verification.compare import compare


def test_compare_rejects_identical_tags():
    with pytest.raises(ValueError, match="tags must differ"):
        compare(("run1", "run1"))
