use leptos::{html::AnyElement, *};
use web_sys::MouseEvent;

use crate::registry::primitives::{p_primitive::Primitive, Attributes};

#[component]
pub fn PrimitiveLabelRoot(
    #[prop(optional, into)] for_html: MaybeProp<String>,

    #[prop(default=(|_|{}).into(), into)] on_mouse_down: Callback<MouseEvent>,

    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Attributes,
    children: ChildrenFn,

    #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
    view! {
        <Primitive
            {..attrs}
            attr:for=for_html
            element=html::label
            on:mousedown=move |ev: MouseEvent| {
                leptos::Callable::call(&on_mouse_down, ev.clone());
                if ev.default_prevented() && ev.detail() > 1 {
                    ev.prevent_default();
                }
            }
            node_ref=node_ref
            as_child=as_child
        >
            {children()}
        </Primitive>
    }
}
