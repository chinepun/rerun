# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/testing/archetypes/fuzzy.fbs".

# You can extend this class by creating a "AffixFuzzer2Ext" class in "affix_fuzzer2_ext.py".

from __future__ import annotations

from typing import Any

from attrs import define, field
from rerun._baseclasses import Archetype
from rerun.error_utils import catch_and_log_exceptions

from .. import components, datatypes

__all__ = ["AffixFuzzer2"]


@define(str=False, repr=False, init=False)
class AffixFuzzer2(Archetype):
    def __init__(
        self: Any,
        fuzz1101: datatypes.AffixFuzzer1ArrayLike,
        fuzz1102: datatypes.AffixFuzzer1ArrayLike,
        fuzz1103: datatypes.AffixFuzzer1ArrayLike,
        fuzz1104: datatypes.AffixFuzzer1ArrayLike,
        fuzz1105: datatypes.AffixFuzzer1ArrayLike,
        fuzz1106: datatypes.AffixFuzzer1ArrayLike,
        fuzz1107: components.AffixFuzzer7ArrayLike,
        fuzz1108: components.AffixFuzzer8ArrayLike,
        fuzz1109: components.AffixFuzzer9ArrayLike,
        fuzz1110: components.AffixFuzzer10ArrayLike,
        fuzz1111: components.AffixFuzzer11ArrayLike,
        fuzz1112: components.AffixFuzzer12ArrayLike,
        fuzz1113: components.AffixFuzzer13ArrayLike,
        fuzz1114: datatypes.AffixFuzzer3ArrayLike,
        fuzz1115: datatypes.AffixFuzzer3ArrayLike,
        fuzz1116: components.AffixFuzzer16ArrayLike,
        fuzz1117: components.AffixFuzzer17ArrayLike,
        fuzz1118: components.AffixFuzzer18ArrayLike,
    ):
        """Create a new instance of the AffixFuzzer2 archetype."""

        # You can define your own __init__ function as a member of AffixFuzzer2Ext in affix_fuzzer2_ext.py
        with catch_and_log_exceptions(context=self.__class__.__name__):
            self.__attrs_init__(
                fuzz1101=fuzz1101,
                fuzz1102=fuzz1102,
                fuzz1103=fuzz1103,
                fuzz1104=fuzz1104,
                fuzz1105=fuzz1105,
                fuzz1106=fuzz1106,
                fuzz1107=fuzz1107,
                fuzz1108=fuzz1108,
                fuzz1109=fuzz1109,
                fuzz1110=fuzz1110,
                fuzz1111=fuzz1111,
                fuzz1112=fuzz1112,
                fuzz1113=fuzz1113,
                fuzz1114=fuzz1114,
                fuzz1115=fuzz1115,
                fuzz1116=fuzz1116,
                fuzz1117=fuzz1117,
                fuzz1118=fuzz1118,
            )
            return
        self.__attrs_clear__()

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            fuzz1101=None,  # type: ignore[arg-type]
            fuzz1102=None,  # type: ignore[arg-type]
            fuzz1103=None,  # type: ignore[arg-type]
            fuzz1104=None,  # type: ignore[arg-type]
            fuzz1105=None,  # type: ignore[arg-type]
            fuzz1106=None,  # type: ignore[arg-type]
            fuzz1107=None,  # type: ignore[arg-type]
            fuzz1108=None,  # type: ignore[arg-type]
            fuzz1109=None,  # type: ignore[arg-type]
            fuzz1110=None,  # type: ignore[arg-type]
            fuzz1111=None,  # type: ignore[arg-type]
            fuzz1112=None,  # type: ignore[arg-type]
            fuzz1113=None,  # type: ignore[arg-type]
            fuzz1114=None,  # type: ignore[arg-type]
            fuzz1115=None,  # type: ignore[arg-type]
            fuzz1116=None,  # type: ignore[arg-type]
            fuzz1117=None,  # type: ignore[arg-type]
            fuzz1118=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> AffixFuzzer2:
        """Produce an empty AffixFuzzer2, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    fuzz1101: components.AffixFuzzer1Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer1Batch._required,  # type: ignore[misc]
    )
    fuzz1102: components.AffixFuzzer2Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer2Batch._required,  # type: ignore[misc]
    )
    fuzz1103: components.AffixFuzzer3Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer3Batch._required,  # type: ignore[misc]
    )
    fuzz1104: components.AffixFuzzer4Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer4Batch._required,  # type: ignore[misc]
    )
    fuzz1105: components.AffixFuzzer5Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer5Batch._required,  # type: ignore[misc]
    )
    fuzz1106: components.AffixFuzzer6Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer6Batch._required,  # type: ignore[misc]
    )
    fuzz1107: components.AffixFuzzer7Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer7Batch._required,  # type: ignore[misc]
    )
    fuzz1108: components.AffixFuzzer8Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer8Batch._required,  # type: ignore[misc]
    )
    fuzz1109: components.AffixFuzzer9Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer9Batch._required,  # type: ignore[misc]
    )
    fuzz1110: components.AffixFuzzer10Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer10Batch._required,  # type: ignore[misc]
    )
    fuzz1111: components.AffixFuzzer11Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer11Batch._required,  # type: ignore[misc]
    )
    fuzz1112: components.AffixFuzzer12Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer12Batch._required,  # type: ignore[misc]
    )
    fuzz1113: components.AffixFuzzer13Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer13Batch._required,  # type: ignore[misc]
    )
    fuzz1114: components.AffixFuzzer14Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer14Batch._required,  # type: ignore[misc]
    )
    fuzz1115: components.AffixFuzzer15Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer15Batch._required,  # type: ignore[misc]
    )
    fuzz1116: components.AffixFuzzer16Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer16Batch._required,  # type: ignore[misc]
    )
    fuzz1117: components.AffixFuzzer17Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer17Batch._required,  # type: ignore[misc]
    )
    fuzz1118: components.AffixFuzzer18Batch = field(
        metadata={"component": "required"},
        converter=components.AffixFuzzer18Batch._required,  # type: ignore[misc]
    )
    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__
