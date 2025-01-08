use leptos::prelude::*;
use rustui_merge::*;

use crate::components::ui::_styles::STYLES;

#[component]
pub fn Card(#[prop(into, optional)] class: Signal<String>, children: Children) -> impl IntoView {
    let class =
        Memo::new(move |_| tw_merge!("rounded-lg border bg-card shadow p-4 w-full", class()));

    view! {
        <div class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardHeader(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| tw_merge!("flex flex-col space-y-1.5", class()));

    view! {
        <div  class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitle(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| {
        tw_merge!(
            "text-2xl font-semibold leading-none tracking-tight",
            class()
        )
    });

    view! {
        <h3  class=class>
            {children()}
        </h3>
    }
}

#[component]
pub fn CardDescription(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| tw_merge!("text-sm", STYLES.TEXT_MUTED_FOREGROUND, class()));

    view! {
        <p  class=class>
            {children()}
        </p>
    }
}

#[component]
pub fn CardContent(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| tw_merge!("pt-4", class()));

    view! {
        <div  class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardFooter(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| tw_merge!("mt-4", STYLES.FLEX_ITEMS_CENTER, class()));

    view! {
        <div  class=class>
            {children()}
        </div>
    }
}
