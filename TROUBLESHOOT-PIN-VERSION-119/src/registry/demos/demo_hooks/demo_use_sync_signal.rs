use leptos::*;
use leptos_use::sync_signal;

// TODO 🪝 Input

#[component]
pub fn DemoUseSyncSignal() -> impl IntoView {
    let (a, set_a) = create_signal(String::new());
    let (b, set_b) = create_signal(String::new());

    let _ = sync_signal((a, set_a), (b, set_b));

    view! {
        <input
            prop:value=a
            on:input=move |e| set_a(event_target_value(&e))
            placeholder="A"
            type="text"
        />
        <input
            prop:value=b
            on:input=move |e| set_b(event_target_value(&e))
            placeholder="B"
            type="text"
        />
    }
}
