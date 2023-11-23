// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/view_coordinates.fbs".

#include "view_coordinates.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {
    const char ViewCoordinates::INDICATOR_COMPONENT_NAME[] =
        "rerun.components.ViewCoordinatesIndicator";
}

namespace rerun {

    Result<std::vector<DataCell>> AsComponents<archetypes::ViewCoordinates>::serialize(
        const archetypes::ViewCoordinates& archetype
    ) {
        using namespace archetypes;
        std::vector<DataCell> cells;
        cells.reserve(2);

        {
            auto result = rerun::components::ViewCoordinates::to_data_cell(&archetype.xyz, 1);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }
        {
            auto indicator = ViewCoordinates::IndicatorComponent();
            auto result = ViewCoordinates::IndicatorComponent::to_data_cell(&indicator, 1);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
