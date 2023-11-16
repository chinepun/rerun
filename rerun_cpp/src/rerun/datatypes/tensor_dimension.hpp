// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/datatypes/tensor_dimension.fbs".

#pragma once

#include "../result.hpp"

#include <cstdint>
#include <memory>
#include <optional>
#include <string>

namespace arrow {
    class DataType;
    class MemoryPool;
    class StructBuilder;
} // namespace arrow

namespace rerun::datatypes {
    /// **Datatype**: A single dimension within a multi-dimensional tensor.
    struct TensorDimension {
        /// The length of this dimension.
        uint64_t size;

        /// The name of this dimension, e.g. "width", "height", "channel", "batch', ….
        std::optional<std::string> name;

      public:
        // Extensions to generated type defined in 'tensor_dimension_ext.cpp'

        /// Nameless dimension.
        TensorDimension(size_t size_) : size(size_) {}

        /// Dimension with name.
        TensorDimension(size_t size_, std::string name_) : size(size_), name(std::move(name_)) {}

      public:
        TensorDimension() = default;

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Creates a new array builder with an array of this type.
        static Result<std::shared_ptr<arrow::StructBuilder>> new_arrow_array_builder(
            arrow::MemoryPool* memory_pool
        );

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::StructBuilder* builder, const TensorDimension* elements, size_t num_elements
        );
    };
} // namespace rerun::datatypes
