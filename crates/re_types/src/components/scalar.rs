// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/components/scalar.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

use ::re_types_core::external::arrow2;

/// **Component**: A double-precision scalar.
///
/// Used for time series plots.
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(transparent)]
pub struct Scalar(pub f64);

impl From<f64> for Scalar {
    #[inline]
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<Scalar> for f64 {
    #[inline]
    fn from(value: Scalar) -> Self {
        value.0
    }
}

impl<'a> From<Scalar> for ::std::borrow::Cow<'a, Scalar> {
    #[inline]
    fn from(value: Scalar) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a Scalar> for ::std::borrow::Cow<'a, Scalar> {
    #[inline]
    fn from(value: &'a Scalar) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl ::re_types_core::Loggable for Scalar {
    type Name = ::re_types_core::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.components.Scalar".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use arrow2::datatypes::*;
        DataType::Float64
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> ::re_types_core::SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| {
                        let Self(data0) = datum.into_owned();
                        data0
                    });
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            PrimitiveArray::new(
                Self::arrow_datatype(),
                data0.into_iter().map(|v| v.unwrap_or_default()).collect(),
                data0_bitmap,
            )
            .boxed()
        })
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> ::re_types_core::DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        Ok(arrow_data
            .as_any()
            .downcast_ref::<Float64Array>()
            .ok_or_else(|| {
                ::re_types_core::DeserializationError::datatype_mismatch(
                    DataType::Float64,
                    arrow_data.data_type().clone(),
                )
            })
            .with_context("rerun.components.Scalar#value")?
            .into_iter()
            .map(|opt| opt.copied())
            .map(|v| v.ok_or_else(::re_types_core::DeserializationError::missing_data))
            .map(|res| res.map(|v| Some(Self(v))))
            .collect::<::re_types_core::DeserializationResult<Vec<Option<_>>>>()
            .with_context("rerun.components.Scalar#value")
            .with_context("rerun.components.Scalar")?)
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn from_arrow(
        arrow_data: &dyn arrow2::array::Array,
    ) -> ::re_types_core::DeserializationResult<Vec<Self>>
    where
        Self: Sized,
    {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        if let Some(validity) = arrow_data.validity() {
            if validity.unset_bits() != 0 {
                return Err(::re_types_core::DeserializationError::missing_data());
            }
        }
        Ok({
            let slice = arrow_data
                .as_any()
                .downcast_ref::<Float64Array>()
                .ok_or_else(|| {
                    ::re_types_core::DeserializationError::datatype_mismatch(
                        DataType::Float64,
                        arrow_data.data_type().clone(),
                    )
                })
                .with_context("rerun.components.Scalar#value")?
                .values()
                .as_slice();
            {
                re_tracing::profile_scope!("collect");
                slice.iter().copied().map(|v| Self(v)).collect::<Vec<_>>()
            }
        })
    }
}
