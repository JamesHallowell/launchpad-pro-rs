#[macro_export]
macro_rules! wasm_app {
    ($app:expr) => {
        #[no_mangle]
        pub static __LAUNCHPAD_APP: &dyn $crate::hal::LaunchpadApp = &$app;
    };
}