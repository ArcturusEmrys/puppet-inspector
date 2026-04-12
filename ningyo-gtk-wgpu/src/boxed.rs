//! Boxed structs for WGPU types that need to pass through GValue.

#[derive(Debug, Clone, glib::Boxed)]
#[boxed_type(name = "NGBoxedWGPUDeviceDescriptor")]
pub struct BoxedWgpuDeviceDescriptor(pub wgpu::DeviceDescriptor<'static>, pub glib::GString);

impl From<(wgpu::DeviceDescriptor<'static>, glib::GString)> for BoxedWgpuDeviceDescriptor {
    fn from(desc: (wgpu::DeviceDescriptor<'static>, glib::GString)) -> Self {
        Self(desc.0, desc.1)
    }
}

#[derive(Debug, Clone, glib::Boxed)]
#[boxed_type(name = "NGBoxedWGPUTextureUsages")]
pub struct BoxedWgpuTextureUsages(pub wgpu::TextureUsages);

impl From<wgpu::TextureUsages> for BoxedWgpuTextureUsages {
    fn from(usage: wgpu::TextureUsages) -> Self {
        Self(usage)
    }
}

impl Into<wgpu::TextureUsages> for BoxedWgpuTextureUsages {
    fn into(self) -> wgpu::TextureUsages {
        self.0
    }
}

#[derive(Debug, Clone, glib::Boxed)]
#[boxed_type(name = "NGBoxedWGPUTexture")]
pub struct BoxedWgpuTexture(pub wgpu::Texture);

impl From<wgpu::Texture> for BoxedWgpuTexture {
    fn from(usage: wgpu::Texture) -> Self {
        Self(usage)
    }
}

impl Into<wgpu::Texture> for BoxedWgpuTexture {
    fn into(self) -> wgpu::Texture {
        self.0
    }
}
