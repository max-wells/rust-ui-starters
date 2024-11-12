use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn Alert(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!("relative p-4 mx-4 w-full rounded-lg border [&amp;&gt;svg+div]:translate-y-[-3px] [&amp;&gt;svg]:absolute [&amp;&gt;svg]:left-4 [&amp;&gt;svg]:top-4 [&amp;&gt;svg]:text-foreground [&amp;&gt;svg~*]:pl-7 bg-background text-foreground", class())
    });

    view! {
        <div {..attributes} class=class role="alert">
            {children()}
        </div>
    }
}

#[component]
pub fn AlertTitle(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class =
        create_memo(move |_| tw_merge!("mb-1 font-medium tracking-tight leading-none", class()));

    view! {
        <h4 {..attributes} class=class>
            {children()}
        </h4>
    }
}

#[component]
pub fn AlertDescription(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("text-sm [&amp;_p]:leading-relaxed", class()));

    view! {
        <p {..attributes} class=class>
            {children()}
        </p>
    }
}
