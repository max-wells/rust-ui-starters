use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoCursorGooeyFluid() -> impl IntoView {
    view! {
        <Stylesheet id="cursor-gooey-fluid" href="/components-nooo/cursor-gooey-fluid.css" />
        <script src="/components/cursor-gooey-fluid.js" />

        <div class="overflow-x-hidden">
            <div class="min-h-screen page-wrap">
                <h1>Fluid Motion</h1>
            </div>

            <div id="cursor"></div>
        </div>
    }
}
