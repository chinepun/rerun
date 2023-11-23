// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/points2d.fbs".

#include "points2d.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {
    const char Points2D::INDICATOR_COMPONENT_NAME[] = "rerun.components.Points2DIndicator";
}

namespace rerun {

    Result<std::vector<DataCell>> AsComponents<archetypes::Points2D>::serialize(
        const archetypes::Points2D& archetype
    ) {
        using namespace archetypes;
        std::vector<DataCell> cells;
        cells.reserve(9);

        {
            auto result = rerun::components::Position2D::to_data_cell(
                archetype.positions.data(),
                archetype.positions.size()
            );
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }
        if (archetype.radii.has_value()) {
            auto result = rerun::components::Radius::to_data_cell(
                archetype.radii.value().data(),
                archetype.radii.value().size()
            );
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }
        if (archetype.colors.has_value()) {
            auto result = rerun::components::Color::to_data_cell(
                archetype.colors.value().data(),
                archetype.colors.value().size()
            );
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }
        if (archetype.labels.has_value()) {
            auto result = rerun::components::Text::to_data_cell(
                archetype.labels.value().data(),
                archetype.labels.value().size()
            );
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }
        if (archetype.draw_order.has_value()) {
            auto result =
                rerun::components::DrawOrder::to_data_cell(&archetype.draw_order.value(), 1);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }
        if (archetype.class_ids.has_value()) {
            auto result = rerun::components::ClassId::to_data_cell(
                archetype.class_ids.value().data(),
                archetype.class_ids.value().size()
            );
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }
        if (archetype.keypoint_ids.has_value()) {
            auto result = rerun::components::KeypointId::to_data_cell(
                archetype.keypoint_ids.value().data(),
                archetype.keypoint_ids.value().size()
            );
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }
        if (archetype.instance_keys.has_value()) {
            auto result = rerun::components::InstanceKey::to_data_cell(
                archetype.instance_keys.value().data(),
                archetype.instance_keys.value().size()
            );
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }
        {
            auto indicator = Points2D::IndicatorComponent();
            auto result = Points2D::IndicatorComponent::to_data_cell(&indicator, 1);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
