use leptos::*;

#[cfg(feature = "hydrate")]
use leptos::html::Div;
#[cfg(feature = "hydrate")]
use leptos_use::{use_element_hover_with_options, UseElementHoverOptions};

#[component]
pub fn DemoUseHover() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    let demo_component = {
        let el = create_node_ref::<Div>();

        let is_hovered = use_element_hover_with_options(
            el,
            UseElementHoverOptions::default()
                .delay_enter(200)
                .delay_leave(600),
        );

        view! {
            <div class="flex flex-col gap-4">
                <div node_ref=el>
                    {move || if is_hovered.get() { "✅ Thank you!" } else { "Hover me" }}
                </div>

                <p>
                    Delay of enter of <span class="font-bold">200ms</span>and delay of leave of
                    <span class="font-bold">600 ms</span>
                </p>
            </div>
        }
    };

    #[cfg(not(feature = "hydrate"))]
    let demo_component = view! { <div>"Hover functionality is not available on the server."</div> };

    view! { <div>{demo_component}</div> }
}
