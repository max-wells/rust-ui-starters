use leptos::*;
use tailwind_fuse::*;

use crate::registry::ui::_shared::STYLES;

#[component]
pub fn Grid3(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            STYLES.GRID_START,
            "gap-6 sm:grid-cols-2 lg:grid-cols-3",
            class()
        )
    });

    view! {
        <div {..attributes} class=class>
            {children()}
        </div>
    }
}
