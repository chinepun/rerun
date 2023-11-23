// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs".

#pragma once

#include "../datatypes/affix_fuzzer4.hpp"

#include <cstdint>
#include <memory>
#include <optional>
#include <rerun/collection.hpp>
#include <rerun/data_cell.hpp>
#include <rerun/result.hpp>
#include <utility>

namespace arrow {
    class DataType;
    class ListBuilder;
} // namespace arrow

namespace rerun::components {
    struct AffixFuzzer18 {
        std::optional<rerun::Collection<rerun::datatypes::AffixFuzzer4>> many_optional_unions;

        /// Name of the component, used for serialization.
        static const char NAME[];

      public:
        AffixFuzzer18() = default;

        AffixFuzzer18(
            std::optional<rerun::Collection<rerun::datatypes::AffixFuzzer4>> many_optional_unions_
        )
            : many_optional_unions(std::move(many_optional_unions_)) {}

        AffixFuzzer18& operator=(
            std::optional<rerun::Collection<rerun::datatypes::AffixFuzzer4>> many_optional_unions_
        ) {
            many_optional_unions = std::move(many_optional_unions_);
            return *this;
        }

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::ListBuilder* builder, const AffixFuzzer18* elements, size_t num_elements
        );

        /// Creates a Rerun DataCell from an array of AffixFuzzer18 components.
        static Result<rerun::DataCell> to_data_cell(
            const AffixFuzzer18* instances, size_t num_instances
        );
    };
} // namespace rerun::components
