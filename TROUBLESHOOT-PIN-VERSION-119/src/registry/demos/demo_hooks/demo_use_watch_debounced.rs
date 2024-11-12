use leptos::*;
use leptos_use::{watch_debounced_with_options, WatchDebouncedOptions};

#[component]
pub fn DemoUseWatchDebounced() -> impl IntoView {
    let (input, set_input) = create_signal("".to_string());
    let (updated, set_updated) = create_signal(0);

    let _ = watch_debounced_with_options(
        move || input.get(),
        move |_, _, _| {
            set_updated.update(|x| *x += 1);
        },
        1000.0,
        WatchDebouncedOptions::default().max_wait(Some(5000.0)),
    );

    view! {
        <input
            class="block"
            prop:value=move || input.get()
            on:input=move |e| set_input.set(event_target_value(&e))
            placeholder="Try to type anything..."
            type="text"
        />
        <div>
            <code>"ms"</code>
            " is set to "
            <span style="font-weight: bold;">1000ms</span>
            " and "
            <code>"max_wait"</code>
            " is set to "
            <span style="font-weight: bold;">5000ms</span>
            " for this demo."
        </div>
        <p>"Input: " {input}</p>
        <p>"Times Updated: " {updated}</p>
    }
}
