use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoScrollHorizontalAwaards() -> impl IntoView {
    view! {
        <Stylesheet
            id="scroll-horizontal-awaards"
            href="/components-nooo/scroll-horizontal-awaards.css"
        />

        <ul>
            <li>
                <h2>CSS Scroll Animations</h2>
                <img src="https://picsum.photos/400/400?random=1" alt="" />
            </li>
            <li>
                <h2>Check out this rad demo</h2>
                <img src="https://picsum.photos/400/400?random=2" alt="" />
            </li>
            <li>
                <h2>All CSS</h2>
                <img src="https://picsum.photos/400/400?random=3" alt="" />
            </li>
            <li>
                <h2>The same keyframes for every image</h2>
                <img src="https://picsum.photos/400/400?random=4" alt="" />
            </li>
            <li>
                <h2>Use a ViewTimeline</h2>
                <img src="https://picsum.photos/400/400?random=5" alt="" />
            </li>
            <li>
                <h2>With the inline axis</h2>
                <img src="https://picsum.photos/400/400?random=6" alt="" />
            </li>
            <li>
                <h2>Use a keyframes set</h2>
                <img src="https://picsum.photos/400/400?random=7" alt="" />
            </li>
            <li>
                <h2>That rotates its subject</h2>
                <img src="https://picsum.photos/400/400?random=8" alt="" />
            </li>
            <li>
                <h2>Alternate the transform-origin</h2>
                <img src="https://picsum.photos/400/400?random=9" alt="" />
            </li>
            <li>
                <h2>Using :nth-of-type(even)</h2>
                <img src="https://picsum.photos/400/400?random=10" alt="" />
            </li>
            <li>
                <h2>That it!</h2>
            </li>
        </ul>
    }
}
