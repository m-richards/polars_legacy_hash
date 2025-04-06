import polars as pl
import pytest
from polars.exceptions import ComputeError
from polars.testing import assert_frame_equal

import polars_legacy_hash as plh  # noqa: F401


def test_oldhash_int64():
    df =pl.Series([-42],dtype=pl.Int64 ).to_frame("test")
    result = df.select(plh.oldhash(pl.col("test")))
    print(df)
    print(result)
    # result = pl.select(pl.lit(42).nchash.oldhash())  # type: ignore

    expected = pl.DataFrame(
        [
            pl.Series("test", [15244781726809025498], dtype=pl.UInt64),
        ]
    )

    assert_frame_equal(result, expected)

def test_oldhash_int32():
    df =pl.Series([-42],dtype=pl.Int32 ).to_frame("test")
    result = df.select(plh.oldhash(pl.col("test")))
    print(df)
    print(result)
    # result = pl.select(pl.lit(42).nchash.oldhash())  # type: ignore

    expected = pl.DataFrame(
        [
            pl.Series("test", [17010062867703544896], dtype=pl.UInt64),
        ]
    )

    assert_frame_equal(result, expected)
