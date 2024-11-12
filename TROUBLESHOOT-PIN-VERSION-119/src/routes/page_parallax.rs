use leptos::*;

use crate::registry::demos::demo_core::demo_parallax_zoom_words::DemoParallaxZoomWords;
use crate::registry::demos::demo_extensions::demo_parallax1::DemoParallax1;
#[component]
pub fn PageParallax() -> impl IntoView {
    view! {
        <div>
            <DemoParallaxZoomWords />
        // <DemoParallax1 />

        // <div class="border h-[800px] border-sky-500" />
        </div>
    }
}
