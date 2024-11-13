use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoScrollLight() -> impl IntoView {
    view! {
        <Stylesheet id="scroll-light" href="/components/scroll-light.css" />

        <div class="scrollLightMainDiv">
            <div class="scrollbar"></div>
            <div class="container">
                <p>
                    In the enchanted world of frontend development, pixels and code guide your design journey. Entranced by the dance of colors, shapes unfold on the digital canvas. Every line of code is a spell, weaving functionality and aesthetics into an immersive digital tale.
                </p>
                <p>
                    In the enchanted world of frontend development, pixels and code guide your design journey. Entranced by the dance of colors, shapes unfold on the digital canvas. Every line of code is a spell, weaving functionality and aesthetics into an immersive digital tale.
                </p>
                <p>
                    In the enchanted world of frontend development, pixels and code guide your design journey. Entranced by the dance of colors, shapes unfold on the digital canvas. Every line of code is a spell, weaving functionality and aesthetics into an immersive digital tale.
                </p>
                <p>
                    In the enchanted world of frontend development, pixels and code guide your design journey. Entranced by the dance of colors, shapes unfold on the digital canvas. Every line of code is a spell, weaving functionality and aesthetics into an immersive digital tale.
                </p>
            </div>
        </div>
    }
}
