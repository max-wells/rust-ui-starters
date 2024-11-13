use leptos::*;

use crate::registry::ui::status::Status;

#[component]
pub fn DemoStatus() -> impl IntoView {
    view! {
        <Status>
            <div class="rounded-md size-16 bg-neutral-500" />
        </Status>
    }
}
