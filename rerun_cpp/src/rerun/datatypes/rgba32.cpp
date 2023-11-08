// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/datatypes/rgba32.fbs".

#include "rgba32.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun {
    namespace datatypes {
        const std::shared_ptr<arrow::DataType>& Rgba32::arrow_datatype() {
            static const auto datatype = arrow::uint32();
            return datatype;
        }

        Result<std::shared_ptr<arrow::UInt32Builder>> Rgba32::new_arrow_array_builder(
            arrow::MemoryPool* memory_pool
        ) {
            if (memory_pool == nullptr) {
                return rerun::Error(ErrorCode::UnexpectedNullArgument, "Memory pool is null.");
            }

            return Result(std::make_shared<arrow::UInt32Builder>(memory_pool));
        }

        rerun::Error Rgba32::fill_arrow_array_builder(
            arrow::UInt32Builder* builder, const Rgba32* elements, size_t num_elements
        ) {
            if (builder == nullptr) {
                return rerun::Error(
                    ErrorCode::UnexpectedNullArgument,
                    "Passed array builder is null."
                );
            }
            if (elements == nullptr) {
                return rerun::Error(
                    ErrorCode::UnexpectedNullArgument,
                    "Cannot serialize null pointer to arrow array."
                );
            }

            static_assert(sizeof(*elements) == sizeof(elements->rgba));
            ARROW_RETURN_NOT_OK(
                builder->AppendValues(&elements->rgba, static_cast<int64_t>(num_elements))
            );

            return Error::ok();
        }
    } // namespace datatypes
} // namespace rerun
