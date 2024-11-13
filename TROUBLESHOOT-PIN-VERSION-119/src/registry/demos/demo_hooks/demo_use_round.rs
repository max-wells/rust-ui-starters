use leptos::*;
use leptos_use::math::use_round;

use crate::registry::ui::slider::Slider;

#[component]
pub fn DemoUseRound() -> impl IntoView {
    let (value, set_value) = create_signal(5.95);

    let result: Signal<f64> = use_round(value);

    view! {
        <Slider class="max-w-[300px]" min=0.0 max=10.0 step=0.01 value=value on_input=set_value />
        <p>"Value: " {move || value.get()}</p>
        <p>"Rounded: " {move || result.get()}</p>
    }
}
