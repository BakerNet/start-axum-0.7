pub mod app;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    let log_level = log::Level::Debug;
    _ = console_log::init_with_level(log_level);
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
