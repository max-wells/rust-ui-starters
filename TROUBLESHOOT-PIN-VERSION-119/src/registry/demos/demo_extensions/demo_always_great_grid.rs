use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoAlwaysGreatGrid() -> impl IntoView {
    view! {
        <Stylesheet id="always-great-grid" href="/components/always-great-grid.css" />
        <script src="/components/always-great-grid.js" />

        <div class="container">
            <main class="always-great-grid" id="grid">
                <div class="box" style="view-transition-name: b0"></div>
                <div class="box" style="view-transition-name: b1"></div>
                <div class="box" style="view-transition-name: b2"></div>
                <div class="box" style="view-transition-name: b3"></div>
                <div class="box" style="view-transition-name: b4"></div>

            // view transition names are inline so
            // they stay attached to the element,
            // as opposed to using :nth-child()
            // which means each elements name shifts
            // when a box is added/removed
            </main>
        </div>

        <footer>
            <button onclick="addBox()">Add a box</button>
            <button onclick="removeBox()" type="reset">
                Remove a box
            </button>
        </footer>
    }
}
