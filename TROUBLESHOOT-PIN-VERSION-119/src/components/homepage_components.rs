use leptos::*;

#[component]
pub fn SeparatorWaveLine() -> impl IntoView {
    view! {
        <div
            aria-hidden
            class="mx-auto h-[20px] w-[180px] my-[64px] bg-[url('/line.svg')] filter invert homeLineAriaHidden"
        />
    }
}
