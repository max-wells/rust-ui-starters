use leptos::*;
use ev::SubmitEvent;
use html::Input;
use std::rc::Rc;
use validator::{Validate, ValidationErrors};

use crate::api::api_books::add_book;
use crate::components::toaster_custom::{toast::ToastVariant, types::Toasts};
use crate::components::ui::{button::Button, headings::H2, input::Input};
use crate::models::model_books::{ AllBooksTag, NewBook};
use crate::models::form_validator_example::FormValidatorExample;
use crate::utils::hooks::queries::queries_books::useAllBooks;
use crate::utils::toast_utils::{handle_error_toast, show_toast};



#[component]
pub fn FormBooksValidate() -> impl IntoView {
    let all_server_books_query = Rc::new(useAllBooks());
    let toast_context = expect_context::<Toasts>();
    
    let (title, set_title) = create_signal("TITLE".to_string());
    let (author, set_author) = create_signal("AUTHOR".to_string());
    let (errors, set_errors) = create_signal(None::<ValidationErrors>);
    let (is_valid, set_is_valid) = create_signal(true);

    let input_title: NodeRef<Input> = create_node_ref();
    let input_author: NodeRef<Input> = create_node_ref();

    // TODO. Improve this with a function
    let handle_success = {
        let all_server_books_query = Rc::clone(&all_server_books_query);
        move || {
            show_toast(toast_context, ToastVariant::Success, "✅ Book added");

            let _ = all_server_books_query.invalidate_query(AllBooksTag);
        }
    };

    let validate_form = move || { // Add this block
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

    let on_submit = {
        // let all_server_books_query = Rc::clone(&all_server_books_query);
        move |ev: SubmitEvent| {
            ev.prevent_default();
            validate_form();

            if !is_valid.get() { // Add this block
                return;
            }

            let value_input_title = input_title().expect("<input> should be mounted").value();
            let value_input_author = input_author().expect("<input> should be mounted").value();

            set_title(value_input_title.clone());
            set_author(value_input_author.clone());

            let new_server_book = NewBook {
                title: value_input_title.clone(),
                author: value_input_author.clone(),
            };

            // Call the async function to add the book to the server
            wasm_bindgen_futures::spawn_local({
                let handle_success = handle_success.clone();
                // let toast_context = toast_context;
                async move {
                    match add_book(new_server_book).await {
                        Ok(_server_book) => handle_success(),
                        Err(_err) => handle_error_toast(toast_context),
                    }
                }
            });
        }
    };

    let on_input_title = move |ev| { // Add this block
        set_title(event_target_value(&ev));
        validate_form();
    };

    let on_input_author = move |ev| { // Add this block
        set_author(event_target_value(&ev));
        validate_form();
    };

    let is_submit_disabled = create_memo(move |_| !is_valid.get());

    view! {
        <div class="flex flex-col gap-4 p-2 border border-sky-500">
            <H2>"Validate"</H2>

            <form on:submit=on_submit class="flex flex-col gap-2 p-1 border max-w-[320px]">
                <Input value=Some(title) on:input=on_input_title node_ref=input_title />
                <Input value=Some(author) on:input=on_input_author node_ref=input_author />
                <Button r#type="submit" disabled=is_submit_disabled>
                    "Submit"
                </Button>
            </form>

            // Add this block
            <div>
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

