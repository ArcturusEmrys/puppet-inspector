use ash::vk::Result as VkResult;
use thiserror::Error;
use wgpu_hal::DeviceError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Wgpu (HAL) error: {0}")]
    HalDeviceError(#[from] DeviceError),

    #[error("Vulkan API error: {0}")]
    VulkanError(#[from] VkResult),

    #[cfg(target_os = "windows")]
    #[error("Windows/DirectX API error: {0}")]
    WindowsError(#[from] windows_result::Error),

    #[error("API of object does not match capabilities of called function")]
    WrongAPIError,

    #[error(
        "An extension necessary to process the request is missing. Please ensure your instance and device were both created with the appropriate _with_extensions method."
    )]
    MissingExtension,

    #[error("The texture cannot be exported as it is already external and opaque to the device")]
    OpaqueExport,

    #[error("The texture cannot be transferred to the requested device as they are not the same")]
    InvalidTransferTarget,

    #[cfg(target_os = "linux")]
    #[error("The file descriptor provided or obtained was invalid.")]
    InvalidFd,

    #[cfg(target_os = "windows")]
    #[error("The HANDLE provided or obtained was invalid.")]
    InvalidHandle,

    #[error(
        "The texture format cannot be represented in the target API or as an exportable texture."
    )]
    InvalidFormat,

    #[error("No valid memory types found to store exportable texture.")]
    NoValidMemoryType,

    #[error("The Vulkan graphics device does not also have a DX12 identity.")]
    NoDx12Identity,

    #[error("GDK yielded the following error: {0}")]
    GdkError(#[from] glib::Error),
}
