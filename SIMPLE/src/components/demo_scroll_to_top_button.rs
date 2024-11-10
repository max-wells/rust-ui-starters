use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoScrollToTopButton() -> impl IntoView {
    view! {
        <Stylesheet id="scroll-to-top-button" href="/components/scroll-to-top-button.css" />

        <div class="mainDiv">

            <h1>"👇 Scroll down"</h1>

            <div class="scroll-snap">
                <section id="s1">
                    <h2>Section s1</h2>
                    <p>
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Illo inventore quae expedita dolore voluptatibus quia, numquam animi reiciendis incidunt unde dolorem perferendis sunt deleniti possimus deserunt dolor, odit sapiente.
                    </p>
                    <p>
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Illo inventore quae expedita dolore voluptatibus quia, numquam animi reiciendis incidunt unde dolorem perferendis sunt deleniti possimus deserunt dolor, odit sapiente.
                    </p>
                    <p>
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Illo inventore quae expedita dolore voluptatibus quia, numquam animi reiciendis incidunt unde dolorem perferendis sunt deleniti possimus deserunt dolor, odit sapiente.
                    </p>
                    <p>
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Illo inventore quae expedita dolore voluptatibus quia, numquam animi reiciendis incidunt unde dolorem perferendis sunt deleniti possimus deserunt dolor, odit sapiente.
                    </p>
                    <p>
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Illo inventore quae expedita dolore voluptatibus quia, numquam animi reiciendis incidunt unde dolorem perferendis sunt deleniti possimus deserunt dolor, odit sapiente.
                    </p>
                    <p>
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Illo inventore quae expedita dolore voluptatibus quia, numquam animi reiciendis incidunt unde dolorem perferendis sunt deleniti possimus deserunt dolor, odit sapiente.
                    </p>
                </section>
                <section id="s2">
                    <h2>Section s2</h2>
                    <p>
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Illo inventore quae expedita dolore voluptatibus quia, numquam animi reiciendis incidunt unde dolorem perferendis sunt deleniti possimus deserunt dolor, odit sapiente.
                    </p>
                    <p>
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Illo inventore quae expedita dolore voluptatibus quia, numquam animi reiciendis incidunt unde dolorem perferendis sunt deleniti possimus deserunt dolor, odit sapiente.
                    </p>
                    <p>
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Illo inventore quae expedita dolore voluptatibus quia, numquam animi reiciendis incidunt unde dolorem perferendis sunt deleniti possimus deserunt dolor, odit sapiente.
                    </p>
                </section>
                <section id="s3">
                    <h2>Section s3</h2>
                    <p>
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Illo inventore quae expedita dolore voluptatibus quia, numquam animi reiciendis incidunt unde dolorem perferendis sunt deleniti possimus deserunt dolor, odit sapiente.
                    </p>
                    <p>
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Illo inventore quae expedita dolore voluptatibus quia, numquam animi reiciendis incidunt unde dolorem perferendis sunt deleniti possimus deserunt dolor, odit sapiente.
                    </p>
                    <p>
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Illo inventore quae expedita dolore voluptatibus quia, numquam animi reiciendis incidunt unde dolorem perferendis sunt deleniti possimus deserunt dolor, odit sapiente.
                    </p>
                    <p>
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Illo inventore quae expedita dolore voluptatibus quia, numquam animi reiciendis incidunt unde dolorem perferendis sunt deleniti possimus deserunt dolor, odit sapiente.
                    </p>
                    <p>
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Illo inventore quae expedita dolore voluptatibus quia, numquam animi reiciendis incidunt unde dolorem perferendis sunt deleniti possimus deserunt dolor, odit sapiente.
                    </p>
                </section>
                <section id="s4">
                    <h2>Section s4</h2>
                    <p>
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Illo inventore quae expedita dolore voluptatibus quia, numquam animi reiciendis incidunt unde dolorem perferendis sunt deleniti possimus deserunt dolor, odit sapiente.
                    </p>
                    <p>
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Illo inventore quae expedita dolore voluptatibus quia, numquam animi reiciendis incidunt unde dolorem perferendis sunt deleniti possimus deserunt dolor, odit sapiente.
                    </p>
                    <p>
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Illo inventore quae expedita dolore voluptatibus quia, numquam animi reiciendis incidunt unde dolorem perferendis sunt deleniti possimus deserunt dolor, odit sapiente.
                    </p>
                </section>
                <section id="s5">
                    <h2>Section s5</h2>
                    <p>
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Illo inventore quae expedita dolore voluptatibus quia, numquam animi reiciendis incidunt unde dolorem perferendis sunt deleniti possimus deserunt dolor, odit sapiente.
                    </p>
                    <p>
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Illo inventore quae expedita dolore voluptatibus quia, numquam animi reiciendis incidunt unde dolorem perferendis sunt deleniti possimus deserunt dolor, odit sapiente.
                    </p>
                    <p>
                        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Illo inventore quae expedita dolore voluptatibus quia, numquam animi reiciendis incidunt unde dolorem perferendis sunt deleniti possimus deserunt dolor, odit sapiente.
                    </p>
                </section>
            </div>

            <a class="to-top" href="#s1">
                <svg viewBox="0 0 24 24">
                    <path d="M13,20H11V8L5.5,13.5L4.08,12.08L12,4.16L19.92,12.08L18.5,13.5L13,8V20Z"></path>
                </svg>
            </a>

        </div>
    }
}
