use leptos::*;
use leptos_meta::Stylesheet;

use crate::registry::ui::sticky_cursor_links::{StickyLink, StickyLinksNav};

#[component]
pub fn DemoStickyCursorLinks() -> impl IntoView {
    view! {
        <Stylesheet id="sticky-cursor-links" href="/components/sticky-cursor-links.css" />

        <div class="flex justify-center items-center w-full h-[500px] bg-zinc-950">
            <StickyLinksNav class="py-10 px-4 border border-input">
                <StickyLink text="Home" />
                <StickyLink text="Our Story" />
                <StickyLink text="Studio" />
                <StickyLink text="Contact" />
            </StickyLinksNav>
        </div>
    }
}
