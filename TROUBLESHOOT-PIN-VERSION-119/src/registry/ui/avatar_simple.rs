use leptos::*;
use tailwind_fuse::*;

use crate::registry::ui::_shared::STYLES;

#[component]
pub fn AvatarRootSimple(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            "shadow-[0_2px_10px] inline-flex h-[45px] w-[45px] select-none items-center justify-center overflow-hidden rounded-full align-middle",
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
pub fn AvatarImageSimple(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(into)] src: String,
    #[prop(into)] alt: String,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!("rounded-[inherit] object-cover", STYLES.SIZE_FULL, class())
    });

    view! { <img {..attributes} class=class src=src alt=alt /> }
}
