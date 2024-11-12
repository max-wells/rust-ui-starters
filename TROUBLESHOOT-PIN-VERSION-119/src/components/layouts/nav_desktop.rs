use leptos::*;
use leptos_router::{use_location, A as Link};

use crate::{
    __registry__::sidenav_stuff::{
        sidenav_core_core::SIDENAV_CORE_CORE, sidenav_core_others::SIDENAV_CORE_OTHERS,
        sidenav_core_primitives::SIDENAV_CORE_PRIMITIVES,
        sidenav_core_visualizations::SIDENAV_CORE_VISUALIZATIONS,
        sidenav_extensions_css::SIDENAV_EXTENSIONS_CSS,
        sidenav_extensions_javascript::SIDENAV_EXTENSIONS_JAVASCRIPT,
        sidenav_extensions_others::SIDENAV_EXTENSIONS_OTHERS,
        sidenav_hooks_browser::SIDENAV_HOOKS_BROWSER,
        sidenav_hooks_elements::SIDENAV_HOOKS_ELEMENTS, sidenav_hooks_maths::SIDENAV_HOOKS_MATHS,
        sidenav_hooks_others::SIDENAV_HOOKS_OTHERS,
        sidenav_hooks_reactivity::SIDENAV_HOOKS_REACTIVITY,
        sidenav_hooks_sensors::SIDENAV_HOOKS_SENSORS,
    },
    components::layouts::logo_home_link::LogoHomeLink,
};

#[component]
pub fn NavDesktop() -> impl IntoView {
    let location = use_location();

    let is_active = move |path: &str| -> bool { location.pathname.get().starts_with(path) };

    let length_core = [
        &SIDENAV_CORE_CORE,
        &SIDENAV_CORE_OTHERS,
        &SIDENAV_CORE_PRIMITIVES,
        &SIDENAV_CORE_VISUALIZATIONS,
    ]
    .iter()
    .map(|s| s.len())
    .sum::<usize>();

    let length_extensions = [
        &SIDENAV_EXTENSIONS_CSS,
        &SIDENAV_EXTENSIONS_JAVASCRIPT,
        &SIDENAV_EXTENSIONS_OTHERS,
    ]
    .iter()
    .map(|s| s.len())
    .sum::<usize>();

    let length_hooks = [
        &SIDENAV_HOOKS_BROWSER,
        &SIDENAV_HOOKS_ELEMENTS,
        &SIDENAV_HOOKS_MATHS,
        &SIDENAV_HOOKS_OTHERS,
        &SIDENAV_HOOKS_REACTIVITY,
        &SIDENAV_HOOKS_SENSORS,
    ]
    .iter()
    .map(|s| s.len())
    .sum::<usize>();

    view! {
        <div class="hidden gap-4 items-center md:flex">

            <LogoHomeLink />

            <Link
                href="/demos-core/button"
                class=move || { if is_active("/demos-core") { "font-bold" } else { "" } }
            >
                {format!("Core ({})", length_core)}
            </Link>
            <Link
                href="/demos-extensions/css-pills"
                class=move || if is_active("/demos-extensions") { "font-bold" } else { "" }
            >
                {format!("Extensions ({})", length_extensions)}
            </Link>

            <Link
                href="/demos-hooks/use-click-outside"
                class=move || if is_active("/demos-hooks") { "font-bold" } else { "" }
            >
                {format!("Hooks ({})", length_hooks)}
            </Link>
        </div>
    }
}
