#![cfg_attr(target_device = "launchpad", no_std)]
#![cfg_attr(target_device = "launchpad", no_main)]

mod life;

use launchpad_pro_rs::hal;
use launchpad_pro_rs::register_event_listener;
use launchpad_pro_rs::hal::{EventListener, SurfaceEvent};

use life::Life;

#[cfg(target_device = "launchpad")]
use core::panic::PanicInfo;
use spin::Mutex;

/// The number of frames per second in our simulation.
const FRAMES_PER_SECOND: i32 = 4;

/// The number of timer ticks per frame. Timer ticks happen at a frequency of ~1ms.
const TICKS_PER_FRAME: i32 = 1000 / FRAMES_PER_SECOND;

/// The Launchpad Pro app.
struct App {
    /// A flag to indicate whether the Game of Life simulation is running.
    is_running: Mutex<bool>,
    /// Our Game of Life state.
    life: Mutex<Life>,
}

impl App {
    /// Create the app.
    const fn new() -> Self {
        Self {
            is_running: Mutex::new(false),
            life: Mutex::new(Life::new()),
        }
    }

    /// Draw the Game of Life universe on the Launchpad Pro grid.
    fn draw_universe(&self) {
        let life = self.life.lock();
        for point in hal::Grid::points() {
            hal::plot_led(
                point.to_index(),
                match life.get(point) {
                    life::Cell::Alive => hal::Rgb::new(0, 255, 0),
                    _ => hal::Rgb::new(0, 0, 0),
                },
            );
        }
    }

    /// Toggle the state of the cell at the point on the grid.
    fn toggle_cell(&self, point: hal::Point) {
        let mut life = self.life.lock();
        let toggled_state = !life.get(point);
        life.set(point, toggled_state);
    }

    /// Toggle whether the simulation is running.
    fn toggle_is_running(&self) {
        let mut is_running = self.is_running.lock();
        *is_running = !*is_running;
    }
}

/// Implement the event listener trait for our app in order to be notified of events that occur on
/// the Launchpad Pro hardware.
impl EventListener for App {
    fn timer_event(&self) {
        static TICKS: Mutex<i32> = Mutex::new(0);

        let mut ticks = TICKS.lock();
        if *ticks == TICKS_PER_FRAME {
            if *self.is_running.lock() {
                self.life.lock().tick();
                self.draw_universe();
            }
            *ticks = 0;
        } else {
            *ticks += 1;
        }
    }

    fn surface_event(&self, surface_event: SurfaceEvent) {
        if surface_event.value == hal::SurfaceEventValue::Release {
            match surface_event.surface_event_type {
                hal::SurfaceEventType::Pad => {
                    self.toggle_cell(surface_event.point);
                    self.draw_universe();
                },
                hal::SurfaceEventType::Setup => {
                    self.toggle_is_running();
                }
            }
        }
    }
}

/// Create a static instance of our app.
static APP: App = App::new();

/// Register the static instance of our app to receive events from the hardware.
register_event_listener!(APP);

#[cfg(target_device = "launchpad")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(target_device = "launchpad"))]
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use launchpad_pro_rs::hal::{SurfaceEventType, SurfaceEventValue};

    #[test]
    fn app_starts_paused_until_setup_button_is_pressed() {
        // create our app
        let app = App::new();

        // we expect that our newly created app will start paused
        assert_eq!(*app.is_running.lock(), false);

        // create a single cell that will immediately die once the simulation starts
        app.surface_event(SurfaceEvent {
            surface_event_type: SurfaceEventType::Pad,
            point: hal::Point::new(5, 5),
            value: SurfaceEventValue::Release
        });

        // expect that the cell we created is now alive
        assert_eq!(
            app.life.lock().get(hal::Point::new(5, 5)),
            life::Cell::Alive
        );

        // call the timer until the simulation is progressed by one tick (if it was running...)
        for _ in 0..TICKS_PER_FRAME {
            app.timer_event();
        }

        // check that our cell is still alive
        assert_eq!(
            app.life.lock().get(hal::Point::new(5, 5)),
            life::Cell::Alive
        );

        // press the setup button to unpause the simulation
        app.surface_event(SurfaceEvent {
            surface_event_type: SurfaceEventType::Setup,
            point: hal::Point::new(0, 9),
            value: SurfaceEventValue::Release
        });

        // check that our button press was registered
        assert_eq!(*app.is_running.lock(), true);

        // call the timer until the simulation is progressed by one tick
        for _ in 0..TICKS_PER_FRAME {
            app.timer_event();
        }

        // now that the simulation as started we expect that our solitary cell has died
        assert_eq!(
            app.life.lock().get(hal::Point::new(5, 5)),
            life::Cell::Dead
        );
    }
}