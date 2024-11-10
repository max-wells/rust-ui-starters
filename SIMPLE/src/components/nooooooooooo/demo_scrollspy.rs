use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoScrollspy() -> impl IntoView {
    view! {
        <Stylesheet id="scrollspy" href="/components-nooo/scrollspy.css" />

        <aside>
            <ul>
                <li>section 1</li>
                <li>section 2</li>
                <li>section 3</li>
                <li>section 4</li>
                <li>section 5</li>
                <li>section 6</li>
            </ul>
        </aside>
        <div class="wrapper">
            <main>
                <h1>Scroll driven vertical using <code>animation-timeline</code></h1>
                <section id="section-1">section 1</section>
                <section id="section-2">section 2</section>
                <section id="section-3">section 3</section>
                <section id="section-4">section 4</section>
                <section id="section-5">section 5</section>
                <section id="section-6">section 6</section>
            </main>
        </div>
    }
}
