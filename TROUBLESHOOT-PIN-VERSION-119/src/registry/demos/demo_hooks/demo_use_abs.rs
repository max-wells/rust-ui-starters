use leptos::*;
use leptos_use::math::use_abs;

use crate::registry::ui::slider::Slider;

// TODO: 🪝🐛 Little bug I think

#[component]
pub fn DemoUseAbs() -> impl IntoView {
    let (value, set_value) = create_signal(-32.25);

    let result: Signal<f64> = use_abs(value);

    view! {
        <Slider class="max-w-[300px]" min=-30.0 max=10.0 step=0.1 value=value on_input=set_value />
        <p>"Value: " {move || value.get()}</p>
        <p>"Absolute: " {move || result.get()}</p>
    }
}
