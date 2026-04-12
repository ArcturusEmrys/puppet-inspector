//! Texture structures that are common to all texture sharing systems.

/// Represents a texture that has been created with the necessary usages,
/// extensions, or other permissions to be exported to a texture sharing
/// backend.
///
/// You can only obtain an ExportableTexture by using the method on the
/// `DeviceExt` trait to create one. Normal textures are not automatically
/// exportable on all backends.
#[derive(Debug, Clone)]
pub struct ExportableTexture {
    pub(crate) texture: wgpu::Texture,
}

impl ExportableTexture {
    pub fn texture(&self) -> &wgpu::Texture {
        &self.texture
    }
}
