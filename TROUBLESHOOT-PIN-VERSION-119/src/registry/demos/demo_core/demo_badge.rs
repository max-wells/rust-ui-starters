use leptos::*;

use crate::registry::ui::badge::Badge;

#[component]
pub fn DemoBadge() -> impl IntoView {
    view! { <Badge>"Default"</Badge> }
}
