use leptos::*;
use leptos_use::{use_raf_fn, utils::Pausable};

use crate::registry::ui::button::Button;

// TODO. Improve disabled props.

#[component]
pub fn DemoUseRafFn() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let Pausable {
        pause,
        resume,
        is_active,
    } = use_raf_fn(move |_| {
        set_count.update(|count| *count += 1);
    });

    view! {
        <div class="flex flex-col gap-4 justify-center items-center">
            <div class="mx-auto">Count: {count}</div>

            <div class="flex gap-2">
                <Button
                    on:click=move |_| pause()
                    disabled=MaybeSignal::Dynamic(Signal::derive(move || !is_active()))
                >
                    Pause
                </Button>
                <Button on:click=move |_| resume() disabled=MaybeSignal::Dynamic(is_active)>
                    Resume
                </Button>
            </div>

        </div>
    }
}
