# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/components/depth_meter.fbs".

# You can extend this class by creating a "DepthMeterExt" class in "depth_meter_ext.py".

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field

from .._baseclasses import BaseBatch, BaseExtensionType, ComponentBatchMixin
from .depth_meter_ext import DepthMeterExt

__all__ = ["DepthMeter", "DepthMeterArrayLike", "DepthMeterBatch", "DepthMeterLike", "DepthMeterType"]


@define(init=False)
class DepthMeter(DepthMeterExt):
    """A component indicating how long a meter is, expressed in native units."""

    def __init__(self: Any, value: DepthMeterLike):
        """Create a new instance of the DepthMeter component."""

        # You can define your own __init__ function as a member of DepthMeterExt in depth_meter_ext.py
        self.__attrs_init__(value=value)

    value: float = field(converter=float)

    def __array__(self, dtype: npt.DTypeLike = None) -> npt.NDArray[Any]:
        # You can define your own __array__ function as a member of DepthMeterExt in depth_meter_ext.py
        return np.asarray(self.value, dtype=dtype)

    def __float__(self) -> float:
        return float(self.value)


if TYPE_CHECKING:
    DepthMeterLike = Union[DepthMeter, float]
else:
    DepthMeterLike = Any

DepthMeterArrayLike = Union[DepthMeter, Sequence[DepthMeterLike], float, npt.NDArray[np.float32]]


class DepthMeterType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.components.DepthMeter"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.float32(), self._TYPE_NAME)


class DepthMeterBatch(BaseBatch[DepthMeterArrayLike], ComponentBatchMixin):
    _ARROW_TYPE = DepthMeterType()

    @staticmethod
    def _native_to_pa_array(data: DepthMeterArrayLike, data_type: pa.DataType) -> pa.Array:
        return DepthMeterExt.native_to_pa_array_override(data, data_type)


# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(DepthMeterType())


if hasattr(DepthMeterExt, "deferred_patch_class"):
    DepthMeterExt.deferred_patch_class(DepthMeter)
