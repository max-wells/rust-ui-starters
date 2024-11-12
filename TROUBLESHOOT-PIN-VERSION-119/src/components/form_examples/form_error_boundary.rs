use leptos::*;

use crate::registry::ui::{headings::H1, label::Label};

// Source (improved): https://book.leptos.dev/view/07_errors.html

#[component]
pub fn FormErrorBoundary() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
        <div class="p-4 border border-sky-500 max-w-[600px]">

            <H1>"Form Error Boundary"</H1>

            <div class="flex flex-col gap-6">
                <Label>"Type a number (or something that's not a number!)"</Label>
                <input class="border border-input" type="number" on:input=on_input />
                // the fallback receives a signal containing current errors
                <ErrorBoundary fallback=|errors| {
                    view! {
                        <div class="p-2 text-red-500 border border-red-500">
                            <p>"Not a number! Errors: "</p>
                            // we can render a list of errors as strings, if we'd like
                            <ul>
                                {move || {
                                    errors
                                        .get()
                                        .into_iter()
                                        .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                        .collect_view()
                                }}
                            </ul>
                        </div>
                    }
                }>
                    <p>"You entered " <strong>{value}</strong></p>
                </ErrorBoundary>
            </div>

        </div>
    }
}
