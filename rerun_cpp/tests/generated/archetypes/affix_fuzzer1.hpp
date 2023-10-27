// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/testing/archetypes/fuzzy.fbs".

#pragma once

#include "../components/affix_fuzzer1.hpp"
#include "../components/affix_fuzzer10.hpp"
#include "../components/affix_fuzzer11.hpp"
#include "../components/affix_fuzzer12.hpp"
#include "../components/affix_fuzzer13.hpp"
#include "../components/affix_fuzzer14.hpp"
#include "../components/affix_fuzzer15.hpp"
#include "../components/affix_fuzzer16.hpp"
#include "../components/affix_fuzzer17.hpp"
#include "../components/affix_fuzzer18.hpp"
#include "../components/affix_fuzzer19.hpp"
#include "../components/affix_fuzzer2.hpp"
#include "../components/affix_fuzzer20.hpp"
#include "../components/affix_fuzzer21.hpp"
#include "../components/affix_fuzzer3.hpp"
#include "../components/affix_fuzzer4.hpp"
#include "../components/affix_fuzzer5.hpp"
#include "../components/affix_fuzzer6.hpp"
#include "../components/affix_fuzzer7.hpp"
#include "../components/affix_fuzzer8.hpp"
#include "../components/affix_fuzzer9.hpp"

#include <cstdint>
#include <rerun/component_batch.hpp>
#include <rerun/data_cell.hpp>
#include <rerun/indicator_component.hpp>
#include <rerun/result.hpp>
#include <utility>
#include <vector>

namespace rerun {
    namespace archetypes {
        struct AffixFuzzer1 {
            rerun::components::AffixFuzzer1 fuzz1001;

            rerun::components::AffixFuzzer2 fuzz1002;

            rerun::components::AffixFuzzer3 fuzz1003;

            rerun::components::AffixFuzzer4 fuzz1004;

            rerun::components::AffixFuzzer5 fuzz1005;

            rerun::components::AffixFuzzer6 fuzz1006;

            rerun::components::AffixFuzzer7 fuzz1007;

            rerun::components::AffixFuzzer8 fuzz1008;

            rerun::components::AffixFuzzer9 fuzz1009;

            rerun::components::AffixFuzzer10 fuzz1010;

            rerun::components::AffixFuzzer11 fuzz1011;

            rerun::components::AffixFuzzer12 fuzz1012;

            rerun::components::AffixFuzzer13 fuzz1013;

            rerun::components::AffixFuzzer14 fuzz1014;

            rerun::components::AffixFuzzer15 fuzz1015;

            rerun::components::AffixFuzzer16 fuzz1016;

            rerun::components::AffixFuzzer17 fuzz1017;

            rerun::components::AffixFuzzer18 fuzz1018;

            rerun::components::AffixFuzzer19 fuzz1019;

            rerun::components::AffixFuzzer20 fuzz1020;

            rerun::components::AffixFuzzer21 fuzz1021;

            /// Name of the indicator component, used to identify the archetype when converting to a list of components.
            static const char INDICATOR_COMPONENT_NAME[];
            /// Indicator component, used to identify the archetype when converting to a list of components.
            using IndicatorComponent = components::IndicatorComponent<INDICATOR_COMPONENT_NAME>;

          public:
            AffixFuzzer1() = default;
            AffixFuzzer1(AffixFuzzer1&& other) = default;

            explicit AffixFuzzer1(
                rerun::components::AffixFuzzer1 _fuzz1001,
                rerun::components::AffixFuzzer2 _fuzz1002,
                rerun::components::AffixFuzzer3 _fuzz1003,
                rerun::components::AffixFuzzer4 _fuzz1004,
                rerun::components::AffixFuzzer5 _fuzz1005,
                rerun::components::AffixFuzzer6 _fuzz1006,
                rerun::components::AffixFuzzer7 _fuzz1007,
                rerun::components::AffixFuzzer8 _fuzz1008,
                rerun::components::AffixFuzzer9 _fuzz1009,
                rerun::components::AffixFuzzer10 _fuzz1010,
                rerun::components::AffixFuzzer11 _fuzz1011,
                rerun::components::AffixFuzzer12 _fuzz1012,
                rerun::components::AffixFuzzer13 _fuzz1013,
                rerun::components::AffixFuzzer14 _fuzz1014,
                rerun::components::AffixFuzzer15 _fuzz1015,
                rerun::components::AffixFuzzer16 _fuzz1016,
                rerun::components::AffixFuzzer17 _fuzz1017,
                rerun::components::AffixFuzzer18 _fuzz1018,
                rerun::components::AffixFuzzer19 _fuzz1019,
                rerun::components::AffixFuzzer20 _fuzz1020,
                rerun::components::AffixFuzzer21 _fuzz1021
            )
                : fuzz1001(std::move(_fuzz1001)),
                  fuzz1002(std::move(_fuzz1002)),
                  fuzz1003(std::move(_fuzz1003)),
                  fuzz1004(std::move(_fuzz1004)),
                  fuzz1005(std::move(_fuzz1005)),
                  fuzz1006(std::move(_fuzz1006)),
                  fuzz1007(std::move(_fuzz1007)),
                  fuzz1008(std::move(_fuzz1008)),
                  fuzz1009(std::move(_fuzz1009)),
                  fuzz1010(std::move(_fuzz1010)),
                  fuzz1011(std::move(_fuzz1011)),
                  fuzz1012(std::move(_fuzz1012)),
                  fuzz1013(std::move(_fuzz1013)),
                  fuzz1014(std::move(_fuzz1014)),
                  fuzz1015(std::move(_fuzz1015)),
                  fuzz1016(std::move(_fuzz1016)),
                  fuzz1017(std::move(_fuzz1017)),
                  fuzz1018(std::move(_fuzz1018)),
                  fuzz1019(std::move(_fuzz1019)),
                  fuzz1020(std::move(_fuzz1020)),
                  fuzz1021(std::move(_fuzz1021)) {}

            /// Returns the number of primary instances of this archetype.
            size_t num_instances() const {
                return 1;
            }
        };

    } // namespace archetypes

    template <typename T>
    struct AsComponents;

    template <>
    struct AsComponents<archetypes::AffixFuzzer1> {
        /// Serialize all set component batches.
        static Result<std::vector<SerializedComponentBatch>> serialize(
            const archetypes::AffixFuzzer1& archetype
        );
    };
} // namespace rerun
