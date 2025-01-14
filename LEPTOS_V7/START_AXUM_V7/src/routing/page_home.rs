use leptos::prelude::*;

use crate::components::demos::{
    demo_button::DemoButton, demo_card::DemoCard, demo_css_pill_lighthouse::DemoCssPillLighthouse,
    demo_input::DemoInput,
};

#[component]
pub fn PageHome() -> impl IntoView {
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <DemoButton />
        <DemoCard />
        <DemoInput />

        <DemoCssPillLighthouse />
    }
}
