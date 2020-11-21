#![no_std]

/// The interface to the Launchpad Pro hardware.
pub mod hal;
mod launchpad;
mod wasm;

#[macro_export]
macro_rules! launchpad_app {
    ($app:expr) => {
        #[cfg(target_arch="wasm32")]
        $crate::wasm_app!($app);
        #[cfg(not(target_arch="wasm32"))]
        $crate::hardware_app!($app);
    }
}