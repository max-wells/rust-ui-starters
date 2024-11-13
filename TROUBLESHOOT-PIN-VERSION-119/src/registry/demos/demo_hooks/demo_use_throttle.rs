use leptos::*;
use leptos_use::signal_throttled;

use crate::registry::ui::input::Input;

#[component]
pub fn DemoUseThrottle() -> impl IntoView {
    let (input, set_input) = create_signal("".to_string());
    let throttled: Signal<String> = signal_throttled(input, 1000.0);

    view! {
        <div>
            <Input
                r#type="text"
                value=Some(input)
                on:input=move |event| set_input(event_target_value(&event))
                placeholder="Try to type quickly..."
            />
            <p>Delay is set to 1000ms for this demo.</p>
            <p>Input signal: {input}</p>
            <p>Throttled signal: {throttled}</p>
        </div>
    }
}
