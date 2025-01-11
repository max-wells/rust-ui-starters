use leptos::*;
use ev::SubmitEvent;
use html::Input;
use std::rc::Rc;

use crate::api::api_books::add_book;
use crate::components::toaster_custom::toast::ToastVariant;
use crate::components::toaster_custom::types::Toasts;
use crate::components::ui::button::Button;
use crate::components::ui::headings::H2;
use crate::components::ui::input::Input;
use crate::models::model_books::{ AllBooksTag, NewBook};
use crate::utils::hooks::queries::queries_books::useAllBooks;
use crate::utils::toast_utils::{handle_error_toast, show_toast};



#[component]
pub fn FormBooksToast() -> impl IntoView {
    let all_books_query = Rc::new(useAllBooks());
    let toast_context = expect_context::<Toasts>();
    
    let (title, set_title) = create_signal("TITLE".to_string());
    let (author, set_author) = create_signal("AUTHOR".to_string());

    let input_title: NodeRef<Input> = create_node_ref();
    let input_author: NodeRef<Input> = create_node_ref();

    // TODO. Improve this with a function
    let handle_success = {
        let all_books_query = Rc::clone(&all_books_query);
        move || {
            show_toast(toast_context, ToastVariant::Success, "✅ Book added");

            let _ = all_books_query.invalidate_query(AllBooksTag);
        }
    };

    let on_submit = {
        // let all_server_books_query = Rc::clone(&all_server_books_query);
        move |ev: SubmitEvent| {
            ev.prevent_default();

            let value_input_title = input_title().expect("<input> should be mounted").value();
            let value_input_author = input_author().expect("<input> should be mounted").value();

            set_title(value_input_title.clone());
            set_author(value_input_author.clone());

            let new_book = NewBook {
                title: value_input_title.clone(),
                author: value_input_author.clone(),
            };

            // Call the async function to add the book to the server
            wasm_bindgen_futures::spawn_local({
                let handle_success = handle_success.clone();
                // let toast_context = toast_context;
                async move {
                    match add_book(new_book).await {
                        Ok(_server_book) => handle_success(),
                        Err(_err) => handle_error_toast(toast_context),
                    }
                }
            });
        }
    };

    view! {
        <form on:submit=on_submit class="flex flex-col gap-2 p-1 border max-w-[320px]">
            <H2>"Simple (Toast)"</H2>

            <Input value=Some(title) node_ref=input_title />
            <Input value=Some(author) node_ref=input_author />
            <Button r#type="submit">"Submit"</Button>
        </form>
    }
}

