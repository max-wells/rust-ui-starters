use leptos::*;

use crate::registry::ui::card::{Card, CardContent, CardFooter, CardHeader, CardTitle};

#[component]
pub fn DemoCard() -> impl IntoView {
    view! {
        <Card class="w-full max-w-lg lg:max-w-2xl overflow-none">
            <CardHeader>
                <CardTitle>Card Title</CardTitle>
            </CardHeader>
            <CardContent>Hello</CardContent>
            <CardFooter class="bg-neutral-200">
                <div>footer</div>
            </CardFooter>
        </Card>
    }
}
