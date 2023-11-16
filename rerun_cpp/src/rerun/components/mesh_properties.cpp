// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/components/mesh_properties.fbs".

#include "mesh_properties.hpp"

#include "../datatypes/mesh_properties.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun::components {
    const char MeshProperties::NAME[] = "rerun.components.MeshProperties";

    const std::shared_ptr<arrow::DataType>& MeshProperties::arrow_datatype() {
        static const auto datatype = rerun::datatypes::MeshProperties::arrow_datatype();
        return datatype;
    }

    Result<std::shared_ptr<arrow::StructBuilder>> MeshProperties::new_arrow_array_builder(
        arrow::MemoryPool* memory_pool
    ) {
        if (memory_pool == nullptr) {
            return rerun::Error(ErrorCode::UnexpectedNullArgument, "Memory pool is null.");
        }

        return Result(rerun::datatypes::MeshProperties::new_arrow_array_builder(memory_pool).value);
    }

    rerun::Error MeshProperties::fill_arrow_array_builder(
        arrow::StructBuilder* builder, const MeshProperties* elements, size_t num_elements
    ) {
        if (builder == nullptr) {
            return rerun::Error(ErrorCode::UnexpectedNullArgument, "Passed array builder is null.");
        }
        if (elements == nullptr) {
            return rerun::Error(
                ErrorCode::UnexpectedNullArgument,
                "Cannot serialize null pointer to arrow array."
            );
        }

        static_assert(sizeof(rerun::datatypes::MeshProperties) == sizeof(MeshProperties));
        RR_RETURN_NOT_OK(rerun::datatypes::MeshProperties::fill_arrow_array_builder(
            builder,
            reinterpret_cast<const rerun::datatypes::MeshProperties*>(elements),
            num_elements
        ));

        return Error::ok();
    }

    Result<rerun::DataCell> MeshProperties::to_data_cell(
        const MeshProperties* instances, size_t num_instances
    ) {
        // TODO(andreas): Allow configuring the memory pool.
        arrow::MemoryPool* pool = arrow::default_memory_pool();

        auto builder_result = MeshProperties::new_arrow_array_builder(pool);
        RR_RETURN_NOT_OK(builder_result.error);
        auto builder = std::move(builder_result.value);
        if (instances && num_instances > 0) {
            RR_RETURN_NOT_OK(
                MeshProperties::fill_arrow_array_builder(builder.get(), instances, num_instances)
            );
        }
        std::shared_ptr<arrow::Array> array;
        ARROW_RETURN_NOT_OK(builder->Finish(&array));

        return rerun::DataCell::create(
            MeshProperties::NAME,
            MeshProperties::arrow_datatype(),
            std::move(array)
        );
    }
} // namespace rerun::components
