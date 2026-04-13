use crate::error::Error as OurError;
use crate::vulkan::conv;
use crate::vulkan::ext;
use crate::vulkan::image;
use ash::vk;
use std::ptr::null;
use wgpu_hal::vulkan::{Adapter, Api, Device, Instance, Texture, TextureMemory};
use wgpu_hal::{
    DeviceError, DynDevice, InstanceDescriptor, InstanceError, OpenDevice, TextureDescriptor,
};

/// Convert the high-level InstanceDescriptor into one wgpu_hal cares about.
pub fn instance_descriptor_convert(
    name: &str,
    desc: wgpu::InstanceDescriptor,
) -> InstanceDescriptor<'_> {
    InstanceDescriptor {
        name,
        backend_options: desc.backend_options,
        flags: desc.flags,
        memory_budget_thresholds: desc.memory_budget_thresholds,
        telemetry: None,
        display: None,
    }
}

/// Create a Vulkan instance with texture sharing extensions enabled.
pub fn instance_init(desc: &InstanceDescriptor<'_>) -> Result<Instance, InstanceError> {
    // SAFETY: i dunno lol wgpu_hal doesn't say anything about init
    unsafe {
        Instance::init_with_callback(
            desc,
            Some(Box::new(|args| {
                args.extensions.push(c"VK_KHR_external_memory_capabilities");
            })),
        )
    }
}

/// Trait for adapters that adds our extensions to them.
pub trait AdapterExt {
    unsafe fn open_with_extensions<'a>(
        &self,
        features: wgpu::Features,
        limits: &wgpu::Limits,
        memory_hints: &wgpu::MemoryHints,
    ) -> Result<OpenDevice<Api>, DeviceError>;
}

impl AdapterExt for Adapter {
    unsafe fn open_with_extensions<'a>(
        &self,
        features: wgpu::Features,
        limits: &wgpu::Limits,
        memory_hints: &wgpu::MemoryHints,
    ) -> Result<OpenDevice<Api>, DeviceError> {
        unsafe {
            self.open_with_callback(
                features,
                limits,
                memory_hints,
                Some(Box::new(|args| {
                    args.extensions.push(c"VK_KHR_external_memory");

                    #[cfg(target_os = "linux")]
                    {
                        args.extensions.push(c"VK_KHR_external_memory_fd");
                        args.extensions.push(c"VK_EXT_image_drm_format_modifier");
                    }

                    #[cfg(target_os = "windows")]
                    {
                        args.extensions.push(c"VK_KHR_external_memory_win32");
                    }
                })),
            )
        }
    }
}

pub trait DeviceExt {
    /// Create an exportable Vulkan texture with all of the extensions
    /// necessary to be exported from a Vulkan context.
    fn create_texture_exportable(
        &self,
        texture: &TextureDescriptor<'_>,
    ) -> Result<(Texture, u64), OurError>;

    /// Retrieve the LUID of the device.
    fn physical_device_luid(&self) -> Option<[u8; 8]>;
}

