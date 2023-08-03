#![cfg_attr(target_arch = "arm", no_std)]
#![cfg_attr(target_arch = "arm", no_main)]

use launchpad_pro_rs::{hal, hal::LaunchpadApp, launchpad_app};

/// The Launchpad Pro app.
#[derive(Default)]
pub struct App;

// Register an app type to receive events from the hardware.
launchpad_app!(App => 100 ms);

/// Implementation of the EventListener trait to handle events from the Launchpad Pro.
impl LaunchpadApp for App {
    fn init_event(&mut self, _adc: hal::surface::Pads) {}
    fn timer_event(&mut self) {}
    fn midi_event(&mut self, _port: hal::midi::Port, _message: hal::midi::Message) {}
    fn sysex_event(&mut self, _port: hal::midi::Port, _data: &[u8]) {}
    fn cable_event(&mut self, _cable_event: hal::midi::CableEvent) {}
    fn button_event(&mut self, _button_event: hal::surface::ButtonEvent) {}
    fn aftertouch_event(&mut self, _aftertouch_event: hal::surface::AftertouchEvent) {}
}
