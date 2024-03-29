use core::ops::Add;

#[doc(hidden)]
pub use spin::Mutex;

#[cfg(target_arch = "arm")]
extern "C" {
    fn hal_plot_led(t: u8, index: u8, red: u8, green: u8, blue: u8);
    fn hal_read_led(t: u8, index: u8, red: *mut u8, green: *mut u8, blue: *mut u8);
    fn hal_send_midi(port: u8, status: u8, data1: u8, data2: u8);
    fn hal_send_sysex(port: u8, data: *const u8, length: u16);
}

#[cfg(not(target_arch = "arm"))]
unsafe fn hal_plot_led(t: u8, index: u8, red: u8, green: u8, blue: u8) {
    println!(
        "plot_led, type: {}, index: {}, color: ({}, {}, {})",
        t, index, red, green, blue
    );
}

#[cfg(not(target_arch = "arm"))]
unsafe fn hal_read_led(t: u8, index: u8, _red: *mut u8, _green: *mut u8, _blue: *mut u8) {
    println!("read_led, type: {}, index: {}", t, index);
}

#[cfg(not(target_arch = "arm"))]
unsafe fn hal_send_midi(port: u8, status: u8, data1: u8, data2: u8) {
    println!(
        "send_midi, port: {}, status: {}, data: ({}, {})",
        port, status, data1, data2
    );
}

#[cfg(not(target_arch = "arm"))]
unsafe fn hal_send_sysex(port: u8, _data: *const u8, length: u16) {
    println!("send_sysex, port: {}, length: {}", port, length);
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
    ///
    /// # Example
    ///
    /// ```
    /// use launchpad_pro_rs::hal::{Grid, Rgb, Red, Green, Blue};
    /// use launchpad_pro_rs::hal::surface::set_led;
    ///
    /// // set every led on the grid to blue
    /// for point in Grid::points() {
    ///     set_led(point, Rgb::new(Red::new(0), Green::new(0), Blue::new(255)));
    /// }
    ///
    /// ```
    pub fn points() -> impl Iterator<Item = Point> {
        (0..Grid::size()).map(Point::from_index)
    }
}

/// An RGB color.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rgb(Red, Green, Blue);

/// The red component of an RGB color.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Red(u8);

impl Red {
    pub const fn new(red: u8) -> Self {
        Self(convert_to_6_bit(red))
    }
}

/// The green component of an RGB color.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Green(u8);

impl Green {
    pub const fn new(green: u8) -> Self {
        Self(convert_to_6_bit(green))
    }
}

/// The blue component of an RGB color.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Blue(u8);

impl Blue {
    pub const fn new(blue: u8) -> Self {
        Self(convert_to_6_bit(blue))
    }
}

/// Map an 8-bit value to a 6-bit range.
const fn convert_to_6_bit(led: u8) -> u8 {
    ((63 * (led as u16)) / (255)) as u8
}

impl Rgb {
    /// Construct a new 18-bit RGB color. The arguments can be in the range `[0, 255]` but will be
    /// mapped and stored as 6-bits internally.
    ///
    /// # Example
    ///
    /// ```
    /// use launchpad_pro_rs::hal::{Rgb, Red, Green, Blue};
    ///
    /// let red = Rgb::new(Red::new(255), Green::new(0), Blue::new(0));
    /// let green = Rgb::new(Red::new(0), Green::new(255), Blue::new(0));
    /// let blue = Rgb::new(Red::new(0), Green::new(0), Blue::new(255));
    /// ```
    pub const fn new(red: Red, green: Green, blue: Blue) -> Self {
        Rgb(red, green, blue)
    }

    pub const RED: Self = Self(Red::new(255), Green::new(0), Blue::new(0));
    pub const GREEN: Self = Self(Red::new(0), Green::new(255), Blue::new(0));
    pub const BLUE: Self = Self(Red::new(0), Green::new(0), Blue::new(255));
    pub const YELLOW: Self = Self(Red::new(255), Green::new(255), Blue::new(0));
    pub const CYAN: Self = Self(Red::new(0), Green::new(255), Blue::new(255));
    pub const MAGENTA: Self = Self(Red::new(255), Green::new(0), Blue::new(255));
    pub const WHITE: Self = Self(Red::new(255), Green::new(255), Blue::new(255));
    pub const BLACK: Self = Self(Red::new(0), Green::new(0), Blue::new(0));
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
    pub const fn new(x: i8, y: i8) -> Self {
        Point {
            x: (((x % Grid::width() as i8) + Grid::width() as i8) % Grid::width() as i8),
            y: (((y % Grid::height() as i8) + Grid::height() as i8) % Grid::height() as i8),
        }
    }

