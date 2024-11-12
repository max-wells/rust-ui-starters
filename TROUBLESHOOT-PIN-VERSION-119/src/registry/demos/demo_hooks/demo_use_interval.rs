use leptos::*;
use leptos_use::{use_interval, UseIntervalReturn};

#[component]
pub fn DemoUseInterval() -> impl IntoView {
    let UseIntervalReturn {
        counter, ..
    } = use_interval(1000);

    view! {
        <div>
            <p>"Interval fired: " {counter}</p>
        </div>
    }
}
