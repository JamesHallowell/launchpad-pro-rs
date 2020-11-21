#![no_std]

/// The interface to the Launchpad Pro hardware.
pub mod hal;
mod launchpad;
mod wasm;

#[macro_export]
macro_rules! launchpad_app {
    ($app:expr) => {
        cfg_if::cfg_if! {
            if #[cfg(target_arch="wasm32")] {
                $crate::wasm_app!($app);
            } else {
                $crate::hardware_app!($app);
            }
        }
    }
}