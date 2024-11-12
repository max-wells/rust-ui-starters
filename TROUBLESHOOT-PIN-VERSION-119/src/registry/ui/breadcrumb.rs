use leptos::*;
use tailwind_fuse::*;

use crate::registry::icons::{chevrons::chevron_right::ChevronRight, others::ellipsis::Ellipsis};

#[component]
pub fn Breadcrumb(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("", class()));

    view! {
        <nav {..attributes} class=class aria-label="breadcrumb">
            {children()}
        </nav>
    }
}

#[component]
pub fn BreadcrumbList(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "flex flex-wrap gap-1 items-center text-sm break-words sm:gap-2 text-muted-foreground",
            class()
        )
    });

    view! {
        <ol {..attributes} class=class>
            {children()}
        </ol>
    }
}

#[component]
pub fn BreadcrumbItem(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("inline-flex gap-1 items-center", class()));

    view! {
        <li {..attributes} class=class>
            {children()}
        </li>
    }
}

#[component]
pub fn BreadcrumbLink(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(into, optional)] href: &'static str,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("transition-colors hover:text-foreground", class()));

    view! {
        <a {..attributes} class=class href=href>
            {children()}
        </a>
    }
}

#[component]
pub fn BreadcrumbSeparator(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("[&amp;&gt;svg]:size-3.5", class()));

    view! {
        <li {..attributes} class=class role="presentation" aria-hidden="true">
            <ChevronRight class="size-4" />
        </li>
    }
}

#[component]
pub fn BreadcrumbPage(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("font-normal text-foreground", class()));

    view! {
        <span {..attributes} class=class role="link" aria-disabled="true" aria-current="page">
            {children()}
        </span>
    }
}

#[component]
pub fn BreadcrumbEllipsis(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            id="radix-:R3n8qmfnnj7q6ja:"
            aria-haspopup="menu"
            aria-expanded="false"
            data-state="closed"
            class="flex gap-1 items-center"
        >
            <span
                class="flex justify-center items-center size-4"
                role="presentation"
                aria-hidden="true"
            >
                <Ellipsis {..attributes} class=class />
                <span class="sr-only">More</span>
            </span>
            <span class="sr-only">Toggle menu</span>
        </button>
    }
}
