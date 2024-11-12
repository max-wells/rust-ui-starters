use leptos::*;

use crate::registry::primitives::p_toggle_custom::ToggleRootCustom;

#[component]
pub fn DemoToggleCustom() -> impl IntoView {
    view! {
        <ToggleRootCustom attr:aria-label="Toggle italic" class="bg-sky-500">
            <i class="italic">"I"</i>
        </ToggleRootCustom>
    }
}
