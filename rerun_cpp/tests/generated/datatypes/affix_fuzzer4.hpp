// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/testing/datatypes/fuzzy.fbs".

#pragma once

#include "affix_fuzzer3.hpp"

#include <cstdint>
#include <cstring>
#include <memory>
#include <new>
#include <optional>
#include <rerun/result.hpp>
#include <utility>
#include <vector>

namespace arrow {
    class DataType;
    class DenseUnionBuilder;
    class MemoryPool;
} // namespace arrow

namespace rerun {
    namespace datatypes {
        namespace detail {
            enum class AffixFuzzer4Tag : uint8_t {
                /// Having a special empty state makes it possible to implement move-semantics. We need to be able to leave the object in a state which we can run the destructor on.
                None = 0,
                single_required,
                many_required,
                many_optional,
            };

            union AffixFuzzer4Data {
                rerun::datatypes::AffixFuzzer3 single_required;

                std::vector<rerun::datatypes::AffixFuzzer3> many_required;

                std::optional<std::vector<rerun::datatypes::AffixFuzzer3>> many_optional;

                AffixFuzzer4Data() {
                    std::memset(reinterpret_cast<void*>(this), 0, sizeof(AffixFuzzer4Data));
                }

                ~AffixFuzzer4Data() {}

                void swap(AffixFuzzer4Data& other) noexcept {
                    // This bitwise swap would fail for self-referential types, but we don't have any of those.
                    char temp[sizeof(AffixFuzzer4Data)];
                    void* otherbytes = reinterpret_cast<void*>(&other);
                    void* thisbytes = reinterpret_cast<void*>(this);
                    std::memcpy(temp, thisbytes, sizeof(AffixFuzzer4Data));
                    std::memcpy(thisbytes, otherbytes, sizeof(AffixFuzzer4Data));
                    std::memcpy(otherbytes, temp, sizeof(AffixFuzzer4Data));
                }
            };
        } // namespace detail

        struct AffixFuzzer4 {
            AffixFuzzer4() : _tag(detail::AffixFuzzer4Tag::None) {}

            /// Copy constructor
            AffixFuzzer4(const AffixFuzzer4& other) : _tag(other._tag) {
                switch (other._tag) {
                    case detail::AffixFuzzer4Tag::single_required: {
                        using TypeAlias = rerun::datatypes::AffixFuzzer3;
                        new (&_data.single_required) TypeAlias(other._data.single_required);
                    } break;
                    case detail::AffixFuzzer4Tag::many_required: {
                        using TypeAlias = std::vector<rerun::datatypes::AffixFuzzer3>;
                        new (&_data.many_required) TypeAlias(other._data.many_required);
                    } break;
                    case detail::AffixFuzzer4Tag::many_optional: {
                        using TypeAlias =
                            std::optional<std::vector<rerun::datatypes::AffixFuzzer3>>;
                        new (&_data.many_optional) TypeAlias(other._data.many_optional);
                    } break;
                    case detail::AffixFuzzer4Tag::None: {
                    } break;
                }
            }

            AffixFuzzer4& operator=(const AffixFuzzer4& other) noexcept {
                AffixFuzzer4 tmp(other);
                this->swap(tmp);
                return *this;
            }

            AffixFuzzer4(AffixFuzzer4&& other) noexcept : AffixFuzzer4() {
                this->swap(other);
            }

            AffixFuzzer4& operator=(AffixFuzzer4&& other) noexcept {
                this->swap(other);
                return *this;
            }

            ~AffixFuzzer4() {
                switch (this->_tag) {
                    case detail::AffixFuzzer4Tag::None: {
                        // Nothing to destroy
                    } break;
                    case detail::AffixFuzzer4Tag::single_required: {
                        using TypeAlias = rerun::datatypes::AffixFuzzer3;
                        _data.single_required.~TypeAlias();
                    } break;
                    case detail::AffixFuzzer4Tag::many_required: {
                        using TypeAlias = std::vector<rerun::datatypes::AffixFuzzer3>;
                        _data.many_required.~TypeAlias();
                    } break;
                    case detail::AffixFuzzer4Tag::many_optional: {
                        using TypeAlias =
                            std::optional<std::vector<rerun::datatypes::AffixFuzzer3>>;
                        _data.many_optional.~TypeAlias();
                    } break;
                }
            }

            void swap(AffixFuzzer4& other) noexcept {
                std::swap(this->_tag, other._tag);
                this->_data.swap(other._data);
            }

            static AffixFuzzer4 single_required(rerun::datatypes::AffixFuzzer3 single_required) {
                AffixFuzzer4 self;
                self._tag = detail::AffixFuzzer4Tag::single_required;
                new (&self._data.single_required)
                    rerun::datatypes::AffixFuzzer3(std::move(single_required));
                return self;
            }

            static AffixFuzzer4 many_required(
                std::vector<rerun::datatypes::AffixFuzzer3> many_required
            ) {
                AffixFuzzer4 self;
                self._tag = detail::AffixFuzzer4Tag::many_required;
                new (&self._data.many_required)
                    std::vector<rerun::datatypes::AffixFuzzer3>(std::move(many_required));
                return self;
            }

            static AffixFuzzer4 many_optional(
                std::optional<std::vector<rerun::datatypes::AffixFuzzer3>> many_optional
            ) {
                AffixFuzzer4 self;
                self._tag = detail::AffixFuzzer4Tag::many_optional;
                new (&self._data.many_optional
                ) std::optional<std::vector<rerun::datatypes::AffixFuzzer3>>(std::move(many_optional
                ));
                return self;
            }

            /// Return a pointer to single_required if the union is in that state, otherwise `nullptr`.
            const rerun::datatypes::AffixFuzzer3* get_single_required() const {
                if (_tag == detail::AffixFuzzer4Tag::single_required) {
                    return &_data.single_required;
                } else {
                    return nullptr;
                }
            }

            /// Return a pointer to many_required if the union is in that state, otherwise `nullptr`.
            const std::vector<rerun::datatypes::AffixFuzzer3>* get_many_required() const {
                if (_tag == detail::AffixFuzzer4Tag::many_required) {
                    return &_data.many_required;
                } else {
                    return nullptr;
                }
            }

            /// Return a pointer to many_optional if the union is in that state, otherwise `nullptr`.
            const std::optional<std::vector<rerun::datatypes::AffixFuzzer3>>* get_many_optional(
            ) const {
                if (_tag == detail::AffixFuzzer4Tag::many_optional) {
                    return &_data.many_optional;
                } else {
                    return nullptr;
                }
            }

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static Result<std::shared_ptr<arrow::DenseUnionBuilder>> new_arrow_array_builder(
                arrow::MemoryPool* memory_pool
            );

            /// Fills an arrow array builder with an array of this type.
            static rerun::Error fill_arrow_array_builder(
                arrow::DenseUnionBuilder* builder, const AffixFuzzer4* elements, size_t num_elements
            );

          private:
            detail::AffixFuzzer4Tag _tag;
            detail::AffixFuzzer4Data _data;
        };
    } // namespace datatypes
} // namespace rerun
