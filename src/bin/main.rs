#![cfg_attr(target_arch="arm", no_std)]
#![cfg_attr(target_arch="arm", no_main)]

#[cfg(target_arch="arm")]
use core::panic::PanicInfo;

use launchpad_pro_rs::hal::LaunchpadApp;
use launchpad_pro_rs::launchpad_app;

/// The Launchpad Pro app.
struct App;

// Register an app instance to receive events from the hardware.
launchpad_app!(App);

/// Implementation of the EventListener trait to handle events from the Launchpad Pro.
impl LaunchpadApp for App {
    fn new() -> Self {
        App
    }
}

#[cfg(target_arch="arm")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(target_arch="arm"))]
fn main() {}
