use leptos::*;
use validator::{Validate, ValidationErrors};

use crate::{
    models::models_forms::form_validator_example::FormValidatorExample,
    registry::ui::{button::Button, headings::H1},
};

#[component]
pub fn FormValidator() -> impl IntoView {
    let (title, set_title) = create_signal("FORM".to_string());
    let (author, set_author) = create_signal("VALIDATOR".to_string());
    let (errors, set_errors) = create_signal(None::<ValidationErrors>);

    let input_title: NodeRef<html::Input> = create_node_ref();
    let input_author: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let value_input_title = input_title().expect("<input> should be mounted").value();
        let value_input_author = input_author().expect("<input> should be mounted").value();

        let form_validator_example = FormValidatorExample {
            title: value_input_title.clone(),
            author: value_input_author.clone(),
        };

        match form_validator_example.validate() {
            Ok(_) => {
                set_title(value_input_title);
                set_author(value_input_author);
                set_errors(None);
            }
            Err(e) => {
                set_errors(Some(e));
            }
        }
    };

    view! {
        <div class="flex flex-col gap-4 p-4 border border-sky-500 max-w-[600px]">

            <H1>"Form Validator"</H1>

            <form on:submit=on_submit>
                <input
                    type="text"
                    value=move || title.get()
                    on:input=move |ev| set_title(event_target_value(&ev))
                    node_ref=input_title
                />
                <input
                    type="text"
                    value=move || author.get()
                    on:input=move |ev| set_author(event_target_value(&ev))
                    node_ref=input_author
                />
                <Button r#type="submit">"Submit"</Button>
            </form>

            <div>
                <p>"Title is: " {move || title.get()}</p>
                <p>"Author is: " {move || author.get()}</p>
                {move || {
                    if let Some(errors) = errors.get() {
                        view! {
                            <div class="p-2 text-red-500 border border-red-500">
                                <p>"Validation Errors: "</p>
                                <ul>
                                    {errors
                                        .field_errors()
                                        .iter()
                                        .map(|(field, field_errors)| {
                                            view! {
                                                <li>{format!("{}: {:?}", field, field_errors)}</li>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </ul>
                            </div>
                        }
                            .into_view()
                    } else {
                        view! {
                            <div class="p-2 text-green-500 border border-green-500">
                                "No errors"
                            </div>
                        }
                            .into_view()
                    }
                }}
            </div>
        </div>
    }
}
