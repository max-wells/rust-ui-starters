use leptos::*;
use leptos_use::{use_window_size, UseWindowSizeReturn};

#[component]
pub fn DemoUseWindowSize() -> impl IntoView {
    let UseWindowSizeReturn {
        width,
        height,
    } = use_window_size();

    view! { <p>{width}x {height}</p> }
}
