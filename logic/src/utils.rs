pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[macro_export]
macro_rules! global {
    ($name:expr) => {
        unsafe { $name }
    };
}
