# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/datatypes/keypoint_id.fbs".

# You can extend this class by creating a "KeypointIdExt" class in "keypoint_id_ext.py".

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field

from .._baseclasses import BaseBatch, BaseExtensionType
from .keypoint_id_ext import KeypointIdExt

__all__ = ["KeypointId", "KeypointIdArrayLike", "KeypointIdBatch", "KeypointIdLike", "KeypointIdType"]


@define
class KeypointId(KeypointIdExt):
    """
    A 16-bit ID representing a type of semantic keypoint within a class.

    `KeypointId`s are only meaningful within the context of a [`rerun.components.ClassDescription`][].

    Used to look up an [`rerun.components.AnnotationInfo`][] for a Keypoint within the
    [`rerun.components.AnnotationContext`].
    """

    # You can define your own __init__ function as a member of KeypointIdExt in keypoint_id_ext.py

    id: int = field(converter=int)

    def __array__(self, dtype: npt.DTypeLike = None) -> npt.NDArray[Any]:
        # You can define your own __array__ function as a member of KeypointIdExt in keypoint_id_ext.py
        return np.asarray(self.id, dtype=dtype)

    def __int__(self) -> int:
        return int(self.id)


if TYPE_CHECKING:
    KeypointIdLike = Union[KeypointId, int]
else:
    KeypointIdLike = Any

KeypointIdArrayLike = Union[
    KeypointId,
    Sequence[KeypointIdLike],
    int,
    npt.NDArray[np.uint8],
    npt.NDArray[np.uint16],
    npt.NDArray[np.uint32],
    npt.NDArray[np.uint64],
]


class KeypointIdType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.datatypes.KeypointId"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.uint16(), self._TYPE_NAME)


class KeypointIdBatch(BaseBatch[KeypointIdArrayLike]):
    _ARROW_TYPE = KeypointIdType()

    @staticmethod
    def _native_to_pa_array(data: KeypointIdArrayLike, data_type: pa.DataType) -> pa.Array:
        return KeypointIdExt.native_to_pa_array_override(data, data_type)


# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(KeypointIdType())


if hasattr(KeypointIdExt, "deferred_patch_class"):
    KeypointIdExt.deferred_patch_class(KeypointId)
