use crate::texshare::TryIntoGdkTexture;
use ningyo_texshare::ExportableTexture;

impl TryIntoGdkTexture for ExportableTexture {
    fn into_gdk_texture(
        self,
        device: &wgpu::Device,
        _display: &gdk4::Display,
    ) -> Result<gdk4::Texture, Box<dyn std::error::Error>> {
        let dmabuf =
            ningyo_texshare::linux::ExportedTexture::export_to_dmabuf(&device, &self).unwrap();

        dmabuf.into_gdk_texture()
    }
}
