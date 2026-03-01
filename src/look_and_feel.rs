#[cfg(windows)]
mod windows;

pub fn init() {
    #[cfg(windows)]
    windows::init();
}
