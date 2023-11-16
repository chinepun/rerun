// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs".

#include "affix_fuzzer20.hpp"

#include "../datatypes/affix_fuzzer20.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun::components {
    const char AffixFuzzer20::NAME[] = "rerun.testing.components.AffixFuzzer20";

    const std::shared_ptr<arrow::DataType>& AffixFuzzer20::arrow_datatype() {
        static const auto datatype = rerun::datatypes::AffixFuzzer20::arrow_datatype();
        return datatype;
    }

    Result<std::shared_ptr<arrow::StructBuilder>> AffixFuzzer20::new_arrow_array_builder(
        arrow::MemoryPool* memory_pool
    ) {
        if (memory_pool == nullptr) {
            return rerun::Error(ErrorCode::UnexpectedNullArgument, "Memory pool is null.");
        }

        return Result(rerun::datatypes::AffixFuzzer20::new_arrow_array_builder(memory_pool).value);
    }

    rerun::Error AffixFuzzer20::fill_arrow_array_builder(
        arrow::StructBuilder* builder, const AffixFuzzer20* elements, size_t num_elements
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

        static_assert(sizeof(rerun::datatypes::AffixFuzzer20) == sizeof(AffixFuzzer20));
        RR_RETURN_NOT_OK(rerun::datatypes::AffixFuzzer20::fill_arrow_array_builder(
            builder,
            reinterpret_cast<const rerun::datatypes::AffixFuzzer20*>(elements),
            num_elements
        ));

        return Error::ok();
    }

    Result<rerun::DataCell> AffixFuzzer20::to_data_cell(
        const AffixFuzzer20* instances, size_t num_instances
    ) {
        // TODO(andreas): Allow configuring the memory pool.
        arrow::MemoryPool* pool = arrow::default_memory_pool();

        auto builder_result = AffixFuzzer20::new_arrow_array_builder(pool);
        RR_RETURN_NOT_OK(builder_result.error);
        auto builder = std::move(builder_result.value);
        if (instances && num_instances > 0) {
            RR_RETURN_NOT_OK(
                AffixFuzzer20::fill_arrow_array_builder(builder.get(), instances, num_instances)
            );
        }
        std::shared_ptr<arrow::Array> array;
        ARROW_RETURN_NOT_OK(builder->Finish(&array));

        return rerun::DataCell::create(
            AffixFuzzer20::NAME,
            AffixFuzzer20::arrow_datatype(),
            std::move(array)
        );
    }
} // namespace rerun::components
