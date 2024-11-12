use leptos::*;
use leptos_use::use_throttle_fn;

use crate::registry::ui::button::Button;

#[component]
pub fn DemoUseThrottleFn() -> impl IntoView {
    let (click_count, set_click_count) = create_signal(0);
    let (throttled_count, set_throttled_count) = create_signal(0);

    let throttled_fn = use_throttle_fn(
        move || set_throttled_count.set(throttled_count.get_untracked() + 1),
        1000.0,
    );

    view! {
        <Button on:click=move |_| {
            set_click_count.set(click_count.get_untracked() + 1);
            throttled_fn();
        }>

            "Smash me!"
        </Button>

        <p>Delay is set to <span class="font-mono">1000</span>ms for this demo.</p>
        <p>"Button clicked: " {click_count}</p>
        <p>"Event handler called: " {throttled_count}</p>
    }
}
