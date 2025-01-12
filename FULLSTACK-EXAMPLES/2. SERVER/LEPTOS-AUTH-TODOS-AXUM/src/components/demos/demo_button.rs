use leptos::prelude::*;

use crate::components::ui::button::Button;

#[component]
pub fn DemoButton() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <Button on:click=on_click>"Click Me: " {count}</Button>

        <Button class="bg-sky-500">"Button Sky"</Button>
    }
}
