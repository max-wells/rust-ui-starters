use leptos::*;

#[component]
pub fn DemoBlurryBlob() -> impl IntoView {
    view! {
        <div class="justify-center items-center w-full bg-white min-h-80">
            <div class="relative w-full max-w-lg">
                <div class="absolute -right-24 -top-28 p-8 bg-blue-400 rounded-sm animate-popBlob opacity-45 mix-blend-multiply blur-3xl filter size-72"></div>
                <div class="absolute -left-40 -top-64 p-8 bg-blue-400 rounded-sm animate-popBlob opacity-45 mix-blend-multiply blur-3xl filter size-72"></div>
            </div>
        </div>
    }
}
