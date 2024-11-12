use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn FormField(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| tw_merge!("flex items-center gap-2", class()));

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}
