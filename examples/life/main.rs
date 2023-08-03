#![cfg_attr(target_arch = "arm", no_std)]
#![cfg_attr(target_arch = "arm", no_main)]

mod life;

use {
    launchpad_pro_rs::{
        hal::{self, LaunchpadApp},
        launchpad_app,
    },
    life::Life,
};

pub struct App {
    /// A flag to indicate whether the Game of Life simulation is running.
    is_running: bool,
    /// Our Game of Life state.
    life: Life,
}

impl Default for App {
    fn default() -> Self {
        Self {
            is_running: false,
            life: Life::new(),
        }
    }
}

impl App {
    /// Draw the Game of Life universe on the Launchpad Pro grid.
    fn draw_universe(&self) {
        for point in hal::Grid::points() {
            hal::surface::set_led(
                point,
                match self.life.get(point) {
                    life::Cell::Alive => hal::Rgb::GREEN,
                    life::Cell::Dead => hal::Rgb::BLACK,
                },
            );
        }
    }

    /// Move the simulation forward by one tick.
    fn tick(&mut self) {
        self.life.tick();
    }

    /// Toggle the state of the cell at the point on the grid.
    fn toggle_cell(&mut self, point: hal::Point) {
        let toggled_state = !self.life.get(point);
        self.life.set(point, toggled_state);
    }

    fn is_running(&self) -> bool {
        self.is_running
    }

    /// Toggle whether the simulation is running.
    fn toggle_is_running(&mut self) {
        self.is_running = !self.is_running;
    }
}

/// Implement the LaunchpadApp trait for our app in order to be notified of events that occur on
/// the Launchpad Pro hardware.
impl LaunchpadApp for App {
    fn init_event(&mut self, _pads: hal::surface::Pads) {}

    fn timer_event(&mut self) {
        if self.is_running() {
            self.tick();
            self.draw_universe();
        }
    }

    fn button_event(&mut self, button_event: hal::surface::ButtonEvent) {
        if let hal::surface::Event::Release = button_event.event {
            match button_event.button {
                hal::surface::Button::Pad(point) => {
                    self.toggle_cell(point);
                    self.draw_universe();
                }
                hal::surface::Button::Setup => {
                    self.toggle_is_running();
                }
            }
        }
    }
}

// Register our app to receive events from the hardware, with a timer running at 100 ms intervals.
launchpad_app!(App => 100 ms);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_starts_paused_until_setup_button_is_pressed() {
        // create our app
        let mut app = App::default();

        // we expect that our newly created app will start paused
        assert_eq!(app.is_running, false);

        // create a single cell that will immediately die once the simulation starts
        app.button_event(hal::surface::ButtonEvent {
            button: hal::surface::Button::Pad(hal::Point::new(5, 5)),
            event: hal::surface::Event::Release,
        });

        // expect that the cell we created is now alive
        assert_eq!(app.life.get(hal::Point::new(5, 5)), life::Cell::Alive);

        // progress the simulation
        app.timer_event();

        // check that our cell is still alive
        assert_eq!(app.life.get(hal::Point::new(5, 5)), life::Cell::Alive);

        // press the setup button to unpause the simulation
        app.button_event(hal::surface::ButtonEvent {
            button: hal::surface::Button::Setup,
            event: hal::surface::Event::Release,
        });

        // check that our button press was registered
        assert_eq!(app.is_running, true);

        // progress the simulation
        app.timer_event();

        // now that the simulation as started we expect that our solitary cell has died
        assert_eq!(app.life.get(hal::Point::new(5, 5)), life::Cell::Dead);
    }
}
