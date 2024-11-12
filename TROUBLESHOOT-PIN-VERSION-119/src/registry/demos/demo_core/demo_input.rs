use crate::registry::ui::input::Input;
use leptos::*;

// TODO Fix 🐛 Input type="number" can take "e" as a valid input

#[component]
pub fn DemoInput() -> impl IntoView {
    view! {
        <div class="space-y-4 w-full max-w-lg">
            <h2 class="text-2xl font-bold">Input Demo</h2>

            <Input placeholder="Default input" />
            <Input r#type="email" placeholder="Email input" />
            <Input r#type="password" placeholder="Password input" />
            <Input
                class="border-2 border-purple-500 focus:border-purple-700"
                placeholder="Custom styled input"
            />
            <Input r#type="number" placeholder="Number input" />
        </div>
    }
}
