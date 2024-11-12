use leptos::*;
use leptos_meta::Stylesheet;

use crate::registry::icons::others::file_question::FileQuestion;

#[component]
pub fn DemoDocker() -> impl IntoView {
    let button_titles = vec![
        "Settings",
        "Browser",
        "Mail",
        "Map",
        "Messages",
        "Music Player",
        "Apps",
        "Documents",
    ];

    view! {
        <Stylesheet id="docker" href="/components/docker.css" />
        // <script src="/xx.js"></script>

        <div class="h-[600px]">

            <h1>Dock magnification using <code>:has()</code></h1>
            <nav class="flex fixed gap-1 justify-center items-end dockerNav">
                {button_titles
                    .into_iter()
                    .map(|title| {
                        view! {
                            <button
                                type="button"
                                class="relative border-none transition-all duration-300 ease-in-out cursor-pointer outline-none bg-[rgba(0,0,0,0.75)] text-[rgba(215,255,255,1)] w-[var(--btn-width,var(--btn-size))] h-[var(--btn-height,var(--btn-size))] aspect-ratio-1 rounded-[calc(var(--btn-size)*.25)]"
                                data-title=title
                            >
                                <FileQuestion class="size-14" />
                            </button>
                        }
                    })
                    .collect::<Vec<_>>()}
            </nav>

        </div>
    }
}
