use leptos::{html::AnyElement, Callable, *};
use tailwind_fuse::*;
use web_sys::MouseEvent;

use crate::registry::{
    primitives::{
        p_primitive_custom::PrimitiveCustom,
        p_utils_create_controllable_signal::{
            create_controllable_signal, CreateControllableSignalProps,
        },
        Attributes,
    },
    ui::_shared::STYLES,
};

#[component]
pub fn ToggleRootCustom(
    #[prop(optional, into)] pressed: MaybeProp<bool>,
    #[prop(optional, into)] default_pressed: MaybeProp<bool>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(default=(|_|{}).into(), into)] on_pressed_changed: Callback<bool>,
    #[prop(default=(|_|{}).into(), into)] on_click: Callback<MouseEvent>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Attributes,
    #[prop(optional, into)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
    #[prop(into, optional)] class: MaybeSignal<String>,
) -> impl IntoView {
    let (pressed, set_pressed) = create_controllable_signal(CreateControllableSignalProps {
        value: Signal::derive(move || pressed.get()),
        default_value: Signal::derive(move || default_pressed.get()),
        on_change: on_pressed_changed,
    });

    let class = create_memo(move |_| {
        tw_merge!(
            STYLES.FLEX_ITEMS_JUSTIFY_CENTER,
            STYLES.RING_OFFSET_BG,
            STYLES.FOCUS_VISIBLE_RING,
            STYLES.BG_TRANSPARENT,
            STYLES.TRANSITION_COLORS,
            STYLES.DISABLED_EVENTS_NONE,
            STYLES.DATA_STATE_ON_TOGGLE,
            STYLES.WIDTH_FIT,
            "h-10 px-3",
            "rounded-md text-sm font-medium",
            "hover:bg-muted hover:text-muted-foreground",
            class()
        )
    });

    view! {
        <PrimitiveCustom
            {..attrs}
            attr:type="button"
            attr:aria-pressed=move || pressed.get().unwrap_or_default().to_string()
            attr:data-state=move || { if pressed.get().unwrap_or_default() { "on" } else { "off" } }
            attr:data-disabled=disabled
            element=html::button
            on:click=move |ev: MouseEvent| {
                Callable::call(&on_click, ev.clone());
                if !disabled.get() {
                    set_pressed.update(|pressed| *pressed = Some(!pressed.unwrap_or(false)));
                }
            }
            node_ref=node_ref
            as_child=as_child
            class=class
        >
            {children()}
        </PrimitiveCustom>
    }
}
