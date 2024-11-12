use leptos::*;

#[component]
pub fn DemoCursorMultiColor() -> impl IntoView {
    view! {
        <script src="/components/cursor-multi-color.js" />

        <div class="border border-sky-500 h-[500px]">
            <pointer-particles></pointer-particles>
        </div>
    }
}
