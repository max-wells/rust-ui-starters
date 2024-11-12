use leptos::*;
use leptos_use::{use_timeout_fn, UseTimeoutFnReturn};

use crate::registry::ui::button::Button;

#[component]
pub fn DemoUseTimeoutFn() -> impl IntoView {
    const DEFAULT_TEXT: &str = "Please wait for 2 seconds";

    let (text, set_text) = create_signal(DEFAULT_TEXT.to_string());
    let UseTimeoutFnReturn {
        start,
        is_pending,
        ..
    } = use_timeout_fn(
        move |_| {
            set_text("Fired!".to_string());
        },
        2000.0,
    );

    let restart = move |_| {
        set_text(DEFAULT_TEXT.to_string());
        start(());
    };

    view! {
        <p>{text}</p>

        <Button on:click=restart disabled=is_pending>
            "Restart"
        </Button>
    }
}
