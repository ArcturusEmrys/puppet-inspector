use crate::error::WgpuRendererError;
use crate::texture::DeviceTexture;
use inox2d::model::Model;
use inox2d::texture::decode_model_textures;

use wgpu::util::{BufferInitDescriptor, DeviceExt};

/// Cast Vec2 to array.
///
/// SAFETY: This inherits the safety considerations of glam's own
/// `upload_array_to_gl`. Specifically, we rely on the fact that it's own Vec2
/// struct is plain-ol-data and we're only working with immutables.
///
/// NOTE: At some point, rewrite inox2D's vertex arrays to use bytemuck and a
/// custom Vec2 struct.
pub fn cast_vec2(array: &[glam::Vec2]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(array.as_ptr() as *const u8, std::mem::size_of_val(array)) }
}

/// Cast u16s to array.
///
/// SAFETY: This inherits the safety considerations of glam's own
/// `upload_array_to_gl`.
///
/// NOTE: This probably can already be bytemucked
pub fn cast_index(array: &[u32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(array.as_ptr() as *const u8, std::mem::size_of_val(array)) }
}

/// All of the puppet-specific textures for this renderer.
///
/// Multiple renderers may share a puppet (e.g. to render to different views)
pub struct WgpuUploads {
    pub(crate) verts: wgpu::Buffer,
    pub(crate) uvs: wgpu::Buffer,
    pub(crate) deforms: wgpu::Buffer,
    pub(crate) indices: wgpu::Buffer,

    pub(crate) model_textures: Vec<DeviceTexture>,
    pub(crate) model_sampler: wgpu::Sampler,
}

impl WgpuUploads {
    pub fn new(
        model: &Model,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Self, WgpuRendererError> {
        let inox_buffers = model
            .puppet
            .render_ctx
            .as_ref()
            .ok_or(WgpuRendererError::ModelRenderingNotInitialized)?;

        //TODO: Change inox2d upstream to use a bytemuck-able array
        let verts = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(&format!(
                "Inox2D {}::Verts",
                model
                    .puppet
                    .meta
                    .name
                    .as_deref()
                    .unwrap_or("<NAME NOT SPECIFIED>")
            )),
            contents: cast_vec2(inox_buffers.vertex_buffers.verts.as_slice()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        let uvs = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(&format!(
                "Inox2D {}::UVs",
                model
                    .puppet
                    .meta
                    .name
                    .as_deref()
                    .unwrap_or("<NAME NOT SPECIFIED>")
            )),
            contents: cast_vec2(inox_buffers.vertex_buffers.uvs.as_slice()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        let deforms = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(&format!(
                "Inox2D {}::Deforms",
                model
                    .puppet
                    .meta
                    .name
                    .as_deref()
                    .unwrap_or("<NAME NOT SPECIFIED>")
            )),
            contents: cast_vec2(inox_buffers.vertex_buffers.deforms.as_slice()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        let indices = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(&format!(
                "Inox2D {}::Indices",
                model
                    .puppet
                    .meta
                    .name
                    .as_deref()
                    .unwrap_or("<NAME NOT SPECIFIED>")
            )),
            contents: cast_index(inox_buffers.vertex_buffers.indices.as_slice()),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        });

        let decoded_textures = decode_model_textures(model.textures.iter());
        let mut texture_handles = vec![];
        for (index, texture) in decoded_textures.iter().enumerate() {
            texture_handles.push(DeviceTexture::new_from_model(
                &device, &queue, model, index, texture,
            ));
        }

        let model_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToBorder,
            address_mode_v: wgpu::AddressMode::ClampToBorder,
            address_mode_w: wgpu::AddressMode::ClampToBorder,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Ok(WgpuUploads {
            verts,
            uvs,
            deforms,
            indices,
            model_textures: texture_handles,
            model_sampler,
        })
    }
}
