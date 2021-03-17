pub fn set_panic(){
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}