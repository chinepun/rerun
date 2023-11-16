// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs".

#include "affix_fuzzer17.hpp"

#include "../datatypes/affix_fuzzer3.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun::components {
    const char AffixFuzzer17::NAME[] = "rerun.testing.components.AffixFuzzer17";

    const std::shared_ptr<arrow::DataType>& AffixFuzzer17::arrow_datatype() {
        static const auto datatype = arrow::list(
            arrow::field("item", rerun::datatypes::AffixFuzzer3::arrow_datatype(), false)
        );
        return datatype;
    }

    Result<std::shared_ptr<arrow::ListBuilder>> AffixFuzzer17::new_arrow_array_builder(
        arrow::MemoryPool* memory_pool
    ) {
        if (memory_pool == nullptr) {
            return rerun::Error(ErrorCode::UnexpectedNullArgument, "Memory pool is null.");
        }

        return Result(std::make_shared<arrow::ListBuilder>(
            memory_pool,
            rerun::datatypes::AffixFuzzer3::new_arrow_array_builder(memory_pool).value
        ));
    }

    rerun::Error AffixFuzzer17::fill_arrow_array_builder(
        arrow::ListBuilder* builder, const AffixFuzzer17* elements, size_t num_elements
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

        auto value_builder = static_cast<arrow::DenseUnionBuilder*>(builder->value_builder());
        ARROW_RETURN_NOT_OK(builder->Reserve(static_cast<int64_t>(num_elements)));
        ARROW_RETURN_NOT_OK(value_builder->Reserve(static_cast<int64_t>(num_elements * 1)));

        for (size_t elem_idx = 0; elem_idx < num_elements; elem_idx += 1) {
            const auto& element = elements[elem_idx];
            if (element.many_optional_unions.has_value()) {
                ARROW_RETURN_NOT_OK(builder->Append());
                if (element.many_optional_unions.value().data()) {
                    RR_RETURN_NOT_OK(rerun::datatypes::AffixFuzzer3::fill_arrow_array_builder(
                        value_builder,
                        element.many_optional_unions.value().data(),
                        element.many_optional_unions.value().size()
                    ));
                }
            } else {
                ARROW_RETURN_NOT_OK(builder->AppendNull());
            }
        }

        return Error::ok();
    }

    Result<rerun::DataCell> AffixFuzzer17::to_data_cell(
        const AffixFuzzer17* instances, size_t num_instances
    ) {
        // TODO(andreas): Allow configuring the memory pool.
        arrow::MemoryPool* pool = arrow::default_memory_pool();

        auto builder_result = AffixFuzzer17::new_arrow_array_builder(pool);
        RR_RETURN_NOT_OK(builder_result.error);
        auto builder = std::move(builder_result.value);
        if (instances && num_instances > 0) {
            RR_RETURN_NOT_OK(
                AffixFuzzer17::fill_arrow_array_builder(builder.get(), instances, num_instances)
            );
        }
        std::shared_ptr<arrow::Array> array;
        ARROW_RETURN_NOT_OK(builder->Finish(&array));

        return rerun::DataCell::create(
            AffixFuzzer17::NAME,
            AffixFuzzer17::arrow_datatype(),
            std::move(array)
        );
    }
} // namespace rerun::components
