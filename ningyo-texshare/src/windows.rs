use std::ffi::c_void;
use std::ptr::null;

use wgpu_hal::vulkan::TextureMemory;

use ash::ext::image_drm_format_modifier;
use ash::khr::external_memory_win32;
use ash::vk::{
    ExternalMemoryHandleTypeFlags, ImageDrmFormatModifierPropertiesEXT, MemoryGetWin32HandleInfoKHR,
};

use windows::Win32::Foundation::{DuplicateHandle, HANDLE, LUID};
use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_11_0;
use windows::Win32::Graphics::Direct3D12::{self, D3D12GetDebugInterface, ID3D12Debug};
use windows::Win32::Graphics::Direct3D12::{
    D3D12_RESOURCE_DESC, D3D12CreateDevice, ID3D12Device, ID3D12Heap, ID3D12Resource,
};
use windows::Win32::Graphics::Dxgi;
use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory1, IDXGIAdapter1, IDXGIFactory4};
use windows::core::{IUnknown, Interface};

use crate::error::Error as OurError;
use crate::texture::ExportableTexture;
use crate::vulkan::DeviceExt;

/// A texture that has been exported as a D3D12 resource handle.
#[derive(Debug)]
pub struct ExportedTexture {
    texture: wgpu::Texture,
    handle: isize,
    alignment: u64,
}

impl ExportedTexture {
    pub fn export_to_d3d12_resource(
        device: &wgpu::Device,
        texture: &ExportableTexture,
    ) -> Result<ExportedTexture, OurError> {
        unsafe {
            let Some(inner_texture) = texture.texture().as_hal::<wgpu_hal::api::Vulkan>() else {
                return Err(OurError::WrongAPIError);
            };
            let device = device.as_hal::<wgpu_hal::api::Vulkan>().unwrap();
            let instance = device.shared_instance();
            if instance
                .extensions()
                .iter()
                .find(|ext| ext.to_bytes() == "VK_KHR_external_memory_capabilities".as_bytes())
                .is_none()
            {
                return Err(OurError::MissingExtension);
            }

            let memory = match inner_texture.memory() {
                TextureMemory::Allocation(alloc) => alloc.memory(),
                TextureMemory::Dedicated(memory) => *memory,
                TextureMemory::External => return Err(OurError::OpaqueExport),
            };
            dbg!(inner_texture.memory());

            let ext_mem_api =
                external_memory_win32::Device::new(instance.raw_instance(), device.raw_device());
            let mut ext_mem_props = MemoryGetWin32HandleInfoKHR::default();

            ext_mem_props.memory = memory;
            ext_mem_props.handle_type = ExternalMemoryHandleTypeFlags::OPAQUE_WIN32;

            let handle = ext_mem_api.get_memory_win32_handle(&ext_mem_props)?;

            dbg!(handle);

            Ok(ExportedTexture {
                texture: texture.texture().clone(),
                handle,
                alignment: texture.alignment,
            })
        }
    }

    fn raw_handle(&self) -> isize {
        self.handle
    }

    pub fn convert_to_id3d12_resource(
        &self,
        device: &wgpu::Device,
        target_device: &ID3D12Device,
    ) -> Result<ID3D12Resource, OurError> {
        let Some(device_vk) = (unsafe { device.as_hal::<wgpu_hal::api::Vulkan>() }) else {
            return Err(OurError::WrongAPIError);
        };

        let Some(device_luid) = device_vk.physical_device_luid() else {
            return Err(OurError::NoDx12Identity);
        };
        dbg!(device_luid);

        unsafe {
            if target_device.GetAdapterLuid() != std::mem::transmute(device_luid) {
                return Err(OurError::InvalidTransferTarget);
            };

            let nthandle = HANDLE(self.raw_handle() as *mut c_void);

            let mut resource: Option<ID3D12Resource> = None;
            target_device.OpenSharedHandle(nthandle, &mut resource).map_err(|e| OurError::from(e))?;

            /* Alternative code path: Attempt to open the handle as a heap 
            let mut heap: Option<ID3D12Heap> = None;
            eprintln!("about to ask for heap");
            target_device
                .OpenSharedHandle::<ID3D12Heap>(nthandle, &mut heap)
                .map_err(|e| OurError::from(e))?;

            let heap = heap.unwrap();
            dbg!(&heap);
            eprintln!("We got the heap");

            let mut resource_desc = D3D12_RESOURCE_DESC::default();
            resource_desc.Dimension = match self.texture.dimension() {
                wgpu::TextureDimension::D1 => Direct3D12::D3D12_RESOURCE_DIMENSION_TEXTURE1D,
                wgpu::TextureDimension::D2 => Direct3D12::D3D12_RESOURCE_DIMENSION_TEXTURE2D,
                wgpu::TextureDimension::D3 => Direct3D12::D3D12_RESOURCE_DIMENSION_TEXTURE3D,
            };
            resource_desc.Alignment = self.alignment;
            resource_desc.Width = self.texture.width() as u64;
            resource_desc.Height = self.texture.height();
            resource_desc.DepthOrArraySize = self.texture.depth_or_array_layers() as u16;
            resource_desc.MipLevels = self.texture.mip_level_count() as u16;
            resource_desc.Format = match self.texture.format() {
                wgpu::TextureFormat::Rgba8Unorm => Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM,
                _ => unimplemented!(),
            };
            resource_desc.SampleDesc = Dxgi::Common::DXGI_SAMPLE_DESC {
                Count: self.texture.sample_count(),
                Quality: 1, //TODO: huh?
            };
            resource_desc.Layout = Direct3D12::D3D12_TEXTURE_LAYOUT_UNKNOWN;
            resource_desc.Flags = Direct3D12::D3D12_RESOURCE_FLAG_NONE; //TODO: Any wgpu flags go here?

            dbg!(resource_desc);

            let mut resource = None;
            target_device
                .CreatePlacedResource(
                    &heap,
                    0,
                    &resource_desc,
                    Direct3D12::D3D12_RESOURCE_STATE_COMMON,
                    None,
                    &mut resource,
                )
                .map_err(|e| OurError::from(e))?;
            dbg!(&resource);
            eprintln!("We got the resource"); */

            resource.ok_or(OurError::InvalidHandle)
        }
    }
}
