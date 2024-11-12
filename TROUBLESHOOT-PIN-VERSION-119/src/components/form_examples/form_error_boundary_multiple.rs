use leptos::*;

use crate::registry::ui::{headings::H1, label::Label};

// Source (improved): https://book.leptos.dev/view/07_errors.html

// TODO. Be able to create a reactive input (string) that works (even without error handling for now)

#[component]
pub fn FormErrorBoundaryMultiple() -> impl IntoView {
    let (age, set_age) = create_signal(Ok(0));
    let (fav_number, set_fav_number) = create_signal(Ok(0));
    let (_name, _set_name) = create_signal(String::new());

    let on_age_input = move |ev| {
        let value = event_target_value(&ev).parse::<i32>();
        set_age(value);
    };

    let on_fav_number_input = move |ev| {
        let value = event_target_value(&ev).parse::<i32>();
        set_fav_number(value);
    };

    // let on_name_input = move |ev| {
    //     let value = event_target_value(&ev);
    //     set_name(value);
    // };

    view! {
        <div class="p-4 border border-sky-500 max-w-[600px]">

            <H1>"Form Error Boundary (Multiple)"</H1>

            <div class="flex flex-col gap-6">
                <div class="flex gap-2">
                    <Label>"Age: "</Label>
                    <ErrorBoundary fallback=|errors| {
                        view! {
                            <div class="p-2 text-red-500 border border-red-500">
                                <p>"Not a number! Errors: "</p>
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
                        <input
                            class="border border-input text-neutral-500"
                            type="number"
                            on:input=on_age_input
                        />
                        {move || match age.get() {
                            Ok(value) => {
                                view! { <p>"You entered Age: " <strong>{value}</strong></p> }
                            }
                            Err(ref e) => view! { <p class="text-red-500">{e.to_string()}</p> },
                        }}
                    </ErrorBoundary>
                </div>

                <div class="flex gap-2">
                    <Label>"Fav number: "</Label>
                    <ErrorBoundary fallback=|errors| {
                        view! {
                            <div class="p-2 text-red-500 border border-red-500">
                                <p>"Not a number! Errors: "</p>
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
                        <input
                            class="border border-input text-neutral-500"
                            type="number"
                            on:input=on_fav_number_input
                        />
                        {move || match fav_number.get() {
                            Ok(value) => {
                                view! { <p>"You entered Fav number: " <strong>{value}</strong></p> }
                            }
                            Err(ref e) => view! { <p class="text-red-500">{e.to_string()}</p> },
                        }}
                    </ErrorBoundary>
                </div>

            // <div class="flex gap-2">
            // <Label>"Name: "</Label>
            // <input class="border border-input" type="text" on:input=on_name_input />
            // {move || view! { <p>"You entered Name: " <strong>{name.get()}</strong></p> }}
            // </div>
            </div>

        </div>
    }
}