    /// Returns the points x coordinate.
    pub const fn x(&self) -> i8 {
        self.x
    }

    /// Returns the points y coordinate.
    pub const fn y(&self) -> i8 {
        self.y
    }

    /// Construct a point from an index in the range `[0, 100)`. Any index outside this range will
    /// be wrapped.
    ///
    /// # Example
    ///
    /// ```
    /// use launchpad_pro_rs::hal::Point;
    ///
    /// let point = Point::from_index(55);
    /// assert_eq!(point, Point::new(5, 5));
    ///
    /// let point = Point::from_index(100);
    /// assert_eq!(point, Point::new(0, 0));
    /// ```
    pub const fn from_index(index: u8) -> Self {
        Point::new((index % 10) as i8, (index / 10) as i8)
    }

    /// Returns an index in the range `[0, 100)` corresponding to this point.
    ///
    /// # Example
    ///
    /// ```
    /// use launchpad_pro_rs::hal::Point;
    ///
    /// let point = Point::new(5, 5);
    /// assert_eq!(point.to_index(), 55);
    ///
    /// let point = Point::new(10, 10);
    /// assert_eq!(point.to_index(), 0);
    /// ```
    pub const fn to_index(&self) -> u8 {
        ((self.y * Grid::height() as i8) + self.x) as u8
    }
}

/// Respond to events on the Launchpad Pro surface and control the LEDs.
pub mod surface {
    use crate::hal::{Blue, Green, Point, Red, Rgb};

    /// Set the colour of an LED on the grid.
    ///
    /// # Example
    ///
    /// ```
    ///
    /// use launchpad_pro_rs::hal::surface::set_led;
    /// use launchpad_pro_rs::hal::{Point, Rgb, Red, Green, Blue};
    ///
    /// set_led(Point::new(5, 5), Rgb::new(Red::new(255), Green::new(127), Blue::new(0)));
    /// ```
    pub fn set_led(point: Point, Rgb(Red(red), Green(green), Blue(blue)): Rgb) {
        if point.to_index() < super::Grid::size() {
            unsafe {
                super::hal_plot_led(0, point.to_index(), red, green, blue);
            };
        }
    }

    /// Read the color of an LED on the grid.
    ///
    /// # Example
    ///
    /// ```
    /// use launchpad_pro_rs::hal::surface::read_led;
    /// use launchpad_pro_rs::hal::Point;
    ///
    /// let color = read_led(Point::new(0, 0));
    /// ```
    pub fn read_led(point: Point) -> Option<Rgb> {
        if point.to_index() < super::Grid::size() {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            unsafe {
                super::hal_read_led(0, point.to_index(), &mut red, &mut green, &mut blue);
            };
            return Some(Rgb(Red(red), Green(green), Blue(blue)));
        }
        None
    }

    /// The types of button on the surface of the Launchpad Pro.
    pub enum Button {
        /// A pad button.
        Pad(Point),
        /// The setup button.
        Setup,
    }

    /// The types of event that can occur on a button.
    pub enum Event {
        /// A button has been pressed. Contains the value of the button press.
        Press(u8),
        /// A button has been released.
        Release,
    }

    /// Button events occur when a button is pressed or released on the Launchpad Pro.
    pub struct ButtonEvent {
        /// The button that was pressed or released.
        pub button: Button,
        /// Whether the button was pressed or released.
        pub event: Event,
    }

    /// Aftertouch events occur when an aftertouch (pad pressure) event is reported.
    pub struct AftertouchEvent {
        pub point: Point,
        pub value: u8,
    }

    /// A wrapper around the raw ADC pointer to allow reading values from the pads.
    pub struct Pads {
        adc: *const u16,
    }

    unsafe impl Sync for Pads {}

    impl Pads {
        /// Construct a new Pads instance from a raw ADC pointer.
        pub const fn new(adc: *const u16) -> Self {
            Self { adc }
        }

        /// Read a 12-bit value from a pad at a given point on the grid. If there isn't a pad at the
        /// point provided then this function will return None.
        pub fn read(&self, pos: Point) -> Option<u16> {
            Self::point_to_offset(pos).map(|offset| unsafe { *self.adc.add(offset) })
        }

