// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/line_strips2d.fbs".

#pragma once

#include "../collection.hpp"
#include "../components/class_id.hpp"
#include "../components/color.hpp"
#include "../components/draw_order.hpp"
#include "../components/instance_key.hpp"
#include "../components/line_strip2d.hpp"
#include "../components/radius.hpp"
#include "../components/text.hpp"
#include "../data_cell.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"
#include "../warning_macros.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: 2D line strips with positions and optional colors, radii, labels, etc.
    ///
    /// ## Example
    ///
    /// ### `line_strip2d_batch`:
    /// ![image](https://static.rerun.io/line_strip2d_batch/d8aae7ca3d6c3b0e3b636de60b8067fa2f0b6db9/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// #include <vector>
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_line_strip2d");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     std::vector<rerun::Vec2D> strip1 = {{0.f, 0.f}, {2.f, 1.f}, {4.f, -1.f}, {6.f, 0.f}};
    ///     std::vector<rerun::Vec2D> strip2 =
    ///         {{0.f, 3.f}, {1.f, 4.f}, {2.f, 2.f}, {3.f, 4.f}, {4.f, 2.f}, {5.f, 4.f}, {6.f, 3.f}};
    ///     rec.log(
    ///         "strips",
    ///         rerun::LineStrips2D({strip1, strip2})
    ///             .with_colors({0xFF0000FF, 0x00FF00FF})
    ///             .with_radii({0.025f, 0.005f})
    ///             .with_labels({"one strip here", "and one strip there"})
    ///     );
    ///
    ///     // Log an extra rect to set the view bounds
    ///     rec.log("bounds", rerun::Boxes2D::from_centers_and_sizes({{3.0f, 1.5f}}, {{8.0f, 9.0f}}));
    /// }
    /// ```
    struct LineStrips2D {
        /// All the actual 2D line strips that make up the batch.
        Collection<rerun::components::LineStrip2D> strips;

        /// Optional radii for the line strips.
        std::optional<Collection<rerun::components::Radius>> radii;

        /// Optional colors for the line strips.
        std::optional<Collection<rerun::components::Color>> colors;

        /// Optional text labels for the line strips.
        std::optional<Collection<rerun::components::Text>> labels;

        /// An optional floating point value that specifies the 2D drawing order of each line strip.
        ///
        /// Objects with higher values are drawn on top of those with lower values.
        std::optional<rerun::components::DrawOrder> draw_order;

        /// Optional `ClassId`s for the lines.
        ///
        /// The class ID provides colors and labels if not specified explicitly.
        std::optional<Collection<rerun::components::ClassId>> class_ids;

        /// Unique identifiers for each individual line strip in the batch.
        std::optional<Collection<rerun::components::InstanceKey>> instance_keys;

        /// Name of the indicator component, used to identify the archetype when converting to a list of components.
        static const char INDICATOR_COMPONENT_NAME[];
        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = components::IndicatorComponent<INDICATOR_COMPONENT_NAME>;

      public:
        LineStrips2D() = default;
        LineStrips2D(LineStrips2D&& other) = default;

        explicit LineStrips2D(Collection<rerun::components::LineStrip2D> _strips)
            : strips(std::move(_strips)) {}

        /// Optional radii for the line strips.
        LineStrips2D with_radii(Collection<rerun::components::Radius> _radii) && {
            radii = std::move(_radii);
            // See: https://github.com/rerun-io/rerun/issues/4027
            WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional colors for the line strips.
        LineStrips2D with_colors(Collection<rerun::components::Color> _colors) && {
            colors = std::move(_colors);
            // See: https://github.com/rerun-io/rerun/issues/4027
            WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional text labels for the line strips.
        LineStrips2D with_labels(Collection<rerun::components::Text> _labels) && {
            labels = std::move(_labels);
            // See: https://github.com/rerun-io/rerun/issues/4027
            WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// An optional floating point value that specifies the 2D drawing order of each line strip.
        ///
        /// Objects with higher values are drawn on top of those with lower values.
        LineStrips2D with_draw_order(rerun::components::DrawOrder _draw_order) && {
            draw_order = std::move(_draw_order);
            // See: https://github.com/rerun-io/rerun/issues/4027
            WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional `ClassId`s for the lines.
        ///
        /// The class ID provides colors and labels if not specified explicitly.
        LineStrips2D with_class_ids(Collection<rerun::components::ClassId> _class_ids) && {
            class_ids = std::move(_class_ids);
            // See: https://github.com/rerun-io/rerun/issues/4027
            WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Unique identifiers for each individual line strip in the batch.
        LineStrips2D with_instance_keys(Collection<rerun::components::InstanceKey> _instance_keys
        ) && {
            instance_keys = std::move(_instance_keys);
            // See: https://github.com/rerun-io/rerun/issues/4027
            WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Returns the number of primary instances of this archetype.
        size_t num_instances() const {
            return strips.size();
        }
    };

} // namespace rerun::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<archetypes::LineStrips2D> {
        /// Serialize all set component batches.
        static Result<std::vector<SerializedComponentBatch>> serialize(
            const archetypes::LineStrips2D& archetype
        );
    };
} // namespace rerun
