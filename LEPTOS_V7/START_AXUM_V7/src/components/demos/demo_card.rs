use leptos::prelude::*;

use crate::components::ui::card::{Card, CardContent, CardFooter, CardHeader, CardTitle};

#[component]
pub fn DemoCard() -> impl IntoView {
    view! {
        <Card class="w-full max-w-lg lg:max-w-2xl">
            <CardHeader>
                <CardTitle class="text-red-500">"Card Title"</CardTitle>
            </CardHeader>
            <CardContent>"Hello"</CardContent>
            <CardFooter>
                <p>"footer"</p>
            </CardFooter>
        </Card>
    }
}
