

impl TryIntoGdkTexture for ExportableTexture {
    fn into_gdk_texture(
        self,
        device: &wgpu::Device,
        display: &gdk4::Display
    ) -> Result<gdk4::Texture, Box<dyn std::error::Error>> {
        let dmabuf = ningyo_texshare::linux::ExportedTexture::export_to_dmabuf(
            &self.wgpu_device.borrow().as_ref().unwrap(),
            &self.wgpu_texture.borrow().as_ref().unwrap().1,
        )
        .unwrap();
        *self.texture.borrow_mut() = Some(dmabuf.into_gdk_texture().expect("gdk4 import"));
    }
}