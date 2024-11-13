use leptos::{html::Input, *};

use crate::registry::primitives::p_label::PrimitiveLabelRoot;

#[component]
pub fn DemoPrimitiveLabel() -> impl IntoView {
    let node_ref = NodeRef::<Input>::new();
    node_ref.on_load(|node| {
        node.set_default_value("Pedro Duarte");
    });

    view! {
        <div class="flex gap-4">
            <PrimitiveLabelRoot
                attr:class="text-[15px] font-semibold leading-[35px] dark:text-white text-mauve11"
                for_html="firstName"
            >
                "First name"
            </PrimitiveLabelRoot>
            <input
                node_ref=node_ref
                class="inline-flex flex-1 justify-center items-center leading-none appearance-none outline-none dark:text-white bg-blackA2 shadow-blackA6 h-[35px] rounded-[4px] px-[10px] text-[15px] text-blackA7 shadow-[0_0_0_1px] selection:text-white selection:bg-black/50 focus:shadow-[0_0_0_2px_black]"
                type="text"
                id="firstName"
            />
        </div>
    }
}
