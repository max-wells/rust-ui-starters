use leptos::*;
use ev::SubmitEvent;
use html::Input;
use std::rc::Rc;
use validator::{Validate, ValidationErrors};

use crate::api::api_blogs::add_blog;
use crate::components::toaster_custom::{toast::ToastVariant, types::Toasts};
use crate::components::ui::{button::Button, headings::H2, input::Input};
use crate::models::model_blogs::{ AllBlogsQKey, NewBlog};
use crate::models::form_validator_example::FormValidatorExample;
use crate::utils::hooks::queries::queries_blogs::useAllBlogs;
use crate::utils::toast_utils::{handle_error_toast, show_toast};



#[component]
pub fn FormBlogsValidate() -> impl IntoView {
    let all_server_blogs_query = Rc::new(useAllBlogs());
    let toast_context = expect_context::<Toasts>();
    
    let (title, set_title) = create_signal("TITLE".to_string());
    let (content, set_content) = create_signal("CONTENT".to_string());
    let (author, set_author) = create_signal("AUTHOR".to_string());
    let (image_url, set_image_url) = create_signal("IMAGE_URL".to_string());
    let (tags, set_tags) = create_signal(vec![1, 2]);

    let (errors, set_errors) = create_signal(None::<ValidationErrors>);
    let (is_valid, set_is_valid) = create_signal(true);

    let input_title: NodeRef<Input> = create_node_ref();
    let input_content: NodeRef<Input> = create_node_ref();
    let input_image_url: NodeRef<Input> = create_node_ref();
    let input_author: NodeRef<Input> = create_node_ref();

    // TODO. Improve this with a function
    let handle_success = {
        let all_server_blogs_query = Rc::clone(&all_server_blogs_query);
        move || {
            show_toast(toast_context, ToastVariant::Success, "✅ Blog added");

            let _ = all_server_blogs_query.invalidate_query(AllBlogsQKey);
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
        // let all_server_blogs_query = Rc::clone(&all_server_blogs_query);
        move |ev: SubmitEvent| {
            ev.prevent_default();
            validate_form();

            if !is_valid.get() { // Add this block
                return;
            }

            let value_input_title = input_title().expect("<input> should be mounted").value();
            let value_input_content = input_content().expect("<input> should be mounted").value();
            let value_input_author = input_author().expect("<input> should be mounted").value();
            let value_input_image_url = input_image_url().expect("<input> should be mounted").value();
            
            set_title(value_input_title.clone());
            set_content(value_input_content.clone());
            set_author(value_input_author.clone());

            let new_server_blog = NewBlog {
                title: value_input_title.clone(),
                content: value_input_content.clone(),
                author: value_input_author.clone(),
                image_url: value_input_image_url.clone(),
                tags: tags.get().to_vec(),
            };

            // Call the async function to add the blog to the server
            wasm_bindgen_futures::spawn_local({
                let handle_success = handle_success.clone();
                // let toast_context = toast_context;
                async move {
                    match add_blog(new_server_blog).await {
                        Ok(_server_blog) => handle_success(),
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

