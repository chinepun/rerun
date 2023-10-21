// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/points3d.fbs".

#pragma once

#include "../component_batch.hpp"
#include "../components/class_id.hpp"
#include "../components/color.hpp"
#include "../components/instance_key.hpp"
#include "../components/keypoint_id.hpp"
#include "../components/position3d.hpp"
#include "../components/radius.hpp"
#include "../components/text.hpp"
#include "../data_cell.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun {
    namespace archetypes {
        /// **Archetype**: A 3D point cloud with positions and optional colors, radii, labels, etc.
        ///
        /// ## Example
        ///
        /// ### Randomly distributed 3D points with varying color and radius
        /// ```cpp,ignore
        /// #include <rerun.hpp>
        ///
        /// #include <algorithm>
        /// #include <random>
        ///
        /// int main() {
        ///     auto rec = rerun::RecordingStream("rerun_example_points3d_random");
        ///     rec.connect().throw_on_failure();
        ///
        ///     std::default_random_engine gen;
        ///     std::uniform_real_distribution<float> dist_pos(-5.0f, 5.0f);
        ///     std::uniform_real_distribution<float> dist_radius(0.1f, 1.0f);
        ///     std::uniform_int_distribution<uint8_t> dist_color(0, 255);
        ///
        ///     std::vector<rerun::components::Position3D> points3d(10);
        ///     std::generate(points3d.begin(), points3d.end(), [&] {
        ///         return rerun::components::Position3D(dist_pos(gen), dist_pos(gen), dist_pos(gen));
        ///     });
        ///     std::vector<rerun::components::Color> colors(10);
        ///     std::generate(colors.begin(), colors.end(), [&] {
        ///         return rerun::components::Color(dist_color(gen), dist_color(gen), dist_color(gen));
        ///     });
        ///     std::vector<rerun::components::Radius> radii(10);
        ///     std::generate(radii.begin(), radii.end(), [&] { return dist_radius(gen); });
        ///
        ///     rec.log("random", rerun::Points3D(points3d).with_colors(colors).with_radii(radii));
        /// }
        /// ```
        struct Points3D {
            /// All the 3D positions at which the point cloud shows points.
            ComponentBatch<rerun::components::Position3D> positions;

            /// Optional radii for the points, effectively turning them into circles.
            std::optional<ComponentBatch<rerun::components::Radius>> radii;

            /// Optional colors for the points.
            std::optional<ComponentBatch<rerun::components::Color>> colors;

            /// Optional text labels for the points.
            std::optional<ComponentBatch<rerun::components::Text>> labels;

            /// Optional class Ids for the points.
            ///
            /// The class ID provides colors and labels if not specified explicitly.
            std::optional<ComponentBatch<rerun::components::ClassId>> class_ids;

            /// Optional keypoint IDs for the points, identifying them within a class.
            ///
            /// If keypoint IDs are passed in but no class IDs were specified, the class ID will
            /// default to 0.
            /// This is useful to identify points within a single classification (which is identified
            /// with `class_id`).
            /// E.g. the classification might be 'Person' and the keypoints refer to joints on a
            /// detected skeleton.
            std::optional<ComponentBatch<rerun::components::KeypointId>> keypoint_ids;

            /// Unique identifiers for each individual point in the batch.
            std::optional<ComponentBatch<rerun::components::InstanceKey>> instance_keys;

            /// Name of the indicator component, used to identify the archetype when converting to a list of components.
            static const char INDICATOR_COMPONENT_NAME[];
            /// Indicator component, used to identify the archetype when converting to a list of components.
            using IndicatorComponent = components::IndicatorComponent<INDICATOR_COMPONENT_NAME>;

          public:
            Points3D() = default;
            Points3D(Points3D&& other) = default;

            explicit Points3D(ComponentBatch<rerun::components::Position3D> _positions)
                : positions(std::move(_positions)) {}

            /// Optional radii for the points, effectively turning them into circles.
            Points3D with_radii(ComponentBatch<rerun::components::Radius> _radii) && {
                radii = std::move(_radii);
                return std::move(*this);
            }

            /// Optional colors for the points.
            Points3D with_colors(ComponentBatch<rerun::components::Color> _colors) && {
                colors = std::move(_colors);
                return std::move(*this);
            }

            /// Optional text labels for the points.
            Points3D with_labels(ComponentBatch<rerun::components::Text> _labels) && {
                labels = std::move(_labels);
                return std::move(*this);
            }

            /// Optional class Ids for the points.
            ///
            /// The class ID provides colors and labels if not specified explicitly.
            Points3D with_class_ids(ComponentBatch<rerun::components::ClassId> _class_ids) && {
                class_ids = std::move(_class_ids);
                return std::move(*this);
            }

            /// Optional keypoint IDs for the points, identifying them within a class.
            ///
            /// If keypoint IDs are passed in but no class IDs were specified, the class ID will
            /// default to 0.
            /// This is useful to identify points within a single classification (which is identified
            /// with `class_id`).
            /// E.g. the classification might be 'Person' and the keypoints refer to joints on a
            /// detected skeleton.
            Points3D with_keypoint_ids(ComponentBatch<rerun::components::KeypointId> _keypoint_ids
            ) && {
                keypoint_ids = std::move(_keypoint_ids);
                return std::move(*this);
            }

            /// Unique identifiers for each individual point in the batch.
            Points3D with_instance_keys(
                ComponentBatch<rerun::components::InstanceKey> _instance_keys
            ) && {
                instance_keys = std::move(_instance_keys);
                return std::move(*this);
            }

            /// Returns the number of primary instances of this archetype.
            size_t num_instances() const {
                return positions.size();
            }
        };

    } // namespace archetypes

    template <typename T>
    struct AsComponents;

    template <>
    struct AsComponents<archetypes::Points3D> {
        /// Serialize all set component batches.
        static Result<std::vector<SerializedComponentBatch>> serialize(
            const archetypes::Points3D& archetype
        );
    };
} // namespace rerun
