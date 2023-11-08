// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/testing/datatypes/fuzzy.fbs".

#pragma once

#include <cstdint>
#include <memory>
#include <optional>
#include <rerun/result.hpp>

namespace arrow {
    template <typename T>
    class NumericBuilder;

    class DataType;
    class FloatType;
    class MemoryPool;
    using FloatBuilder = NumericBuilder<FloatType>;
} // namespace arrow

namespace rerun {
    namespace datatypes {
        struct AffixFuzzer2 {
            std::optional<float> single_float_optional;

          public:
            AffixFuzzer2() = default;

            AffixFuzzer2(std::optional<float> single_float_optional_)
                : single_float_optional(single_float_optional_) {}

            AffixFuzzer2& operator=(std::optional<float> single_float_optional_) {
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
                arrow::FloatBuilder* builder, const AffixFuzzer2* elements, size_t num_elements
            );
        };
    } // namespace datatypes
} // namespace rerun
