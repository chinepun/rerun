// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/annotation_context.fbs".

#include "annotation_context.hpp"

#include "../indicator_component.hpp"

namespace rerun {
    namespace archetypes {
        const char AnnotationContext::INDICATOR_COMPONENT_NAME[] =
            "rerun.components.AnnotationContextIndicator";

        AnonymousComponentBatch AnnotationContext::indicator() {
            return ComponentBatch<
                components::IndicatorComponent<AnnotationContext::INDICATOR_COMPONENT_NAME>>(
                nullptr,
                1
            );
        }

        std::vector<AnonymousComponentBatch> AnnotationContext::as_component_batches() const {
            std::vector<AnonymousComponentBatch> comp_batches;
            comp_batches.reserve(1);

            comp_batches.emplace_back(context);
            comp_batches.emplace_back(AnnotationContext::indicator());

            return comp_batches;
        }
    } // namespace archetypes
} // namespace rerun
