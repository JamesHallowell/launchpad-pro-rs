#![cfg_attr(target_device = "launchpad", no_std)]
#![cfg_attr(target_device = "launchpad", no_main)]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn app_surface_event(event: u8, index: u8, value: u8) {}

#[no_mangle]
pub extern "C" fn app_midi_event(_port: u8, _status: u8, _d1: u8, _d2: u8) {}

#[no_mangle]
pub extern "C" fn app_sysex_event(_port: u8, _data: *mut u8, _count: u16) {}

#[no_mangle]
pub extern "C" fn app_aftertouch_event(_index: u8, _value: u8) {}

#[no_mangle]
extern "C" fn app_cable_event(_event: u8, _value: u8) {}

#[no_mangle]
pub extern "C" fn app_timer_event() {}

#[no_mangle]
pub extern "C" fn app_init(_adc_raw: *const u16) {}

#[cfg(target_device = "launchpad")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(target_device = "launchpad"))]
fn main() {}
