use leptos::*;

use crate::registry::ui::button::Button;

#[component]
pub fn DemoButtonReactive() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { <Button on:click=on_click>"Click Me: " {count}</Button> }
}
