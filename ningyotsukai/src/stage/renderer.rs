#[cfg(not(any(target_os = "linux", target_os = "windows")))]
mod opengl;

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
pub use opengl::StageRenderer;

#[cfg(any(target_os = "linux", target_os = "windows"))]
mod wgpu;

#[cfg(any(target_os = "linux", target_os = "windows"))]
pub use wgpu::StageRenderer;
