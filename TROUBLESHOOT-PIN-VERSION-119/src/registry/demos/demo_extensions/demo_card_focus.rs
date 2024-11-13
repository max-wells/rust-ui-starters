use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoCardFocus() -> impl IntoView {
    view! {
        <Stylesheet id="card-focus" href="/components/card-focus.css" />

        <div class="cards">
            <div class="card red">
                <p class="tip">Hover Me</p>
            </div>
            <div class="card blue">
                <p class="tip">Hover Me</p>
            </div>
            <div class="card green">
                <p class="tip">Hover Me</p>
            </div>
        </div>
    }
}
