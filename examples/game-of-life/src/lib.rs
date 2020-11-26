#![cfg_attr(target_arch="arm", no_std)]
#![cfg_attr(target_arch="arm", no_main)]

mod life;

use launchpad_pro_hal::hal;
use launchpad_pro_hal::hal::LaunchpadApp;
use launchpad_pro_hal::launchpad_app;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

use life::Life;

/// The Launchpad Pro app state.
struct State {
    /// A flag to indicate whether the Game of Life simulation is running.
    is_running: bool,
    /// Our Game of Life state.
    life: Life,
}

impl State {
    /// Create the app.
    const fn new() -> Self {
        Self {
            is_running: false,
            life: Life::new(),
        }
    }

    /// Draw the Game of Life universe on the Launchpad Pro grid.
    fn draw_universe(&self) {
        for point in hal::Grid::points() {
            self.draw_cell(point);
        }
    }

    fn draw_cell(&self, point: hal::Point) {
        hal::surface::set_led(
            point,
            match self.life.get(point) {
                life::Cell::Alive => hal::Rgb::GREEN,
                life::Cell::Dead => hal::Rgb::BLACK,
            },
        );
    }

    /// Move the simulation forward by one tick.
    fn tick(&mut self) {
        self.life.tick();
    }

    /// Toggle the state of the cell at the point on the grid.
    fn toggle_cell(&mut self, point: hal::Point) {
        let toggled_state = !self.life.get(point);
        self.life.set(point, toggled_state);
        self.draw_cell(point);
    }

    /// Returns true if the simulation is currently running.
    fn is_running(&self) -> bool {
        self.is_running
    }

    /// Toggle whether the simulation is running.
    fn toggle_is_running(&mut self) {
        self.is_running = ! self.is_running;
    }
}

struct App {
    state: hal::Mutex<State>
}

impl App {
    const fn new() -> Self {
        Self {
            state: hal::Mutex::new(State::new())
        }
    }
}

/// The number of frames per second in our simulation.
const FRAMES_PER_SECOND: i32 = 10;
/// The number of timer ticks per frame. Timer ticks happen at a frequency of ~1ms.
const TICKS_PER_FRAME: i32 = 1000 / FRAMES_PER_SECOND;

/// Implement the LaunchpadApp trait for our app in order to be notified of events that occur on
/// the Launchpad Pro hardware.
impl LaunchpadApp for App {
    fn timer_event(&self) {
        /// A count of the number of timer callbacks.
        static mut TICKS: i32 = 0;

        unsafe {
            if TICKS == TICKS_PER_FRAME {
                let mut state = self.state.lock();
                if state.is_running() {
                    state.tick();
                    state.draw_universe();
                }
                TICKS = 0;
            } else {
                TICKS += 1;
            }
        }
    }

    fn button_event(&self, button_event: hal::surface::ButtonEvent) {
        if let hal::surface::Event::Release = button_event.event {
            let mut state = self.state.lock();

            match button_event.button {
                hal::surface::Button::Pad(point) => {
                    state.toggle_cell(point);
                }
                hal::surface::Button::Setup => {
                    state.toggle_is_running();
                }
            }
        }
    }
}

/// Create a static instance of our app.
static APP: App = App::new();

// Register our app to receive events from the hardware.
launchpad_app!(APP);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_starts_paused_until_setup_button_is_pressed() {
        // create our app
        let app = App::new();

        // we expect that our newly created app will start paused
        assert_eq!(app.state.lock().is_running, false);

        // create a single cell that will immediately die once the simulation starts
        app.button_event(hal::surface::ButtonEvent {
            button: hal::surface::Button::Pad(hal::Point::new(5, 5)),
            event: hal::surface::Event::Release,
        });

        // expect that the cell we created is now alive
        assert_eq!(
            app.state.lock().life.get(hal::Point::new(5, 5)),
            life::Cell::Alive
        );

        // call the timer until the simulation is progressed by one tick (if it was running...)
        for _ in 0..TICKS_PER_FRAME {
            app.timer_event();
        }

        // check that our cell is still alive
        assert_eq!(
            app.state.lock().life.get(hal::Point::new(5, 5)),
            life::Cell::Alive
        );

        // press the setup button to unpause the simulation
        app.button_event(hal::surface::ButtonEvent {
            button: hal::surface::Button::Setup,
            event: hal::surface::Event::Release,
        });

        // check that our button press was registered
        assert_eq!(app.state.lock().is_running, true);

        // call the timer until the simulation is progressed by one tick
        for _ in 0..TICKS_PER_FRAME {
            app.timer_event();
        }

        // now that the simulation as started we expect that our solitary cell has died
        assert_eq!(app.state.lock().life.get(hal::Point::new(5, 5)), life::Cell::Dead);
    }
}
