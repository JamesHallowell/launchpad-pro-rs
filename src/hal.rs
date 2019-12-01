use core::ops::Add;

#[cfg(target_device = "launchpad")]
extern "C" {
    fn hal_plot_led(t: u8, index: u8, red: u8, green: u8, blue: u8);
}

/// The Launchpad Pro grid.
pub struct Grid;

impl Grid {
    /// The width of the grid.
    pub const fn width() -> u8 {
        10
    }

    /// The height of the grid.
    pub const fn height() -> u8 {
        10
    }

    /// The area of the grid.
    pub const fn size() -> u8 {
        Grid::width() * Grid::height()
    }

    /// Returns an iterator over all the points in the grid.
    pub fn points() -> impl Iterator<Item = Point> {
        (0..Grid::size()).map(Point::from_index)
    }
}

/// An RGB color.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rgb(u8, u8, u8);

impl Rgb {
    const MAX_LED: u8 = 63;
}

fn clamp_led(led: u8) -> u8 {
    if led > Rgb::MAX_LED {
        Rgb::MAX_LED
    } else {
        led
    }
}

impl Rgb {
    /// Construct a new RGB color. The color channels have a valid range of [0, 64). Any values
    /// outside this range will be clamped.
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Rgb(clamp_led(red), clamp_led(green), clamp_led(blue))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// A point on the Launchpad Pro grid.
pub struct Point {
    x: i8,
    y: i8,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl Point {
    /// Construct a new point. Coordinates that exceed the bounds of the grid will be wrapped.
    pub fn new(x: i8, y: i8) -> Self {
        Point {
            x: (((x % Grid::width() as i8) + Grid::width() as i8) % Grid::width() as i8),
            y: (((y % Grid::height() as i8) + Grid::height() as i8) % Grid::height() as i8),
        }
    }

    /// Returns the points x coordinate.
    pub fn x(&self) -> i8 {
        self.x
    }

    /// Returns the points y coordinate.
    pub fn y(&self) -> i8 {
        self.y
    }

    /// Construct a point from an index in the range [0, 100). Any index outside this range will be
    /// wrapped.
    pub fn from_index(index: u8) -> Self {
        Point::new((index % 10) as i8, (index / 10) as i8)
    }

    /// Returns an index in the range [0, 100) corresponding to this point.
    pub fn to_index(&self) -> u8 {
        ((self.y * Grid::height() as i8) + self.x) as u8
    }
}

/// Set the colour of an LED on the grid.
pub fn plot_led(index: u8, rgb: Rgb) {
    #[cfg(target_device = "launchpad")]
    unsafe {
        if index < Grid::size() {
            hal_plot_led(0, index, rgb.0, rgb.1, rgb.2);
        }
    }
}

pub trait EventHandler: Sync {
    fn init_event(&self);
    fn timer_event(&self);
    fn midi_event(&self);
    fn sysex_event(&self);
    fn cable_event(&self);
    fn surface_event(&self);
    fn aftertouch_event(&self);
}

#[macro_export]
macro_rules! register_event_handler {
    ($handler:expr) => {
        #[no_mangle]
        pub static EVENT_HANDLER: &dyn EventHandler = &$handler;

        #[no_mangle]
        pub extern "C" fn app_surface_event(event: u8, index: u8, value: u8) {
            EVENT_HANDLER.surface_event();
        }
        #[no_mangle]
        pub extern "C" fn app_midi_event(port: u8, status: u8, value1: u8, value2: u8) {
            EVENT_HANDLER.midi_event();
        }

        #[no_mangle]
        pub extern "C" fn app_sysex_event(port: u8, data: *mut u8, count: u16) {
            EVENT_HANDLER.sysex_event();
        }

        #[no_mangle]
        pub extern "C" fn app_aftertouch_event(_index: u8, _value: u8) {
            EVENT_HANDLER.aftertouch_event();
        }

        #[no_mangle]
        extern "C" fn app_cable_event(_event: u8, _value: u8) {
            EVENT_HANDLER.cable_event();
        }

        #[no_mangle]
        pub extern "C" fn app_timer_event() {
            EVENT_HANDLER.timer_event();
        }

        #[no_mangle]
        pub extern "C" fn app_init(_adc_raw: *const u16) {
            EVENT_HANDLER.init_event();
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_construct_point() {
        let p = Point::new(7, 3);
        assert_eq!(p.x(), 7);
        assert_eq!(p.y(), 3);
        assert_eq!(p.to_index(), 37);
    }

    #[test]
    fn points_are_wrapped() {
        let p = Point::new(-1, -1);
        assert_eq!(p.x(), 9);
        assert_eq!(p.y(), 9);
        assert_eq!(p.to_index(), 99);
    }

    #[test]
    fn can_construct_point_from_index() {
        let p = Point::from_index(42);
        assert_eq!(p.x(), 2);
        assert_eq!(p.y(), 4);
    }

    #[test]
    fn can_add_points() {
        let p = Point::new(0, 0).add(Point::new(-1, -1));
        assert_eq!(p.x(), 9);
        assert_eq!(p.y(), 9);
        assert_eq!(p.to_index(), 99);
    }

    #[test]
    fn can_iterate_over_all_grid_points() {
        assert_eq!(Grid::points().count(), Grid::size() as usize);

        let mut points = Grid::points();
        assert_eq!(points.next().unwrap(), Point::new(0, 0));
        assert_eq!(points.next().unwrap(), Point::new(1, 0));
        assert_eq!(points.next().unwrap(), Point::new(2, 0));
        assert_eq!(points.next().unwrap(), Point::new(3, 0));
        assert_eq!(points.next().unwrap(), Point::new(4, 0));
        assert_eq!(points.next().unwrap(), Point::new(5, 0));
        assert_eq!(points.next().unwrap(), Point::new(6, 0));
        assert_eq!(points.next().unwrap(), Point::new(7, 0));
        assert_eq!(points.next().unwrap(), Point::new(8, 0));
        assert_eq!(points.next().unwrap(), Point::new(9, 0));
        assert_eq!(points.next().unwrap(), Point::new(0, 1));
        assert_eq!(points.next().unwrap(), Point::new(1, 1));
        // ... and so on
    }
}
