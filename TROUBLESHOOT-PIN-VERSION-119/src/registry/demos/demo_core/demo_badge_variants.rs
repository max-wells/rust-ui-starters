use leptos::*;

use crate::registry::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn DemoBadgeVariants() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-2 items-center w-full">
            <Badge>Default</Badge>
            <Badge variant=BadgeVariant::Secondary>Secondary</Badge>
            <Badge variant=BadgeVariant::Destructive>Destructive</Badge>
            <Badge variant=BadgeVariant::Outline>Outline</Badge>
        </div>
    }
}
