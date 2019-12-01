#![cfg_attr(target_device = "launchpad", no_std)]
#![cfg_attr(target_device = "launchpad", no_main)]

mod life;

use launchpad_pro_rs::hal;
use launchpad_pro_rs::register_event_handler;
use launchpad_pro_rs::hal::{EventHandler, SurfaceEvent};

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

/// Create and register our event handler.
struct Events;
register_event_handler!(Events);

/// Implement handlers for the events we are interested in.
impl EventHandler for Events {
    fn timer_event(&self) {
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

    fn surface_event(&self, surface_event: SurfaceEvent) {
        if surface_event.value == hal::SurfaceEventValue::Release {
            match surface_event.surface_event_type {
                hal::SurfaceEventType::Pad => {
                    // toggle the cell and redraw the universe
                    life().set(surface_event.point, !life().get(surface_event.point));
                    draw_universe();
                },
                hal::SurfaceEventType::Setup => {
                    // pause/unpause the simulation
                    unsafe { IS_RUNNING = !IS_RUNNING; }
                }
            }
        }
    }
}

#[cfg(target_device = "launchpad")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(target_device = "launchpad"))]
fn main() {}
