#[cfg(feature = "import-obj")]
pub mod obj;

#[cfg(feature = "import-gltf")]
pub mod gltf;

use macaw::Vec3Ext as _;
use tobj::LoadError;

use crate::{mesh::MeshError, renderer::MeshInstance, resource_managers::ResourceManagerError};

#[derive(thiserror::Error, Debug)]
pub enum ImporterError {
    #[error("failed loading obj")]
    ObjLoadingFailed(#[from] LoadError),

    #[error(transparent)]
    CannotCreateResource(#[from] ResourceManagerError),

    #[error(transparent)]
    WrongMesh(#[from] MeshError),

    #[error(transparent)]
    SliceFromImportError(#[from] gltf::GltfErr),

    #[error("Unsupported texture format {format:?}")]
    UnsupportedTextureFormat { format: gltf::GltfFormat },

    #[error("Gltf primitives must have indices")]
    MissingGltfIndices,

    #[error("Gltf primitives must have positions")]
    MissingGltfPositions,

    #[error("Only a single set of texture coordinates is supported")]
    TexCoordNotSupported,

    #[error("empty mesh")]
    EmptyMesh,

    #[error("mesh {} (name {:?})", index, name)]
    UnabletoReMesh { index: usize, name: String },
}

pub fn to_uniform_scale(scale: glam::Vec3) -> f32 {
    if scale.has_equal_components(0.001) {
        scale.x
    } else {
        let uniform_scale = (scale.x * scale.y * scale.z).cbrt();
        re_log::warn!("mesh has non-uniform scale ({:?}). This is currently not supported. Using geometric mean {}", scale,uniform_scale);
        uniform_scale
    }
}

pub fn calculate_bounding_box(instances: &[MeshInstance]) -> macaw::BoundingBox {
    macaw::BoundingBox::from_points(
        instances
            .iter()
            .filter_map(|mesh_instance| {
                mesh_instance.mesh.as_ref().map(|mesh| {
                    mesh.vertex_positions
                        .iter()
                        .map(|p| mesh_instance.world_from_mesh.transform_point3(*p))
                })
            })
            .flatten(),
    )
}
