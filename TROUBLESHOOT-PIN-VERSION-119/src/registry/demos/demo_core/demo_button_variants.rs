use leptos::*;

use crate::registry::ui::button::{Button, ButtonVariant};

#[component]
pub fn DemoButtonVariants() -> impl IntoView {
    view! {
        <div class="flex gap-4">
            <Button>Default</Button>

            <Button variant=ButtonVariant::Secondary>Secondary</Button>
            <Button variant=ButtonVariant::Outline>Outline</Button>
            <Button variant=ButtonVariant::Ghost>Ghost</Button>
            <Button variant=ButtonVariant::Destructive>Destructive</Button>
        </div>
    }
}