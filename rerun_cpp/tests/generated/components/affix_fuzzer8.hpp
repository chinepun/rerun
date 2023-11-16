// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs".

#pragma once

#include <cstdint>
#include <memory>
#include <optional>
#include <rerun/data_cell.hpp>
#include <rerun/result.hpp>

namespace arrow {
    /// \private
    template <typename T>
    class NumericBuilder;

    class DataType;
    class FloatType;
    class MemoryPool;
    using FloatBuilder = NumericBuilder<FloatType>;
} // namespace arrow

namespace rerun::components {
    struct AffixFuzzer8 {
        std::optional<float> single_float_optional;

        /// Name of the component, used for serialization.
        static const char NAME[];

      public:
        AffixFuzzer8() = default;

        AffixFuzzer8(std::optional<float> single_float_optional_)
            : single_float_optional(single_float_optional_) {}

        AffixFuzzer8& operator=(std::optional<float> single_float_optional_) {
            single_float_optional = single_float_optional_;
            return *this;
        }

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Creates a new array builder with an array of this type.
        static Result<std::shared_ptr<arrow::FloatBuilder>> new_arrow_array_builder(
            arrow::MemoryPool* memory_pool
        );

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::FloatBuilder* builder, const AffixFuzzer8* elements, size_t num_elements
        );

        /// Creates a Rerun DataCell from an array of AffixFuzzer8 components.
        static Result<rerun::DataCell> to_data_cell(
            const AffixFuzzer8* instances, size_t num_instances
        );
    };
} // namespace rerun::components
