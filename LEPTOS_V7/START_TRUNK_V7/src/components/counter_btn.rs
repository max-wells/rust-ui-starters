use leptos::prelude::*;

/// A parameterized incrementing button
#[component]
pub fn Button(#[prop(default = 1)] increment: i32) -> impl IntoView {
    let (count, set_count) = signal(0);
    view! {
        <button class="bg-blue-500 text-white px-4 py-2 rounded-md" on:click=move |_| {
            set_count(count() + increment)
        }>

            "Click me: " {count}
        </button>
    }
}
