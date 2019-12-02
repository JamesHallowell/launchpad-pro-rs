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

pub enum SurfaceEventType {
    Pad,
    Setup,
}

#[derive(PartialEq)]
pub enum SurfaceEventValue {
    Press(u8),
    Release,
}

pub struct SurfaceEvent {
    pub surface_event_type: SurfaceEventType,
    pub point: Point,
    pub value: SurfaceEventValue,
}

pub struct AftertouchEvent {
    pub point: Point,
    pub value: u8,
}

pub trait EventListener: Sync {
    fn init_event(&self) {}
    fn timer_event(&self) {}
    fn midi_event(&self, port: midi::Port, midi_event: midi::Message) {}
    fn sysex_event(&self, port: midi::Port, data: &[u8]) {}
    fn cable_event(&self, cable_event: midi::CableEvent) {}
    fn surface_event(&self, surface_event: SurfaceEvent) {}
    fn aftertouch_event(&self, aftertouch_event: AftertouchEvent) {}
}

pub mod midi {
    pub enum Port {
        Standalone,
        USB,
        DIN,
    }

    pub struct Message {
        pub status: u8,
        pub data: (u8, u8),
    }

    pub enum Cable {
        MidiIn,
        MidiOut,
    }

    pub enum CableEvent {
        Connect(Cable),
        Disconnect(Cable),
    }
}

#[macro_export]
macro_rules! register_event_listener {
    ($handler:expr) => {
        #[no_mangle]
        pub static EVENT_LISTENER: &dyn EventListener = &$handler;

        #[no_mangle]
        pub extern "C" fn app_surface_event(event: u8, index: u8, value: u8) {
            EVENT_LISTENER.surface_event($crate::hal::SurfaceEvent {
                surface_event_type: if event == 1 {
                    $crate::hal::SurfaceEventType::Setup
                } else {
                    $crate::hal::SurfaceEventType::Pad
                },
                point: $crate::hal::Point::from_index(index),
                value: if value == 0 {
                    $crate::hal::SurfaceEventValue::Release
                } else {
                    $crate::hal::SurfaceEventValue::Press(value)
                },
            });
        }

        #[no_mangle]
        pub extern "C" fn app_midi_event(port: u8, status: u8, data1: u8, data2: u8) {
            let port = match port {
                0 => Some($crate::hal::midi::Port::Standalone),
                1 => Some($crate::hal::midi::Port::USB),
                2 => Some($crate::hal::midi::Port::DIN),
                _ => None,
            };

            if let Some(port) = port {
                EVENT_LISTENER.midi_event(port, $crate::hal::midi::Message {
                    status,
                    data: (data1, data2),
                });
            }
        }

        #[no_mangle]
        pub extern "C" fn app_sysex_event(port: u8, data: *mut u8, count: u16) {
            let port = match port {
                0 => Some($crate::hal::midi::Port::Standalone),
                1 => Some($crate::hal::midi::Port::USB),
                2 => Some($crate::hal::midi::Port::DIN),
                _ => None,
            };

            if let Some(port) = port {
                let data = unsafe { core::slice::from_raw_parts(data, count as usize) };
                EVENT_LISTENER.sysex_event(port, data);
            }
        }

        #[no_mangle]
        pub extern "C" fn app_aftertouch_event(index: u8, value: u8) {
            EVENT_LISTENER.aftertouch_event($crate::hal::AftertouchEvent {
                point: $crate::hal::Point::from_index(index),
                value,
            });
        }

        #[no_mangle]
        extern "C" fn app_cable_event(cable_type: u8, value: u8) {
            let cable_type = match cable_type {
                0 => Some($crate::hal::midi::Cable::MidiIn),
                1 => Some($crate::hal::midi::Cable::MidiOut),
                _ => None,
            };

            if let Some(cable_type) = cable_type {
                EVENT_LISTENER.cable_event(match value {
                    0 => $crate::hal::midi::CableEvent::Disconnect(cable_type),
                    _ => $crate::hal::midi::CableEvent::Connect(cable_type),
                });
            }
        }

        #[no_mangle]
        pub extern "C" fn app_timer_event() {
            EVENT_LISTENER.timer_event();
        }

        #[no_mangle]
        pub extern "C" fn app_init(_adc_raw: *const u16) {
            EVENT_LISTENER.init_event();
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
