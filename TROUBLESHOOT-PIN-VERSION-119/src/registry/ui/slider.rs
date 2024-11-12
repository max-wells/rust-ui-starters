use leptos::*;
use tailwind_fuse::*;

use crate::registry::ui::_shared::STYLES;

#[component]
pub fn Slider(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] min: f64,
    #[prop(optional, into)] max: f64,
    #[prop(optional, into)] step: f64,
    value: ReadSignal<f64>,
    on_input: WriteSignal<f64>,
) -> impl IntoView {
    let class = tw_merge!(STYLES.BLOCK_WIDTH_FULL, "", class);

    view! {
        <input
            type="range"
            class=class
            min=min.to_string()
            max=max.to_string()
            step=step.to_string()
            prop:value=move || value.get().to_string()
            on:input=move |e| on_input.set(event_target_value(&e).parse().unwrap())
        />
    }
}
