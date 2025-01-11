pub mod app;
pub mod auth;
pub mod error_template;
pub mod errors;
pub mod features;
#[cfg(feature = "ssr")]
pub mod state;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    // use crate::features::todos::todos_refactor::*;
    use crate::app::App;

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);
}
