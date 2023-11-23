// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/components/view_coordinates.fbs".

#include "view_coordinates.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun::components {
    const char ViewCoordinates::NAME[] = "rerun.components.ViewCoordinates";

    const std::shared_ptr<arrow::DataType>& ViewCoordinates::arrow_datatype() {
        static const auto datatype =
            arrow::fixed_size_list(arrow::field("item", arrow::uint8(), false), 3);
        return datatype;
    }

    rerun::Error ViewCoordinates::fill_arrow_array_builder(
        arrow::FixedSizeListBuilder* builder, const ViewCoordinates* elements, size_t num_elements
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

        auto value_builder = static_cast<arrow::UInt8Builder*>(builder->value_builder());

        ARROW_RETURN_NOT_OK(builder->AppendValues(static_cast<int64_t>(num_elements)));
        static_assert(sizeof(elements[0].coordinates) == sizeof(elements[0]));
        ARROW_RETURN_NOT_OK(value_builder->AppendValues(
            elements[0].coordinates.data(),
            static_cast<int64_t>(num_elements * 3),
            nullptr
        ));

        return Error::ok();
    }

    Result<rerun::DataCell> ViewCoordinates::to_data_cell(
        const ViewCoordinates* instances, size_t num_instances
    ) {
        // TODO(andreas): Allow configuring the memory pool.
        arrow::MemoryPool* pool = arrow::default_memory_pool();
        auto datatype = arrow_datatype();

        ARROW_ASSIGN_OR_RAISE(auto builder, arrow::MakeBuilder(datatype, pool))
        if (instances && num_instances > 0) {
            RR_RETURN_NOT_OK(ViewCoordinates::fill_arrow_array_builder(
                static_cast<arrow::FixedSizeListBuilder*>(builder.get()),
                instances,
                num_instances
            ));
        }
        std::shared_ptr<arrow::Array> array;
        ARROW_RETURN_NOT_OK(builder->Finish(&array));

        static const Result<ComponentTypeHandle> component_type =
            ComponentType(NAME, datatype).register_component();
        RR_RETURN_NOT_OK(component_type.error);

        DataCell cell;
        cell.num_instances = num_instances;
        cell.array = std::move(array);
        cell.component_type = component_type.value;
        return cell;
    }
} // namespace rerun::components
