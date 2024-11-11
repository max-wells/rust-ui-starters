use leptos::*;

use crate::components::demo_cards_repushing::DemoCardsRepushing;
use crate::components::demo_parallax1::DemoParallax1;

#[component]
pub fn PageTest() -> impl IntoView {
    view! {
        <div class="p-4">
            <h1>Page Test</h1>
            <DemoCardsRepushing />
        </div>
    }
}
