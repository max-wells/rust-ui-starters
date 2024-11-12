use codee::string::FromToStringCodec;
use leptos::*;
use leptos_use::storage::use_local_storage;
use leptos_use::{use_interval, UseIntervalReturn};

#[component]
pub fn DemoUseIntervalLocalStorage() -> impl IntoView {
    let UseIntervalReturn {
        counter, ..
    } = use_interval(1000);
    let (state, set_state, ..) = use_local_storage::<String, FromToStringCodec>("test-state");

    view! {
        <p>{counter}</p>
        <input
            class="block"
            prop:value=move || state.get()
            on:input=move |e| set_state.update(|s| *s = event_target_value(&e))
            type="text"
        />
    }
}
