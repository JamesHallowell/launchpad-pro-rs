#![cfg_attr(target_device = "launchpad", no_std)]
#![cfg_attr(target_device = "launchpad", no_main)]

#[cfg(target_device = "launchpad")]
use core::panic::PanicInfo;

use launchpad_pro_rs::hal;
use launchpad_pro_rs::hal::EventListener;
use launchpad_pro_rs::register_event_listener;

/// The Launchpad Pro app.
struct App;
register_event_listener!(App);

/// Implementation of the EventListener trait to handle events from the Launchpad Pro.
impl EventListener for App {
    fn init_event(&self, _adc: hal::surface::Pads) {}
    fn timer_event(&self) {}
    fn midi_event(&self, _port: hal::midi::Port, _message: hal::midi::Message) {}
    fn sysex_event(&self, _port: hal::midi::Port, _data: &[u8]) {}
    fn cable_event(&self, _cable_event: hal::midi::CableEvent) {}
    fn button_event(&self, _button_event: hal::surface::ButtonEvent) {}
    fn aftertouch_event(&self, _aftertouch_event: hal::surface::AftertouchEvent) {}
}

#[cfg(target_device = "launchpad")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(target_device = "launchpad"))]
fn main() {}
