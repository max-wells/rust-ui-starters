use crate::registry::ui::checkbox::Checkbox;
use crate::registry::ui::label::Label;
use leptos::*;

#[component]
pub fn DemoLabel() -> impl IntoView {
    view! {
        <div class="space-y-4">
            <h2 class="text-2xl font-bold">Label Demo</h2>
            <div class="flex items-center space-x-2">
                <Checkbox id="terms" checked=false />
                <Label r#for="terms">Accept terms and conditions</Label>
            </div>
        </div>
    }
}
