use leptos::{html::AnyElement, leptos_dom::helpers::AnimationFrameRequestHandle, *};
use tailwind_fuse::*;
use web_sys::{FocusEvent, KeyboardEvent, MouseEvent};

use crate::registry::{
    primitives::{
        p_presence::create_presence,
        p_primitive_custom::PrimitiveCustom, // TODO. PrimitiveCustom.
        p_roving_focus::{RovingFocusGroup, RovingFocusGroupItem},
        p_utils_create_controllable_signal::{
            create_controllable_signal, CreateControllableSignalProps,
        },
        p_utils_create_id::create_id,
        Attributes,
        Direction,
        Orientation,
    },
    ui::_shared::STYLES,
};

// TODO. We must find a way of using <Primitive type="button" />. The classes are applied in the HTML, but not showing in the UI...
pub const SHORTFIX_TABS_TRIGGER_CLASS: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 data-[state=active]:bg-background data-[state=active]:text-foreground data-[state=active]:shadow-sm   w-full";

#[derive(Clone)]
struct TabsContextValue {
    base_id: Signal<String>,
    value: Signal<Option<String>>,
    on_value_change: Callback<String>,
    orientation: Signal<Orientation>,
    direction: Signal<Direction>,
    activation_mode: Signal<ActivationMode>,
}

#[derive(Clone, Default, PartialEq)]
pub enum ActivationMode {
    #[default]
    Automatic,
    Manual,
}

#[component]
pub fn TabsCustom(
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(optional, into)] default_value: MaybeProp<String>,
    #[prop(optional, into)] orientation: MaybeSignal<Orientation>,
    #[prop(optional, into)] direction: MaybeSignal<Direction>,
    #[prop(optional, into)] activation_mode: MaybeSignal<ActivationMode>,
    #[prop(default=(|_|{}).into(), into)] on_value_change: Callback<String>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Attributes,
    #[prop(optional)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
    #[prop(into, optional)] class: MaybeSignal<String>,
) -> impl IntoView {
    let (value, set_value) = create_controllable_signal(CreateControllableSignalProps {
        value: Signal::derive(move || value.get()),
        default_value: Signal::derive(move || default_value.get()),
        on_change: on_value_change,
    });

    provide_context(TabsContextValue {
        base_id: create_id(),
        value: Signal::derive(move || value.get()),
        on_value_change: Callback::new(move |value| {
            set_value.set(value);
        }),
        direction: Signal::derive(move || direction.get()),
        orientation: Signal::derive(move || orientation.get()),
        activation_mode: Signal::derive(move || activation_mode.get()),
    });

    let class = create_memo(move |_| tw_merge!("", class()));

    view! {
        <PrimitiveCustom
            element=html::div
            node_ref=node_ref
            attrs=attrs
            as_child=as_child
            class=class
        >
            {children()}
        </PrimitiveCustom>
    }
}

#[component]
pub fn TabsList(
    #[prop(default=true.into(), into)] should_loop: MaybeSignal<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Attributes,
    #[prop(optional, into)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
    #[prop(into, optional)] class: MaybeSignal<String>,
) -> impl IntoView {
    let TabsContextValue {
        orientation,
        direction,
        ..
    } = use_context().expect("TabsList must be used in the Tabs component");

    let children = StoredValue::new(children);

    let class = create_memo(move |_| {
        tw_merge!(
            STYLES.FLEX_ITEMS_JUSTIFY_CENTER,
            STYLES.TEXT_MUTED_FOREGROUND,
            STYLES.FULL_CENTER_INLINE,
            STYLES.BG_MUTED,
            "h-10 rounded-md p-1",
            class()
        )
    });

    view! {
        <RovingFocusGroup
            as_child=true
            orientation=Signal::derive(move || orientation.get())
            direction=Signal::derive(move || direction.get())
            should_loop=Signal::derive(move || should_loop.get())
        >
            <PrimitiveCustom
                {..attrs.clone()}
                attr:role="tablist"
                attr:aria-orientation=move || orientation.get().to_string()
                element=html::div
                node_ref=node_ref
                as_child=as_child
                class=class
            >
                {children.with_value(|children| children())}
            </PrimitiveCustom>
        </RovingFocusGroup>
    }
}

