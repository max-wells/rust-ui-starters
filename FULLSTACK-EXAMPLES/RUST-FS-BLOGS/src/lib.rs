use cfg_if::cfg_if;
pub mod app;
pub mod components;
pub mod error_template;
pub mod fileserv;
// Added after
pub mod api;
pub mod models;
pub mod routes;

pub mod utils;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Warn);
        // TODO. └──> The Defalt is in Debug mode but too many logs...
        console_error_panic_hook::set_once();

        leptos::mount_to_body(move || {
            view! { <App/> }
        });
    }
}}
