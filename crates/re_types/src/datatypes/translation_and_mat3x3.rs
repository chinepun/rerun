// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/datatypes/translation_and_mat3x3.fbs".

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

/// **Datatype**: Representation of an affine transform via a 3x3 affine matrix paired with a translation.
///
/// First applies the matrix, then the translation.
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct TranslationAndMat3x3 {
    /// 3D translation, applied after the matrix.
    pub translation: Option<crate::datatypes::Vec3D>,

    /// 3x3 matrix for scale, rotation & shear.
    pub mat3x3: Option<crate::datatypes::Mat3x3>,

    /// If true, this transform is from the parent space to the space where the transform was logged.
    ///
    /// If false (default), the transform maps from this space to its parent,
    /// i.e. the translation is the position in the parent space.
    pub from_parent: bool,
}

impl<'a> From<TranslationAndMat3x3> for ::std::borrow::Cow<'a, TranslationAndMat3x3> {
    #[inline]
    fn from(value: TranslationAndMat3x3) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a TranslationAndMat3x3> for ::std::borrow::Cow<'a, TranslationAndMat3x3> {
    #[inline]
    fn from(value: &'a TranslationAndMat3x3) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl ::re_types_core::Loggable for TranslationAndMat3x3 {
    type Name = ::re_types_core::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.TranslationAndMat3x3".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use arrow2::datatypes::*;
        DataType::Struct(vec![
            Field {
                name: "translation".to_owned(),
                data_type: <crate::datatypes::Vec3D>::arrow_datatype(),
                is_nullable: true,
                metadata: [].into(),
            },
            Field {
                name: "mat3x3".to_owned(),
                data_type: <crate::datatypes::Mat3x3>::arrow_datatype(),
                is_nullable: true,
                metadata: [].into(),
            },
            Field {
                name: "from_parent".to_owned(),
                data_type: DataType::Boolean,
                is_nullable: false,
                metadata: [].into(),
            },
        ])
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
                <crate::datatypes::TranslationAndMat3x3>::arrow_datatype(),
                vec![
                    {
                        let (somes, translation): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum
                                    .as_ref()
                                    .map(|datum| {
                                        let Self { translation, .. } = &**datum;
                                        translation.clone()
                                    })
                                    .flatten();
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let translation_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                            let translation_inner_data: Vec<_> = translation
                                .iter()
                                .map(|datum| {
                                    datum
                                        .map(|datum| {
                                            let crate::datatypes::Vec3D(data0) = datum;
                                            data0
                                        })
                                        .unwrap_or_default()
                                })
                                .flatten()
                                .map(Some)
                                .collect();
                            let translation_inner_bitmap: Option<arrow2::bitmap::Bitmap> =
                                translation_bitmap.as_ref().map(|bitmap| {
                                    bitmap
                                        .iter()
                                        .map(|i| std::iter::repeat(i).take(3usize))
                                        .flatten()
                                        .collect::<Vec<_>>()
                                        .into()
                                });
                            FixedSizeListArray::new(
                                DataType::FixedSizeList(
                                    Box::new(Field {
                                        name: "item".to_owned(),
                                        data_type: DataType::Float32,
                                        is_nullable: false,
                                        metadata: [].into(),
                                    }),
                                    3usize,
                                ),
                                PrimitiveArray::new(
                                    DataType::Float32,
                                    translation_inner_data
                                        .into_iter()
                                        .map(|v| v.unwrap_or_default())
                                        .collect(),
                                    translation_inner_bitmap,
                                )
                                .boxed(),
                                translation_bitmap,
                            )
                            .boxed()
                        }
                    },
                    {
                        let (somes, mat3x3): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum
                                    .as_ref()
                                    .map(|datum| {
                                        let Self { mat3x3, .. } = &**datum;
                                        mat3x3.clone()
                                    })
                                    .flatten();
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let mat3x3_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                            let mat3x3_inner_data: Vec<_> = mat3x3
                                .iter()
                                .map(|datum| {
                                    datum
                                        .map(|datum| {
                                            let crate::datatypes::Mat3x3(data0) = datum;
                                            data0
                                        })
                                        .unwrap_or_default()
                                })
                                .flatten()
                                .map(Some)
                                .collect();
                            let mat3x3_inner_bitmap: Option<arrow2::bitmap::Bitmap> =
                                mat3x3_bitmap.as_ref().map(|bitmap| {
                                    bitmap
                                        .iter()
                                        .map(|i| std::iter::repeat(i).take(9usize))
                                        .flatten()
                                        .collect::<Vec<_>>()
                                        .into()
                                });
                            FixedSizeListArray::new(
                                DataType::FixedSizeList(
                                    Box::new(Field {
                                        name: "item".to_owned(),
                                        data_type: DataType::Float32,
                                        is_nullable: false,
                                        metadata: [].into(),
                                    }),
                                    9usize,
                                ),
                                PrimitiveArray::new(
                                    DataType::Float32,
                                    mat3x3_inner_data
                                        .into_iter()
                                        .map(|v| v.unwrap_or_default())
                                        .collect(),
                                    mat3x3_inner_bitmap,
                                )
                                .boxed(),
                                mat3x3_bitmap,
                            )
                            .boxed()
                        }
                    },
                    {
                        let (somes, from_parent): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| {
                                    let Self { from_parent, .. } = &**datum;
                                    from_parent.clone()
                                });
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let from_parent_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        BooleanArray::new(
                            DataType::Boolean,
                            from_parent
                                .into_iter()
                                .map(|v| v.unwrap_or_default())
                                .collect(),
                            from_parent_bitmap,
                        )
                        .boxed()
                    },
                ],
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
                        DataType::Struct(vec![
                            Field {
                                name: "translation".to_owned(),
                                data_type: <crate::datatypes::Vec3D>::arrow_datatype(),
                                is_nullable: true,
                                metadata: [].into(),
                            },
                            Field {
                                name: "mat3x3".to_owned(),
                                data_type: <crate::datatypes::Mat3x3>::arrow_datatype(),
                                is_nullable: true,
                                metadata: [].into(),
                            },
                            Field {
                                name: "from_parent".to_owned(),
                                data_type: DataType::Boolean,
                                is_nullable: false,
                                metadata: [].into(),
                            },
                        ]),
                        arrow_data.data_type().clone(),
                    )
                })
                .with_context("rerun.datatypes.TranslationAndMat3x3")?;
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
                let translation = {
                    if !arrays_by_name.contains_key("translation") {
                        return Err(::re_types_core::DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "translation",
                        ))
                        .with_context("rerun.datatypes.TranslationAndMat3x3");
                    }
                    let arrow_data = &**arrays_by_name["translation"];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<arrow2::array::FixedSizeListArray>()
                            .ok_or_else(|| {
                                ::re_types_core::DeserializationError::datatype_mismatch(
                                    DataType::FixedSizeList(
                                        Box::new(Field {
                                            name: "item".to_owned(),
                                            data_type: DataType::Float32,
                                            is_nullable: false,
                                            metadata: [].into(),
                                        }),
                                        3usize,
                                    ),
                                    arrow_data.data_type().clone(),
                                )
                            })
                            .with_context("rerun.datatypes.TranslationAndMat3x3#translation")?;
                        if arrow_data.is_empty() {
                            Vec::new()
                        } else {
                            let offsets = (0..)
                                .step_by(3usize)
                                .zip((3usize..).step_by(3usize).take(arrow_data.len()));
                            let arrow_data_inner = {
                                let arrow_data_inner = &**arrow_data.values();
                                arrow_data_inner
                                    .as_any()
                                    .downcast_ref::<Float32Array>()
                                    .ok_or_else(|| {
                                        ::re_types_core::DeserializationError::datatype_mismatch(
                                            DataType::Float32,
                                            arrow_data_inner.data_type().clone(),
                                        )
                                    })
                                    .with_context(
                                        "rerun.datatypes.TranslationAndMat3x3#translation",
                                    )?
                                    .into_iter()
                                    .map(|opt| opt.copied())
                                    .collect::<Vec<_>>()
                            };
                            arrow2::bitmap::utils::ZipValidity::new_with_validity(
                                offsets,
                                arrow_data.validity(),
                            )
                            .map(|elem| {
                                elem.map(|(start, end)| {
                                    debug_assert!(end - start == 3usize);
                                    if end as usize > arrow_data_inner.len() {
                                        return Err(
                                            ::re_types_core::DeserializationError::offset_slice_oob(
                                                (start, end),
                                                arrow_data_inner.len(),
                                            ),
                                        );
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    let data = unsafe {
                                        arrow_data_inner.get_unchecked(start as usize..end as usize)
                                    };
                                    let data = data.iter().cloned().map(Option::unwrap_or_default);
                                    let arr = array_init::from_iter(data).unwrap();
                                    Ok(arr)
                                })
                                .transpose()
                            })
                            .map(|res_or_opt| {
                                res_or_opt.map(|res_or_opt| {
                                    res_or_opt.map(|v| crate::datatypes::Vec3D(v))
                                })
                            })
                            .collect::<::re_types_core::DeserializationResult<Vec<Option<_>>>>()?
                        }
                        .into_iter()
                    }
                };
                let mat3x3 = {
                    if !arrays_by_name.contains_key("mat3x3") {
                        return Err(::re_types_core::DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "mat3x3",
                        ))
                        .with_context("rerun.datatypes.TranslationAndMat3x3");
                    }
                    let arrow_data = &**arrays_by_name["mat3x3"];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<arrow2::array::FixedSizeListArray>()
                            .ok_or_else(|| {
                                ::re_types_core::DeserializationError::datatype_mismatch(
                                    DataType::FixedSizeList(
                                        Box::new(Field {
                                            name: "item".to_owned(),
                                            data_type: DataType::Float32,
                                            is_nullable: false,
                                            metadata: [].into(),
                                        }),
                                        9usize,
                                    ),
                                    arrow_data.data_type().clone(),
                                )
                            })
                            .with_context("rerun.datatypes.TranslationAndMat3x3#mat3x3")?;
                        if arrow_data.is_empty() {
                            Vec::new()
                        } else {
                            let offsets = (0..)
                                .step_by(9usize)
                                .zip((9usize..).step_by(9usize).take(arrow_data.len()));
                            let arrow_data_inner = {
                                let arrow_data_inner = &**arrow_data.values();
                                arrow_data_inner
                                    .as_any()
                                    .downcast_ref::<Float32Array>()
                                    .ok_or_else(|| {
                                        ::re_types_core::DeserializationError::datatype_mismatch(
                                            DataType::Float32,
                                            arrow_data_inner.data_type().clone(),
                                        )
                                    })
                                    .with_context("rerun.datatypes.TranslationAndMat3x3#mat3x3")?
                                    .into_iter()
                                    .map(|opt| opt.copied())
                                    .collect::<Vec<_>>()
                            };
                            arrow2::bitmap::utils::ZipValidity::new_with_validity(
                                offsets,
                                arrow_data.validity(),
                            )
                            .map(|elem| {
                                elem.map(|(start, end)| {
                                    debug_assert!(end - start == 9usize);
                                    if end as usize > arrow_data_inner.len() {
                                        return Err(
                                            ::re_types_core::DeserializationError::offset_slice_oob(
                                                (start, end),
                                                arrow_data_inner.len(),
                                            ),
                                        );
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    let data = unsafe {
                                        arrow_data_inner.get_unchecked(start as usize..end as usize)
                                    };
                                    let data = data.iter().cloned().map(Option::unwrap_or_default);
                                    let arr = array_init::from_iter(data).unwrap();
                                    Ok(arr)
                                })
                                .transpose()
                            })
                            .map(|res_or_opt| {
                                res_or_opt.map(|res_or_opt| {
                                    res_or_opt.map(|v| crate::datatypes::Mat3x3(v))
                                })
                            })
                            .collect::<::re_types_core::DeserializationResult<Vec<Option<_>>>>()?
                        }
                        .into_iter()
                    }
                };
                let from_parent = {
                    if !arrays_by_name.contains_key("from_parent") {
                        return Err(::re_types_core::DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "from_parent",
                        ))
                        .with_context("rerun.datatypes.TranslationAndMat3x3");
                    }
                    let arrow_data = &**arrays_by_name["from_parent"];
                    arrow_data
                        .as_any()
                        .downcast_ref::<BooleanArray>()
                        .ok_or_else(|| {
                            ::re_types_core::DeserializationError::datatype_mismatch(
                                DataType::Boolean,
                                arrow_data.data_type().clone(),
                            )
                        })
                        .with_context("rerun.datatypes.TranslationAndMat3x3#from_parent")?
                        .into_iter()
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(translation, mat3x3, from_parent),
                    arrow_data.validity(),
                )
                .map(|opt| {
                    opt.map(|(translation, mat3x3, from_parent)| {
                        Ok(Self {
                            translation,
                            mat3x3,
                            from_parent: from_parent
                                .ok_or_else(::re_types_core::DeserializationError::missing_data)
                                .with_context("rerun.datatypes.TranslationAndMat3x3#from_parent")?,
                        })
                    })
                    .transpose()
                })
                .collect::<::re_types_core::DeserializationResult<Vec<_>>>()
                .with_context("rerun.datatypes.TranslationAndMat3x3")?
            }
        })
    }
}