#[component]
pub fn TabsTrigger(
    #[prop(optional, into)] value: MaybeSignal<String>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(default=(|_|{}).into(), into)] on_mouse_down: Callback<MouseEvent>,
    #[prop(default=(|_|{}).into(), into)] on_key_down: Callback<KeyboardEvent>,
    #[prop(default=(|_|{}).into(), into)] on_focus: Callback<FocusEvent>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Attributes,
    #[prop(optional, into)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let TabsContextValue {
        base_id,
        value: context_value,
        on_value_change,
        activation_mode,
        ..
    } = use_context().expect("TabsTrigger must be used in the Tabs component");

    let trigger_value = value.clone();
    let trigger_id =
        Signal::derive(move || format!("{}-trigger-{}", base_id.get(), trigger_value.get()));

    let content_value = value.clone();
    let content_id =
        Signal::derive(move || format!("{}-content-{}", base_id.get(), content_value.get()));

    let is_selected_value = value.clone();
    let is_selected = Signal::derive(move || context_value.get() == Some(is_selected_value.get()));

    let children = StoredValue::new(children);
    let value = StoredValue::new(value);

    view! {
        <RovingFocusGroupItem
            as_child=true
            focusable=Signal::derive(move || !disabled.get())
            active=is_selected
        >
            <PrimitiveCustom
                {..attrs.clone()}
                element=html::button
                attr:type="button"
                attr:role="tab"
                attr:aria-selected=is_selected
                attr:aria-controls=content_id
                attr:data-state=move || { if is_selected.get() { "active" } else { "inactive" } }
                attr:data-disabled=move || disabled.get().then_some("")
                attr:disabled=disabled
                attr:id=trigger_id
                on:mousedown=move |ev: MouseEvent| {
                    leptos::Callable::call(&on_mouse_down, ev.clone());
                    if !disabled.get() && ev.button() == 0 && !ev.ctrl_key() {
                        leptos::Callable::call(&on_value_change, value.get_value().get());
                    } else {
                        ev.prevent_default();
                    }
                }
                on:keydown=move |ev: KeyboardEvent| {
                    leptos::Callable::call(&on_key_down, ev.clone());
                    if [" ", "Enter"].contains(&ev.key().as_str()) {
                        leptos::Callable::call(&on_value_change, value.get_value().get());
                    }
                }
                on:focus=move |ev: FocusEvent| {
                    leptos::Callable::call(&on_focus, ev.clone());
                    let is_automatic_activation = activation_mode.get() != ActivationMode::Manual;
                    if !is_selected.get() && !disabled.get() && is_automatic_activation {
                        leptos::Callable::call(&on_value_change, value.get_value().get());
                    }
                }
                node_ref=node_ref
                as_child=as_child
            >
                {children.with_value(|children| children())}
            </PrimitiveCustom>
        </RovingFocusGroupItem>
    }
}

#[component]
pub fn TabsContent(
    #[prop(optional, into)] value: MaybeSignal<String>,
    #[prop(optional, into)] force_mount: MaybeSignal<bool>,
    #[prop(optional)] node_ref: NodeRef<AnyElement>,
    #[prop(attrs)] attrs: Attributes,
    #[prop(optional, into)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
    #[prop(into, optional)] class: MaybeSignal<String>,
) -> impl IntoView {
    let TabsContextValue {
        base_id,
        value: context_value,
        orientation,
        ..
    } = use_context().expect("TabsContent must be used in the Tabs component");

    let trigger_value = value.clone();
    let trigger_id =
        Signal::derive(move || format!("{}-trigger-{}", base_id.get(), trigger_value.get()));
    let content_value = value.clone();
    let content_id =
        Signal::derive(move || format!("{}-content-{}", base_id.get(), content_value.get()));

    let is_selected_value = value.clone();
    let is_selected = Signal::derive(move || context_value.get() == Some(is_selected_value.get()));
    let is_mount_animation_prevented = StoredValue::new(is_selected.get_untracked());

    let is_present = Signal::derive(move || is_selected.get() || force_mount.get());

    let presence = create_presence(is_present, node_ref);
    let animation_frame_handle = StoredValue::<Option<AnimationFrameRequestHandle>>::new(None);

    Effect::new(move |_| {
        if let Ok(handle) = request_animation_frame_with_handle(move || {
            is_mount_animation_prevented.set_value(false);
        }) {
            animation_frame_handle.set_value(Some(handle));
        }
    });

    on_cleanup(move || {
        if let Some(handle) = animation_frame_handle.get_value() {
            handle.cancel();
        }
    });

    Effect::new(move |_| {
        let Some(node) = node_ref.get() else {
            return;
        };

        _ = presence.get();

        if is_mount_animation_prevented.get_value() {
            _ = node.style("animation-duration", "0s");
        }
    });

    let children = StoredValue::new(children);

    // focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2

    let class = create_memo(move |_| {
        tw_merge!(
            STYLES.FOCUS_VISIBLE_RING,
            STYLES.RING_OFFSET_BG,
            "mt-2",
            class()
        )
    });

    view! {
        <Show when=move || presence.get()>
            <PrimitiveCustom
                {..attrs.clone()}
                attr:role="tabpanel"
                attr:data-state=move || { if is_selected.get() { "active" } else { "inactive" } }
                attr:data-orientation=move || orientation.get().to_string()
                attr:aria-labelledby=trigger_id
                attr:hidden=move || !is_present.get()
                attr:id=content_id
                attr:tabindex=0
                element=html::div
                node_ref=node_ref
                as_child=as_child
                class=class
            >
                {children.with_value(|children| children())}
            </PrimitiveCustom>
        </Show>
    }
}
