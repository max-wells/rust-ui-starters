use leptos::html::Textarea;
use leptos::*;
use leptos_use::use_resize_observer;

// TODO 🪝 TextArea

#[component]
pub fn DemoUseResizeObserver() -> impl IntoView {
    let el = create_node_ref::<Textarea>();
    let (text, set_text) = create_signal("".to_string());

    use_resize_observer(el, move |entries, _| {
        let rect = entries[0].content_rect();
        set_text.set(format!(
            "width: {:.0}\nheight: {:.0}",
            rect.width(),
            rect.height()
        ));
    });

    view! {
        <p class="mb-2">"Resize the box to see changes"</p>
        <textarea
            node_ref=el
            readonly
            class="p-4 text-2xl leading-10 rounded-md resize w-[200px] h-[100px] text-neutral-500"
            prop:value=move || text.get()
        />
    }
}
