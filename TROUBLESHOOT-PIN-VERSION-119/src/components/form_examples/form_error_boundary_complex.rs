use leptos::*;

use crate::registry::ui::{headings::H1, label::Label};

// Source (improved): https://book.leptos.dev/view/07_errors.html

#[component]
pub fn FormErrorBoundaryComplex() -> impl IntoView {
    let (age, set_age) = create_signal(Ok(0));
    let (fav_number, set_fav_number) = create_signal(Ok(0));

    let on_age_input = move |ev| set_age(event_target_value(&ev).parse::<i32>());
    let on_fav_number_input = move |ev| set_fav_number(event_target_value(&ev).parse::<i32>());

    view! {
        <div class="p-4 border border-sky-500 max-w-[600px]">

            <H1>"Form Error Boundary (Complex)"</H1>

            <div class="flex flex-col gap-6">
                <div class="flex gap-2">
                    <Label>"Age: "</Label>
                    <input class="border border-input" type="number" on:input=on_age_input />
                </div>

                <div class="flex gap-2">
                    <Label>"Fav number: "</Label>
                    <input class="border border-input" type="number" on:input=on_fav_number_input />
                </div>

                // * 💁 The fallback receives a signal containing current errors
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
                    <p>
                        "You entered Age: " <strong>{age}</strong> ", Fav number: "
                        <strong>{fav_number}</strong>
                    </p>
                </ErrorBoundary>
            </div>

        </div>
    }
}
