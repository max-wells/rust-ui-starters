use leptos::*;
use tailwind_fuse::*;

use crate::components::ui::{
    _shared::STYLES,
    button::{Button, ButtonVariant},
};

pub type AlertDialogVariant = ButtonVariant;

const POPOVER_ID_TARGET: &str = "mypopover";

#[component]
pub fn AlertDialogComponent(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class =
        create_memo(move |_| tw_merge!("p-6 max-w-[500px] rounded-lg border-none", class()));

    view! {
        <div class="">
            <dialog
                {..attributes}
                class=class
                id=POPOVER_ID_TARGET
                // * 💁 💡 Manual mode means can't close the popover by clicking outside of it.
                // * ---> It needs the popovertargetaction="hide" (or "submit") to be clicked.
                popover="manual"
            >
                {children()}
            </dialog>
        </div>
    }
}

#[component]
pub fn AlertDialogTrigger(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(into, optional)] variant: MaybeSignal<ButtonVariant>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("", class()));

    view! {
        <Button {..attributes} class=class popovertarget=POPOVER_ID_TARGET variant=variant>
            {children()}
        </Button>
    }
}

#[component]
pub fn AlertDialogTitle(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("text-lg font-semibold", class()));

    view! {
        <h2 {..attributes} class=class>
            {children()}
        </h2>
    }
}

#[component]
pub fn AlertDialogDescription(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("text-sm", STYLES.TEXT_MUTED_FOREGROUND, class()));

    view! {
        <p {..attributes} class=class>
            {children()}
        </p>
    }
}

#[component]
pub fn AlertDialogFooter(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("flex justify-end gap-2", class()));

    view! {
        <footer {..attributes} class=class>
            {children()}
        </footer>
    }
}

#[component]
pub fn AlertDialogCancel(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    #[prop(optional, default = AlertDialogVariant::Outline)] variant: AlertDialogVariant,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("", class()));

    view! {
        <Button
            {..attributes}
            class=class
            variant=variant
            popovertarget=POPOVER_ID_TARGET
            popovertargetaction="hide"
        >
            {children()}
        </Button>
    }
}

#[component]
pub fn AlertDialogSubmit(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("", class()));

    view! {
        <Button
            {..attributes}
            class=class
            popovertarget=POPOVER_ID_TARGET
            popovertargetaction="submit"
        >
            {children()}
        </Button>
    }
}
