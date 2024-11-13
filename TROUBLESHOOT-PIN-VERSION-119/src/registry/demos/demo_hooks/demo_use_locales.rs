use leptos::*;
use leptos_use::use_locales;

#[component]
pub fn DemoUseLocales() -> impl IntoView {
    let locales = use_locales();

    view! { <pre>{move || format!("Locales:\n    {}", locales().join("\n    "))}</pre> }
}
