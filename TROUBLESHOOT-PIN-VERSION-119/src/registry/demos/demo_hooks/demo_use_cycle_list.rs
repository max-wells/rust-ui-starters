use leptos::*;
use leptos_use::{use_cycle_list, UseCycleListReturn};

use crate::registry::ui::button::Button;

const DEMO_VEC: &[&str] = &["Dog", "Cat", "Lizard", "Fox", "Whale"];

#[component]
pub fn DemoUseCycleList() -> impl IntoView {
    let UseCycleListReturn {
        state,
        next,
        prev,
        ..
    } = use_cycle_list(DEMO_VEC.to_vec());

    view! {
        <div class="flex flex-col gap-4 items-center">
            <div class="flex flex-wrap gap-2 p-2 rounded-md border">
                <For each=move || DEMO_VEC.to_vec() key=|item| *item let:data>
                    <div class="mx-2">{data}</div>
                </For>
            </div>

            <div class="text-lg font-bold text-primary">{state}</div>
            <div class="flex gap-2">
                <Button on:click=move |_| { prev() }>"Prev"</Button>
                <Button on:click=move |_| { next() }>"Next"</Button>
            </div>
        </div>
    }
}
