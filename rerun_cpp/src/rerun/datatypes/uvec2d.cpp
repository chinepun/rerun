// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/datatypes/uvec2d.fbs".

#include "uvec2d.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun {
    namespace datatypes {
        const std::shared_ptr<arrow::DataType>& UVec2D::arrow_datatype() {
            static const auto datatype =
                arrow::fixed_size_list(arrow::field("item", arrow::uint32(), false), 2);
            return datatype;
        }

        Result<std::shared_ptr<arrow::FixedSizeListBuilder>> UVec2D::new_arrow_array_builder(
            arrow::MemoryPool* memory_pool
        ) {
            if (memory_pool == nullptr) {
                return Error(ErrorCode::UnexpectedNullArgument, "Memory pool is null.");
            }

            return Result(std::make_shared<arrow::FixedSizeListBuilder>(
                memory_pool,
                std::make_shared<arrow::UInt32Builder>(memory_pool),
                2
            ));
        }

        Error UVec2D::fill_arrow_array_builder(
            arrow::FixedSizeListBuilder* builder, const UVec2D* elements, size_t num_elements
        ) {
            if (builder == nullptr) {
                return Error(ErrorCode::UnexpectedNullArgument, "Passed array builder is null.");
            }
            if (elements == nullptr) {
                return Error(
                    ErrorCode::UnexpectedNullArgument,
                    "Cannot serialize null pointer to arrow array."
                );
            }

            auto value_builder = static_cast<arrow::UInt32Builder*>(builder->value_builder());

            ARROW_RETURN_NOT_OK(builder->AppendValues(static_cast<int64_t>(num_elements)));
            static_assert(sizeof(elements[0].xy) == sizeof(elements[0]));
            ARROW_RETURN_NOT_OK(value_builder->AppendValues(
                elements[0].xy,
                static_cast<int64_t>(num_elements * 2),
                nullptr
            ));

            return Error::ok();
        }
    } // namespace datatypes
} // namespace rerun
