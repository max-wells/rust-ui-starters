use leptos::*;

use crate::registry::{
    icons::others::terminal::Terminal,
    ui::alert::{Alert, AlertDescription, AlertTitle},
};

#[component]
pub fn DemoAlert() -> impl IntoView {
    view! {
        <Alert>
            <Terminal />
            <AlertTitle>"Heads up !"</AlertTitle>
            <AlertDescription>"You can add components to your app using the cli."</AlertDescription>
        </Alert>
    }
}
