use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoParticlesVercel() -> impl IntoView {
    view! {
        <Stylesheet id="particles-vercel" href="/components/particles-vercel.css" />
        <script src="/components/particles-vercel.js" />

        <div class="border border-sky-500">
            <canvas></canvas>
        </div>
    }
}
