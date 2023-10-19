// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/blueprint/panel_view.fbs".

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

/// **Blueprint**: The state of the panels.
///
/// Unstable. Used for the ongoing blueprint experimentations.
#[derive(Clone, Debug, Copy)]
pub struct PanelView {
    pub is_expanded: bool,
}

impl From<bool> for PanelView {
    #[inline]
    fn from(is_expanded: bool) -> Self {
        Self { is_expanded }
    }
}

impl From<PanelView> for bool {
    #[inline]
    fn from(value: PanelView) -> Self {
        value.is_expanded
    }
}

impl<'a> From<PanelView> for ::std::borrow::Cow<'a, PanelView> {
    #[inline]
    fn from(value: PanelView) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a PanelView> for ::std::borrow::Cow<'a, PanelView> {
    #[inline]
    fn from(value: &'a PanelView) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl ::re_types_core::Loggable for PanelView {
    type Name = ::re_types_core::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.blueprint.PanelView".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use arrow2::datatypes::*;
        DataType::Struct(vec![Field {
            name: "is_expanded".to_owned(),
            data_type: DataType::Boolean,
            is_nullable: false,
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
                <crate::blueprint::PanelView>::arrow_datatype(),
                vec![{
                    let (somes, is_expanded): (Vec<_>, Vec<_>) = data
                        .iter()
                        .map(|datum| {
                            let datum = datum.as_ref().map(|datum| {
                                let Self { is_expanded, .. } = &**datum;
                                is_expanded.clone()
                            });
                            (datum.is_some(), datum)
                        })
                        .unzip();
                    let is_expanded_bitmap: Option<arrow2::bitmap::Bitmap> = {
                        let any_nones = somes.iter().any(|some| !*some);
                        any_nones.then(|| somes.into())
                    };
                    BooleanArray::new(
                        DataType::Boolean,
                        is_expanded
                            .into_iter()
                            .map(|v| v.unwrap_or_default())
                            .collect(),
                        is_expanded_bitmap,
                    )
                    .boxed()
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
                            name: "is_expanded".to_owned(),
                            data_type: DataType::Boolean,
                            is_nullable: false,
                            metadata: [].into(),
                        }]),
                        arrow_data.data_type().clone(),
                    )
                })
                .with_context("rerun.blueprint.PanelView")?;
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
                let is_expanded = {
                    if !arrays_by_name.contains_key("is_expanded") {
                        return Err(::re_types_core::DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "is_expanded",
                        ))
                        .with_context("rerun.blueprint.PanelView");
                    }
                    let arrow_data = &**arrays_by_name["is_expanded"];
                    arrow_data
                        .as_any()
                        .downcast_ref::<BooleanArray>()
                        .ok_or_else(|| {
                            ::re_types_core::DeserializationError::datatype_mismatch(
                                DataType::Boolean,
                                arrow_data.data_type().clone(),
                            )
                        })
                        .with_context("rerun.blueprint.PanelView#is_expanded")?
                        .into_iter()
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(is_expanded),
                    arrow_data.validity(),
                )
                .map(|opt| {
                    opt.map(|(is_expanded)| {
                        Ok(Self {
                            is_expanded: is_expanded
                                .ok_or_else(::re_types_core::DeserializationError::missing_data)
                                .with_context("rerun.blueprint.PanelView#is_expanded")?,
                        })
                    })
                    .transpose()
                })
                .collect::<::re_types_core::DeserializationResult<Vec<_>>>()
                .with_context("rerun.blueprint.PanelView")?
            }
        })
    }
}
