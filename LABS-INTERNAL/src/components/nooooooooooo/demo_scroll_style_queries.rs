use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoScrollStyleQueries() -> impl IntoView {
    view! {
        <Stylesheet id="scroll-style-queries" href="/components-nooo/scroll-style-queries.css" />

        <div role="img" class="intro" aria-label="Scroll down for the magic">
            <span>"👋"</span>
            <span>"⬇️"</span>
        </div>
        <div class="box">
            <p class="text">hello</p>
        </div>
        <div class="box">
            <p class="text">is it me you looking for?</p>
        </div>
        <div class="box">
            <p class="text">I can see it in your eyes</p>
        </div>
        <div class="box">
            <p class="text">I can see it in your smile</p>
        </div>
        <div class="smile" role="img" aria-hidden>
            <div>
                <span>"☺️"</span>
                <span>"🙂"</span>
                <span>"😉"</span>
            </div>
        </div>
    }
}
