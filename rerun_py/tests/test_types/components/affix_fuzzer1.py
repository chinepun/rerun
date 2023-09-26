# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs".

# You can extend this class by creating a "AffixFuzzer1Ext" class in "affix_fuzzer1_ext.py".

from __future__ import annotations

from rerun._baseclasses import ComponentBatchMixin

from .. import datatypes

__all__ = ["AffixFuzzer1", "AffixFuzzer1Batch", "AffixFuzzer1Type"]


class AffixFuzzer1(datatypes.AffixFuzzer1):
    # You can define your own __init__ function as a member of AffixFuzzer1Ext in affix_fuzzer1_ext.py

    # Note: there are no fields here because AffixFuzzer1 delegates to datatypes.AffixFuzzer1
    pass


class AffixFuzzer1Type(datatypes.AffixFuzzer1Type):
    _TYPE_NAME: str = "rerun.testing.components.AffixFuzzer1"


class AffixFuzzer1Batch(datatypes.AffixFuzzer1Batch, ComponentBatchMixin):
    _ARROW_TYPE = AffixFuzzer1Type()


# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(AffixFuzzer1Type())
