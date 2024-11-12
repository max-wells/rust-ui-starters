use leptos::*;

use crate::registry::ui::headings::{H1, H2, H3, H4};

#[component]
pub fn DemoHeadings() -> impl IntoView {
    view! {
        <div>
            <H1>"Heading 1"</H1>
            <H2>"Heading 2"</H2>
            <H3>"Heading 3"</H3>
            <H4>"Heading 4"</H4>
        </div>
    }
}
