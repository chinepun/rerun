// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/datatypes/angle.fbs".

#pragma once

#include "../result.hpp"

#include <cstdint>
#include <cstring>
#include <memory>
#include <utility>

namespace arrow {
    class DataType;
    class DenseUnionBuilder;
    class MemoryPool;
} // namespace arrow

namespace rerun {
    namespace datatypes {
        namespace detail {
            enum class AngleTag : uint8_t {
                /// Having a special empty state makes it possible to implement move-semantics. We need to be able to leave the object in a state which we can run the destructor on.
                NONE = 0,
                Radians,
                Degrees,
            };

            union AngleData {
                float radians;

                float degrees;

                AngleData() {}

                ~AngleData() {}

                void swap(AngleData& other) noexcept {
                    // This bitwise swap would fail for self-referential types, but we don't have any of those.
                    char temp[sizeof(AngleData)];
                    void* otherbytes = reinterpret_cast<void*>(&other);
                    void* thisbytes = reinterpret_cast<void*>(this);
                    std::memcpy(temp, thisbytes, sizeof(AngleData));
                    std::memcpy(thisbytes, otherbytes, sizeof(AngleData));
                    std::memcpy(otherbytes, temp, sizeof(AngleData));
                }
            };
        } // namespace detail

        /// **Datatype**: Angle in either radians or degrees.
        struct Angle {
            Angle() : _tag(detail::AngleTag::NONE) {}

            /// Copy constructor
            Angle(const Angle& other) : _tag(other._tag) {
                const void* otherbytes = reinterpret_cast<const void*>(&other._data);
                void* thisbytes = reinterpret_cast<void*>(&this->_data);
                std::memcpy(thisbytes, otherbytes, sizeof(detail::AngleData));
            }

            Angle& operator=(const Angle& other) noexcept {
                Angle tmp(other);
                this->swap(tmp);
                return *this;
            }

            Angle(Angle&& other) noexcept : Angle() {
                this->swap(other);
            }

            Angle& operator=(Angle&& other) noexcept {
                this->swap(other);
                return *this;
            }

            void swap(Angle& other) noexcept {
                std::swap(this->_tag, other._tag);
                this->_data.swap(other._data);
            }

            static Angle radians(float radians) {
                Angle self;
                self._tag = detail::AngleTag::Radians;
                self._data.radians = std::move(radians);
                return self;
            }

            static Angle degrees(float degrees) {
                Angle self;
                self._tag = detail::AngleTag::Degrees;
                self._data.degrees = std::move(degrees);
                return self;
            }

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static Result<std::shared_ptr<arrow::DenseUnionBuilder>> new_arrow_array_builder(
                arrow::MemoryPool* memory_pool
            );

            /// Fills an arrow array builder with an array of this type.
            static Error fill_arrow_array_builder(
                arrow::DenseUnionBuilder* builder, const Angle* elements, size_t num_elements
            );

          private:
            detail::AngleTag _tag;
            detail::AngleData _data;
        };
    } // namespace datatypes
} // namespace rerun
