use crate::registry::ui::checkbox::Checkbox;
use leptos::*;

#[component]
pub fn DemoCheckbox() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <Checkbox checked=false />
        </div>
    }
}
