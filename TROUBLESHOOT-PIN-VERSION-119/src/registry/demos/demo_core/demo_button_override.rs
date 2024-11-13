use leptos::*;

use crate::registry::ui::button::Button;

#[component]
pub fn DemoButtonOverride() -> impl IntoView {
    view! { <Button class="hover:bg-pink-500 bg-sky-500">Fancy Button</Button> }
}
