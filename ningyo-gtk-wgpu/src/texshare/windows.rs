use std::ffi::c_void;
use std::ptr::null_mut;

use crate::texshare::TryIntoGdkTexture;
use gdk4_win32::D3D12TextureBuilder;
use gdk4_win32::ffi;
use glib::prelude::*;
use glib::translate::{ToGlibPtr, from_glib_full};
use ningyo_texshare::ExportableTexture;

use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_11_0;
use windows::Win32::Graphics::Direct3D12::D3D12CreateDevice;
use windows::Win32::Graphics::Direct3D12::ID3D12Device;
use windows::Win32::Graphics::Direct3D12::ID3D12Resource;
use windows::Win32::Graphics::Dxgi::CreateDXGIFactory1;
use windows::Win32::Graphics::Dxgi::IDXGIFactory;
use windows::core::IUnknown;
use windows::core::Interface;

unsafe extern "C" fn ng_gtk_wgpu_into_gdk_texture_destroy(data: glib::ffi::gpointer) {
    let _ = unsafe { Box::from_raw(data as *mut ExportableTexture) };
}

fn get_d3d_texture(
    me: &ExportableTexture,
    device: &wgpu::Device,
) -> Result<ID3D12Resource, Box<dyn std::error::Error>> {
    if let Some(d3d_texture) = unsafe { me.texture().as_hal::<wgpu_hal::dx12::Api>() } {
        return Ok(unsafe { d3d_texture.raw_resource().clone() });
    }

    panic!("Vulkan export to DX12 does not work.");

    let d3d_device = unsafe {
        let factory: IDXGIFactory = CreateDXGIFactory1().unwrap();
        let mut device = None;
        D3D12CreateDevice(None, D3D_FEATURE_LEVEL_11_0, &mut device).unwrap();

        device.unwrap()
    };

    let d3d_tex = me.as_d3d12_resource(device)?;
    let d3d_resource = d3d_tex.convert_to_id3d12_resource(device, &d3d_device)?;

    dbg!(&d3d_resource);

    Ok(d3d_resource)
}

impl TryIntoGdkTexture for ExportableTexture {
    fn into_gdk_texture(
        self,
        device: &wgpu::Device,
        display: &gdk4::Display,
    ) -> Result<gdk4::Texture, Box<dyn std::error::Error>> {
        let d3d_resource = get_d3d_texture(&self, device)?;
        let builder = D3D12TextureBuilder::new();

        unsafe {
            ffi::gdk_d3d12_texture_builder_set_resource(
                ToGlibPtr::to_glib_none(&builder).0,
                std::mem::transmute(d3d_resource),
            );
        }

        let boxed_self = Box::into_raw(Box::new(self));
        let mut error = null_mut();

        let maybe_builder = unsafe {
            ffi::gdk_d3d12_texture_builder_build(
                ToGlibPtr::to_glib_none(&builder).0,
                Some(ng_gtk_wgpu_into_gdk_texture_destroy),
                boxed_self as *mut c_void,
                &mut error,
            )
        };

        if maybe_builder.is_null() {
            Err(unsafe {
                let g_error: glib::Error = from_glib_full(error);

                g_error
            }
            .into())
        } else {
            Ok(unsafe { from_glib_full(maybe_builder) })
        }
    }
}
