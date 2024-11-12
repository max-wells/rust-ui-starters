use leptos::*;
use serde_json::json;
use validator::{Validate, ValidationErrors};

use crate::{
    models::models_forms::form_validator_example::FormValidatorExample,
    registry::ui::{button::Button, form::FormField, headings::H1, input::Input, label::Label},
};

#[component]
pub fn FormValidatorDisplayDisable() -> impl IntoView {
    let (title, set_title) = create_signal("".to_string());
    let (author, set_author) = create_signal("AUTHOR".to_string());
    let (errors, set_errors) = create_signal(None::<ValidationErrors>);
    let (is_valid, set_is_valid) = create_signal(true);

    let input_title: NodeRef<html::Input> = create_node_ref();
    let input_author: NodeRef<html::Input> = create_node_ref();

    let validate_form = move || {
        let form_validator_example = FormValidatorExample {
            title: title.get().clone(),
            author: author.get().clone(),
        };

        match form_validator_example.validate() {
            Ok(_) => {
                set_errors(None);
                set_is_valid(true);
            }
            Err(e) => {
                set_errors(Some(e));
                set_is_valid(false);
            }
        }
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        validate_form();

        let form_validator_example = FormValidatorExample {
            title: title.get().clone(),
            author: author.get().clone(),
        };

        if is_valid.get() {
            let json_object = json!(form_validator_example);
            web_sys::window()
                .unwrap()
                .alert_with_message(&json_object.to_string())
                .unwrap();
        }
    };

    let on_input_title = move |ev| {
        set_title(event_target_value(&ev));
        validate_form();
    };

    let on_input_author = move |ev| {
        set_author(event_target_value(&ev));
        validate_form();
    };

    let is_submit_disabled = create_memo(move |_| !is_valid.get());

    view! {
        <div class="flex flex-col gap-4 p-4 border border-sky-500 max-w-[600px]">

            <H1>"Form Validator Display Disable Alert"</H1>

            <form on:submit=on_submit class="flex flex-col gap-2">
                <FormField>
                    <Label>"Title"</Label>
                    <Input value=Some(title) on:input=on_input_title node_ref=input_title />
                </FormField>
                <FormField>
                    <Label>"Author"</Label>
                    <Input value=Some(author) on:input=on_input_author node_ref=input_author />
                </FormField>
                <Button r#type="submit" disabled=is_submit_disabled>
                    "Submit"
                </Button>
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
                                        .flat_map(|(_, field_errors)| {
                                            field_errors
                                                .iter()
                                                .map(|field_error| {
                                                    view! {
                                                        <li>
                                                            {field_error
                                                                .message
                                                                .clone()
                                                                .unwrap_or_else(|| "Invalid input".to_string().into())}
                                                        </li>
                                                    }
                                                })
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
