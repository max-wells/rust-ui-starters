use leptos::*;

use crate::registry::ui::{
    headings::H3,
    separator::{Separator, SeparatorOrientation},
};

#[component]
pub fn DemoSeparator() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
            <H3>"Rust UI"</H3>
            <p>"An open-source UI component library 🦀."</p>

            <Separator />

            <div class="flex gap-4 items-center h-5">
                <p>"Blog"</p>
                <Separator orientation=SeparatorOrientation::Vertical />
                <p>"Docs"</p>
                <Separator orientation=SeparatorOrientation::Vertical />
                <p>"Source"</p>
            </div>
        </div>
    }
}
