use leptos::html::Textarea;
use leptos::*;
use leptos_use::docs::Note;
use leptos_use::{use_element_bounding, UseElementBoundingReturn};

// TODO 🪝 TextArea

#[component]
pub fn DemoUseElementBounding() -> impl IntoView {
    let el = create_node_ref::<Textarea>();

    let UseElementBoundingReturn {
        width,
        height,
        left,
        right,
        top,
        bottom,
        x,
        y,
        ..
    } = use_element_bounding(el);

    let text = move || {
        format!(
            "width: {}\nheight: {}\nleft: {}\nright: {}\ntop: {}\nbottom: {}\nx: {}\ny: {}",
            width.get(),
            height.get(),
            left.get(),
            right.get(),
            top.get(),
            bottom.get(),
            x.get(),
            y.get()
        )
    };

    view! {
        <Note class="mb-2">Resize the box to see changes</Note>
        <textarea
            node_ref=el
            readonly
            class="p-4 text-2xl leading-10 rounded-md resize w-[335px] h-[175px] text-neutral-500"
            prop:value=text
        />
    }
}
