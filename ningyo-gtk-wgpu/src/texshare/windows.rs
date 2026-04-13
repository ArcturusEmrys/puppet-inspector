use std::ffi::c_void;
use std::ptr::null_mut;

use crate::texshare::TryIntoGdkTexture;
use gdk4_win32::D3D12TextureBuilder;
use gdk4_win32::ffi;
use glib::translate::{ToGlibPtr, from_glib_full};
use ningyo_texshare::ExportableTexture;
use glib::prelude::*;

use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_11_0;
use windows::Win32::Graphics::Direct3D12::D3D12CreateDevice;
use windows::Win32::Graphics::Direct3D12::ID3D12Device;
use windows::Win32::Graphics::Dxgi::CreateDXGIFactory1;
use windows::Win32::Graphics::Dxgi::IDXGIFactory;
use windows::core::IUnknown;
use windows::core::Interface;

/* It'd be very nice, but we're not actually ALLOWED to call this.
unsafe extern "C" {
    unsafe fn gdk_win32_display_get_d3d12_device(me: *mut gdk4_win32::ffi::GdkWin32Display) -> *mut ID3D12Device;
}

trait GdkWin32DisplayExt {
    fn d3d12_device(&self) -> Option<ID3D12Device>;
}

impl<T> GdkWin32DisplayExt for T where T: IsA<gdk4_win32::Win32Display> {
    fn d3d12_device(&self) -> Option<ID3D12Device> {
        unsafe {
            let borrowed_device = gdk_win32_display_get_d3d12_device(ToGlibPtr::to_glib_none(self).0 as *mut gdk4_win32::ffi::GdkWin32Display) as *mut c_void;

            let cooked = Interface::from_raw_borrowed(&borrowed_device);

            cooked.cloned()
        }
    }
}*/

unsafe extern "C" fn ng_gtk_wgpu_into_gdk_texture_destroy(data: glib::ffi::gpointer) {
    let _ = unsafe { Box::from_raw(data as *mut ExportableTexture) };
}

impl TryIntoGdkTexture for ExportableTexture {
    fn into_gdk_texture(
        self,
        device: &wgpu::Device,
        display: &gdk4::Display
    ) -> Result<gdk4::Texture, Box<dyn std::error::Error>> {
        let Some(win32_display) = display.downcast_ref::<gdk4_win32::Win32Display>() else {
            return Err("Can only be called on Windows!!!".into());
        };
        
        let d3d_device = unsafe {
            let factory:IDXGIFactory = CreateDXGIFactory1().unwrap();
            let mut device = None;
            D3D12CreateDevice(None, D3D_FEATURE_LEVEL_11_0, &mut device).unwrap();

            device.unwrap()
        };

        let d3d_tex = self.as_d3d12_resource(device)?;

        let builder = D3D12TextureBuilder::new();
        let d3d_resource = d3d_tex.convert_to_id3d12_resource(device, &d3d_device)?;

        dbg!(&d3d_resource);

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
