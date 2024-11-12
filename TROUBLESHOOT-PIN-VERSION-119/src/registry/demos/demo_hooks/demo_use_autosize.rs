use leptos::*;
use leptos_use::{use_textarea_autosize, UseTextareaAutosizeReturn};

#[component]
pub fn DemoUseAutosize() -> impl IntoView {
    let textarea = create_node_ref::<html::Textarea>();

    let UseTextareaAutosizeReturn {
        content,
        set_content,
        ..
    } = use_textarea_autosize(textarea);

    view! {
        <div class="flex flex-col gap-4 min-h-[450px]">
            <p class="mb-4">"Type, the textarea will grow:"</p>
            <textarea
                value=content
                on:input=move |evt| set_content.set(event_target_value(&evt))
                node_ref=textarea
                class="resize-none box-border"
                placeholder="What's on your mind?"
            />
        </div>
    }
}