        /// For technical reasons the offsets from the ADC pointer use a slightly odd scheme.
        /// This function converts points in the grid to offsets into this ADC pointer corresponding
        /// to that point. If there isn't a pad at the point provided then this function will return
        /// None.
        const fn point_to_offset(pos: Point) -> Option<usize> {
            if pos.y >= 1 && pos.y <= 4 && pos.x >= 1 && pos.x <= 8 {
                let y_offset = (pos.y - 1) * 16;
                let x_offset = (pos.x - 1) * 2;
                Some((x_offset + y_offset) as usize)
            } else if pos.y >= 5 && pos.y <= 8 && pos.x >= 1 && pos.x <= 8 {
                let y_offset = (pos.y - 5) * 16;
                let x_offset = (pos.x - 1) * 2;
                Some((x_offset + y_offset + 1) as usize)
            } else {
                None
            }
        }
    }

    #[test]
    fn adc_offset_calculation() {
        assert_eq!(Pads::point_to_offset(Point::new(0, 0)), None);
        assert_eq!(Pads::point_to_offset(Point::new(1, 1)), Some(0));
        assert_eq!(Pads::point_to_offset(Point::new(2, 2)), Some(18));
        assert_eq!(Pads::point_to_offset(Point::new(3, 3)), Some(36));
        assert_eq!(Pads::point_to_offset(Point::new(4, 4)), Some(54));
        assert_eq!(Pads::point_to_offset(Point::new(5, 5)), Some(9));
        assert_eq!(Pads::point_to_offset(Point::new(6, 6)), Some(27));
        assert_eq!(Pads::point_to_offset(Point::new(7, 7)), Some(45));
        assert_eq!(Pads::point_to_offset(Point::new(8, 8)), Some(63));
        assert_eq!(Pads::point_to_offset(Point::new(9, 9)), None);
        assert_eq!(Pads::point_to_offset(Point::new(8, 1)), Some(14));
        assert_eq!(Pads::point_to_offset(Point::new(4, 3)), Some(38));
        assert_eq!(Pads::point_to_offset(Point::new(3, 6)), Some(21));
        assert_eq!(Pads::point_to_offset(Point::new(1, 8)), Some(49));
        assert_eq!(Pads::point_to_offset(Point::new(4, 4)), Some(54));
        assert_eq!(Pads::point_to_offset(Point::new(5, 5)), Some(9));
        assert_eq!(Pads::point_to_offset(Point::new(11, 11)), Some(0));
        assert_eq!(Pads::point_to_offset(Point::new(7, 5)), Some(13));
        assert_eq!(Pads::point_to_offset(Point::new(8, 8)), Some(63));
        assert_eq!(Pads::point_to_offset(Point::new(10, 10)), None);
        assert_eq!(Pads::point_to_offset(Point::from_index(11)), Some(0));
        assert_eq!(Pads::point_to_offset(Point::from_index(51)), Some(1));
        assert_eq!(Pads::point_to_offset(Point::from_index(12)), Some(2));
        assert_eq!(Pads::point_to_offset(Point::from_index(52)), Some(3));
        assert_eq!(Pads::point_to_offset(Point::from_index(0)), None);
    }

    #[test]
    fn read_adc_value() {
        let mut values = [0u16; 64];
        let pads = Pads::new(values.as_ptr());

        assert_eq!(pads.read(Point::new(0, 0)), None);

        values[16] = 7;
        assert_eq!(pads.read(Point::new(1, 2)), Some(7));

        values[16] = 34;
        assert_eq!(pads.read(Point::new(1, 2)), Some(34));
    }
}

/// Send and receive MIDI messages.
pub mod midi {
    /// The MIDI ports available on the Launchpad Pro.
    pub enum Port {
        Standalone = 0,
        USB = 1,
        DIN = 2,
    }

    /// A MIDI message.
    pub struct Message {
        pub status: u8,
        pub data: (u8, u8),
    }

    impl Message {
        /// Construct a new MIDI message.
        pub const fn new(status: u8, data: (u8, u8)) -> Self {
            Self { status, data }
        }
    }

    /// The MIDI DIN socket types available.
    pub enum Cable {
        MidiIn,
        MidiOut,
    }

    /// The events that can occur for the MIDI DIN sockets.
    pub enum CableEvent {
        Connect(Cable),
        Disconnect(Cable),
    }

    /// Send a MIDI message to one of the ports available on the device.
    ///
    /// # Example
    ///
    /// ```
    /// use launchpad_pro_rs::hal::midi::{send_message, Port, Message};
    ///
    /// send_message(Port::DIN, Message::new(0x90, (60, 127)));
    /// ```
    pub fn send_message(port: Port, message: Message) {
        unsafe {
            super::hal_send_midi(port as u8, message.status, message.data.0, message.data.1);
        }
    }

