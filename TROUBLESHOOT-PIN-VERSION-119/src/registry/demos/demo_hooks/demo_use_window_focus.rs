use leptos::*;
use leptos_use::use_window_focus;

#[component]
pub fn DemoUseWindowFocus() -> impl IntoView {
    let start_message = "💡 Click somewhere outside of the document to unfocus.";

    let (message, set_message) = create_signal(start_message);

    let focused = use_window_focus();

    let _ = watch(
        focused,
        move |focused, _, _| {
            if *focused {
                set_message(start_message);
            } else {
                set_message("ℹ Tab is unfocused")
            }
        },
        false,
    );

    view! { <div>{message}</div> }
}
