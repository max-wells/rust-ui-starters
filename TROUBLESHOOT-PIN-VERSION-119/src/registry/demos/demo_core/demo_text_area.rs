use leptos::*;

use crate::registry::ui::text_area::TextArea;

#[component]
pub fn DemoTextArea() -> impl IntoView {
    view! {
        <div>
            <TextArea class="w-full">"Initial content"</TextArea>
        </div>
    }
}
