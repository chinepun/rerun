// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/components/scalar_scattering.fbs".

#include "scalar_scattering.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun {
    namespace components {
        const char ScalarScattering::NAME[] = "rerun.components.ScalarScattering";

        const std::shared_ptr<arrow::DataType>& ScalarScattering::arrow_datatype() {
            static const auto datatype = arrow::boolean();
            return datatype;
        }

        Result<std::shared_ptr<arrow::BooleanBuilder>> ScalarScattering::new_arrow_array_builder(
            arrow::MemoryPool* memory_pool
        ) {
            if (memory_pool == nullptr) {
                return Error(ErrorCode::UnexpectedNullArgument, "Memory pool is null.");
            }

            return Result(std::make_shared<arrow::BooleanBuilder>(memory_pool));
        }

        Error ScalarScattering::fill_arrow_array_builder(
            arrow::BooleanBuilder* builder, const ScalarScattering* elements, size_t num_elements
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

            static_assert(sizeof(*elements) == sizeof(elements->scattered));
            ARROW_RETURN_NOT_OK(builder->AppendValues(
                reinterpret_cast<const uint8_t*>(&elements->scattered),
                static_cast<int64_t>(num_elements)
            ));

            return Error::ok();
        }

        Result<rerun::DataCell> ScalarScattering::to_data_cell(
            const ScalarScattering* instances, size_t num_instances
        ) {
            // TODO(andreas): Allow configuring the memory pool.
            arrow::MemoryPool* pool = arrow::default_memory_pool();

            auto builder_result = ScalarScattering::new_arrow_array_builder(pool);
            RR_RETURN_NOT_OK(builder_result.error);
            auto builder = std::move(builder_result.value);
            if (instances && num_instances > 0) {
                RR_RETURN_NOT_OK(ScalarScattering::fill_arrow_array_builder(
                    builder.get(),
                    instances,
                    num_instances
                ));
            }
            std::shared_ptr<arrow::Array> array;
            ARROW_RETURN_NOT_OK(builder->Finish(&array));

            return rerun::DataCell::create(
                ScalarScattering::NAME,
                ScalarScattering::arrow_datatype(),
                std::move(array)
            );
        }
    } // namespace components
} // namespace rerun
