//! Texture structures that are common to all texture sharing systems.

use crate::error::Error;

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
    pub(crate) alignment: u64,
}

impl ExportableTexture {
    pub fn texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    #[cfg(target_os = "linux")]
    pub fn as_dmabuf(&self, device: &wgpu::Device) -> crate::linux::ExportedTexture {
        use crate::linux;

        linux::ExportedTexture::export_to_dmabuf(device, self)
    }

    #[cfg(target_os = "windows")]
    pub fn as_d3d12_resource(
        &self,
        device: &wgpu::Device,
    ) -> Result<crate::windows::ExportedTexture, Error> {
        use crate::windows;

        windows::ExportedTexture::export_to_d3d12_resource(device, self)
    }
}
