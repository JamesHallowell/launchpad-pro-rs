#[macro_export]
macro_rules! hardware_app {
    ($app:expr) => {
        #[no_mangle]
        pub static __LAUNCHPAD_APP: &dyn $crate::hal::LaunchpadApp = &$app;

        #[no_mangle]
        pub extern "C" fn app_init(adc: *const u16) {
            $crate::hal::set_listener(__LAUNCHPAD_APP);
            __LAUNCHPAD_APP.init_event($crate::hal::surface::Pads::new(Some(adc)));
        }

        #[cfg(target_arch="arm")]
        #[panic_handler]
        fn panic(_info: &core::panic::PanicInfo) -> ! {
            loop {}
        }

        #[cfg(not(target_arch="arm"))]
        fn main() {}
    };
}