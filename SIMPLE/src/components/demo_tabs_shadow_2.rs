use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoTabsShadow2() -> impl IntoView {
    view! {
        <Stylesheet id="tabs-shadow-2" href="/components-nooo/tabs-shadow-2.css" />
        <script src="/components/tabs-shadow-2.js" />

        <div class="border mainDiv border-sky-500">

            <button aria-pressed="false" class="direction-handler">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 50 50">
                    <rect
                        width="32"
                        height="32"
                        x="25"
                        y="2.38"
                        stroke="currentColor"
                        stroke-width="2"
                        rx="3"
                        transform="rotate(45 25 2.38)"
                    />
                    <path fill="currentColor" d="M14 29.2v-8.4h11.11V14L36 25 25.11 36v-6.8H14Z" />
                </svg>
                <span class="sr-only">Change orientation</span>
            </button>
            <nav data-magnetic>
                <ul>
                    <li>
                        <a href="#home" id="home">
                            Home
                        </a>
                    </li>
                    <li>
                        <a href="#links" id="links">
                            Links
                        </a>
                    </li>
                    <li>
                        <a href="#rates" id="rates">
                            Rates
                        </a>
                    </li>
                    <li>
                        <a href="#speaking" id="speaking">
                            Speaking
                        </a>
                    </li>
                    <li>
                        <a href="#ai" id="ai">
                            AI
                        </a>
                    </li>
                    <li>
                        <a
                            href="https://twitter.com/intent/follow?screen_name=jh3yy"
                            id="follow"
                            target="_blank"
                            rel="noreferrer noopener"
                        >
                            Follow
                        </a>
                    </li>
                </ul>
            </nav>

        </div>
    }
}
