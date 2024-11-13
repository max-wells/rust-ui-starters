use leptos::*;
use tailwind_fuse::*;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// TODO UI. FaqProvider for FaqSections
#[component]
pub fn Faq(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class =
        create_memo(move |_| tw_merge!("flex flex-col gap-3 w-full max-w-screen-md", class()));

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn FaqSection(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class =
        create_memo(move |_| tw_merge!("w-full rounded bg-accent/30 hover:bg-accent", class()));

    view! {
        <div {..attributes} class=class>
            <div class="flex flex-col">{children()}</div>
        </div>
    }
}

#[component]
pub fn FaqContent(children: Children) -> impl IntoView {
    view! {
        <div class="grid overflow-hidden mt-2 transition-all duration-500 grid-rows-[0fr] peer-checked:grid-rows-[1fr]">
            <div class="px-4 min-h-[0]">{children()}</div>
        </div>
    }
}

#[component]
pub fn FaqTitle(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("text-lg text-primary", class()));

    view! {
        <span {..attributes} class=class>
            {children()}
        </span>
    }
}

#[component]
pub fn FaqDescription(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("pr-6 mb-2 text-muted-foreground", class()));

    view! {
        <p {..attributes} class=class>
            {children()}
        </p>
    }
}

#[component]
pub fn FaqLabel(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    #[prop(into)] for_attr: &'static str,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "flex justify-between items-center py-2 px-4 mt-2 w-full cursor-pointer",
            class()
        )
    });

    view! {
        <label {..attributes} class=class for=for_attr>
            {children()}
        </label>
    }
}

#[component]
pub fn FaqInput(
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    #[prop(into)] id: &'static str,
) -> impl IntoView {
    view! { <input {..attributes} id=id type="checkbox" class="ml-auto sr-only peer" /> }
}
