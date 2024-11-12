use leptos::*;

use crate::registry::primitives::p_toggle::PrimitiveToggleRoot;

#[component]
pub fn DemoToggle() -> impl IntoView {
    view! {
        <PrimitiveToggleRoot
            attr:aria-label="Toggle italic"
            attr:class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors hover:bg-muted hover:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[state=on]:bg-accent data-[state=on]:text-accent-foreground  bg-transparent   h-10 px-3  w-fit"
        >
            <i class="italic">"I"</i>
        </PrimitiveToggleRoot>
    }
}
