// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/arrows3d.fbs".

#include "arrows3d.hpp"

#include "../indicator_component.hpp"

namespace rerun {
    namespace archetypes {
        const char Arrows3D::INDICATOR_COMPONENT_NAME[] = "rerun.components.Arrows3DIndicator";

        AnonymousComponentBatch Arrows3D::indicator() {
            return ComponentBatch<
                components::IndicatorComponent<Arrows3D::INDICATOR_COMPONENT_NAME>>(nullptr, 1);
        }

        std::vector<AnonymousComponentBatch> Arrows3D::as_component_batches() const {
            std::vector<AnonymousComponentBatch> comp_batches;
            comp_batches.reserve(7);

            comp_batches.emplace_back(vectors);
            if (origins.has_value()) {
                comp_batches.emplace_back(origins.value());
            }
            if (radii.has_value()) {
                comp_batches.emplace_back(radii.value());
            }
            if (colors.has_value()) {
                comp_batches.emplace_back(colors.value());
            }
            if (labels.has_value()) {
                comp_batches.emplace_back(labels.value());
            }
            if (class_ids.has_value()) {
                comp_batches.emplace_back(class_ids.value());
            }
            if (instance_keys.has_value()) {
                comp_batches.emplace_back(instance_keys.value());
            }
            comp_batches.emplace_back(Arrows3D::indicator());

            return comp_batches;
        }
    } // namespace archetypes
} // namespace rerun