    /// Send a SysEx message to one of the ports on the device.
    /// The caller is responsible for ensuring that the message is correctly formatted:
    ///     - Starts with 0xF0 and ends with 0xF7.
    /// The message must not exceed 320 bytes. Messages longer than 320 bytes will be discarded.
    ///
    /// # Example
    ///
    /// ```
    /// use launchpad_pro_rs::hal::midi::{send_sysex, Port};
    ///
    /// let sysex_message = [0xF0, 0xDE, 0xAD, 0xBE, 0xEF, 0xF7];
    /// send_sysex(Port::USB, &sysex_message);
    /// ```
    pub fn send_sysex(port: Port, data: &[u8]) {
        if data.len() <= 320 {
            unsafe {
                crate::hal::hal_send_sysex(port as u8, data.as_ptr(), data.len() as u16);
            }
        }
    }
}

/// The LaunchpadApp trait can be implemented to receive events from the Launchpad Pro hardware.
pub trait LaunchpadApp {
    /// Called on startup.
    fn init_event(&mut self, _pads: surface::Pads) {}
    /// A timer. The interval is specified in milliseconds when using the `launchpad_app` macro.
    fn timer_event(&mut self) {}
    /// Called when a MIDI message is received from USB or DIN.
    fn midi_event(&mut self, _port: midi::Port, _midi_event: midi::Message) {}
    /// Called when a SysEx message is received from USB or DIN.
    fn sysex_event(&mut self, _port: midi::Port, _data: &[u8]) {}
    /// Called when a MIDI DIN cable is connected or disconnected.
    fn cable_event(&mut self, _cable_event: midi::CableEvent) {}
    /// Called when the user presses or releases a button or pad on the surface.
    fn button_event(&mut self, _button_event: surface::ButtonEvent) {}
    /// Called when an aftertouch (pad pressure) event is reported by the low level firmware.
    fn aftertouch_event(&mut self, _aftertouch_event: surface::AftertouchEvent) {}
}

/// Register a type that implements the [`LaunchpadApp`] trait to receive event
/// notifications from the Launchpad Pro hardware. The type must implement the [`Default`] trait,
/// as it will be instantiated when the hardware is initialised.
///
/// This macro should only be called once.
///
/// # Example
///
/// ```
/// use launchpad_pro_rs::{
///     launchpad_app,
///     hal::{midi::{Message, Port, CableEvent},
///     surface::{Pads, set_led, AftertouchEvent, ButtonEvent},
///     LaunchpadApp, Point, Rgb, Red, Green, Blue}
/// };
///
/// #[derive(Default)]
/// pub struct App; // define our app type
///
/// impl LaunchpadApp for App { // implement the LaunchpadApp trait for our app
///     fn init_event(&mut self, _: Pads) {
///         // when the Launchpad is initialised we will set a white LED at the center of the grid
///         set_led(Point::new(5, 5), Rgb::new(Red::new(255), Green::new(255), Blue::new(255)));
///     }
/// }
///
/// // Register our app with the hardware.
/// // The timer interval is specified in milliseconds, and determines how often the `timer_event`
/// // method will be called.
/// launchpad_app!(App => 100 ms);
/// ```
#[macro_export]
macro_rules! launchpad_app {
    ($t:ty => $timer_interval:literal ms) => {
        static __LAUNCHPAD_APP: $crate::hal::Mutex<Option<$t>> = $crate::hal::Mutex::new(None);

        #[no_mangle]
        pub extern "C" fn app_init(adc: *const u16) {
            let mut app = __LAUNCHPAD_APP.lock();
            if app.is_none() {
                let app = app.insert(<$t>::default());
                $crate::hal::app_init_event(app, adc);
            }
        }

        #[no_mangle]
        pub extern "C" fn app_surface_event(event: u8, index: u8, value: u8) {
            let mut app = __LAUNCHPAD_APP.lock();
            if let Some(app) = app.as_mut() {
                $crate::hal::app_surface_event(app, event, index, value);
            }
        }

        #[no_mangle]
        pub extern "C" fn app_midi_event(port: u8, status: u8, data1: u8, data2: u8) {
            let mut app = __LAUNCHPAD_APP.lock();
            if let Some(app) = app.as_mut() {
                $crate::hal::app_midi_event(app, port, status, data1, data2);
            }
        }

        #[no_mangle]
        pub extern "C" fn app_sysex_event(port: u8, data: *mut u8, count: u16) {
            let mut app = __LAUNCHPAD_APP.lock();
            if let Some(app) = app.as_mut() {
                $crate::hal::app_sysex_event(app, port, data, count);
            }
        }

        #[no_mangle]
        pub extern "C" fn app_aftertouch_event(index: u8, value: u8) {
            let mut app = __LAUNCHPAD_APP.lock();
            if let Some(app) = app.as_mut() {
                $crate::hal::app_aftertouch_event(&mut *app, index, value);
            }
        }

        #[no_mangle]
        extern "C" fn app_cable_event(cable_type: u8, value: u8) {
            let mut app = __LAUNCHPAD_APP.lock();
            if let Some(app) = app.as_mut() {
                $crate::hal::app_cable_event(app, cable_type, value);
            }
        }

        #[no_mangle]
        pub extern "C" fn app_timer_event() {
            static mut ONE_MS_TICKS: i32 = 0;

            unsafe {
                if ONE_MS_TICKS == $timer_interval {
                    ONE_MS_TICKS = 0;
                } else {
                    ONE_MS_TICKS += 1;
                    return;
                }
            }

            // If the timer is running too fast we can get too much contention on the lock, which
            // grinds everything to a halt, so we only run the timer event if the lock is free.
            if let Some(mut app) = __LAUNCHPAD_APP.try_lock() {
                if let Some(app) = app.as_mut() {
                    $crate::hal::app_timer_event(app);
                }
            }
        }

        #[cfg(target_arch = "arm")]
        #[panic_handler]
        fn panic(_info: &core::panic::PanicInfo) -> ! {
            loop {}
        }

        #[cfg(not(target_arch = "arm"))]
        fn main() {}
    };
}

