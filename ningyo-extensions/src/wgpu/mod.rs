#[derive(Debug, thiserror::Error)]
pub enum SurfaceError {
    /// A timeout was encountered while trying to acquire the next frame.
    ///
    /// Applications should skip the current frame and try again later.
    #[error("A timeout was encountered while trying to acquire the next frame.")]
    Timeout,

    /// The window is occluded (e.g. minimized or behind another window).
    ///
    /// Applications should skip the current frame and try again once the window
    /// is no longer occluded.
    #[error("The window is occluded (e.g. minimized or behind another window).")]
    Occluded,

    /// The underlying surface has changed, and therefore the surface configuration is outdated.
    ///
    /// Call [`Surface::configure()`] and try again.
    #[error("The underlying surface has changed, and therefore the surface configuration is outdated.")]
    Outdated,

    /// The surface has been lost and needs to be recreated.
    ///
    /// If the device as a whole is lost (see [`set_device_lost_callback()`][crate::Device::set_device_lost_callback]), then
    /// you need to recreate the device and all resources.
    /// Otherwise, call [`Instance::create_surface()`] to recreate the surface,
    /// then [`Surface::configure()`], and try again.
    #[error("The surface has been lost and needs to be recreated.")]
    Lost,

    /// A validation error inside [`Surface::get_current_texture()`] was raised
    /// and caught by an [error scope](crate::Device::push_error_scope) or
    /// [`on_uncaptured_error()`][crate::Device::on_uncaptured_error].
    ///
    /// Applications should attend to the validation error and try again.
    #[error("A validation error inside Surface::get_current_texture() was raised.")]
    Validation,
}

pub enum SurfaceOptimal {
    Optimal,
    Suboptimal
}

pub struct SurfaceTexture {
    pub texture: wgpu::SurfaceTexture,
    pub optimal: SurfaceOptimal
}

pub trait CurrentSurfaceTextureExt {
    fn as_surface_texture(self) -> Result<SurfaceTexture, SurfaceError>;
}

impl CurrentSurfaceTextureExt for wgpu::CurrentSurfaceTexture {
    fn as_surface_texture(self) -> Result<SurfaceTexture, SurfaceError> {
        match self {
            Self::Success(st) => Ok(SurfaceTexture { texture: st, optimal: SurfaceOptimal::Optimal}),
            Self::Suboptimal(st) => Ok(SurfaceTexture { texture: st, optimal: SurfaceOptimal::Suboptimal}),
            Self::Timeout => Err(SurfaceError::Timeout),
            Self::Occluded => Err(SurfaceError::Occluded),
            Self::Outdated => Err(SurfaceError::Outdated),
            Self::Lost => Err(SurfaceError::Lost),
            Self::Validation => Err(SurfaceError::Validation)
        }
    }
}