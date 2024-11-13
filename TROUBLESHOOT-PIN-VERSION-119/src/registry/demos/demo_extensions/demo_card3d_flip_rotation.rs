use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoCard3dFlipRotation() -> impl IntoView {
    view! {
        <Stylesheet id="card-3d-flip-rotation" href="/components/card-3d-flip-rotation.css" />

        <div class="mainContainer">
            <div class="card">
                <div class="front">
                    <p class="front-heading">Front card</p>
                    <p>Follow Me For More</p>
                </div>
                <div class="back">
                    <p class="back-heading">Back card</p>
                    <p>Follow Me For More</p>
                </div>
            </div>
        </div>
    }
}
