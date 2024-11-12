use leptos::*;
use tailwind_fuse::*;

use crate::registry::ui::_shared::STYLES;

#[component]
pub fn TextArea(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let class = create_memo(move |_| {
        tw_merge!(
            STYLES.BORDER_INPUT,
            STYLES.DISABLED_NOT_ALLOWED,
            STYLES.FOCUS_VISIBLE_RING,
            STYLES.PLACEHOLDER_MUTED_FOREGROUND,
            STYLES.RING_OFFSET_BG,
            STYLES.FLEX_WIDTH_FULL,
            "min-h-[80px] rounded-md bg-background px-3 py-2 text-sm",
            class()
        )
    });

    view! {
        <textarea {..attributes} class=class>
            {children()}
        </textarea>
    }
}
