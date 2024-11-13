use leptos::*;

use crate::registry::ui::{
    animate::{Animate, AnimateVariant},
    headings::H1,
};

#[component]
pub fn DemoHeadingsAnimate() -> impl IntoView {
    view! {
        <Animate
            variant=AnimateVariant::FadeUp
            style="animation-delay: 0.25s; animation-fill-mode: forwards;"
        >
            <H1>"This is a Heading using Animate"</H1>
        </Animate>
    }
}
