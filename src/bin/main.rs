#![cfg_attr(target_device = "launchpad", no_std)]
#![cfg_attr(target_device = "launchpad", no_main)]

#[cfg(target_device = "launchpad")]
use core::panic::PanicInfo;

use launchpad_pro_rs::register_event_handler;
use launchpad_pro_rs::hal::EventHandler;

struct Events;

register_event_handler!(Events);

impl EventHandler for Events {
    fn init_event(&self) {
    }

    fn timer_event(&self) {
    }

    fn midi_event(&self) {
    }

    fn sysex_event(&self) {
    }

    fn cable_event(&self) {
    }

    fn surface_event(&self) {
    }

    fn aftertouch_event(&self) {
    }
}

#[cfg(target_device = "launchpad")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(target_device = "launchpad"))]
fn main() {
}
