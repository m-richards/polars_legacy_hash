import polars as pl
from polars.testing import assert_frame_equal

import polars_legacy_hash as plh  # noqa: F401


def test_oldhash_int64(expected_int64_neg_42):
    df = pl.Series([-42], dtype=pl.Int64).to_frame("test")
    result = df.select(plh.oldhash(pl.col("test")))
    assert_frame_equal(result, expected_int64_neg_42.to_frame())


def test_oldhash_int32(expected_int32_neg_42):
    df = pl.Series([-42], dtype=pl.Int32).to_frame("test")
    result = df.select(plh.oldhash(pl.col("test")))
    assert_frame_equal(result, expected_int32_neg_42.to_frame())

def test_oldhash_int8(expected_int8_42):
    df = pl.Series([42], dtype=pl.Int8).to_frame("test")
    result = df.select(plh.oldhash(pl.col("test")))
    assert_frame_equal(result, expected_int8_42.to_frame())



def test_int_struct(raw_struct_df, expected_int_struct):
    df = raw_struct_df
    print(df.dtypes)
    result = pl.select(plh.oldhash(df.to_struct("test")))
    print(result)

    assert_frame_equal(result, expected_int_struct.to_frame())


def test_int_struct_singular(raw_struct_df_singular, expected_int_struct_singular):
    df = raw_struct_df_singular
    print(df.dtypes)
    result = pl.select(plh.oldhash(df.to_struct("test")))
    print(result)

    assert_frame_equal(result, expected_int_struct_singular.to_frame())



def test_long_struct():
    df = pl.DataFrame({
        "a":list(range(11)),
        "b":[1 for _ in range(11)],
        "c":[3 for _ in range(11)],
        "d":[1 for _ in range(11)],
        "e":[1 for _ in range(11)],
    }).cast({
        "a":pl.Int64,
        "b":pl.Int64,
        "c":pl.Int8,
        "d":pl.Int8,
        "e":pl.Int8,
    })

    result = pl.select(plh.oldhash(df.to_struct("test")))
    assert_frame_equal(result, df)
