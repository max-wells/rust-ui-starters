use leptos::*;

use crate::registry::icons::_icon_shared::SvgIcon;

#[component]
pub fn Plus(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <SvgIcon {..attributes} class=class>
            <title>"Rust Icons - Plus"</title>

            <path d="M5 12h14" />
            <path d="M12 5v14" />
        </SvgIcon>
    }
}
