pub mod app;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
// Added after
pub mod __registry__;
pub mod api;
pub mod components;
pub mod constants;
pub mod models;
pub mod registry;
pub mod routes;
pub mod utils;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
