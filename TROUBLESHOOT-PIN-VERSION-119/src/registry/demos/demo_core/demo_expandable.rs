use leptos::*;

use crate::registry::ui::expandable::{ExpandableContent, ExpandableTransition, ExpandableTrigger};

#[component]
pub fn DemoExpandable() -> impl IntoView {
    view! {
        <ExpandableTrigger class="mx-auto">
            <ExpandableTransition>
                <p>Mikael Ainalem</p>
            </ExpandableTransition>

            <ExpandableContent class="bg-pink-200">
                <p>"This is a dummy content"</p>
            </ExpandableContent>
        </ExpandableTrigger>
    }
}
