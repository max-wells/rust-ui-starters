use codee::string::FromToStringCodec;
use leptos::*;
use leptos_use::{use_broadcast_channel, UseBroadcastChannelReturn};

use crate::registry::ui::button::Button;

// TODO 🪝 Input
// TODO 🪝 Button --> Loading state

#[component]
pub fn DemoUseBroadcastChannel() -> impl IntoView {
    let UseBroadcastChannelReturn {
        is_supported,
        message,
        post,
        error,
        ..
    } = use_broadcast_channel::<String, FromToStringCodec>("leptos-use-demo-channel");

    let (input_value, set_input_value) = create_signal(String::new());

    view! {
        <p>Please open this page in at least two tabs</p>

        <Show
            when=move || is_supported()
            fallback=move || view! { <p>"BroadcastChannel not supported"</p> }
        >
            <form on:submit={
                let post = post.clone();
                move |ev: web_sys::SubmitEvent| {
                    ev.prevent_default();
                    let value = input_value();
                    post(&value);
                }
            }>
                <input
                    value=input_value
                    on:input=move |event| {
                        set_input_value(event_target_value(&event));
                    }
                    class="text-neutral-500"

                    type="text"
                />
                <Button r#type="submit">Send Message</Button>
            </form>

            <Show when=move || message().is_some()>
                <p>"Received message: " {move || message().as_ref().unwrap().to_string()}</p>
            </Show>

            <Show when=move || error.with(|e| e.is_some())>
                <p>"Error: " {move || error.with(|e| format!("{:?}", e.as_ref().unwrap()))}</p>
            </Show>
        </Show>
    }
}
