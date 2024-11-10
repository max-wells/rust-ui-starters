use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoTemplate() -> impl IntoView {
    view! {
        <Stylesheet id="radar-mini" href="/components-nooo/radwar-mini.css" />

        <div class="mx-auto w-full max-w-2xl">
            <p>"Hello"</p>
        </div>
    }
}
