use leptos::*;
use leptos_use::docs::BooleanDisplay;
use leptos_use::use_prefers_reduced_motion;

#[component]
pub fn DemoUsePrefersReducedMotion() -> impl IntoView {
    let is_reduced_motion_preferred = use_prefers_reduced_motion();

    view! {
        <div>
            <p>Prefers reduced motions: <BooleanDisplay value=is_reduced_motion_preferred /></p>
            <p>
                Update reduce motion preference
                <a
                    class="text-blue-700"
                    href="https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-motion#user_preferences"
                >
                    documentation.
                </a>
            </p>
        </div>
    }
}
