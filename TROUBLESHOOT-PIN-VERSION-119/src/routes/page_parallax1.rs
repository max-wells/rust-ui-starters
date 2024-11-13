use leptos::*;

use crate::registry::demos::demo_extensions::demo_parallax1::DemoParallax1;

#[component]
pub fn PageParallax1() -> impl IntoView {
    view! {
        <div>
            <DemoParallax1 />
        </div>
    }
}
