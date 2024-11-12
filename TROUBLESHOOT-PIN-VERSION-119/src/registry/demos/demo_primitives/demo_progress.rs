use leptos::*;
use leptos_use::{use_interval_fn, utils::Pausable};

use crate::registry::primitives::p_progress::{PrimitiveProgressIndicator, PrimitiveProgressRoot};

#[component]
pub fn DemoProgress() -> impl IntoView {
    let progress = RwSignal::new(25.0f64);
    let (indicator_style, set_indicator_style) = create_signal(format!(
        "transform: translateX(-{}%)",
        100.0 - progress.get_untracked()
    ));

    Effect::new(move |_| {
        let Pausable {
            pause, ..
        } = use_interval_fn(
            move || {
                progress.update(|progress| {
                    if *progress < 100.0 {
                        *progress += 25.0;
                    } else {
                        *progress = 0.0;
                    }
                });

                set_indicator_style.set(format!(
                    "transform: translateX(-{}%)",
                    100.0 - (progress.get_untracked() % 101.0)
                ));
            },
            1000,
        );

        on_cleanup(move || {
            pause();
        });
    });

    view! {
        <PrimitiveProgressRoot
            attr:class="relative overflow-hidden bg-black/25 rounded-full w-[300px] h-[25px] drop-shadow-md"
            attr:style="transform: translateZ(0)"
            value=progress
        >
            <PrimitiveProgressIndicator
                attr:class="bg-white w-full h-full transition-transform duration-[660ms] ease-[cubic-bezier(0.65, 0, 0.35, 1)]"
                attr:style=move || indicator_style.get()
            />
        </PrimitiveProgressRoot>
    }
}
