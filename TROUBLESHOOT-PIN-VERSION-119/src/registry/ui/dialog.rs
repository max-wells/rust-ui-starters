use leptos::html::Dialog;
use leptos::*;
use tailwind_fuse::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::registry::ui::{
    _shared::STYLES,
    button::{Button, ButtonVariant},
};

#[derive(Clone)]
pub struct DialogContext {
    pub dialog_ref: NodeRef<Dialog>,
}

pub fn use_dialog_context() -> DialogContext {
    use_context::<DialogContext>().expect("DialogContext not found")
}

#[allow(unused_braces)]
#[component]
pub fn DialogProvider(children: Children) -> impl IntoView {
    let dialog_ref = create_node_ref::<Dialog>();

    provide_context(DialogContext {
        dialog_ref,
    });

    view! { {children()} }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn DialogComponent(
    #[prop(into, optional)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let dialog_context = use_dialog_context();
    let dialog_ref = dialog_context.dialog_ref;

    let class = tw_merge!(
        STYLES.DIALOG_BACKDROP,
        STYLES.DIALOG_OPEN_STATE,
        STYLES.DIALOG_OPACITY_TRANSITION,
        STYLES.BLOCK_INSET_ZERO,
        "duration-300 translate-y-20",
        "p-0 w-2/3 rounded-2xl text-md",
        "text-primary",
        // "mx-auto", // TODO Does not fix issue for triggering from Navbar...
        class()
    );

    let close_dialog = move |_| {
        if let Some(dialog) = dialog_ref.get() {
            dialog.close();
        }
    };

    view! {
        <dialog
            _ref=dialog_ref
            on:click=move |ev| {
                let target = ev.target().unwrap();
                if let Some(element) = target.dyn_ref::<HtmlElement>() {
                    if element.tag_name() == "DIALOG" {
                        if let Some(dialog) = dialog_ref.get() {
                            dialog.close();
                        }
                    }
                }
            }
            on:close=move |ev| {
                #[allow(unused_variables)]
                let _target = ev.target().unwrap();
            }
            class=class
        >
            <div class="relative">
                <button
                    type="button"
                    on:click=close_dialog
                    class="flex absolute top-4 right-4 justify-center items-center p-3 text-xl bg-gray-100 rounded-md size-8"
                >
                    <span class="sr-only">"close"</span>
                    "X"
                </button>
                {children()}
            </div>
        </dialog>
    }
}

#[component]
pub fn DialogTrigger(children: Children) -> impl IntoView {
    let dialog_context = use_dialog_context();
    let dialog_ref = dialog_context.dialog_ref;

    let show_dialog = move |_| {
        if let Some(dialog) = dialog_ref.get() {
            dialog.show_modal().unwrap();
        }
    };

    view! {
        <Button on:click=show_dialog variant=ButtonVariant::Outline>
            {children()}
        </Button>
    }
}

//
#[component]
pub fn DialogForm(children: Children) -> impl IntoView {
    view! { <form method="dialog">{children()}</form> }
}

#[component]
pub fn DialogContent(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "py-16 px-8 space-y-3 bg-background shadow-lg border",
            class()
        )
    });

    view! {
        <main {..attributes} class=class>
            {children()}
        </main>
    }
}

#[component]
pub fn DialogTitle(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("text-2xl font-bold", class()));

    view! {
        <h2 {..attributes} class=class>
            {children()}
        </h2>
    }
}

#[component]
pub fn DialogBody(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("", class()));

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn DialogFooter(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("flex gap-6 justify-end py-4 px-8", class()));

    view! {
        <footer {..attributes} class=class>
            {children()}
        </footer>
    }
}

#[component]
pub fn DialogButtonFormCancel(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("", class()));

    view! {
        <Button
            {..attributes}
            class=class
            variant=ButtonVariant::Outline
            formmethod="dialog"
            value="cancel"
        >
            {children()}
        </Button>
    }
}

#[component]
pub fn DialogButtonFormSubmit(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("", class()));

    view! {
        <Button {..attributes} class=class formmethod="dialog" value="submit">
            {children()}
        </Button>
    }
}
