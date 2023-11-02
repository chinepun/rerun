use itertools::Itertools;
use re_renderer::{resource_managers::ResourceLifeTime, RenderContext, Rgba32Unmul};
use re_types::{
    archetypes::{Asset3D, Mesh3D},
    components::MediaType,
};

use crate::mesh_cache::AnyMesh;

pub struct LoadedMesh {
    name: String,

    // TODO(andreas): We should only have MeshHandles here (which are generated by the MeshManager!)
    // Can't do that right now because it's too hard to pass the render context through.
    pub mesh_instances: Vec<re_renderer::renderer::MeshInstance>,

    bbox: macaw::BoundingBox,
}

impl LoadedMesh {
    pub fn load(
        name: String,
        mesh: AnyMesh<'_>,
        render_ctx: &RenderContext,
    ) -> anyhow::Result<Self> {
        // TODO(emilk): load CpuMesh in background thread.
        match mesh {
            AnyMesh::Asset(asset3d) => Self::load_asset3d(name, asset3d, render_ctx),
            AnyMesh::Mesh(mesh3d) => Ok(Self::load_mesh3d(name, mesh3d, render_ctx)?),
        }
    }

    pub fn load_asset3d_parts(
        name: String,
        media_type: &MediaType,
        bytes: &[u8],
        render_ctx: &RenderContext,
    ) -> anyhow::Result<Self> {
        re_tracing::profile_function!();

        let mesh_instances = match media_type.as_str() {
            MediaType::GLTF | MediaType::GLB => re_renderer::importer::gltf::load_gltf_from_buffer(
                &name,
                bytes,
                ResourceLifeTime::LongLived,
                render_ctx,
            )?,
            MediaType::OBJ => re_renderer::importer::obj::load_obj_from_buffer(
                bytes,
                ResourceLifeTime::LongLived,
                render_ctx,
            )?,
            _ => anyhow::bail!("{media_type} files are not supported"),
        };

        let bbox = re_renderer::importer::calculate_bounding_box(&mesh_instances);

        Ok(Self {
            name,
            bbox,
            mesh_instances,
        })
    }

    fn load_asset3d(
        name: String,
        asset3d: &Asset3D,
        render_ctx: &RenderContext,
    ) -> anyhow::Result<Self> {
        re_tracing::profile_function!();

        let Asset3D {
            blob,
            media_type,
            transform: _,
        } = asset3d;

        let media_type = MediaType::or_guess_from_data(media_type.clone(), blob.0.as_slice())
            .ok_or_else(|| anyhow::anyhow!("couldn't guess media type"))?;
        let slf = Self::load_asset3d_parts(name, &media_type, blob.0.as_slice(), render_ctx)?;

        Ok(slf)
    }

    fn load_mesh3d(
        name: String,
        mesh3d: &Mesh3D,
        render_ctx: &RenderContext,
    ) -> anyhow::Result<Self> {
        re_tracing::profile_function!();

        let Mesh3D {
            vertex_positions,
            mesh_properties,
            vertex_normals,
            vertex_colors,
            mesh_material,
            class_ids: _,
            instance_keys: _,
        } = mesh3d;

        let vertex_positions: &[glam::Vec3] = bytemuck::cast_slice(vertex_positions.as_slice());
        let num_positions = vertex_positions.len();

        let triangle_indices = if let Some(indices) = mesh_properties
            .as_ref()
            .and_then(|props| props.indices.as_ref())
        {
            re_tracing::profile_scope!("copy_indices");
            anyhow::ensure!(indices.len() % 3 == 0);
            let indices: &[glam::UVec3] = bytemuck::cast_slice(indices);
            indices.to_vec()
        } else {
            re_tracing::profile_scope!("generate_indices");
            anyhow::ensure!(num_positions % 3 == 0);
            (0..num_positions as u32)
                .tuples::<(_, _, _)>()
                .map(glam::UVec3::from)
                .collect::<Vec<_>>()
        };
        let num_indices = triangle_indices.len() * 3;

        let vertex_colors = if let Some(vertex_colors) = vertex_colors {
            re_tracing::profile_scope!("copy_colors");
            vertex_colors
                .iter()
                .map(|c| Rgba32Unmul::from_rgba_unmul_array(c.to_array()))
                .collect()
        } else {
            vec![Rgba32Unmul::WHITE; num_positions]
        };

        let vertex_normals = if let Some(normals) = vertex_normals {
            re_tracing::profile_scope!("collect_normals");
            normals.iter().map(|v| v.0.into()).collect::<Vec<_>>()
        } else {
            // TODO(andreas): Calculate normals
            // TODO(cmc): support textured raw meshes
            vec![glam::Vec3::ZERO; num_positions]
        };

        let vertex_texcoords = vec![glam::Vec2::ZERO; vertex_normals.len()];
        let albedo_factor = mesh_material.as_ref().and_then(|mat| mat.albedo_factor);

        let bbox = {
            re_tracing::profile_scope!("bbox");
            macaw::BoundingBox::from_points(vertex_positions.iter().copied())
        };

        let mesh = re_renderer::mesh::Mesh {
            label: name.clone().into(),
            triangle_indices,
            vertex_positions: vertex_positions.into(),
            vertex_colors,
            vertex_normals,
            vertex_texcoords,
            materials: smallvec::smallvec![re_renderer::mesh::Material {
                label: name.clone().into(),
                index_range: 0..num_indices as _,
                albedo: render_ctx
                    .texture_manager_2d
                    .white_texture_unorm_handle()
                    .clone(),
                albedo_multiplier: albedo_factor.map_or(re_renderer::Rgba::WHITE, |c| c.into()),
            }],
        };

        let mesh_instances = vec![re_renderer::renderer::MeshInstance {
            gpu_mesh: render_ctx.mesh_manager.write().create(
                render_ctx,
                &mesh,
                ResourceLifeTime::LongLived,
            )?,
            ..Default::default()
        }];

        Ok(Self {
            name,
            bbox,
            mesh_instances,
        })
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn bbox(&self) -> macaw::BoundingBox {
        self.bbox
    }
}
