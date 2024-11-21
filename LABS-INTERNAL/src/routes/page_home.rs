use leptos::*;

use crate::components::ui::button::Button;

#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <div class="flex flex-col justify-center items-center mx-auto w-full max-w-3xl border mt-[100px] h-[600px] border-neutral-400">
            <Button on:click=on_click class="bg-sky-500">
                "Click Me: "
                {count}
            </Button>
        </div>
    }
}
