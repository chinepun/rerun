// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/archetypes/time_series_scalar.fbs".

#pragma once

#include "../component_batch.hpp"
#include "../components/color.hpp"
#include "../components/radius.hpp"
#include "../components/scalar.hpp"
#include "../components/scalar_scattering.hpp"
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
        /// **Archetype**: Log a double-precision scalar that will be visualized as a time-series plot.
        ///
        /// The current simulation time will be used for the time/X-axis, hence scalars
        /// cannot be timeless!
        ///
        /// ## Example
        ///
        /// ### Simple line plot
        /// ```cpp,ignore
        /// #include <rerun.hpp>
        ///
        /// #include <cmath>
        ///
        /// int main() {
        ///     auto rec = rerun::RecordingStream("rerun_example_scalar");
        ///     rec.connect("127.0.0.1:9876").throw_on_failure();
        ///
        ///     for (int step = 0; step <64; ++step) {
        ///         rec.set_time_sequence("step", step);
        ///         rec.log("scalar", rerun::TimeSeriesScalar(std::sin(static_cast<double>(step) / 10.0)));
        ///     }
        /// }
        /// ```
        struct TimeSeriesScalar {
            /// The scalar value to log.
            rerun::components::Scalar scalar;

            /// An optional radius for the point.
            ///
            /// Points within a single line do not have to share the same radius, the line
            /// will have differently sized segments as appropriate.
            ///
            /// If all points within a single entity path (i.e. a line) share the same
            /// radius, then this radius will be used as the line width too. Otherwise, the
            /// line will use the default width of `1.0`.
            std::optional<rerun::components::Radius> radius;

            /// Optional color for the scalar entry.
            ///
            /// If left unspecified, a pseudo-random color will be used instead. That
            /// same color will apply to all points residing in the same entity path
            /// that don't have a color specified.
            ///
            /// Points within a single line do not have to share the same color, the line
            /// will have differently colored segments as appropriate.
            /// If all points within a single entity path (i.e. a line) share the same
            /// color, then this color will be used as the line color in the plot legend.
            /// Otherwise, the line will appear gray in the legend.
            std::optional<rerun::components::Color> color;

            /// An optional label for the point.
            ///
            /// TODO(#1289): This won't show up on points at the moment, as our plots don't yet
            /// support displaying labels for individual points.
            /// If all points within a single entity path (i.e. a line) share the same label, then
            /// this label will be used as the label for the line itself. Otherwise, the
            /// line will be named after the entity path. The plot itself is named after
            /// the space it's in.
            std::optional<rerun::components::Text> label;

            /// Specifies whether a point in a scatter plot should form a continuous line.
            ///
            /// If set to true, this scalar will be drawn as a point, akin to a scatterplot.
            /// Otherwise, it will form a continuous line with its neighbors.
            /// Points within a single line do not have to all share the same scatteredness:
            /// the line will switch between a scattered and a continuous representation as
            /// required.
            std::optional<rerun::components::ScalarScattering> scattered;

            /// Name of the indicator component, used to identify the archetype when converting to a list of components.
            static const char INDICATOR_COMPONENT_NAME[];
            /// Indicator component, used to identify the archetype when converting to a list of components.
            using IndicatorComponent = components::IndicatorComponent<INDICATOR_COMPONENT_NAME>;

          public:
            TimeSeriesScalar() = default;
            TimeSeriesScalar(TimeSeriesScalar&& other) = default;

            explicit TimeSeriesScalar(rerun::components::Scalar _scalar)
                : scalar(std::move(_scalar)) {}

            /// An optional radius for the point.
            ///
            /// Points within a single line do not have to share the same radius, the line
            /// will have differently sized segments as appropriate.
            ///
            /// If all points within a single entity path (i.e. a line) share the same
            /// radius, then this radius will be used as the line width too. Otherwise, the
            /// line will use the default width of `1.0`.
            TimeSeriesScalar with_radius(rerun::components::Radius _radius) && {
                radius = std::move(_radius);
                return std::move(*this);
            }

            /// Optional color for the scalar entry.
            ///
            /// If left unspecified, a pseudo-random color will be used instead. That
            /// same color will apply to all points residing in the same entity path
            /// that don't have a color specified.
            ///
            /// Points within a single line do not have to share the same color, the line
            /// will have differently colored segments as appropriate.
            /// If all points within a single entity path (i.e. a line) share the same
            /// color, then this color will be used as the line color in the plot legend.
            /// Otherwise, the line will appear gray in the legend.
            TimeSeriesScalar with_color(rerun::components::Color _color) && {
                color = std::move(_color);
                return std::move(*this);
            }

            /// An optional label for the point.
            ///
            /// TODO(#1289): This won't show up on points at the moment, as our plots don't yet
            /// support displaying labels for individual points.
            /// If all points within a single entity path (i.e. a line) share the same label, then
            /// this label will be used as the label for the line itself. Otherwise, the
            /// line will be named after the entity path. The plot itself is named after
            /// the space it's in.
            TimeSeriesScalar with_label(rerun::components::Text _label) && {
                label = std::move(_label);
                return std::move(*this);
            }

            /// Specifies whether a point in a scatter plot should form a continuous line.
            ///
            /// If set to true, this scalar will be drawn as a point, akin to a scatterplot.
            /// Otherwise, it will form a continuous line with its neighbors.
            /// Points within a single line do not have to all share the same scatteredness:
            /// the line will switch between a scattered and a continuous representation as
            /// required.
            TimeSeriesScalar with_scattered(rerun::components::ScalarScattering _scattered) && {
                scattered = std::move(_scattered);
                return std::move(*this);
            }

            /// Returns the number of primary instances of this archetype.
            size_t num_instances() const {
                return 1;
            }
        };

    } // namespace archetypes

    template <typename T>
    struct AsComponents;

    template <>
    struct AsComponents<archetypes::TimeSeriesScalar> {
        /// Serialize all set component batches.
        static Result<std::vector<SerializedComponentBatch>> serialize(
            const archetypes::TimeSeriesScalar& archetype
        );
    };
} // namespace rerun
