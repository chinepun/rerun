// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/clear.fbs".

#pragma once

#include "../collection.hpp"
#include "../components/clear_is_recursive.hpp"
#include "../data_cell.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: Empties all the components of an entity.
    ///
    /// ## Example
    ///
    /// ### Flat
    /// ![image](https://static.rerun.io/clear_simple/2f5df95fcc53e9f0552f65670aef7f94830c5c1a/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// #include <cmath>
    /// #include <numeric>
    /// #include <string> // to_string
    /// #include <vector>
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_clear_simple");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     std::vector<rerun::Vector3D> vectors = {
    ///         {1.0, 0.0, 0.0},
    ///         {0.0, -1.0, 0.0},
    ///         {-1.0, 0.0, 0.0},
    ///         {0.0, 1.0, 0.0},
    ///     };
    ///     std::vector<rerun::Position3D> origins = {
    ///         {-0.5, 0.5, 0.0},
    ///         {0.5, 0.5, 0.0},
    ///         {0.5, -0.5, 0.0},
    ///         {-0.5, -0.5, 0.0},
    ///     };
    ///     std::vector<rerun::Color> colors = {
    ///         {200, 0, 0},
    ///         {0, 200, 0},
    ///         {0, 0, 200},
    ///         {200, 0, 200},
    ///     };
    ///
    ///     // Log a handful of arrows.
    ///     for (size_t i = 0; i <vectors.size(); ++i) {
    ///         auto entity_path = "arrows/" + std::to_string(i);
    ///         rec.log(
    ///             entity_path,
    ///             rerun::Arrows3D::from_vectors(vectors[i])
    ///                 .with_origins(origins[i])
    ///                 .with_colors(colors[i])
    ///         );
    ///     }
    ///
    ///     // Now clear them, one by one on each tick.
    ///     for (size_t i = 0; i <vectors.size(); ++i) {
    ///         auto entity_path = "arrows/" + std::to_string(i);
    ///         rec.log(entity_path, rerun::Clear::FLAT);
    ///     }
    /// }
    /// ```
    struct Clear {
        rerun::components::ClearIsRecursive is_recursive;

        /// Name of the indicator component, used to identify the archetype when converting to a list of components.
        static const char INDICATOR_COMPONENT_NAME[];
        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = components::IndicatorComponent<INDICATOR_COMPONENT_NAME>;

      public:
        // Extensions to generated type defined in 'clear_ext.cpp'

        static const Clear FLAT;

        static const Clear RECURSIVE;

        Clear(bool _is_recursive = false) : Clear(components::ClearIsRecursive(_is_recursive)) {}

      public:
        Clear() = default;
        Clear(Clear&& other) = default;

        explicit Clear(rerun::components::ClearIsRecursive _is_recursive)
            : is_recursive(std::move(_is_recursive)) {}

        /// Returns the number of primary instances of this archetype.
        size_t num_instances() const {
            return 1;
        }
    };

} // namespace rerun::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<archetypes::Clear> {
        /// Serialize all set component batches.
        static Result<std::vector<SerializedComponentBatch>> serialize(
            const archetypes::Clear& archetype
        );
    };
} // namespace rerun
