use leptos::*;
use leptos_use::use_timestamp;

#[component]
pub fn DemoUseTimestamp() -> impl IntoView {
    let timestamp = use_timestamp();

    view! { <div>Timestamp: {timestamp}</div> }
}
