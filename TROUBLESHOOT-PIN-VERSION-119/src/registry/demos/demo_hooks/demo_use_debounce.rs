use leptos::*;
use leptos_use::signal_debounced;

use crate::registry::ui::input::Input;

#[component]
pub fn DemoUseDebounce() -> impl IntoView {
    let (input, set_input) = create_signal("".to_string());
    let debounced: Signal<String> = signal_debounced(input, 1000.0);

    view! {
        <div>
            <Input
                r#type="text"
                value=Some(input)
                on:input=move |event| set_input(event_target_value(&event))
                placeholder="Try to type quickly, then stop..."
            />
            <p>"Delay is set to 1000ms for this demo"</p>
            <p>Input signal: {input}</p>
            <p>Debounced signal: {debounced}</p>
        </div>
    }
}
