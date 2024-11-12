use leptos::*;

#[cfg(feature = "hydrate")]
use leptos_use::docs::Note;

#[cfg(feature = "hydrate")]
use crate::registry::ui::button::Button;
#[cfg(feature = "hydrate")]
use crate::registry::ui::input::Input;
#[cfg(feature = "hydrate")]
use leptos_use::{
    use_clipboard_with_options, use_permission, UseClipboardOptions, UseClipboardReturn,
};

#[component]
pub fn DemoUseClipboard() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    let clipboard_component = {
        let (input, set_input) = create_signal("".to_owned());

        let UseClipboardReturn {
            is_supported,
            text,
            copied,
            copy,
        } = use_clipboard_with_options(UseClipboardOptions::default().read(true));

        let permission_read = use_permission("clipboard-read");
        let permission_write = use_permission("clipboard-write");

        view! {
            <Show
                when=is_supported
                fallback=|| view! { <p>"Your browser does not support the Clipboard API"</p> }
            >
                <Note>
                    "Clipboard Permission:" " read " <b>{move || permission_read().to_string()}</b>
                    " | " "write " <b>{move || permission_write().to_string()}</b>
                </Note>
                <p>
                    "Currently copied: " <code>{move || text().unwrap_or("none".to_owned())}</code>
                </p>
                // value=input
                <div class="flex gap-4">
                    <Input
                        on:input=move |e| set_input(event_target_value(&e))
                        r#type="text"
                        class="w-[270px]"
                    />

                    <Button on:click={
                        let copy = copy.clone();
                        move |_| copy(&input())
                    }>
                        <Show when=copied fallback=|| "Copy">
                            "Copied!"
                        </Show>
                    </Button>
                </div>
            </Show>
        }
    };

    #[cfg(not(feature = "hydrate"))]
    let clipboard_component = view! { <p>"Clipboard functionality is not available on the server. TODO 👉 Fix this."</p> };

    view! { <div>{clipboard_component}</div> }
}