impl DeviceExt for Device {
    fn create_texture_exportable(
        &self,
        texture: &TextureDescriptor<'_>,
    ) -> Result<(Texture, u64), OurError> {
        let mut handle_types = vk::ExternalMemoryHandleTypeFlags::default();
        let mut tiling = vk::ImageTiling::OPTIMAL;

        #[cfg(target_os = "linux")]
        {
            handle_types |= vk::ExternalMemoryHandleTypeFlags::DMA_BUF_EXT;
            tiling = vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT;
        }

        #[cfg(target_os = "windows")]
        {
            handle_types |= vk::ExternalMemoryHandleTypeFlags::OPAQUE_WIN32;
            //tiling = vk::ImageTiling::LINEAR;
        }

        let external_memory_image_create_info =
            vk::ExternalMemoryImageCreateInfo::default().handle_types(handle_types);

        let (image, mem_req, format, image_type, usage_flags, create_flags) =
            image::create_image_without_memory(
                self,
                texture,
                tiling,
                Some(&mut external_memory_image_create_info.clone()),
            )?;

        dbg!(&mem_req);
        dbg!(format!("{:x}", mem_req.memory_type_bits));

        // Check that our chosen memory format can actually be exported.
        // TODO: Do I need to do this? Wouldn't it have already failed above?
        // TODO: Commented out this validation as it makes Intel drivers segfault
        // looking for a DRM format modifier struct that isn't there.
        // I could populate it but then I have to call 20 more things just to get
        // the winning modifier
        /*

        let mut format_info = vk::PhysicalDeviceImageFormatInfo2::default()
            .flags(create_flags)
            .format(format)
            .tiling(tiling)
            .ty(image_type)
            .usage(usage_flags);

        let mut drm_modifier_info = vk::PhysicalDeviceImageDrmFormatModifierInfoEXT::default()
            .drm_format_modifier();

        format_info = format_info.push_next(&mut drm_modifier_info);

        let mut out_fmt_props = vk::ImageFormatProperties2::default();

        let mut out_external_info = vk::ExternalImageFormatProperties::default();
        out_fmt_props = out_fmt_props.push_next(&mut out_external_info);

        unsafe {
            self.shared_instance()
                .raw_instance()
                .get_physical_device_image_format_properties2(
                    self.raw_physical_device(),
                    &format_info,
                    &mut out_fmt_props,
                )?;
        }

        eprintln!("{:?}", out_fmt_props);
        eprintln!("{:?}", out_external_info);

        //TODO: Seriously? This is the most expressive error I can return?!
        //TODO: Check for DEDICATED_ONLY? I mean, I want to do that anyway.
        if !out_external_info
            .external_memory_properties
            .compatible_handle_types
            .contains(handle_types)
            || !out_external_info
                .external_memory_properties
                .external_memory_features
                .contains(vk::ExternalMemoryFeatureFlags::EXPORTABLE)
        {
            //return Err(OurError::InvalidFormat);
        } */

        let mut format_info = vk::PhysicalDeviceImageFormatInfo2::default()
            .flags(create_flags)
            .format(format)
            .tiling(tiling)
            .ty(image_type)
            .usage(usage_flags);
        let mut physical_device_external_info =
            vk::PhysicalDeviceExternalImageFormatInfo::default().handle_type(handle_types);
        format_info = format_info.push_next(&mut physical_device_external_info);

        let mut out_properties = vk::ImageFormatProperties2::default();
        let mut out_external_props = vk::ExternalImageFormatProperties::default();
        out_properties = out_properties.push_next(&mut out_external_props);

        unsafe {
            self.shared_instance()
                .raw_instance()
                .get_physical_device_image_format_properties2(
                    self.raw_physical_device(),
                    &format_info,
                    &mut out_properties,
                )?;
        }

        dbg!(&out_properties);
        dbg!(&out_external_props);

        // Find a compatible memory heap to store into.
        let mem_props = unsafe {
            self.shared_instance()
                .raw_instance()
                .get_physical_device_memory_properties(self.raw_physical_device())
        };

        for (_heaptype_index, heaptype) in mem_props.memory_heaps_as_slice().iter().enumerate() {
            dbg!(heaptype);
        }

        let mut valid_memory_types = vec![];
        for (memtype_index, memtype) in mem_props.memory_types_as_slice().iter().enumerate() {
            dbg!(memtype);
            // Skip memory types not supported by the image's memory requirements.
            if (mem_req.memory_type_bits >> memtype_index) & 0x01 != 1 {
                continue;
            }

            // Skip non-device memory.
            // TODO: Do we care about the heap properties or do we just grab the first one?
            // TODO: I'm being told I need to use vkGetPhysicalDeviceImageFormatProperties2
            // with a VkExternalImageFormatProperties to check which memory types do and don't
            // support OPAQUE_WIN32 export.
            if memtype
                .property_flags
                .contains(vk::MemoryPropertyFlags::DEVICE_LOCAL)
            {
                valid_memory_types.push(memtype_index);
            }
        }

        dbg!(&valid_memory_types);
        let desired_memory_type = valid_memory_types.first().copied();

        let Some(desired_memory_type) = desired_memory_type else {
            return Err(OurError::NoValidMemoryType);
        };

        // TODO: This is probably not necessary
        let forcibly_64kb_aligned = (mem_req.size + 65535) & !(65535);
        dbg!(forcibly_64kb_aligned);

        let mut allocate_info = vk::MemoryAllocateInfo::default()
            .allocation_size(forcibly_64kb_aligned)
            .memory_type_index(desired_memory_type as u32);

        let mut win32_handle_info = vk::ExportMemoryWin32HandleInfoKHR::default();
        #[cfg(target_os = "windows")]
        {
            win32_handle_info.dw_access = 0; //windows::Win32::Foundation::GENERIC_ALL.0 | windows::Win32::Graphics::Dxgi::DXGI_SHARED_RESOURCE_READ.0 | windows::Win32::Graphics::Dxgi::DXGI_SHARED_RESOURCE_WRITE.0;
            win32_handle_info.p_attributes = null();

            allocate_info = allocate_info.push_next(&mut win32_handle_info);
        }

        let mut external_memory_info =
            vk::ExportMemoryAllocateInfo::default().handle_types(handle_types);
        allocate_info = allocate_info.push_next(&mut external_memory_info);

        let mut dedicated_allocate_info = vk::MemoryDedicatedAllocateInfo::default().image(image);
        allocate_info = allocate_info.push_next(&mut dedicated_allocate_info);

        let memory = unsafe {
            self.raw_device()
                .allocate_memory(&allocate_info, None)
                .map_err(|e| OurError::from(e))?
        };

        self.get_internal_counters()
            .texture_memory
            .add(mem_req.size as isize);

        unsafe {
            self.raw_device()
                .bind_image_memory(image, memory, 0)
                .map_err(|e| OurError::from(e))?;

            dbg!(texture);
            Ok((
                self.texture_from_raw(image, texture, None, TextureMemory::Dedicated(memory)),
                mem_req.alignment,
            ))
        }
    }

    fn physical_device_luid(&self) -> Option<[u8; 8]> {
        let mut out_physical_device_props = vk::PhysicalDeviceProperties2::default();
        let mut out_physical_id_props = vk::PhysicalDeviceIDProperties::default();

        out_physical_device_props = out_physical_device_props.push_next(&mut out_physical_id_props);

        unsafe {
            self.shared_instance()
                .raw_instance()
                .get_physical_device_properties2(
                    self.raw_physical_device(),
                    &mut out_physical_device_props,
                );
        };

        if out_physical_id_props.device_luid_valid == vk::FALSE {
            return None;
        }

        Some(out_physical_id_props.device_luid)
    }
}
