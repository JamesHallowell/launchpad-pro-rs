#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn app_surface_event(event: u8, index: u8, value: u8) {
    use crate::hal;
    if let Some(listener) = hal::get_listener() {
        listener.button_event(hal::surface::ButtonEvent {
            button: if event == 1 {
                hal::surface::Button::Setup
            } else {
                hal::surface::Button::Pad(hal::Point::from_index(index))
            },
            event: if value == 0 {
                hal::surface::Event::Release
            } else {
                hal::surface::Event::Press(value)
            },
        });
    }
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn app_midi_event(port: u8, status: u8, data1: u8, data2: u8) {
    use crate::hal;
    if let Some(listener) = hal::get_listener()  {
        let port = match port {
            0 => Some(hal::midi::Port::Standalone),
            1 => Some(hal::midi::Port::USB),
            2 => Some(hal::midi::Port::DIN),
            _ => None,
        };

        if let Some(port) = port {
            listener.midi_event(port, hal::midi::Message {
                status,
                data: (data1, data2),
            });
        }
    }
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn app_sysex_event(port: u8, data: *mut u8, count: u16) {
    use crate::hal;
    if let Some(listener) = hal::get_listener()  {
        let port = match port {
            0 => Some(hal::midi::Port::Standalone),
            1 => Some(hal::midi::Port::USB),
            2 => Some(hal::midi::Port::DIN),
            _ => None,
        };

        if let Some(port) = port {
            let slice = unsafe { core::slice::from_raw_parts(data, count as usize) };
            listener.sysex_event(port, slice);
        }
    }

}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn app_aftertouch_event(index: u8, value: u8) {
    use crate::hal;
    if let Some(listener) = hal::get_listener()  {
        listener.aftertouch_event(hal::surface::AftertouchEvent {
            point: hal::Point::from_index(index),
            value,
        });
    }
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn app_cable_event(cable_type: u8, value: u8) {
    use crate::hal;
    if let Some(listener) = hal::get_listener()  {
        let cable_type = match cable_type {
            0 => Some(hal::midi::Cable::MidiIn),
            1 => Some(hal::midi::Cable::MidiOut),
            _ => None,
        };

        if let Some(cable_type) = cable_type {
            listener.cable_event(match value {
                0 => hal::midi::CableEvent::Disconnect(cable_type),
                _ => hal::midi::CableEvent::Connect(cable_type),
            });
        }
    }
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn app_timer_event() {
    use crate::hal;
    if let Some(listener) = hal::get_listener() {
        listener.timer_event();
    }
}

#[macro_export]
macro_rules! wasm_app {
    ($app:expr) => {
        #[no_mangle]
        pub static __LAUNCHPAD_APP: &dyn $crate::hal::LaunchpadApp = &$app;

        #[wasm_bindgen]
        pub fn app_init() {
            crate::hal::set_listener(__LAUNCHPAD_APP);
            __LAUNCHPAD_APP.init_event($crate::hal::surface::Pads::new(None));
        }

        fn main() {}
    };
}