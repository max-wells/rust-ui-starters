use leptos::*;
use leptos_meta::Stylesheet;

#[component]
pub fn DemoTabsShadow() -> impl IntoView {
    view! {
        <Stylesheet id="tabs-shadow" href="/components/tabs-shadow.css" />

        <div class="w-full max-w-2xl tab-container">
            <input type="radio" name="tab" id="tab1" class="tab tab--1" />
            <label class="tab_label" for="tab1" class="text-neutral-500">
                Profile
            </label>

            <input type="radio" name="tab" id="tab2" class="tab tab--2" />
            <label class="tab_label" for="tab2" class="text-neutral-500">
                Settings
            </label>

            <input type="radio" name="tab" id="tab3" class="tab tab--3" />
            <label class="tab_label" for="tab3" class="text-neutral-500">
                Notifications
            </label>

            <div class="indicator"></div>
        </div>
    }
}
