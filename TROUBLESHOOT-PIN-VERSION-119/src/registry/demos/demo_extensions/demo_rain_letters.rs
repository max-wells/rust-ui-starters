use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoRainLetters() -> impl IntoView {
    view! {
        <Stylesheet id="rain-letters" href="/components/rain-letters.css" />
        <script src="/components/rain-letters.js" />

        <div class="mainDivRainLetters">

            <div class="containerRainLetters">
                <div class="cloud">
                    <h2>Matrix Rain</h2>
                </div>
            </div>
        </div>
    }
}
