# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/datatypes/translation_and_mat3x3.fbs".

# You can extend this class by creating a "TranslationAndMat3x3Ext" class in "translation_and_mat3x3_ext.py".

from __future__ import annotations

from typing import Sequence, Union

import pyarrow as pa
from attrs import define, field

from .. import datatypes
from .._baseclasses import BaseBatch, BaseExtensionType
from .translation_and_mat3x3_ext import TranslationAndMat3x3Ext

__all__ = [
    "TranslationAndMat3x3",
    "TranslationAndMat3x3ArrayLike",
    "TranslationAndMat3x3Batch",
    "TranslationAndMat3x3Like",
    "TranslationAndMat3x3Type",
]


def _translation_and_mat3x3__translation__special_field_converter_override(
    x: datatypes.Vec3DLike | None
) -> datatypes.Vec3D | None:
    if x is None:
        return None
    elif isinstance(x, datatypes.Vec3D):
        return x
    else:
        return datatypes.Vec3D(x)


def _translation_and_mat3x3__mat3x3__special_field_converter_override(
    x: datatypes.Mat3x3Like | None
) -> datatypes.Mat3x3 | None:
    if x is None:
        return None
    elif isinstance(x, datatypes.Mat3x3):
        return x
    else:
        return datatypes.Mat3x3(x)


@define(init=False)
class TranslationAndMat3x3(TranslationAndMat3x3Ext):
    """
    **Datatype**: Representation of an affine transform via a 3x3 affine matrix paired with a translation.

    First applies the matrix, then the translation.
    """

    # __init__ can be found in translation_and_mat3x3_ext.py

    from_parent: bool = field(converter=bool)
    # If true, this transform is from the parent space to the space where the transform was logged.
    #
    # If false (default), the transform maps from this space to its parent,
    # i.e. the translation is the position in the parent space.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    translation: datatypes.Vec3D | None = field(
        default=None, converter=_translation_and_mat3x3__translation__special_field_converter_override
    )
    # 3D translation, applied after the matrix.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    mat3x3: datatypes.Mat3x3 | None = field(
        default=None, converter=_translation_and_mat3x3__mat3x3__special_field_converter_override
    )
    # 3x3 matrix for scale, rotation & shear.
    #
    # (Docstring intentionally commented out to hide this field from the docs)


TranslationAndMat3x3Like = TranslationAndMat3x3
TranslationAndMat3x3ArrayLike = Union[
    TranslationAndMat3x3,
    Sequence[TranslationAndMat3x3Like],
]


class TranslationAndMat3x3Type(BaseExtensionType):
    _TYPE_NAME: str = "rerun.datatypes.TranslationAndMat3x3"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.struct(
                [
                    pa.field(
                        "translation",
                        pa.list_(pa.field("item", pa.float32(), nullable=False, metadata={}), 3),
                        nullable=True,
                        metadata={},
                    ),
                    pa.field(
                        "mat3x3",
                        pa.list_(pa.field("item", pa.float32(), nullable=False, metadata={}), 9),
                        nullable=True,
                        metadata={},
                    ),
                    pa.field("from_parent", pa.bool_(), nullable=False, metadata={}),
                ]
            ),
            self._TYPE_NAME,
        )


class TranslationAndMat3x3Batch(BaseBatch[TranslationAndMat3x3ArrayLike]):
    _ARROW_TYPE = TranslationAndMat3x3Type()

    @staticmethod
    def _native_to_pa_array(data: TranslationAndMat3x3ArrayLike, data_type: pa.DataType) -> pa.Array:
        raise NotImplementedError  # You need to implement native_to_pa_array_override in translation_and_mat3x3_ext.py
