use leptos::html::Div;
use leptos::*;
use leptos_use::docs::BooleanDisplay;
use leptos_use::{
    use_intersection_observer_with_options, UseIntersectionObserverOptions,
    UseIntersectionObserverReturn,
};

// TODO 🪝 Improve this 💪
// TODO 🪝 Checkbox, Label

#[component]
pub fn DemoUseIntersectionObserver() -> impl IntoView {
    let root = create_node_ref::<Div>();
    let target = create_node_ref::<Div>();
    let (is_visible, set_visible) = create_signal(false);

    let UseIntersectionObserverReturn {
        is_active,
        pause,
        resume,
        ..
    } = use_intersection_observer_with_options(
        target,
        move |entries, _| {
            set_visible.set(entries[0].is_intersecting());
        },
        UseIntersectionObserverOptions::default().root(Some(root)),
    );

    view! {
        <div class="text-center">
            <label class="checkbox">
                <input
                    type="checkbox"
                    prop:checked=move || is_active.get()
                    name="enabled"
                    on:input=move |e| {
                        if event_target_checked(&e) {
                            resume();
                        } else {
                            pause();
                        }
                    }
                />

                <span>"Enabled"</span>
            </label>
        </div>

        <div
            node_ref=root
            class="overflow-y-scroll my-8 mx-4 h-52 border-2 border-gray-300 border-dashed"
        >
            <p class="py-12 mb-72 text-2xl italic text-center opacity-80">"Scroll me down! 👇"</p>
            <div
                node_ref=target
                class="p-2 my-0 mx-8 mb-96 max-h-36 border-2 border-dashed border-brand-color"
            >
                <p>"Hello world!"</p>
            </div>
        </div>

        <div class="text-center">
            "Element "
            <BooleanDisplay
                value=is_visible
                true_str="inside"
                false_str="outside"
                class="font-bold underline"
            /> " the viewport"
        </div>
    }
}
