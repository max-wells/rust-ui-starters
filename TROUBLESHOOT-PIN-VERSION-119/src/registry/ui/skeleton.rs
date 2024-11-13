use leptos::*;
use tailwind_fuse::*;

// TODO UI. Skeleton should be able to receive children (or not).

#[component]
pub fn Skeleton(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("animate-pulse rounded-md bg-muted", class()));

    view! { <div {..attributes} class=class /> }
}
