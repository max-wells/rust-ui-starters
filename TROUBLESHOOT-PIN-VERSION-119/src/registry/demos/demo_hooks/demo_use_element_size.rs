use leptos::*;

#[cfg(feature = "hydrate")]
use leptos::html::Textarea;
#[cfg(feature = "hydrate")]
use leptos_use::{use_element_size, UseElementSizeReturn};

#[component]
pub fn DemoUseElementSize() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    let demo_component = {
        let el = create_node_ref::<Textarea>();
        let UseElementSizeReturn {
            width,
            height,
        } = use_element_size(el);
        let text = move || format!("width: {}\nheight: {}", width.get(), height.get());

        view! {
            <p class="mb-2">"Resize the box to see changes"</p>
            <textarea
                node_ref=el
                readonly
                class="p-4 text-2xl leading-10 rounded-md resize w-[200px] h-[100px] text-neutral-500"
                prop:value=text
            />
        }
    };

    #[cfg(not(feature = "hydrate"))]
    let demo_component = view! {
        <p class="mb-2">"Element size functionality is not available on the server."</p>
        <textarea
            readonly
            class="p-4 text-2xl leading-10 rounded-md resize w-[200px] h-[100px] text-neutral-500"
            prop:value="width: N/A\nheight: N/A"
        />
    };

    view! { <div>{demo_component}</div> }
}
