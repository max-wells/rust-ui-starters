use leptos::*;
use tailwind_fuse::*;

use crate::registry::ui::_shared::STYLES;

#[allow(unused_variables)]
#[component]
pub fn Input(
    #[prop(optional, into)] class: String,
    #[prop(optional, into, default = "text")] r#type: &'static str,
    #[prop(optional_no_strip)] value: Option<ReadSignal<String>>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] min: Option<String>,
    #[prop(optional)] step: Option<String>,
    #[prop(optional)] max: Option<String>,
    #[prop(optional)] autofocus: bool,
    #[prop(optional)] node_ref: NodeRef<html::Input>,
) -> impl IntoView {
    let class = tw_merge!(
        STYLES.PLACEHOLDER_MUTED_FOREGROUND,
        STYLES.FILE_STYLES,
        STYLES.DISABLED_NOT_ALLOWED,
        STYLES.FOCUS_VISIBLE_RING,
        STYLES.RING_OFFSET_BG,
        STYLES.BORDER_INPUT,
        STYLES.FLEX_WIDTH_FULL,
        "h-10 rounded-md bg-background px-3 py-2 text-sm",
        class
    );

    view! {
        <input
            type=r#type
            class=class
            name=name
            id=id
            placeholder=placeholder
            value=value
            min=min
            step=step
            max=max
            node_ref=node_ref
            autofocus=autofocus
        />
    }
}
