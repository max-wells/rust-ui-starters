use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoWheelHeadings() -> impl IntoView {
    view! {
        <Stylesheet id="wheel-headings" href="/components/wheel-headings.css" />

        <div class="w-full max-w-4xl scroller">

            <div class="buffer"></div>

            <div class="snap"></div>
            <div class="snap"></div>
            <div class="snap"></div>
            <div class="snap"></div>
            <div class="snap"></div>
            <div class="snap"></div>
            <div class="snap"></div>
            <div class="snap"></div>

            <div class="spinner-wrap">
                <div class="spinner">
                    <div class="item-wrap">
                        <div class="item">Make a song to be happy.</div>
                        <div class="item">Make a song for a workout.</div>
                        <div class="item">Make a song for a farewell.</div>
                        <div class="item">Make a song for my friend Earl.</div>
                        <div class="item">Make a song about the moon.</div>
                        <div class="item">Make a song about mum cooking.</div>
                        <div class="item">Make a song for lunch.</div>
                        <div class="item">Make a song for a road trip.</div>
                        <div class="item">Make a song for a workout.</div>
                        <div class="item">Make a song for a breakup.</div>
                        <div class="item">Make a happy song.</div>
                        <div class="item">Make a song about mitochrondria.</div>
                        <div class="item">Make a song about how you feel.</div>
                    </div>
                </div>
                <div class="dot"></div>
            </div>
        </div>

        <div class="explainer">
            <input type="checkbox" checked />
            <div></div>
            <p>
                Demo of CSS
                <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/animation-timeline/scroll#browser_compatibility">
                    scroll()
                </a>function in concert with
                <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/scroll-snap-type">
                    scroll-snap
                </a>and
                <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/animation-timeline">
                    animation-timeline
                </a>.
            </p>
            <p>Works in Chrome 117+. Does not support Safari / iOS (yet).</p>
            <p>
                <a href="https://x.com/paul_uiux">paul_uiux@</a>
            </p>
        </div>
    }
}
