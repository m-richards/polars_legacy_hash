from __future__ import annotations

import warnings
from pathlib import Path
from typing import Iterable, Protocol, cast

import polars as pl
from polars.plugins import register_plugin_function

try:
    from polars._typing import IntoExpr, PolarsDataType
except ImportError:
    from polars.type_aliases import IntoExpr, PolarsDataType  # type: ignore[no-redef]

from polars_legacy_hash._internal import __version__ as __version__


def oldhash(expr: IntoExpr) -> pl.Expr:
    """Polars 0.20.10 hash."""
    return register_plugin_function(
        plugin_path=Path(__file__).parent,
        function_name="oldhash",
        args=expr,
        is_elementwise=True,
    )


__all__ = ["oldhash"]
