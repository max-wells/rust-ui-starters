use leptos::*;

use crate::registry::ui::headings::{HeadingVariant, H3};

#[component]
pub fn DemoHeadingsVariants() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2">
            <H3 variant=HeadingVariant::Modern>"Modern"</H3>
            <H3 variant=HeadingVariant::Underline>"Underline"</H3>
        </div>
    }
}
