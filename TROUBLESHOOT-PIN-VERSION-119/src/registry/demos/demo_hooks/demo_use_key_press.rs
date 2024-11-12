use leptos::ev::{keypress, KeyboardEvent};
use leptos::*;
use leptos_use::{use_event_listener, use_window};

// TODO 🪝 Create a specific hook for that

#[component]
pub fn DemoUseKeyPress() -> impl IntoView {
    let (key, set_key) = create_signal("".to_string());

    // window() doesn't work on the server so we provide use_window()
    let _ = use_event_listener(use_window(), keypress, move |evt: KeyboardEvent| {
        set_key(evt.key())
    });

    view! { <p>Press any key: {key}</p> }
}
