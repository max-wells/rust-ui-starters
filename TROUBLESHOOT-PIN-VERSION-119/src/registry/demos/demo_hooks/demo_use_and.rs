use leptos::*;
use leptos_use::docs::BooleanDisplay;
use leptos_use::math::use_and;

#[component]
pub fn DemoUseAnd() -> impl IntoView {
    let (a, set_a) = create_signal(false);
    let (b, set_b) = create_signal(false);

    let a_and_b = use_and(a, b);

    view! {
        <div class="grid gap-4 py-4 px-6 rounded grid-cols-[100px_auto]">
            <label for_="smooth-scrolling-option" class="text-right opacity-75">
                Input
                <code>A</code>
            </label>
            <span>
                <input
                    id="smooth-scrolling-option"
                    prop:checked=a
                    on:input=move |e| set_a.set(event_target_checked(&e))
                    type="checkbox"
                />
            </span>
            <label for_="smooth-scrolling-option" class="text-right opacity-75">
                Input
                <code>B</code>
            </label>
            <span>
                <input
                    id="smooth-scrolling-option"
                    prop:checked=b
                    on:input=move |e| set_b.set(event_target_checked(&e))
                    type="checkbox"
                />
            </span>
            <span class="text-right opacity-75">Output <code>"A && B"</code></span>
            <BooleanDisplay value=a_and_b />
        </div>
    }
}
