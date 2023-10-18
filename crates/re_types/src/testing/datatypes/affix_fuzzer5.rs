// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/testing/datatypes/fuzzy.fbs".

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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AffixFuzzer5 {
    pub single_optional_union: Option<crate::testing::datatypes::AffixFuzzer4>,
}

impl<T: Into<Option<crate::testing::datatypes::AffixFuzzer4>>> From<T> for AffixFuzzer5 {
    fn from(v: T) -> Self {
        Self {
            single_optional_union: v.into(),
        }
    }
}

impl std::borrow::Borrow<Option<crate::testing::datatypes::AffixFuzzer4>> for AffixFuzzer5 {
    #[inline]
    fn borrow(&self) -> &Option<crate::testing::datatypes::AffixFuzzer4> {
        &self.single_optional_union
    }
}

impl std::ops::Deref for AffixFuzzer5 {
    type Target = Option<crate::testing::datatypes::AffixFuzzer4>;

    #[inline]
    fn deref(&self) -> &Option<crate::testing::datatypes::AffixFuzzer4> {
        &self.single_optional_union
    }
}

impl<'a> From<AffixFuzzer5> for ::std::borrow::Cow<'a, AffixFuzzer5> {
    #[inline]
    fn from(value: AffixFuzzer5) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a AffixFuzzer5> for ::std::borrow::Cow<'a, AffixFuzzer5> {
    #[inline]
    fn from(value: &'a AffixFuzzer5) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl ::re_types_core::Loggable for AffixFuzzer5 {
    type Name = ::re_types_core::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.testing.datatypes.AffixFuzzer5".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use arrow2::datatypes::*;
        DataType::Struct(vec![Field {
            name: "single_optional_union".to_owned(),
            data_type: <crate::testing::datatypes::AffixFuzzer4>::arrow_datatype(),
            is_nullable: true,
            metadata: [].into(),
        }])
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
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            StructArray::new(
                <crate::testing::datatypes::AffixFuzzer5>::arrow_datatype(),
                vec![{
                    let (somes, single_optional_union): (Vec<_>, Vec<_>) = data
                        .iter()
                        .map(|datum| {
                            let datum = datum
                                .as_ref()
                                .map(|datum| {
                                    let Self {
                                        single_optional_union,
                                        ..
                                    } = &**datum;
                                    single_optional_union.clone()
                                })
                                .flatten();
                            (datum.is_some(), datum)
                        })
                        .unzip();
                    let single_optional_union_bitmap: Option<arrow2::bitmap::Bitmap> = {
                        let any_nones = somes.iter().any(|some| !*some);
                        any_nones.then(|| somes.into())
                    };
                    {
                        _ = single_optional_union_bitmap;
                        crate::testing::datatypes::AffixFuzzer4::to_arrow_opt(
                            single_optional_union,
                        )?
                    }
                }],
                bitmap,
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
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow2::array::StructArray>()
                .ok_or_else(|| {
                    ::re_types_core::DeserializationError::datatype_mismatch(
                        DataType::Struct(vec![Field {
                            name: "single_optional_union".to_owned(),
                            data_type: <crate::testing::datatypes::AffixFuzzer4>::arrow_datatype(),
                            is_nullable: true,
                            metadata: [].into(),
                        }]),
                        arrow_data.data_type().clone(),
                    )
                })
                .with_context("rerun.testing.datatypes.AffixFuzzer5")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_fields, arrow_data_arrays) =
                    (arrow_data.fields(), arrow_data.values());
                let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data_fields
                    .iter()
                    .map(|field| field.name.as_str())
                    .zip(arrow_data_arrays)
                    .collect();
                let single_optional_union = {
                    if !arrays_by_name.contains_key("single_optional_union") {
                        return Err(::re_types_core::DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "single_optional_union",
                        ))
                        .with_context("rerun.testing.datatypes.AffixFuzzer5");
                    }
                    let arrow_data = &**arrays_by_name["single_optional_union"];
                    crate::testing::datatypes::AffixFuzzer4::from_arrow_opt(arrow_data)
                        .with_context("rerun.testing.datatypes.AffixFuzzer5#single_optional_union")?
                        .into_iter()
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(single_optional_union),
                    arrow_data.validity(),
                )
                .map(|opt| {
                    opt.map(|(single_optional_union)| {
                        Ok(Self {
                            single_optional_union,
                        })
                    })
                    .transpose()
                })
                .collect::<::re_types_core::DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.datatypes.AffixFuzzer5")?
            }
        })
    }
}
