// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs".

#pragma once

#include "../datatypes/affix_fuzzer1.hpp"

#include <cstdint>
#include <memory>
#include <optional>
#include <rerun/data_cell.hpp>
#include <rerun/result.hpp>
#include <utility>
#include <vector>

namespace arrow {
    class DataType;
    class ListBuilder;
    class MemoryPool;
} // namespace arrow

namespace rerun {
    namespace components {
        struct AffixFuzzer7 {
            std::optional<std::vector<rerun::datatypes::AffixFuzzer1>> many_optional;

            /// Name of the component, used for serialization.
            static const char NAME[];

          public:
            AffixFuzzer7() = default;

            AffixFuzzer7(std::optional<std::vector<rerun::datatypes::AffixFuzzer1>> many_optional_)
                : many_optional(std::move(many_optional_)) {}

            AffixFuzzer7& operator=(
                std::optional<std::vector<rerun::datatypes::AffixFuzzer1>> many_optional_
            ) {
                many_optional = std::move(many_optional_);
                return *this;
            }

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static Result<std::shared_ptr<arrow::ListBuilder>> new_arrow_array_builder(
                arrow::MemoryPool* memory_pool
            );

            /// Fills an arrow array builder with an array of this type.
            static rerun::Error fill_arrow_array_builder(
                arrow::ListBuilder* builder, const AffixFuzzer7* elements, size_t num_elements
            );

            /// Creates a Rerun DataCell from an array of AffixFuzzer7 components.
            static Result<rerun::DataCell> to_data_cell(
                const AffixFuzzer7* instances, size_t num_instances
            );
        };
    } // namespace components
} // namespace rerun
