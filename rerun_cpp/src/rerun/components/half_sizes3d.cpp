// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/components/half_sizes3d.fbs".

#include "half_sizes3d.hpp"

#include "../datatypes/vec3d.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun {
    namespace components {
        const char HalfSizes3D::NAME[] = "rerun.components.HalfSizes3D";

        const std::shared_ptr<arrow::DataType>& HalfSizes3D::arrow_datatype() {
            static const auto datatype = rerun::datatypes::Vec3D::arrow_datatype();
            return datatype;
        }

        Result<std::shared_ptr<arrow::FixedSizeListBuilder>> HalfSizes3D::new_arrow_array_builder(
            arrow::MemoryPool* memory_pool
        ) {
            if (memory_pool == nullptr) {
                return rerun::Error(ErrorCode::UnexpectedNullArgument, "Memory pool is null.");
            }

            return Result(rerun::datatypes::Vec3D::new_arrow_array_builder(memory_pool).value);
        }

        rerun::Error HalfSizes3D::fill_arrow_array_builder(
            arrow::FixedSizeListBuilder* builder, const HalfSizes3D* elements, size_t num_elements
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

            static_assert(sizeof(rerun::datatypes::Vec3D) == sizeof(HalfSizes3D));
            RR_RETURN_NOT_OK(rerun::datatypes::Vec3D::fill_arrow_array_builder(
                builder,
                reinterpret_cast<const rerun::datatypes::Vec3D*>(elements),
                num_elements
            ));

            return Error::ok();
        }

        Result<rerun::DataCell> HalfSizes3D::to_data_cell(
            const HalfSizes3D* instances, size_t num_instances
        ) {
            // TODO(andreas): Allow configuring the memory pool.
            arrow::MemoryPool* pool = arrow::default_memory_pool();

            auto builder_result = HalfSizes3D::new_arrow_array_builder(pool);
            RR_RETURN_NOT_OK(builder_result.error);
            auto builder = std::move(builder_result.value);
            if (instances && num_instances > 0) {
                RR_RETURN_NOT_OK(
                    HalfSizes3D::fill_arrow_array_builder(builder.get(), instances, num_instances)
                );
            }
            std::shared_ptr<arrow::Array> array;
            ARROW_RETURN_NOT_OK(builder->Finish(&array));

            return rerun::DataCell::create(
                HalfSizes3D::NAME,
                HalfSizes3D::arrow_datatype(),
                std::move(array)
            );
        }
    } // namespace components
} // namespace rerun
