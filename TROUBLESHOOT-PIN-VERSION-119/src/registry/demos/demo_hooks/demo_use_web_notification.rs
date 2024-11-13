use leptos::*;
use leptos_use::docs::BooleanDisplay;
use leptos_use::{
    use_web_notification_with_options, NotificationDirection, ShowOptions,
    UseWebNotificationOptions, UseWebNotificationReturn,
};

#[component]
pub fn DemoUseWebNotification() -> impl IntoView {
    let UseWebNotificationReturn {
        is_supported,
        show,
        ..
    } = use_web_notification_with_options(
        UseWebNotificationOptions::default()
            .title("Hello World from leptos-use")
            .direction(NotificationDirection::Auto)
            .language("en")
            .renotify(true)
            .tag("test"),
    );

    let show = move || {
        show(ShowOptions::default());
    };

    view! {
        <div>
            <p>Supported: <BooleanDisplay value=is_supported /></p>
        </div>

        <Show
            when=is_supported
            fallback=|| {
                view! { <div>The Notification Web API is not supported in your browser.</div> }
            }
        >

            <button on:click={
                let show = show.clone();
                move |_| show()
            }>Show Notification</button>
        </Show>
    }
}
