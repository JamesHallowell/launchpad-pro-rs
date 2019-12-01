#![cfg_attr(target_device = "launchpad", no_std)]
#![cfg_attr(target_device = "launchpad", no_main)]

mod life;

use launchpad_pro_rs::hal;
use life::Life;

#[cfg(target_device = "launchpad")]
use core::panic::PanicInfo;

/// Returns a mutable reference to the Game of Life instance.
fn life() -> &'static mut Life {
    static mut LIFE: Option<Life> = None;
    unsafe {
        if let None = LIFE {
            LIFE = Some(Life::new());
        }
        LIFE.as_mut().unwrap()
    }
}

/// Indicates whether the simulation is running or not.
static mut IS_RUNNING: bool = false;

/// Draws the Game of Life universe.
fn draw_universe() {
    for point in hal::Grid::points() {
        hal::plot_led(
            point.to_index(),
            match life().get(point) {
                life::Cell::Alive => hal::Rgb::new(0, 255, 0),
                _ => hal::Rgb::new(0, 0, 0),
            },
        );
    }
}

#[no_mangle]
pub extern "C" fn app_surface_event(event: u8, index: u8, value: u8) {
    if event == 0 && value == 0 {
        let point = hal::Point::from_index(index);
        life().set(point, !life().get(point));
        draw_universe();
    }
    if event == 1 && value == 0 {
        unsafe { IS_RUNNING = !IS_RUNNING };
    }
}

#[no_mangle]
pub extern "C" fn app_midi_event(_port: u8, _status: u8, _d1: u8, _d2: u8) {}

#[no_mangle]
pub extern "C" fn app_sysex_event(_port: u8, _data: *mut u8, _count: u16) {}

#[no_mangle]
pub extern "C" fn app_aftertouch_event(_index: u8, _value: u8) {}

#[no_mangle]
extern "C" fn app_cable_event(_event: u8, _value: u8) {}

#[no_mangle]
pub extern "C" fn app_timer_event() {
    static mut TICKS: i32 = 0;
    const FRAMES_PER_SECOND: i32 = 4;
    const TICKS_PER_FRAME: i32 = 1000 / FRAMES_PER_SECOND;
    unsafe {
        if TICKS == TICKS_PER_FRAME {
            if IS_RUNNING {
                life().tick();
                draw_universe();
            }
            TICKS = 0;
        } else {
            TICKS += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn app_init(_adc_raw: *const u16) {}

#[cfg(target_device = "launchpad")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(target_device = "launchpad"))]
fn main() {}
