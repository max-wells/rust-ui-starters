use leptos::*;

use crate::registry::ui::blockquote::{Blockquote, BlockquoteAuthor};

#[component]
pub fn DemoBlockquote() -> impl IntoView {
    view! {
        <Blockquote>
            Happiness lies not in the mere possession of money; it lies in the joy of
            achievement, in the thrill of creative effort.
            <BlockquoteAuthor>Franklin Roosevelt</BlockquoteAuthor>
        </Blockquote>
    }
}
