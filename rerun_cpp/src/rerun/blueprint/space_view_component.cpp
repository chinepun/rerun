// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/blueprint/space_view_component.fbs".

#include "space_view_component.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun {
    namespace blueprint {
        const std::shared_ptr<arrow::DataType>& SpaceViewComponent::arrow_datatype() {
            static const auto datatype = arrow::struct_({
                arrow::field(
                    "space_view",
                    arrow::list(arrow::field("item", arrow::uint8(), false)),
                    false
                ),
            });
            return datatype;
        }

        Result<std::shared_ptr<arrow::StructBuilder>> SpaceViewComponent::new_arrow_array_builder(
            arrow::MemoryPool* memory_pool
        ) {
            if (memory_pool == nullptr) {
                return Error(ErrorCode::UnexpectedNullArgument, "Memory pool is null.");
            }

            return Result(std::make_shared<arrow::StructBuilder>(
                arrow_datatype(),
                memory_pool,
                std::vector<std::shared_ptr<arrow::ArrayBuilder>>({
                    std::make_shared<arrow::ListBuilder>(
                        memory_pool,
                        std::make_shared<arrow::UInt8Builder>(memory_pool)
                    ),
                })
            ));
        }

        Error SpaceViewComponent::fill_arrow_array_builder(
            arrow::StructBuilder* builder, const SpaceViewComponent* elements, size_t num_elements
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

            {
                auto field_builder = static_cast<arrow::ListBuilder*>(builder->field_builder(0));
                auto value_builder =
                    static_cast<arrow::UInt8Builder*>(field_builder->value_builder());
                ARROW_RETURN_NOT_OK(field_builder->Reserve(static_cast<int64_t>(num_elements)));
                ARROW_RETURN_NOT_OK(value_builder->Reserve(static_cast<int64_t>(num_elements * 2)));

                for (size_t elem_idx = 0; elem_idx < num_elements; elem_idx += 1) {
                    const auto& element = elements[elem_idx];
                    ARROW_RETURN_NOT_OK(field_builder->Append());
                    ARROW_RETURN_NOT_OK(value_builder->AppendValues(
                        element.space_view.data(),
                        static_cast<int64_t>(element.space_view.size()),
                        nullptr
                    ));
                }
            }
            ARROW_RETURN_NOT_OK(builder->AppendValues(static_cast<int64_t>(num_elements), nullptr));

            return Error::ok();
        }
    } // namespace blueprint
} // namespace rerun
