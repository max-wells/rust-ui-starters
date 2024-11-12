use leptos::*;
use std::rc::Rc;
use tailwind_fuse::*;
use wasm_bindgen::JsCast;

use crate::registry::ui::{
    _shared::STYLES,
    button::{Button, ButtonVariant},
    dialog::{use_dialog_context, DialogComponent, DialogContent},
};

// TODO UI. If the list of CommandItems is empty, do not display the heading.
// TODO UI. Handle arrow up / down to select item.

#[derive(Clone)]
pub struct CommandContext {
    pub search_query: ReadSignal<String>,
    pub set_search_query: Rc<dyn Fn(String)>,
}

#[allow(unused_braces)]
#[component]
pub fn CommandProvider(children: Children) -> impl IntoView {
    let (search_query, set_search_query) = create_signal(String::new());
    let context = CommandContext {
        search_query,
        set_search_query: Rc::new(set_search_query),
    };

    provide_context(context);

    view! { {children()} }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn CommandDialog(children: Children) -> impl IntoView {
    view! {
        <DialogComponent>
            <DialogContent class="overflow-hidden p-0 shadow-lg h-[300px]">
                <CommandProvider>
                    <Command>{children()}</Command>
                </CommandProvider>
            </DialogContent>
        </DialogComponent>
    }
}

#[component]
pub fn CommandTrigger(children: Children) -> impl IntoView {
    let dialog_context = use_dialog_context();
    let dialog_ref = dialog_context.dialog_ref;

    let show_command_dialog = move |_| {
        if let Some(dialog) = dialog_ref.get() {
            dialog.show_modal().unwrap();
        }
    };

    view! {
        <Button
            on:click=show_command_dialog
            variant=ButtonVariant::Outline
            class="outline-none focus-visible:bg-accent/70"
        >
            {children()}
        </Button>
    }
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[component]
pub fn Command(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "flex size-full flex-col overflow-hidden rounded-md bg-popover text-popover-foreground",
            class()
        )
    });

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CommandList(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "max-h-[300px] overflow-y-auto overflow-x-hidden  shortfix-sidenav-todo-properly",
            class()
        )
    });

    view! {
        <ul {..attributes} class=class>
            {children()}
        </ul>
    }
}

#[component]
pub fn CommandGroup(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    #[prop(into)] heading: String,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("", class()));

    view! {
        <>
            {Some(heading.clone())
                .map(|heading: String| {
                    view! {
                        <h3 class="py-1 px-2 text-xs font-semibold text-muted-foreground">
                            {heading}
                        </h3>
                    }
                })} <li {..attributes} class=class>
                {children()}
            </li>
        </>
    }
}

#[component]
pub fn CommandItem(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    href: &'static str,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            // STYLES.RING_FOCUS_VISIBLE,
            STYLES.FLEX_ITEMS_CENTER,
            STYLES.DISABLED_EVENTS_NONE,
            STYLES.FOCUS_VISIBLE_BG_ACCCENT_70,
            STYLES.HOVER_BG_ACCENT,
            "aria-selected:bg-accent aria-selected:text-accent-foreground",
            "cursor-pointer outline-none",
            "relative py-1.5 px-2 text-sm rounded-sm",
            class()
        )
    });

    view! {
        <a {..attributes} class=class href=href>
            {children()}
        </a>
    }
}

#[allow(unused_variables)]
#[component]
pub fn CommandInput(
    #[prop(optional, into)] class: String,
    #[prop(optional, into, default = "text")] r#type: &'static str,
    #[prop(optional_no_strip)] value: Option<ReadSignal<String>>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] min: Option<String>,
    #[prop(optional)] step: Option<String>,
    #[prop(optional)] max: Option<String>,
    #[prop(optional)] autofocus: bool,
    #[prop(optional)] node_ref: NodeRef<html::Input>,
) -> impl IntoView {
    let context = use_context::<CommandContext>().unwrap();

    let class = tw_merge!(
        STYLES.PLACEHOLDER_MUTED_FOREGROUND,
        STYLES.FILE_STYLES,
        STYLES.DISABLED_NOT_ALLOWED,
        STYLES.RING_OFFSET_BG,
        STYLES.BORDER_INPUT,
        STYLES.FLEX_WIDTH_FULL,
        "outline-none",
        "h-10 rounded-md bg-background px-3 py-2 text-sm",
        class
    );

    view! {
        <input
            type=r#type
            class=class
            name=name
            id=id
            placeholder=placeholder
            value=value
            min=min
            step=step
            max=max
            node_ref=node_ref
            autofocus=autofocus
            on:input=move |e| (context
                .set_search_query)(
                e.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap().value(),
            )
        />
    }
}
