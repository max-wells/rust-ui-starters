use leptos::*;
use leptos_use::docs::BooleanDisplay;
use leptos_use::use_media_query;

#[component]
pub fn DemoUseMediaQuery() -> impl IntoView {
    let is_large_screen = use_media_query("(min-width: 1024px)");
    let is_dark_preferred = use_media_query("(prefers-color-scheme: dark)");

    view! {
        <p>"Is large screen: " <BooleanDisplay value=is_large_screen /></p>
        <p>"Is dark preferred: " <BooleanDisplay value=is_dark_preferred /></p>
    }
}
