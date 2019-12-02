#![cfg_attr(target_device = "launchpad", no_std)]
#![cfg_attr(target_device = "launchpad", no_main)]

#[cfg(target_device = "launchpad")]
use core::panic::PanicInfo;

use launchpad_pro_rs::hal;
use launchpad_pro_rs::register_event_listener;
use launchpad_pro_rs::hal::EventListener;

struct App;

register_event_listener!(App);

impl EventListener for App {
    fn init_event(&self) {}
    fn timer_event(&self) {}
    fn midi_event(&self) {}
    fn sysex_event(&self) {}
    fn cable_event(&self) {}
    fn surface_event(&self, surface_event: hal::SurfaceEvent) {}
    fn aftertouch_event(&self) {}
}

#[cfg(target_device = "launchpad")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(target_device = "launchpad"))]
fn main() {
}
