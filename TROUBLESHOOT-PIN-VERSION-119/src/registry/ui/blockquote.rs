use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn Blockquote(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "bg-gray-100 text-gray-500",
            "relative rounded-lg font-sans text-lg italic leading-relaxed",
            "border-l-8 border-l-gray-700",
            "py-5 pl-16 pr-5",
            "before:absolute before:left-3 before:top-3 before:font-serif before:text-6xl before:text-gray-700 before:content-['“']",
            class()
        )
    });

    view! {
        <q {..attributes} class=class>
            {children()}
        </q>
    }
}

#[component]
pub fn BlockquoteAuthor(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "mt-5 pr-4 text-right font-bold not-italic text-gray-700",
            class()
        )
    });

    view! {
        <p {..attributes} class=class>
            {children()}
        </p>
    }
}