#[doc(hidden)]
pub fn app_init_event(app: &mut dyn LaunchpadApp, adc: *const u16) {
    app.init_event(surface::Pads::new(adc));
}

#[doc(hidden)]
pub fn app_surface_event(app: &mut dyn LaunchpadApp, event: u8, index: u8, value: u8) {
    app.button_event(surface::ButtonEvent {
        button: if event == 1 {
            surface::Button::Setup
        } else {
            surface::Button::Pad(Point::from_index(index))
        },
        event: if value == 0 {
            surface::Event::Release
        } else {
            surface::Event::Press(value)
        },
    });
}

#[doc(hidden)]
pub fn app_midi_event(app: &mut dyn LaunchpadApp, port: u8, status: u8, data1: u8, data2: u8) {
    let port = match port {
        0 => Some(midi::Port::Standalone),
        1 => Some(midi::Port::USB),
        2 => Some(midi::Port::DIN),
        _ => None,
    };

    if let Some(port) = port {
        app.midi_event(
            port,
            midi::Message {
                status,
                data: (data1, data2),
            },
        );
    }
}

#[doc(hidden)]
pub fn app_sysex_event(app: &mut dyn LaunchpadApp, port: u8, data: *mut u8, count: u16) {
    let port = match port {
        0 => Some(midi::Port::Standalone),
        1 => Some(midi::Port::USB),
        2 => Some(midi::Port::DIN),
        _ => None,
    };

    if let Some(port) = port {
        let slice = unsafe { core::slice::from_raw_parts(data, count as usize) };
        app.sysex_event(port, slice);
    }
}

#[doc(hidden)]
pub fn app_aftertouch_event(app: &mut dyn LaunchpadApp, index: u8, value: u8) {
    app.aftertouch_event(surface::AftertouchEvent {
        point: Point::from_index(index),
        value,
    });
}

#[doc(hidden)]
pub fn app_cable_event(app: &mut dyn LaunchpadApp, cable_type: u8, value: u8) {
    let cable_type = match cable_type {
        0 => Some(midi::Cable::MidiIn),
        1 => Some(midi::Cable::MidiOut),
        _ => None,
    };

    if let Some(cable_type) = cable_type {
        app.cable_event(match value {
            0 => midi::CableEvent::Disconnect(cable_type),
            _ => midi::CableEvent::Connect(cable_type),
        });
    }
}

#[doc(hidden)]
pub fn app_timer_event(app: &mut dyn LaunchpadApp) {
    app.timer_event();
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

    #[test]
    fn colors_get_converted_to_18_bit() {
        let Rgb(Red(red), Green(green), Blue(blue)) =
            Rgb::new(Red::new(255), Green::new(127), Blue::new(63));
        assert_eq!(red, 63);
        assert_eq!(green, 31);
        assert_eq!(blue, 15);
    }
}
