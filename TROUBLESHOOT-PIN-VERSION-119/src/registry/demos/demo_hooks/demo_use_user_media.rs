use leptos::*;
use leptos_use::{use_user_media, UseUserMediaReturn};

#[component]
pub fn DemoUseUserMedia() -> impl IntoView {
    let video_ref = create_node_ref::<leptos::html::Video>();

    let UseUserMediaReturn {
        stream,
        enabled,
        set_enabled,
        ..
    } = use_user_media();

    create_effect(move |_| {
        match stream.get() {
            Some(Ok(s)) => {
                if let Some(v) = video_ref.get() {
                    v.set_src_object(Some(&s));
                }
                return;
            }
            Some(Err(e)) => logging::error!("Failed to get media stream: {:?}", e),
            None => logging::log!("No stream yet"),
        }

        if let Some(v) = video_ref.get() {
            v.set_src_object(None);
        }
    });

    view! {
        <div class="flex flex-col gap-4 text-center">
            <div>
                <button on:click=move |_| set_enabled(
                    !enabled(),
                )>{move || if enabled() { "Stop" } else { "Start" }}video</button>
            </div>

            <div>
                <video
                    node_ref=video_ref
                    controls=false
                    autoplay=true
                    muted=true
                    class="w-auto h-96"
                ></video>
            </div>
        </div>
    }
}
