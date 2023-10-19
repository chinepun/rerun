# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/testing/datatypes/fuzzy.fbs".

# You can extend this class by creating a "AffixFuzzer2Ext" class in "affix_fuzzer2_ext.py".

from __future__ import annotations

from typing import Any, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field
from rerun._baseclasses import BaseBatch, BaseExtensionType
from rerun._converters import (
    float_or_none,
)

__all__ = ["AffixFuzzer2", "AffixFuzzer2ArrayLike", "AffixFuzzer2Batch", "AffixFuzzer2Like", "AffixFuzzer2Type"]


@define(init=False)
class AffixFuzzer2:
    def __init__(self: Any, single_float_optional: float | None = None):
        """Create a new instance of the AffixFuzzer2 datatype."""

        # You can define your own __init__ function as a member of AffixFuzzer2Ext in affix_fuzzer2_ext.py
        self.__attrs_init__(single_float_optional=single_float_optional)

    single_float_optional: float | None = field(default=None, converter=float_or_none)

    def __array__(self, dtype: npt.DTypeLike = None) -> npt.NDArray[Any]:
        # You can define your own __array__ function as a member of AffixFuzzer2Ext in affix_fuzzer2_ext.py
        return np.asarray(self.single_float_optional, dtype=dtype)


AffixFuzzer2Like = AffixFuzzer2
AffixFuzzer2ArrayLike = Union[
    AffixFuzzer2,
    Sequence[AffixFuzzer2Like],
]


class AffixFuzzer2Type(BaseExtensionType):
    _TYPE_NAME: str = "rerun.testing.datatypes.AffixFuzzer2"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.float32(), self._TYPE_NAME)


class AffixFuzzer2Batch(BaseBatch[AffixFuzzer2ArrayLike]):
    _ARROW_TYPE = AffixFuzzer2Type()

    @staticmethod
    def _native_to_pa_array(data: AffixFuzzer2ArrayLike, data_type: pa.DataType) -> pa.Array:
        raise NotImplementedError  # You need to implement native_to_pa_array_override in affix_fuzzer2_ext.py
